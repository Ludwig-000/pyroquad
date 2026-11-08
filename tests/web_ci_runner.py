"""Runs tests/consistency_test.py in a headless browser - the CI worker for the
wasm platform.

    python tests/web_ci_runner.py --out consistency-wasm.json [--no-build] [--github] [--warn-only]

Self-contained (run_web.bat is a local, untracked tool):
1. builds the wasm side module with cargo, the same way run_web.bat does
   (needs emsdk: $EMSDK / $EMCC / emcc on PATH, and the wasm32-unknown-emscripten target),
2. packs it with the Python half, serves a test page plus the Pyodide runtime
   (`python web/get_pyodide.py`) and the tests/ folder,
3. opens the page in headless Chromium through Playwright
   (`pip install playwright && python -m playwright install --with-deps chromium`),
4. waits for the test to finish, prints the page log, pulls the fingerprint out
   of the page and evaluates it against the baseline exactly like a native run.

If the test hangs in the browser (an engine call that never returns), whatever
was recorded so far is saved, marked as crashed.
"""

from __future__ import annotations

import argparse
import http.server
import io
import json
import os
import re
import shutil
import subprocess
import sys
import threading
import time
import urllib.parse
import zipfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
WASM = ROOT / "target" / "wasm32-unknown-emscripten" / "release" / "_pyroquad.wasm"
PYSRC = ROOT / "src" / "python" / "pyroquad"
PYODIDE = ROOT / "pyodide"
SCRIPT = "consistency_test.py"
MAX_FILE = 32 * 1024 * 1024
FINISHED = "finished " + SCRIPT

sys.path.insert(0, str(HERE))
import consistency_test as ct  # noqa: E402  (stdlib only at import time)

PAGE = """<!doctype html>
<html><head><meta charset="utf-8"><title>pyroquad consistency (ci)</title>
<style>html,body{margin:0;background:#101014;color:#ddd;font:12px monospace}
#glcanvas{display:block;width:100vw;height:100vh}#log{white-space:pre-wrap}</style></head>
<body><canvas id="glcanvas" tabindex="0"></canvas><pre id="log"></pre>
<script src="/pyodide/pyodide.js"></script>
<script>
(async () => {
  const logEl = document.getElementById("log");
  const log = (m) => { logEl.textContent += m + "\\n"; };
  try {
    log("loading pyodide ...");
    const pyodide = await loadPyodide({ indexURL: "/pyodide/", stdout: log, stderr: log });
    window.pyodide = pyodide;
    log("pyodide " + pyodide.version);
    const pkg = await (await fetch("/pkg.zip")).arrayBuffer();
    pyodide.unpackArchive(pkg, "zip", { extractDir: "/home/pyodide/pkg" });
    pyodide.runPython("import sys; sys.path.insert(0, '/home/pyodide/pkg')");
    const home = "/home/pyodide/mnt/";
    pyodide.FS.mkdirTree(home);
    for (const rel of await (await fetch("/manifest.json")).json()) {
      const data = new Uint8Array(await (await fetch("/files/" + encodeURIComponent(rel))).arrayBuffer());
      pyodide.FS.writeFile(home + rel, data);
    }
    pyodide.runPython(`import os, sys; os.chdir("${home}"); sys.path.insert(0, "${home}")`);
    const src = await (await fetch("/files/__SCRIPT__")).text();
    log("running __SCRIPT__ ...");
    await pyodide.runPythonAsync(src, { filename: "__SCRIPT__" });
    log("finished __SCRIPT__");
  } catch (e) {
    log("FAILED: " + (e && e.message ? e.message : e));
  }
})();
</script></body></html>
""".replace("__SCRIPT__", SCRIPT)


# ---------------------------------------------------------------------------
# build + package (mirrors run_web.bat)
# ---------------------------------------------------------------------------

def find_emcc() -> str | None:
    if os.environ.get("EMCC"):
        return os.environ["EMCC"]
    names = ["emcc.bat", "emcc.exe"] if os.name == "nt" else ["emcc"]
    for root in filter(None, [os.environ.get("EMSDK"), str(ROOT / "emsdk")]):
        for n in names:
            p = Path(root) / "upstream" / "emscripten" / n
            if p.is_file():
                return str(p)
    for n in names:
        if shutil.which(n):
            return shutil.which(n)
    return None


def build() -> None:
    emcc = find_emcc()
    if emcc is None:
        raise RuntimeError("emcc not found (set EMSDK / EMCC or put emcc on PATH)")
    env = dict(os.environ, PYO3_CROSS_PYTHON_VERSION="3.14", EMCC=emcc,
               CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_LINKER=emcc,
               CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_RUSTFLAGS="-C link-arg=-sSIDE_MODULE=2")
    cmd = ["cargo", "build", "--target", "wasm32-unknown-emscripten", "--release", "--features", "abi_314"]
    print(f"emcc: {emcc}\n> {' '.join(cmd)}", flush=True)
    if subprocess.call(cmd, cwd=ROOT, env=env) != 0:
        raise RuntimeError("cargo build for wasm32-unknown-emscripten failed (see the log above)")


def package_zip() -> bytes:
    buf = io.BytesIO()
    with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED, compresslevel=1) as z:
        for path in sorted(PYSRC.rglob("*")):
            if path.is_dir() or "__pycache__" in path.parts or path.suffix in {".pyc", ".pyd", ".so"}:
                continue
            z.write(path, Path("pyroquad") / path.relative_to(PYSRC))
        z.write(WASM, "pyroquad/_pyroquad.so")
    return buf.getvalue()


def test_files() -> list[str]:
    """Flat list of files in tests/ the test may read (big HDRs are skipped)."""
    out = []
    for p in sorted(HERE.iterdir()):
        if p.is_file() and not p.name.startswith(".") and p.suffix not in {".pyc"} and p.stat().st_size <= MAX_FILE:
            out.append(p.name)
    return out


def serve(pkg: bytes, files: list[str], port: int) -> tuple[http.server.ThreadingHTTPServer, str]:
    memory = {"/": (PAGE.encode(), "text/html; charset=utf-8"),
              "/pkg.zip": (pkg, "application/zip"),
              "/manifest.json": (json.dumps(files).encode(), "application/json")}

    class Handler(http.server.SimpleHTTPRequestHandler):
        extensions_map = {**http.server.SimpleHTTPRequestHandler.extensions_map,
                          ".wasm": "application/wasm", ".mjs": "text/javascript", ".js": "text/javascript"}

        def do_GET(self):
            path = urllib.parse.unquote(urllib.parse.urlsplit(self.path).path)
            if path in memory:
                body, ctype = memory[path]
                self.send_response(200)
                self.send_header("Content-Type", ctype)
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                self.wfile.write(body)
                return
            super().do_GET()

        def translate_path(self, path):
            p = urllib.parse.unquote(urllib.parse.urlsplit(path).path)
            for prefix, base in (("/pyodide/", PYODIDE), ("/files/", HERE)):
                if p.startswith(prefix):
                    target = (base / p[len(prefix):]).resolve()
                    if base.resolve() in target.parents:
                        return str(target)
            return str(HERE / "__not_found__")

        def log_message(self, fmt, *args):
            msg = fmt % args
            if " 200 " not in msg:
                print("[http]", msg, flush=True)

    httpd = http.server.ThreadingHTTPServer(("127.0.0.1", port), Handler)
    threading.Thread(target=httpd.serve_forever, daemon=True).start()
    return httpd, f"http://127.0.0.1:{httpd.server_address[1]}/"


# ---------------------------------------------------------------------------
# headless browser
# ---------------------------------------------------------------------------

JS_LOG = "() => document.getElementById('log').textContent"
JS_PY = "(code) => { try { return pyodide.runPython(code); } catch (e) { return 'ERR ' + e; } }"


def run_page(url: str, run_timeout: float) -> tuple[str, str, str | None]:
    """Returns (page log, fingerprint json or '', hang reason or None)."""
    from playwright.sync_api import sync_playwright

    with sync_playwright() as pw:
        flags = [
            "--enable-unsafe-swiftshader",            # WebGL2 without a GPU
            "--use-angle=swiftshader",
            "--ignore-gpu-blocklist",
            "--autoplay-policy=no-user-gesture-required",  # WebAudio without a click
        ]
        # Playwright's headless shell first, then its full Chromium in Chrome's
        # new headless mode, then the system's Google Chrome (preinstalled on
        # GitHub's Ubuntu runners, so this works even if the download failed).
        browser, errors = None, []
        for channel in (None, "chromium", "chrome"):
            try:
                browser = pw.chromium.launch(headless=True, channel=channel, args=flags)
                print(f"browser: {channel or 'chromium headless shell'} {browser.version}", flush=True)
                break
            except Exception as e:
                errors.append(f"{channel or 'headless shell'}: {str(e).splitlines()[0]}")
        if browser is None:
            raise RuntimeError("no Chromium build could be launched - " + " | ".join(errors))
        page = browser.new_page(viewport={"width": 800, "height": 600})
        page.on("pageerror", lambda e: print("[page error]", e, flush=True))
        page.goto(url)
        hang = None
        deadline = time.time() + run_timeout
        last_key, last_change = None, time.time()
        while True:
            log = page.evaluate(JS_LOG)
            # "FAILED: <message>" comes from the page's loader; the test itself
            # prints "FAILED: <n> of ..." and the page then says FINISHED.
            if FINISHED in log or re.search(r"^FAILED: (?!\d)", log, re.M):
                break
            key = page.evaluate(JS_PY, "import sys; str(getattr(sys.modules['__main__'], '_last_key', ''))")
            if key != last_key:
                last_key, last_change = key, time.time()
            if time.time() - last_change > ct.STALL_SECONDS * 2:  # page load + pyodide boot included
                hang = f"hang in the browser: no progress for {ct.STALL_SECONDS * 2}s, last probe started: {key}"
                break
            if time.time() > deadline:
                hang = f"timeout after {run_timeout:.0f}s, last probe started: {key}"
                break
            time.sleep(1)
        log = page.evaluate(JS_LOG)
        result = page.evaluate(JS_PY, "import sys; getattr(sys.modules['__main__'], 'LAST_RESULT_JSON', '')")
        if hang and (not result or str(result).startswith("ERR")):
            result = page.evaluate(
                JS_PY, f"import sys; sys.modules['__main__'].partial_result_json({json.dumps(hang)})")
        browser.close()
    result = "" if not result or str(result).startswith("ERR") else str(result)
    return log, result, hang


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="consistency-wasm.json")
    ap.add_argument("--no-build", action="store_true", help="use the last built module in target/")
    ap.add_argument("--github", action="store_true")
    ap.add_argument("--warn-only", action="store_true")
    ap.add_argument("--port", type=int, default=0)
    ap.add_argument("--run-timeout", type=float, default=1200)
    args = ap.parse_args()
    fail = 0 if args.warn_only else 1
    label = os.environ.get("CONSISTENCY_LABEL", "Browser (wasm)")

    def give_up(what: str, err: object) -> int:
        print(f"{what}: {err}", flush=True)
        if args.github:
            ct.gh_annotation("warning", f"Consistency test did not run on {label}", f"{what}: {err}")
        return fail

    try:
        if not (PYODIDE / "pyodide.js").is_file():
            raise RuntimeError(f"no Pyodide runtime in {PYODIDE} - run: python web/get_pyodide.py")
        if not args.no_build:
            build()
        if not WASM.is_file():
            raise RuntimeError(f"missing {WASM}")
        pkg = package_zip()
        files = test_files()
        print(f"package {len(pkg) / 1e6:.1f} MB, mounting {len(files)} files from {HERE}", flush=True)
        httpd, url = serve(pkg, files, args.port)
    except Exception as e:
        return give_up("could not prepare the browser run", e)

    try:
        log, result, hang = run_page(url, args.run_timeout)
    except Exception as e:
        return give_up("browser run failed", repr(e))
    finally:
        httpd.shutdown()

    if args.github:
        ct.gh_group("Browser page log", log.splitlines())
    else:
        print(log)

    if not result:
        msg = hang or "the page finished without producing a fingerprint (see the page log)"
        print(msg, flush=True)
        if args.github:
            ct.gh_annotation("warning", f"Consistency test crashed on {label}", msg)
        return fail

    data = json.loads(result)
    data["__meta__"]["label"] = label
    ct.write_json(args.out, data)
    print(f"wrote fingerprint ({len(data) - 1} entries) to {args.out}", flush=True)
    n = ct.evaluate(ct.read_json(ct.BASELINE), data, github=args.github)
    return fail if (n or hang) else 0


if __name__ == "__main__":
    raise SystemExit(main())
