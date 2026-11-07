"""Runs tests/consistency_test.py in a headless browser - the CI worker for the
wasm platform.

    python tests/web_ci_runner.py --out consistency-wasm.json [--no-build] [--github] [--warn-only]

1. starts `run_web.bat` (which builds the wasm module unless --no-build, packs
   it with the Python half and serves the test page),
2. opens the page in headless Chromium through Playwright
   (`pip install playwright && python -m playwright install --with-deps chromium`),
3. waits for the test to finish, prints the page log, pulls the fingerprint out
   of the page and evaluates it against the baseline exactly like a native run.

If the test hangs in the browser (an engine call that never returns), whatever
was recorded so far is saved, marked as crashed.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import threading
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(HERE))
import consistency_test as ct  # noqa: E402  (stdlib only at import time)

FINISHED = "finished consistency_test.py"


def start_server(no_build: bool, port: int, build_timeout: float) -> tuple[subprocess.Popen, str]:
    cmd = [sys.executable, "-x", str(ROOT / "run_web.bat"), str(HERE / "consistency_test.py"),
           "--no-open", "--stay", "--port", str(port)]
    if no_build:
        cmd.append("--no-build")
    print(">", " ".join(cmd), flush=True)
    proc = subprocess.Popen(cmd, cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
                            encoding="utf-8", errors="replace", bufsize=1)
    url: list[str] = []

    def pump():
        assert proc.stdout is not None
        for line in proc.stdout:
            print("[run_web]", line.rstrip(), flush=True)
            m = re.search(r"->\s+(http://\S+)", line)
            if m and not url:
                url.append(m.group(1))

    threading.Thread(target=pump, daemon=True).start()
    deadline = time.time() + build_timeout
    while not url:
        if proc.poll() is not None:
            raise RuntimeError(f"run_web.bat exited with code {proc.returncode} before serving the page")
        if time.time() > deadline:
            proc.kill()
            raise RuntimeError("run_web.bat did not start serving in time")
        time.sleep(0.5)
    return proc, url[0]


JS_LOG = "() => { const l = document.getElementById('log'); return l ? l.textContent : ''; }"
JS_PY = """(code) => { try { return pyodide.runPython(code); } catch (e) { return 'ERR ' + e; } }"""


def run_page(url: str, run_timeout: float) -> tuple[str, str, str | None]:
    """Returns (page log, fingerprint json or '', hang reason or None)."""
    from playwright.sync_api import sync_playwright

    with sync_playwright() as pw:
        browser = pw.chromium.launch(headless=True, args=[
            "--enable-unsafe-swiftshader",            # WebGL2 without a GPU
            "--use-angle=swiftshader",
            "--ignore-gpu-blocklist",
            "--autoplay-policy=no-user-gesture-required",  # WebAudio without a click
        ])
        page = browser.new_page(viewport={"width": 800, "height": 600})
        page.on("pageerror", lambda e: print("[page error]", e, flush=True))
        page.goto(url)
        hang = None
        deadline = time.time() + run_timeout
        last_key, last_change = None, time.time()
        while True:
            log = page.evaluate(JS_LOG)
            # "FAILED: <message>" comes from the page's loader; the test itself
            # prints "FAILED: <n> of ..." and then the page says FINISHED.
            if FINISHED in log or re.search(r"^FAILED: (?!\d)", log, re.M):
                break
            key = page.evaluate(JS_PY, "import sys; str(getattr(sys.modules['__main__'], '_last_key', ''))")
            if key != last_key:
                last_key, last_change = key, time.time()
            if time.time() - last_change > ct.STALL_SECONDS:
                hang = f"hang in the browser: no progress for {ct.STALL_SECONDS}s, last probe started: {key}"
                break
            if time.time() > deadline:
                hang = f"timeout after {run_timeout:.0f}s, last probe started: {key}"
                break
            time.sleep(1)
        log = page.evaluate(JS_LOG)
        result = page.evaluate(JS_PY, "import sys; getattr(sys.modules['__main__'], 'LAST_RESULT_JSON', '')")
        if hang and not (result and not str(result).startswith("ERR")):
            result = page.evaluate(
                JS_PY, f"import sys; sys.modules['__main__'].partial_result_json({json.dumps(hang)})")
        browser.close()
    result = "" if not result or str(result).startswith("ERR") else str(result)
    return log, result, hang


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default="consistency-wasm.json")
    ap.add_argument("--no-build", action="store_true")
    ap.add_argument("--github", action="store_true")
    ap.add_argument("--warn-only", action="store_true")
    ap.add_argument("--port", type=int, default=8765)
    ap.add_argument("--build-timeout", type=float, default=3600)
    ap.add_argument("--run-timeout", type=float, default=1200)
    args = ap.parse_args()
    fail = 0 if args.warn_only else 1
    label = os.environ.get("CONSISTENCY_LABEL", "Browser (wasm)")

    try:
        proc, url = start_server(args.no_build, args.port, args.build_timeout)
    except Exception as e:
        print("could not start the web test:", e, flush=True)
        if args.github:
            ct.gh_annotation("warning", f"Consistency test did not run on {label}", str(e))
        return fail
    try:
        log, result, hang = run_page(url, args.run_timeout)
    except Exception as e:
        print("browser run failed:", repr(e), flush=True)
        if args.github:
            ct.gh_annotation("warning", f"Consistency test did not run on {label}", repr(e))
        return fail
    finally:
        proc.terminate()

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
    if label:
        data["__meta__"]["label"] = label
    ct.write_json(args.out, data)
    print(f"wrote fingerprint ({len(data) - 1} entries) to {args.out}", flush=True)
    base = ct.read_json(ct.BASELINE)
    n = ct.evaluate(base, data, github=args.github)
    return fail if (n or hang) else 0


if __name__ == "__main__":
    raise SystemExit(main())
