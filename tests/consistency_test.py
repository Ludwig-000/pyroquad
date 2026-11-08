"""Cross-platform consistency test for pyroquad.

This test does NOT say what the engine *should* do. It records what the engine
*does* - math results, object state, error types, text metrics, decoded images,
audio-format support and rendered pixels - into a fingerprint, and compares that
fingerprint against one recorded earlier (normally on another platform).
Behaviour may change freely between versions; it just has to change the same
way on Windows, Linux, macOS and the browser.

Usage (run from anywhere; paths are resolved relative to this file):

    python tests/consistency_test.py              compare against tests/consistency_baseline.json
    python tests/consistency_test.py --record     (re)write the baseline from this machine
    python tests/consistency_test.py --dump f.json    also write this run's fingerprint to f.json
    python tests/consistency_test.py --images DIR     save every captured frame as PNG into DIR
    python tests/consistency_test.py --compare a.json b.json
                                                  diff two fingerprints offline (no engine needed),
                                                  e.g. one dumped on Windows, one on Linux.

CI (.github/workflows/consistency.yml, every push - warning-only for now):
    --ci            = --warn-only --github, run in a supervised child process so
                    that even a native crash (abort/segfault) becomes a warning
    --warn-only     never exit non-zero; crashes/hangs still write a partial --dump
    --github        log groups + one warning annotation per platform
    --report BASELINE "Label=dump.json" ...
                    markdown overview of all platforms (job summary); no engine needed
    tests/web_ci_runner.py runs the browser half in headless Chromium.

In the browser:  run_web.bat tests/consistency_test.py
(the canvas is resized to the test resolution automatically; the result is printed.
Probes listed in PROCESS_BREAKERS - hangs or aborts on one platform - are not
run there; a placeholder value is recorded instead, which shows up as a difference.)

Workflow when the engine changes on purpose: run with --record on one platform,
commit the baseline, then run the plain compare everywhere else.

How values are compared
-----------------------
Every entry carries its own tolerance, chosen where it is recorded:
  * ints / bools / strings / exception type names: exact
  * floats: |a-b| <= abs + rel*max(|a|,|b|)   (default abs=1e-6, rel=1e-5, i.e. f32 noise)
  * pixels: frames are reduced to robust metrics, so GPU/driver differences in
    anti-aliasing or rasterisation rules do not fail the test, while real
    behaviour changes (wrong colour, shape, position, blending, orientation) do:
      grid   - 16x12 grid of 20x20 px cell averages (RGB), abs tolerance per channel
      probe  - exact pixels at hand-picked points well inside shapes
      bbox   - bounding box of everything that differs from the clear colour
      count  - number of pixels that differ from the clear colour
      sha1   - info only (shows whether two platforms are bit-identical)
Exceptions are recorded as "!TypeName" - an error is behaviour too.
Rust panics are NOT: a panic is never intended, so it is never stored as a
value to compare. It is recorded as {"panic": message} and every panic counts
as a failure in every run - even when the baseline panicked the same way.

Canvas mode (browser vs native)
-------------------------------
In the browser the engine never draws to a native framebuffer: "the screen" is
an offscreen render target that is presented to the <canvas> (src/web/mod.rs).
That surface differs from a native window in ways that are not behaviour:
  * it is always 4x MSAA (natively: Config.sample_count, 1 here), so every
    shape edge is anti-aliased;
  * it is an RGBA texture, so read-back alpha holds blended alpha where a
    native framebuffer reports opaque;
  * its size is the physical canvas size (CSS size x devicePixelRatio).
Whenever the two sides of a comparison come from different surfaces (one side
ran in emscripten, or the MSAA counts differ) the comparer switches to canvas
mode automatically: pixel entries use their canvas tolerances (cabs) - the
coverage count may move by the size of the shape's edge band, grid cells and
probes get a little more room - and entries marked surface-only (alpha,
devicePixelRatio, physical sizes) are skipped. Everything else - colours,
blending, orientation, positions, text metrics, math - stays strict.
High-DPI frames are box-filtered down to logical pixels before fingerprinting.
`--canvas` / `--no-canvas` force the mode on or off; `--msaa 4` renders
natively with 4x MSAA to exercise canvas mode without a browser.
"""

from __future__ import annotations

import hashlib
import json
import math
import os
import struct
import sys
import time
import traceback
import zlib

HERE = os.path.dirname(os.path.abspath(__file__)) if "__file__" in globals() else os.getcwd()
BASELINE = os.path.join(HERE, "consistency_baseline.json")
IS_WEB = sys.platform == "emscripten"

# Test resolution. Everything 2D is drawn in absolute pixels inside this area.
W, H = 320, 240
GX, GY = 16, 12  # grid cells for frame fingerprints (20x20 px each)

FORMAT_VERSION = 1
# --msaa N renders natively with N-sample MSAA: the closest native stand-in for
# the browser canvas, used to check that canvas mode tolerates what it should.
MSAA = int(sys.argv[sys.argv.index("--msaa") + 1]) if "--msaa" in sys.argv else 1
TRACE = "--trace" in sys.argv or os.environ.get("PQ_TRACE") == "1"


# ---------------------------------------------------------------------------
# Value normalisation & recording
# ---------------------------------------------------------------------------

def _f(x: float):
    if math.isnan(x):
        return "nan"
    if math.isinf(x):
        return "inf" if x > 0 else "-inf"
    return x


def norm(v):
    """Turn engine values into plain JSON data."""
    if v is None or isinstance(v, (bool, int, str)):
        return v
    if isinstance(v, float):
        return _f(v)
    if isinstance(v, (bytes, bytearray, memoryview)):
        b = bytes(v)
        return {"len": len(b), "sha1": hashlib.sha1(b).hexdigest()[:16]}
    if isinstance(v, (list, tuple)):
        return [norm(x) for x in v]
    if isinstance(v, (set, frozenset)):
        return sorted(repr(x) for x in v)
    if isinstance(v, dict):
        return {str(k): norm(x) for k, x in v.items()}
    if isinstance(v, BaseException):
        return "!" + type(v).__name__
    n = type(v).__name__
    if n in ("Vec2", "Vec3", "Vec4", "Quat", "BVec2", "BVec3"):
        return [norm(getattr(v, a)) for a in "xyzw" if hasattr(v, a)]
    if n == "Color":
        return [norm(v.r), norm(v.g), norm(v.b), norm(v.a)]
    if n == "Mat4":
        return [norm(getattr(v, a)) for a in ("x_axis", "y_axis", "z_axis", "w_axis")]
    if n == "TextDimensions":
        return [norm(v.width), norm(v.height), norm(v.offset_y)]
    if n == "Image":
        return {"w": v.width, "h": v.height, "bytes": norm(v.bytes)}
    if n == "Texture2D":
        return {"size": norm(v.size())}
    return "<" + n + ">"


def say(msg: str):
    """print() that cannot fail on a non-UTF-8 console (Windows cp1252)."""
    enc = getattr(sys.stdout, "encoding", None) or "utf-8"
    print(msg.encode(enc, "backslashreplace").decode(enc, "replace"), flush=True)


def exc_name(e: BaseException) -> str:
    return "!" + type(e).__name__


def is_panic(e) -> bool:
    """A Rust panic that pyo3 turned into pyo3_runtime.PanicException."""
    return isinstance(e, BaseException) and type(e).__name__ == "PanicException"


def panic_message(e: BaseException) -> str:
    lines = str(e).strip().splitlines()
    return (lines[0] if lines else "panic")[:200]


class Recorder:
    def __init__(self):
        self.data: dict[str, dict] = {}
        self.errors: list[str] = []
        self.panics: list[tuple[str, str]] = []
        self._prefix: list[str] = []
        self.screen_dependent = False  # set while recording camera-based scenes

    def _key(self, key: str) -> str:
        return ".".join(self._prefix + [key])

    def section(self, name: str):
        rec = self

        class _S:
            def __enter__(self):
                rec._prefix.append(name)
                if TRACE or len(rec._prefix) == 1:
                    say("[" + ".".join(rec._prefix) + "]")

            def __exit__(self, et, ev, tb):
                rec._prefix.pop()
                if ev is not None and not isinstance(ev, (KeyboardInterrupt, SystemExit)):
                    # A whole section blew up: record that as behaviour and continue.
                    if is_panic(ev):
                        rec.note_panic(name + ".__section_panic__", ev)
                    else:
                        rec.data[rec._key(name + ".__section_error__")] = {"v": exc_name(ev)}
                    rec.errors.append(f"[{name}] " + "".join(traceback.format_exception_only(et, ev)).strip())
                    return True
                return False

        return _S()

    def __call__(self, key, value, abs=None, rel=None, cabs=None, crel=None, canvas=None, scr=False):
        """Record a value (or the result / exception of calling a thunk).

        abs/rel    tolerance between two runs on the same kind of surface
        cabs/crel  tolerance when the two sides rendered to different surfaces
                   (browser canvas vs native framebuffer) - see "Canvas mode"
        canvas     "skip": the value only describes the surface; ignored in canvas mode
        scr        depends on the screen size (cameras map to the whole screen)
        """
        global _last_key, _last_time
        _last_key, _last_time = self._key(key), time.time()
        stream_write({"at": _last_key})
        if _last_key == os.environ.get("CONSISTENCY_SELFTEST_ABORT_AT"):
            selftest_abort()
        if TRACE:
            say("  .. " + self._key(key))
        breaker = PROCESS_BREAKERS.get(sys.platform, {}).get(self._key(key))
        if breaker:
            value = breaker
        elif callable(value) and not isinstance(value, type):
            probe_window_begin(self._key(key))
            try:
                value = value()
            except (KeyboardInterrupt, SystemExit):
                raise
            except BaseException as e:  # PanicException is a BaseException
                value = e
            finally:
                probe_window_end()
        if is_panic(value):
            self.note_panic(key, value)
            return value
        entry = {"v": norm(value)}
        if abs is not None:
            entry["abs"] = abs
        if rel is not None:
            entry["rel"] = rel
        if cabs is not None:
            entry["cabs"] = cabs
        if crel is not None:
            entry["crel"] = crel
        if canvas is not None:
            entry["canvas"] = canvas
        if scr or self.screen_dependent:
            entry["scr"] = 1
        k = self._key(key)
        if k in self.data:
            raise RuntimeError(f"duplicate key {k}")
        self.data[k] = entry
        stream_write({"k": k, "e": entry})
        return value

    def note_panic(self, key: str, e: BaseException):
        """Panics are never behaviour: kept apart, and always reported."""
        k = self._key(key)
        msg = panic_message(e)
        self.data[k] = {"panic": msg}
        self.panics.append((k, msg))
        stream_write({"k": k, "e": self.data[k]})

    def info(self, key, value):
        """Recorded but never compared (environment, hashes)."""
        self.data[self._key(key)] = {"info": norm(value)}
        stream_write({"k": self._key(key), "e": self.data[self._key(key)]})


rec = Recorder()


def asset(name: str) -> bytes | None:
    """Test assets are read by Python and handed to the engine as bytes, so that
    decoding is tested independently of how each platform resolves paths (the
    engine's own path loading gets dedicated probes)."""
    try:
        with open(os.path.join(HERE, name), "rb") as f:
            return f.read()
    except OSError:
        return None


# Probes that take the whole process down on one platform (a hang that never
# returns, or a native abort), so nothing after them could be tested there.
# On that platform they are not executed but recorded as the value given here,
# which still shows up as a difference - the inconsistency stays visible.
# Remove an entry once the engine is fixed.   {sys.platform: {probe key: value}}
PROCESS_BREAKERS = {
    "emscripten": {
        # WebAudio rejects a 0-frame WAV and the load never reports back (natively: loads)
        "audio.load_bytes.pcm16_zero_frames": "!HangsInBrowser",
        "audio.filedata.pcm16_zero_frames": "!HangsInBrowser",
    },
}

_last_key, _last_time = "<start>", time.time()
STALL_SECONDS = 60


def progress(label: str):
    """Mark progress inside long probes (watchdog / browser diagnostics)."""
    global _last_key, _last_time
    _last_key, _last_time = label, time.time()
    stream_write({"at": label})
    if label == os.environ.get("CONSISTENCY_SELFTEST_ABORT_AT"):
        selftest_abort()


def selftest_abort():
    """Self-test of --ci crash handling: dies like an engine crash does - a native
    library prints its reason to stderr, then calls abort(). The supervisor has to
    find that line again (see crash_cause)."""
    os.write(2, b"*** Terminating app due to uncaught exception 'SelfTestException', "
                b"reason: 'CONSISTENCY_SELFTEST_ABORT_AT reached'\n")
    os.abort()


_stream = None  # --stream PATH: every entry is appended as a JSON line, flushed


def stream_write(obj: dict):
    if _stream is not None:
        _stream.write(json.dumps(obj, ensure_ascii=False) + "\n")
        _stream.flush()


_rust_panics: list[dict] = []  # every "thread ... panicked at" report seen on stderr
_capture = {"on": False, "n": 0, "events": {}, "lock": None}
_MARK = b"\x00PQ-PROBE "
_stderr_log: str | None = None  # --stderr-log PATH (set by supervise): fd 2 goes to this file


def capture_rust_panics():
    """Tee fd 2 through a pipe and pick out Rust's panic reports.

    Rust prints `thread '<name>' (id) panicked at <file:line:col>:` + the message
    for EVERY panic - also for ones on the engine thread that never reach Python
    as an exception. To attribute each report to the probe that caused it, every
    probe is bracketed by begin/end markers written into the same pipe (see
    probe_window_begin/_end): the pipe keeps the order, so a report between a
    probe's markers was printed while that probe ran.

    Under --ci (see supervise) fd 2 goes to a file instead of a pipe. A native crash
    usually prints its reason right before dying (an uncaught Objective-C exception,
    a Rust abort, faulthandler's traceback). In a pipe those last lines die with the
    process before the pump forwards them; in a file they are still there for the
    supervisor to show."""
    if IS_WEB:
        return
    import re
    import threading
    try:
        sys.stderr.flush()
        orig = os.dup(2)
        if _stderr_log:
            r = None
            # O_APPEND: writes from several threads never overwrite each other
            w = os.open(_stderr_log, os.O_WRONLY | os.O_CREAT | os.O_TRUNC | os.O_APPEND
                        | getattr(os, "O_BINARY", 0))
        else:
            r, w = os.pipe()
        os.dup2(w, 2)
        os.close(w)
    except OSError:
        return
    if _stderr_log:
        import faulthandler
        # On SIGABRT / SIGSEGV / access violation: the Python stack of every thread, written
        # straight from the signal handler into fd 2 (= the log), so it survives the crash.
        faulthandler.enable(all_threads=True)
    head = re.compile(rb"thread '([^']*)'(?: \(\d+\))? panicked at (.+?):\s*$")
    _capture["lock"] = threading.Lock()

    def stderr_lines():
        if r is not None:
            with os.fdopen(r, "rb", 0) as src:
                yield from iter(src.readline, b"")
            return
        with open(_stderr_log, "rb") as src:  # follow the file like `tail -f`
            partial = b""
            while True:
                chunk = src.readline()
                if not chunk:
                    time.sleep(0.002)
                    continue
                partial += chunk
                if partial.endswith(b"\n"):
                    yield partial
                    partial = b""

    def pump():
        stack: list[str] = []
        last = "<start>"
        pending = None
        for line in stderr_lines():
            if line.startswith(_MARK):
                kind, n, key = line[len(_MARK):].decode(errors="replace").rstrip("\n").split(" ", 2)
                if kind == "B":
                    stack.append(key)
                elif stack:
                    last = stack.pop()
                with _capture["lock"]:
                    ev = _capture["events"].pop(int(n), None)
                if ev is not None:
                    ev.set()
                continue
            try:
                os.write(orig, line)
            except OSError:
                pass
            m = head.match(line)
            if m:
                pending = {"thread": m.group(1).decode(errors="replace"),
                           "at": m.group(2).decode(errors="replace"),
                           "probe": stack[-1] if stack else None, "after": last}
            elif pending is not None:
                pending["message"] = line.decode(errors="replace").strip()[:200]
                _rust_panics.append(pending)
                pending = None

    threading.Thread(target=pump, daemon=True).start()
    _capture["on"] = True


def _mark(kind: str, key: str = "") -> int:
    with _capture["lock"]:
        _capture["n"] += 1
        n = _capture["n"]
    os.write(2, _MARK + f"{kind} {n} {key}\n".encode("utf-8", "replace"))
    return n


def probe_window_begin(key: str):
    if _capture["on"]:
        _mark("B", key.replace("\n", " "))


def probe_window_end():
    """Wait until the reader has seen everything printed up to now."""
    if not _capture["on"]:
        return
    import threading
    ev = threading.Event()
    with _capture["lock"]:
        n = _capture["n"] + 1
        _capture["events"][n] = ev
    _mark("E")
    ev.wait(2.0)


def merge_rust_panics():
    """Give recorded panics their source location; record the ones Python never saw."""
    probe_window_begin("<end>")
    probe_window_end()
    seen = set()
    for pnc in _rust_panics:
        key, where, msg = pnc["probe"], pnc["at"], pnc.get("message", "")
        if (key, where, msg) in seen:
            continue
        seen.add((key, where, msg))
        entry = rec.data.get(key) if key else None
        if entry is not None and "panic" in entry:
            entry.setdefault("at", where)  # Python saw it: just add where it happened
            continue
        # Never surfaced as a Python exception (e.g. on the engine thread).
        base = f"{key}.__panic__" if key else f"after.{pnc['after']}.__panic__"
        k, i = base, 1
        while k in rec.data:
            i += 1
            k = f"{base}{i}"
        rec.data[k] = {"panic": f"{msg} (thread '{pnc['thread']}', not raised to Python)", "at": where}
        rec.panics.append((k, msg))
        stream_write({"k": k, "e": rec.data[k]})


def start_watchdog():
    """A panic on the engine thread leaves every later call waiting forever.
    Natively, turn such a hang into a failure that names the probe."""
    if IS_WEB:
        return
    import threading

    def watch():
        while True:
            time.sleep(1)
            if time.time() - _last_time > STALL_SECONDS:
                reason = f"hang: no progress for {STALL_SECONDS}s, last probe started: {_last_key}"
                print(f"\nHANG: no progress for {STALL_SECONDS}s, last probe started: {_last_key}\n"
                      "(the engine thread most likely panicked - see the panic message above)", flush=True)
                stacks = macos_native_stacks() if sys.platform == "darwin" else []
                if stacks:
                    gh_group("Native stacks at the hang", stacks) if OPTS["github"] else say("\n".join(stacks))
                report_crash(reason, stacks[:80] or None)
                os._exit(0 if OPTS["warn_only"] else 3)

    threading.Thread(target=watch, daemon=True).start()


def macos_native_stacks(max_lines: int = 300) -> list[str]:
    """Where every thread of this process is stuck, from macOS' `sample` tool.
    On macOS the engine runs on the main thread (on a stack of its own), so a hang
    there shows up only as native frames - Python's tracebacks end at activate_engine."""
    import subprocess
    import tempfile
    path = os.path.join(tempfile.gettempdir(), f"pyroquad-hang-{os.getpid()}.txt")
    cmd = ["/usr/bin/sample", str(os.getpid()), "1", "-mayDie", "-file", path]
    # A python.org build uses the hardened runtime, which keeps even its own user from
    # sampling it; CI runners have password-less sudo.
    for prefix in ([], ["sudo", "-n"]):
        try:
            subprocess.run(prefix + cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=60)
            with open(path, encoding="utf-8", errors="replace") as f:
                text = f.read()
        except (OSError, subprocess.SubprocessError):
            continue
        start = text.find("Call graph:")
        end = text.find("Binary Images:")
        lines = text[start if start >= 0 else 0:end if end >= 0 else None].rstrip().splitlines()
        if len(lines) > max_lines:
            lines = lines[:max_lines] + [f"... {len(lines) - max_lines} more line(s) in {path}"]
        return lines
    return ["(could not sample the process: /usr/bin/sample failed, also with sudo)"]


# ---------------------------------------------------------------------------
# Comparison
# ---------------------------------------------------------------------------

DEF_ABS, DEF_REL = 1e-6, 1e-5


def _num_eq(a, b, ab, rl) -> bool:
    if isinstance(a, str) or isinstance(b, str):
        return a == b
    if isinstance(a, bool) or isinstance(b, bool):
        return a == b
    if isinstance(a, int) and isinstance(b, int) and ab is None and rl is None:
        return a == b
    ab = DEF_ABS if ab is None else ab
    rl = DEF_REL if rl is None else rl
    return abs(a - b) <= ab + rl * max(abs(a), abs(b))


def diff_values(a, b, ab, rl, path=""):
    """Yield (path, a, b) for every mismatch."""
    if isinstance(a, list) and isinstance(b, list):
        if len(a) != len(b):
            yield (path + "[len]", len(a), len(b))
            return
        for i, (x, y) in enumerate(zip(a, b)):
            yield from diff_values(x, y, ab, rl, f"{path}[{i}]")
        return
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b:
                yield (f"{path}.{k}", a.get(k, "<missing>"), b.get(k, "<missing>"))
            else:
                yield from diff_values(a[k], b[k], ab, rl, f"{path}.{k}")
        return
    num = (int, float)
    if isinstance(a, num) and isinstance(b, num) and not isinstance(a, bool) and not isinstance(b, bool):
        if not _num_eq(a, b, ab, rl):
            yield (path, a, b)
        return
    if a != b:
        yield (path, a, b)


def _short(v, n=70):
    s = json.dumps(v) if not isinstance(v, str) else v
    return s if len(s) <= n else s[: n - 3] + "..."


def is_canvas_pair(meta_a: dict, meta_b: dict) -> bool:
    """Frames from two different kinds of surface: the browser's offscreen canvas
    target vs a native framebuffer, or different multisample counts."""
    web = [m.get("platform") == "emscripten" for m in (meta_a, meta_b)]
    msaa = [m.get("msaa", 1) for m in (meta_a, meta_b)]
    return web[0] != web[1] or msaa[0] != msaa[1]


def compare(expected: dict, actual: dict, label_e="baseline", label_a="this run",
            canvas: bool | None = None, only_present: bool = False) -> tuple[int, list[str], list[str]]:
    """Returns (number of failures, report lines, failed keys).
    only_present: compare only keys the actual run recorded (a run that crashed)."""
    failed: list[str] = []
    lines: list[str] = []
    failures = 0
    info_diffs = 0
    skipped = 0
    widened = 0
    if canvas is None:
        canvas = is_canvas_pair(expected.get("__meta__", {}), actual.get("__meta__", {}))
    se = expected.get("__meta__", {}).get("screen")
    sa = actual.get("__meta__", {}).get("screen")
    screen_mismatch = se is not None and sa is not None and se != sa
    unverifiable = 0
    for k in sorted(set(expected) | set(actual)):
        if k == "__meta__":
            continue
        e, a = expected.get(k), actual.get(k)
        if a is not None and "panic" in a:
            # never intended - a failure on every run, whatever the baseline did
            failures += 1
            failed.append(k)
            lines.append(f"PANIC    {k}: {a['panic']}" + (f"  [{a['at']}]" if a.get("at") else ""))
            continue
        if e is not None and "panic" in e:
            continue  # the baseline has no intended value here to compare against
        if a is None and only_present:
            continue
        if e is None or a is None:
            if (e or a).get("info") is not None:
                continue
            failures += 1
            failed.append(k)
            where = label_e if e is None else label_a
            lines.append(f"MISSING  {k}  (not present in {where})")
            continue
        if "info" in e or "info" in a:
            if e.get("info") != a.get("info"):
                info_diffs += 1
            continue
        if canvas and "skip" in (e.get("canvas"), a.get("canvas")):
            skipped += 1
            continue
        if screen_mismatch and (e.get("scr") or a.get("scr")):
            unverifiable += 1
            continue
        ks = ("abs", "cabs") if canvas else ("abs",)
        ab = max([x for t in ks for x in (e.get(t), a.get(t)) if x is not None] + [-1])
        ks = ("rel", "crel") if canvas else ("rel",)
        rl = max([x for t in ks for x in (e.get(t), a.get(t)) if x is not None] + [-1])
        ab = None if ab < 0 else ab
        rl = None if rl < 0 else rl
        if canvas and any(t in x for t in ("cabs", "crel") for x in (e, a)):
            widened += 1
        diffs = list(diff_values(e["v"], a["v"], ab, rl))
        if not diffs:
            continue
        failures += 1
        failed.append(k)
        tol = ""
        if ab is not None or rl is not None:
            tol = f"  (tol abs={ab} rel={rl})"
        if k.endswith(".grid"):
            worst = max(diffs, key=lambda d: abs(d[1] - d[2]) if all(isinstance(x, (int, float)) for x in d[1:]) else 1e9)
            cells = []
            for p, x, y in diffs[:6]:
                try:
                    i = int(p.strip("[]"))
                    c, ch = divmod(i, 3)
                    cy, cx = divmod(c, GX)
                    cells.append(f"cell({cx},{cy}).{'rgb'[ch]} {x}->{y}")
                except ValueError:
                    cells.append(f"{p} {x}->{y}")
            lines.append(f"DIFF     {k}: {len(diffs)} channel(s) off, worst {worst[1]}->{worst[2]}{tol}\n           " + ", ".join(cells))
        elif len(diffs) == 1 and diffs[0][0] == "":
            lines.append(f"DIFF     {k}: {label_e}={_short(diffs[0][1])}  {label_a}={_short(diffs[0][2])}{tol}")
        else:
            body = "; ".join(f"{p}: {_short(x, 30)} -> {_short(y, 30)}" for p, x, y in diffs[:5])
            more = f" (+{len(diffs) - 5} more)" if len(diffs) > 5 else ""
            lines.append(f"DIFF     {k}: {body}{more}{tol}")
    if unverifiable:
        failures += 1
        failed.append("<screen size>")
        lines.append(f"SCREEN   the runs used different screen sizes ({se[0]:g}x{se[1]:g} vs {sa[0]:g}x{sa[1]:g}): "
                     f"{unverifiable} camera/screen-dependent entries could not be compared")
    if canvas:
        lines.append(f"(canvas) canvas mode: {widened} pixel entries compared with canvas tolerances, "
                     f"{skipped} surface-only entries skipped")
    if info_diffs:
        lines.append(f"(info)   {info_diffs} informational entries differ (e.g. frames not bit-identical) - not a failure")
    return failures, lines, failed


# ---------------------------------------------------------------------------
# Small binary helpers (pure python; also work inside Pyodide)
# ---------------------------------------------------------------------------

def png_bytes(w: int, h: int, rgba: bytes, color_type: int = 6) -> bytes:
    """Encode raw pixels as PNG. color_type 6 = RGBA, 2 = RGB, 0 = grey (input is always RGBA)."""
    ch = {6: 4, 2: 3, 0: 1}[color_type]
    raw = bytearray()
    for y in range(h):
        raw.append(0)
        row = rgba[y * w * 4:(y + 1) * w * 4]
        if ch == 4:
            raw += row
        else:
            for x in range(w):
                px = row[x * 4:x * 4 + 4]
                raw += px[:3] if ch == 3 else px[:1]

    def chunk(t, d):
        return struct.pack(">I", len(d)) + t + d + struct.pack(">I", zlib.crc32(t + d) & 0xFFFFFFFF)

    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, color_type, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(bytes(raw), 9)) + chunk(b"IEND", b""))


def wav_bytes(frames: int, rate=44100, channels=1, bits=16, fmt=1, freq=440.0, amp=0.5) -> bytes:
    """A sine WAV. fmt 1 = integer PCM, 3 = IEEE float."""
    data = bytearray()
    for i in range(frames):
        s = amp * math.sin(2 * math.pi * freq * i / rate)
        for _ in range(channels):
            if fmt == 3:
                data += struct.pack("<f", s)
            elif bits == 8:
                data.append(int(round(s * 127)) + 128)
            elif bits == 16:
                data += struct.pack("<h", int(round(s * 32767)))
            elif bits == 24:
                data += int(round(s * 8388607)).to_bytes(3, "little", signed=True)
            elif bits == 32:
                data += struct.pack("<i", int(round(s * 2147483647)))
    block = channels * bits // 8
    fmt_chunk = struct.pack("<HHIIHH", fmt, channels, rate, rate * block, block, bits)
    return (b"RIFF" + struct.pack("<I", 4 + 8 + len(fmt_chunk) + 8 + len(data)) + b"WAVE"
            + b"fmt " + struct.pack("<I", len(fmt_chunk)) + fmt_chunk
            + b"data" + struct.pack("<I", len(data)) + bytes(data))


def checker_rgba(w, h, a=(255, 0, 0, 255), b=(0, 0, 255, 255), cell=1) -> bytes:
    out = bytearray()
    for y in range(h):
        for x in range(w):
            out += bytes(a if ((x // cell) + (y // cell)) % 2 == 0 else b)
    return bytes(out)


def gradient_rgba(w, h) -> bytes:
    out = bytearray()
    for y in range(h):
        for x in range(w):
            out += bytes((x * 255 // max(1, w - 1), y * 255 // max(1, h - 1), 128, 255))
    return bytes(out)


# ---------------------------------------------------------------------------
# Frame capture & fingerprinting
# ---------------------------------------------------------------------------

IMAGES_DIR: str | None = None
_screen_info: dict = {}


class Shot:
    """The top-left WxH area of the screen, top-down RGBA.

    get_screen_data() hands rows over bottom-up (glReadPixels order) natively.
    That is assumed for every platform on purpose - if a backend returned
    top-down rows, every frame would come out mirrored and the test would say so.
    """

    def __init__(self, img, scale: float = 1.0):
        """`scale` = physical pixels per logical pixel (a high-DPI canvas). Such
        frames are box-filtered back down to logical pixels first."""
        iw, ih, raw = img.width, img.height, bytes(img.bytes)
        self.src_w, self.src_h = iw, ih
        if abs(scale - 1.0) < 1e-3:
            rows = []
            for y in range(H):
                sy = ih - 1 - y
                if 0 <= sy < ih:
                    row = raw[sy * iw * 4: sy * iw * 4 + min(W, iw) * 4]
                else:
                    row = b""
                rows.append(row + b"\x00" * (W * 4 - len(row)))
            self.data = b"".join(rows)
            return
        out = bytearray(W * H * 4)
        for y in range(H):
            y0, y1 = int(y * scale), max(int(y * scale) + 1, int((y + 1) * scale))
            for x in range(W):
                x0, x1 = int(x * scale), max(int(x * scale) + 1, int((x + 1) * scale))
                acc = [0, 0, 0, 0]
                n = 0
                for py in range(y0, min(y1, ih)):
                    base = (ih - 1 - py) * iw * 4
                    for px in range(x0, min(x1, iw)):
                        i = base + px * 4
                        acc[0] += raw[i]
                        acc[1] += raw[i + 1]
                        acc[2] += raw[i + 2]
                        acc[3] += raw[i + 3]
                        n += 1
                if n:
                    o = (y * W + x) * 4
                    out[o:o + 4] = bytes(int(round(c / n)) for c in acc)
        self.data = bytes(out)

    def px(self, x, y):
        i = (y * W + x) * 4
        return list(self.data[i:i + 4])

    def grid(self):
        cw, chh = W // GX, H // GY
        sums = [0] * (GX * GY * 3)
        d = self.data
        for y in range(GY * chh):
            base = y * W * 4
            cy = y // chh
            for cx in range(GX):
                s = base + cx * cw * 4
                e = s + cw * 4
                o = (cy * GX + cx) * 3
                sums[o] += sum(d[s:e:4])
                sums[o + 1] += sum(d[s + 1:e:4])
                sums[o + 2] += sum(d[s + 2:e:4])
        n = cw * chh
        return [int(round(v / n)) for v in sums]

    def coverage(self, bg, thresh=40):
        """(count, bbox, edges) of pixels that differ from the clear colour.

        `edges` counts covered pixels with an uncovered 4-neighbour - the band in
        which anti-aliasing (the web canvas is always 4x MSAA) moves pixels across
        the threshold. Canvas mode lets `count` move by that much."""
        bgb = bytes(bg)
        bgrow = bgb * W
        d = self.data
        mask = bytearray(W * H)
        count = 0
        x0, y0, x1, y1 = W, H, -1, -1
        for y in range(H):
            row = d[y * W * 4:(y + 1) * W * 4]
            if row == bgrow:
                continue
            for x in range(W):
                i = x * 4
                if (abs(row[i] - bgb[0]) > thresh or abs(row[i + 1] - bgb[1]) > thresh
                        or abs(row[i + 2] - bgb[2]) > thresh):
                    mask[y * W + x] = 1
                    count += 1
                    if x < x0: x0 = x
                    if x > x1: x1 = x
                    if y < y0: y0 = y
                    if y > y1: y1 = y
        edges = 0
        for y in range(max(y0, 0), y1 + 1):
            o = y * W
            for x in range(max(x0, 0), x1 + 1):
                if mask[o + x] and (x == 0 or x == W - 1 or y == 0 or y == H - 1 or not mask[o + x - 1]
                                    or not mask[o + x + 1] or not mask[o + x - W] or not mask[o + x + W]):
                    edges += 1
        return count, ([x0, y0, x1, y1] if count else None), edges


def _bg_rgba(color) -> list[int]:
    return [int(round(c * 255)) for c in (color.r, color.g, color.b, color.a)]


def capture(key, bg=None, probes=None, grid_tol=10, probe_tol=3, count_rel=0.04, count_abs=40,
            bbox_tol=2):
    """Read back the screen and record its fingerprint under `key`."""
    global _last_key, _last_time
    _last_key, _last_time = key + " (capture)", time.time()
    from pyroquad import get_screen_data, screen_width
    img = get_screen_data()
    # physical vs logical pixels: only ever != 1 on a high-DPI canvas
    sw = screen_width()
    scale = img.width / sw if sw > 0 else 1.0
    shot = Shot(img, scale)
    _screen_info[key] = (shot.src_w, shot.src_h)
    # Canvas tolerances (cabs): MSAA preserves coverage, so cell averages and
    # interior pixels barely move - only the anti-aliased edge band does.
    rec(key + ".grid", shot.grid(), abs=grid_tol, cabs=grid_tol + 6)
    for name, (x, y) in (probes or {}).items():
        rec(f"{key}.probe.{name}", shot.px(x, y)[:3], abs=probe_tol, cabs=probe_tol + 2)
    if bg is not None:
        count, bbox, edges = shot.coverage(_bg_rgba(bg))
        # measured: native 4x MSAA moves ~0.1 pixel per edge pixel; allow 0.5
        rec(key + ".count", count, abs=count_abs, rel=count_rel, cabs=count_abs + edges // 2)
        rec(key + ".bbox", bbox, abs=bbox_tol, cabs=bbox_tol + 1)
        rec.info(key + ".edges", edges)
    rec.info(key + ".sha1", hashlib.sha1(shot.data).hexdigest())
    if IMAGES_DIR:
        with open(os.path.join(IMAGES_DIR, key.replace("/", "_") + ".png"), "wb") as f:
            f.write(png_bytes(W, H, shot.data))
    return shot


# ===========================================================================
# PROBES
# ===========================================================================

def probe_math(p):
    Vec2, Vec3, Vec4, BVec2, BVec3, Quat, Mat4 = p.Vec2, p.Vec3, p.Vec4, p.BVec2, p.BVec3, p.Quat, p.Mat4

    for V, dims in ((Vec2, 2), (Vec3, 3)):
        nm = V.__name__
        with rec.section(nm):
            mk = lambda *xs: V(*xs[:dims])
            a = mk(3.0, -4.0, 12.0)
            b = mk(0.5, 2.0, -1.25)
            c = mk(-1.5, 0.25, 7.0)
            z = mk(0.0, 0.0, 0.0)
            odd = mk(0.1, 0.2, 0.3)
            big = mk(1e30, -1e-30, 3.4e38)
            mask = BVec2(True, False) if dims == 2 else BVec3(True, False, True)

            for const in ("AXES", "INFINITY", "MAX", "MIN", "NAN", "NEG_INFINITY", "NEG_ONE", "NEG_X",
                          "NEG_Y", "NEG_Z", "ONE", "X", "Y", "Z", "ZERO"):
                if hasattr(V, const):
                    rec("const." + const, lambda: getattr(V, const))
            rec("f32_roundtrip", lambda: odd)
            rec("big", lambda: big)
            rec("str", lambda: str(a))
            rec("str_odd", lambda: str(odd))
            rec("str_nan", lambda: str(V.NAN))
            rec("repr_type", lambda: type(repr(a)).__name__)
            rec("eq_same", lambda: a == mk(3.0, -4.0, 12.0))
            rec("eq_other", lambda: a == b)
            rec("eq_none", lambda: a == None)  # noqa: E711
            rec("nan_eq_nan", lambda: V.NAN == V.NAN)
            rec("hash_equal", lambda: hash(a) == hash(mk(3.0, -4.0, 12.0)))

            ops = {
                "add": lambda: a + b, "sub": lambda: a - b, "mul": lambda: a * b, "div": lambda: a / b,
                "mod": lambda: a % b, "mod_neg": lambda: c % b,
                "add_f": lambda: a + 2, "radd_f": lambda: 2 + a, "sub_f": lambda: a - 2.5,
                "rsub_f": lambda: 2.5 - a, "mul_f": lambda: a * 1.5, "rmul_f": lambda: 1.5 * a,
                "div_f": lambda: a / 3, "rdiv_f": lambda: 3 / a, "mod_f": lambda: a % 2.5,
                "rmod_f": lambda: 7.0 % a, "div_zero": lambda: a / z, "div_zero_f": lambda: a / 0.0,
                "zero_div_zero": lambda: z / z, "neg": lambda: -a, "iadd": lambda: _iadd(a, b),
            }
            for k, fn in ops.items():
                rec("op." + k, fn)

            unary = ["abs", "ceil", "element_product", "element_sum", "exp", "floor", "fract", "fract_gl",
                     "is_finite", "is_nan", "is_nan_mask", "is_negative_bitmask", "is_normalized", "length",
                     "length_recip", "length_squared", "max_element", "min_element", "normalize",
                     "normalize_or_zero", "recip", "round", "signum", "to_array", "trunc", "try_normalize",
                     "to_angle", "truncate", "any_orthogonal_vector"]
            for m in unary:
                if hasattr(V, m):
                    for vn, v in (("a", a), ("c", c), ("z", z), ("odd", odd)):
                        rec(f"{m}.{vn}", lambda: getattr(v, m)())
            if hasattr(V, "any_orthonormal_vector"):
                n = a.normalize()
                rec("any_orthonormal_vector", lambda: n.any_orthonormal_vector())
                rec("any_orthonormal_pair", lambda: n.any_orthonormal_pair())

            binary = ["angle_between", "copysign", "cmpeq", "cmpge", "cmpgt", "cmple", "cmplt", "cmpne",
                      "distance", "distance_squared", "div_euclid", "dot", "dot_into_vec", "max", "min",
                      "midpoint", "project_onto", "reject_from", "rem_euclid", "cross"]
            for m in binary:
                if hasattr(V, m):
                    for pn, (x, y) in (("ab", (a, b)), ("cb", (c, b)), ("az", (a, z))):
                        rec(f"{m}.{pn}", lambda: getattr(x, m)(y))
            bn = b.normalize()
            rec("project_onto_normalized", lambda: a.project_onto_normalized(bn))
            rec("reject_from_normalized", lambda: a.reject_from_normalized(bn))
            rec("abs_diff_eq.true", lambda: a.abs_diff_eq(a + 0.05, 0.1))
            rec("abs_diff_eq.false", lambda: a.abs_diff_eq(b, 0.1))
            rec("clamp", lambda: a.clamp(mk(-1, -1, -1), mk(1, 2, 3)))
            rec("clamp_bad_bounds", lambda: a.clamp(mk(1, 1, 1), mk(-1, -1, -1)))
            rec("clamp_length", lambda: a.clamp_length(1.0, 2.0))
            rec("clamp_length_max", lambda: a.clamp_length_max(2.0))
            rec("clamp_length_min", lambda: b.clamp_length_min(10.0))
            rec("lerp", lambda: a.lerp(b, 0.25))
            rec("lerp_extrapolate", lambda: a.lerp(b, -2.0))
            rec("move_towards", lambda: a.move_towards(b, 1.5))
            rec("move_towards_past", lambda: a.move_towards(b, 1000.0))
            rec("mul_add", lambda: a.mul_add(b, c))
            rec("normalize_or", lambda: z.normalize_or(b))
            rec("powf", lambda: b.powf(2.5))
            rec("powf_neg_base", lambda: c.powf(0.5))
            rec("with_x", lambda: a.with_x(9.0))
            rec("with_y", lambda: a.with_y(9.0))
            if dims == 3:
                rec("with_z", lambda: a.with_z(9.0))
            rec("select", lambda: V.select(mask, a, b))
            rec("splat", lambda: V.splat(2.5))
            rec("from_array", lambda: V.from_array([1.5, -2.5, 3.5][:dims]))
            rec("from_array_wrong_len", lambda: V.from_array([1.0]))
            rec("zero", lambda: V.zero())
            rec("ctor_int", lambda: mk(1, 2, 3))
            rec("ctor_str", lambda: mk("1", 2, 3))
            rec("ctor_huge", lambda: mk(1e300, 0, 0))

    with rec.section("BVec"):
        for B, args in ((BVec2, (True, False)), (BVec3, (True, False, True))):
            nm = B.__name__
            v = B(*args)
            o = B.splat(True)
            rec(nm + ".TRUE", lambda: B.TRUE)
            rec(nm + ".FALSE", lambda: B.FALSE)
            for m in ("_not", "all", "any", "bitmask", "into_bool_array", "into_u32_array"):
                rec(f"{nm}.{m}", lambda: getattr(v, m)())
            for m in ("bitand", "bitor", "bitxor"):
                rec(f"{nm}.{m}", lambda: getattr(v, m)(o))
            rec(nm + ".set", lambda: v.set(1, True))
            rec(nm + ".test", lambda: [v.test(i) for i in range(len(args))])
            rec(nm + ".test_oob", lambda: v.test(7))
            rec(nm + ".str", lambda: str(v))
            rec(nm + ".eq", lambda: v == B(*args))

    with rec.section("Vec4"):
        v = Vec4(1.0, -2.0, 0.1, 4.0)
        rec("v", v)
        rec("str", lambda: str(v))
        rec("splat", lambda: Vec4.splat(0.3))
        rec("ONE", lambda: Vec4.ONE)
        rec("ZERO", lambda: Vec4.ZERO)
        rec("eq", lambda: v == Vec4(1.0, -2.0, 0.1, 4.0))

    with rec.section("Quat"):
        for c in ("IDENTITY", "NAN", "ZERO"):
            rec(c, lambda: getattr(Quat, c))
        q = Quat.from_xyzw(0.1, 0.2, 0.3, 0.9)
        rec("from_xyzw", q)
        rec("from_array", lambda: Quat.from_array([0.0, 0.0, 0.7071068, 0.7071068]))
        rec("from_vec4", lambda: Quat.from_vec4(Vec4(0.5, 0.5, 0.5, 0.5)))
        rec("str", lambda: str(q))
        rec("eq", lambda: q == Quat.from_xyzw(0.1, 0.2, 0.3, 0.9))

    with rec.section("Mat4"):
        m = Mat4(Vec4(1, 2, 3, 4), Vec4(5, 6, 7, 8), Vec4(9, 10, 11, 12), Vec4(13, 14, 15, 16))
        rec("axes", m)
        rec("str", lambda: str(m))
        rec("eq", lambda: m == Mat4(Vec4(1, 2, 3, 4), Vec4(5, 6, 7, 8), Vec4(9, 10, 11, 12), Vec4(13, 14, 15, 16)))

    with rec.section("polar"):
        for x, y in ((1, 0), (0, 1), (-1, -1), (3, 4), (0, 0), (-2, 0.0001)):
            rec(f"c2p({x},{y})", lambda: p.cartesian_to_polar(Vec2(x, y)))
        for rho, th in ((1, 0), (2, math.pi / 3), (5, -1.0), (0, 2), (-1, 1)):
            rec(f"p2c({rho},{th:.4f})", lambda: p.polar_to_cartesian(rho, th))


def _iadd(a, b):
    a += b
    return a


def probe_color(p):
    Color = p.Color
    named = {}
    for n in dir(Color):
        if n.isupper():
            c = getattr(Color, n)
            named[n] = "%02x%02x%02x%02x" % tuple(int(round(x * 255)) for x in (c.r, c.g, c.b, c.a))
    rec("named", named)
    rec("named_count", len(named))
    c = Color(0.1, 0.2, 0.3, 0.4)
    rec("ctor", c)
    rec("default", lambda: Color())
    rec("ctor_out_of_range", lambda: Color(2.0, -1.0, 0.5, 9.0))
    rec("ctor_int", lambda: Color(1, 0, 0, 1))

    def setters():
        c2 = Color(0, 0, 0, 0)
        c2.r, c2.g, c2.b, c2.a = 0.25, 0.5, 0.75, 1.0 / 3.0
        return c2

    rec("setters", setters)
    rec("eq_same_values", lambda: Color(0.5, 0.5, 0.5, 1) == Color(0.5, 0.5, 0.5, 1))
    rec("eq_named_twice", lambda: Color.RED == Color.RED)
    rec("eq_other", lambda: Color.WHITE == Color.WHEAT)
    rec("str_type", lambda: type(str(c)).__name__)


def probe_enums(p):
    for E in (p.KeyCode, p.MouseButton, p.FilterMode, p.Projection, p.Comparison, p.EulerRot, p.UniformType,
              p.TouchPhase):
        out = {}
        for n in dir(E):
            if n.startswith("_"):
                continue
            m = getattr(E, n)
            if type(m) is not E:
                continue
            try:
                iv = int(m)
            except BaseException as e:
                if is_panic(e):
                    rec.note_panic(f"{E.__name__}.{n}.__int__", e)
                iv = exc_name(e)
            out[n] = [repr(m), iv]
        rec(E.__name__, out)
    rec("KeyCode.eq", lambda: p.KeyCode.A == p.KeyCode.A)
    rec("KeyCode.ne", lambda: p.KeyCode.A == p.KeyCode.B)
    rec("KeyCode.hash", lambda: hash(p.KeyCode.A) == hash(p.KeyCode.A))


def probe_config(p):
    c = p.Config()
    props = ("window_title", "window_width", "window_height", "fullscreen", "swap_interval", "sample_count",
             "window_resizable", "stop_python_when_closing_window", "draw_call_vertex_capacity",
             "draw_call_index_capacity", "default_filter_mode", "high_dpi")
    rec("defaults", {k: repr(getattr(c, k)) for k in props})
    c.window_width = 123
    c.swap_interval = 0
    c.high_dpi = False
    rec("set", [c.window_width, c.swap_interval, c.high_dpi])
    rec("negative_width", lambda: p.Config(window_width=-5))
    rec("bad_type", lambda: p.Config(window_title=5))
    rp = p.RenderTargetParams()
    rec("RenderTargetParams.defaults", [rp.sample_count, rp.depth])
    sp = p.PlaySoundParams()
    rec("PlaySoundParams.defaults", [sp.looped, sp.volume])
    rec("PlaySoundParams.f32", lambda: p.PlaySoundParams(True, 0.1).volume)
    for n in ("DYNAMIC", "FIXED"):
        C = getattr(p.Collider, n)
        inst = C()
        rec(f"Collider.{n}.defaults", {k: getattr(inst, k) for k in dir(inst)
                                       if not k.startswith("_") and not callable(getattr(inst, k))})
    rec("Collider.eq", lambda: p.Collider.DYNAMIC() == p.Collider.DYNAMIC())
    rec("Collider.ne", lambda: p.Collider.DYNAMIC() == p.Collider.DYNAMIC(gravity_scale=2.0))
    rec("Collider.NONE.len", lambda: len(p.Collider.NONE()))
    rec("TextDimensions", lambda: p.TextDimensions(1.5, 2.5, 0.1))
    t = p.Touch(3, p.TouchPhase.Moved, p.Vec2(1, 2))
    rec("Touch", lambda: [t.id, repr(t.phase), norm(t.position)])


def probe_environment(p):
    """Things that must agree once a W x H window exists."""
    rec("screen_size", lambda: [p.screen_width(), p.screen_height()])
    rec("dpi_scale", p.screen_dpi_scale, canvas="skip")  # devicePixelRatio in the browser
    rec("is_mobile", p.is_mobile)
    rec("is_browser_touch_detected", p.is_browser_touch_detected)
    rec("is_simulating_mouse_with_touch", p.is_simulating_mouse_with_touch)
    rec("touches", p.touches)
    rec("touches_local", p.touches_local)
    rec("keys_down", p.get_keys_down)
    rec("keys_pressed", p.get_keys_pressed)
    rec("keys_released", p.get_keys_released)
    rec("mouse_buttons_down", p.get_mouse_buttons_down)
    rec("char_pressed_type", lambda: type(p.get_char_pressed()).__name__)
    rec("last_key_type", lambda: type(p.get_last_key_pressed()).__name__)
    rec("mouse_pos_type", lambda: type(p.get_mouse_position()).__name__)
    rec("mouse_local_type", lambda: type(p.get_mouse_position_local()).__name__)
    rec("mouse_wheel_type", lambda: type(p.get_mouse_wheel()).__name__)
    rec("mouse_delta_type", lambda: type(p.get_mouse_delta_position()).__name__)
    rec("mouse_inside_type", lambda: type(p.mouse_inside_window()).__name__)
    rec("fps_type", lambda: type(p.get_fps()).__name__)
    rec("delta_time_type", lambda: type(p.get_delta_time()).__name__)
    rec("delta_time_positive", lambda: p.get_delta_time() > 0)
    rec("quit_requested", p.is_quit_requested)
    rec("clear_input_queue", p.clear_input_queue)
    rec("simulate_mouse_with_touch", lambda: p.simulate_mouse_with_touch(False))
    rec.info("platform", f"{sys.platform} {sys.version.split()[0]}")


def probe_images(p):
    Image, Color, Texture2D = p.Image, p.Color, p.Texture2D

    with rec.section("Image"):
        for name, col in (("half", Color(0.5, 0.5, 0.5, 1.0)), ("thirds", Color(1 / 3, 2 / 3, 0.999, 0.001)),
                          ("over", Color(1.5, -0.5, 0.0, 1.0)), ("odd", Color(0.1, 0.2, 0.3, 0.4))):
            img = Image.gen_image_color(3, 2, col)
            rec(f"gen.{name}.bytes", lambda: list(img.bytes[:4]))
            rec(f"gen.{name}.get_pixel", lambda: img.get_pixel(2, 1))
        rec("empty", lambda: Image.empty())
        rec("gen_zero", lambda: Image.gen_image_color(0, 0, Color.RED))

        img = Image.from_bytes(gradient_rgba(8, 4), 8, 4)
        rec("from_bytes", img)
        rec("from_bytes_wrong_len", lambda: Image.from_bytes([1, 2, 3], 8, 4))
        rec("get_image_data", lambda: [list(x) for x in img.get_image_data()[:3]])
        rec("get_pixel_oob", lambda: img.get_pixel(8, 0))
        rec("set_pixel_oob", lambda: img.set_pixel(0, 4, Color.RED))

        def set_get():
            im = Image.gen_image_color(2, 2, Color.BLACK)
            im.set_pixel(1, 0, Color(0.5, 0.25, 0.999, 0.7))
            return [list(im.bytes), im.get_pixel(1, 0)]

        rec("set_get_pixel", set_get)
        rec("slice", lambda: img.slice(1, 1, 5, 3))
        rec("slice_oob", lambda: img.slice(4, 2, 100, 100))
        rec("slice_inverted", lambda: img.slice(5, 3, 1, 1))

        def flips():
            im = Image.from_bytes(gradient_rgba(4, 3), 4, 3)
            im.flip_horizontal()
            h = bytes(im.bytes)
            im.flip_vertical()
            return [h, im.bytes]

        rec("flip", flips)

        def update():
            im = Image.gen_image_color(2, 1, Color.BLACK)
            im.update([Color(0.2, 0.4, 0.6, 0.8), Color(1, 1, 1, 0)])
            return list(im.bytes)

        rec("update", update)
        rec("update_wrong_len", lambda: Image.gen_image_color(2, 1, Color.BLACK).update([Color.RED]))

        def bytes_setter():
            im = Image.gen_image_color(1, 1, Color.BLACK)
            im.bytes = bytes([1, 2, 3, 4])
            return list(im.bytes)

        rec("bytes_setter", bytes_setter)

        def wh_setter():
            im = Image.gen_image_color(2, 2, Color.BLACK)
            im.width = 4
            im.height = 1
            return [im.width, im.height, len(im.bytes)]

        rec("wh_setter", wh_setter)

    with rec.section("decode"):
        # Lossless formats generated here - must decode bit-exactly everywhere.
        src = gradient_rgba(13, 7)
        for ct in (6, 2, 0):
            data = png_bytes(13, 7, src, ct)
            rec(f"png_ct{ct}", lambda: p.FileData.from_raw_bytes(data).to_Image())
        rec("png_truncated", lambda: p.FileData.from_raw_bytes(png_bytes(13, 7, src)[:60]).to_Image())
        rec("garbage", lambda: p.FileData.from_raw_bytes(b"not an image at all").to_Image())
        rec("empty", lambda: p.FileData.from_raw_bytes(b"").to_Image())
        # Lossy JPEG: decoders may legitimately differ by a few levels.
        jpg_bytes = asset("example_image.jpg")
        if jpg_bytes is not None:
            jpg = p.FileData.from_raw_bytes(jpg_bytes).to_Image()
            rec("jpg.size", [jpg.width, jpg.height])
            ws, hs = jpg.width, jpg.height
            pts = [(ws * i // 5, hs * j // 4) for i in range(1, 5) for j in range(1, 4)]
            rec("jpg.samples", [norm(jpg.get_pixel(x, y)) for x, y in pts], abs=3 / 255)
            b = jpg.bytes
            rec("jpg.mean", [sum(b[i::4 * 97]) / len(b[i::4 * 97]) for i in range(4)], abs=1.0)
            rec.info("jpg.sha1", b)
    with rec.section("paths"):
        # how the engine resolves relative paths (cwd = the test folder)
        rec("Image", lambda: norm(p.Image("example_image.jpg"))["w"])
        rec("load_file", lambda: norm(p.load_file("example_image.jpg").to_Image())["w"])
        rec("Font", lambda: type(p.Font("example_font.ttf")).__name__)
        rec("load_file_font", lambda: type(p.load_file("example_font.ttf").to_font()).__name__)

    with rec.section("Texture2D"):
        img = Image.from_bytes(gradient_rgba(8, 4), 8, 4)
        t = Texture2D(img)
        rec("size", t.size)
        rec("wh", lambda: [t.width(), t.height()])
        rec("roundtrip", lambda: t.get_texture_data())
        rec("roundtrip_equal", lambda: bytes(t.get_texture_data().bytes) == bytes(img.bytes))
        rec("empty", lambda: Texture2D.empty())
        rec("empty_data", lambda: Texture2D.empty().get_texture_data())
        t2 = Texture2D.from_rgba8(2, 2, checker_rgba(2, 2))
        rec("from_rgba8", lambda: t2.get_texture_data())
        # NOT probed: Texture2D.from_rgba8 / update_from_bytes with a wrong byte count - that panics
        # miniquad on the engine thread and leaves the engine dead (every later call hangs).

        def upd():
            tt = Texture2D(Image.gen_image_color(4, 4, Color.BLACK))
            tt.update(Image.gen_image_color(4, 4, Color(0.2, 0.4, 0.6, 1.0)))
            a = tt.get_texture_data()
            tt.update_from_bytes(4, 4, checker_rgba(4, 4))
            b = tt.get_texture_data()
            tt.update_part(Image.gen_image_color(2, 2, Color.WHITE), 1, 1, 2, 2)
            c = tt.get_texture_data()
            return [a, b, c, list(c.bytes)]

        rec("updates", upd)
        rec("set_filter", lambda: t.set_filter(p.FilterMode.Nearest))
        rec("to_texture", lambda: img.to_texture())
        rec("eq_self", lambda: t == t)
        rec("eq_none", lambda: t == None)  # noqa: E711


def probe_text(p):
    font = None
    font_bytes = asset("example_font.ttf")
    if font_bytes is not None:
        font = p.Font.load_ttf_font_from_bytes(p.FileData.from_raw_bytes(font_bytes))
    rec("ascii_list", lambda: p.Font.ascii_character_list())
    rec("latin_list", lambda: p.Font.latin_character_list())
    strings = ["Hello", "Hello, World!", "", " ", "gjpqy", "ÄÖÜß", "Tab\tTab", "line1\nline2", "1234567890",
               "WWWWiiii", "日本"]
    for fname, f in (("default", None), ("ttf", font)):
        if fname == "ttf" and f is None:
            continue
        for s in strings:
            for size, scale in ((20, 1.0), (13, 1.0), (40, 0.5), (64, 2.0)):
                rec(f"measure.{fname}.{s!r}.{size}x{scale}", lambda: p.measure_text(s, f, size, scale), abs=1e-3)
        for s in ("Hello", "Center me", ""):
            for rot in (0.0, 1.0):
                rec(f"center.{fname}.{s!r}.r{rot}", lambda: p.get_text_center(s, f, 30, 1.0, rot), abs=1e-3)
    rec("measure_size0", lambda: p.measure_text("abc", None, 0, 1.0))
    rec("measure_neg_scale", lambda: p.measure_text("abc", None, 20, -1.0))
    rec("font_missing", lambda: p.Font("does_not_exist.ttf"))
    rec("font_garbage", lambda: p.Font.load_ttf_font_from_bytes(p.FileData.from_raw_bytes(b"\x00" * 64)))
    if font is not None:
        rec("populate_cache", lambda: font.populate_font_cache(list("abc"), 20))
        rec("set_filter", lambda: font.set_filter(p.FilterMode.Nearest))
        rec("from_filedata", lambda: type(p.FileData.from_raw_bytes(font_bytes).to_font()).__name__)
    return font


def probe_objects2d(p):
    Rectangle, Circle, Vec2, Color = p.Rectangle, p.Circle, p.Vec2, p.Color
    r = Rectangle()
    rec("rect.default", lambda: [norm(r.position), r.rotation, norm(r.scale), norm(r.color), r.texture,
                                 r.has_tick()])
    rec("rect.repr", lambda: repr(r))
    rec("rect.bounds", lambda: [r.min_x(), r.min_y(), r.max_x(), r.max_y()])
    rr = Rectangle(Vec2(50, 60), 0.5, Vec2(40, 20), Color.RED)
    rec("rect.rotated.bounds", lambda: [rr.min_x(), rr.min_y(), rr.max_x(), rr.max_y()])
    rec("rect.rotated.corners", lambda: [rr.x1, rr.y1, rr.x2, rr.y2])
    fx = Rectangle.from_xy(10, 20, 30, 70, Color.BLUE)
    rec("rect.from_xy", lambda: [norm(fx.position), norm(fx.scale), fx.x1, fx.y1, fx.x2, fx.y2])
    rec("rect.from_xy_inverted", lambda: norm(Rectangle.from_xy(30, 70, 10, 20).scale))

    def corner_setters():
        q = Rectangle.from_xy(0, 0, 10, 10)
        q.x1 = -5
        q.y2 = 25
        a = [norm(q.position), norm(q.scale)]
        q.x2 = -20
        return a + [norm(q.position), norm(q.scale), q.x1, q.x2]

    rec("rect.corner_setters", corner_setters)

    def setters():
        q = Rectangle()
        q.position = Vec2(1.5, 2.5)
        q.rotation = 7.0
        q.scale = Vec2(-3, 4)
        q.color = Color(0.1, 0.2, 0.3, 0.4)
        return [norm(q.position), q.rotation, norm(q.scale), norm(q.color)]

    rec("rect.setters", setters)
    c = Circle(Vec2(100, 100), 0.0, 25.0, Color.GREEN)
    rec("circle.default", lambda: [norm(c.position), c.rotation, c.radius, c.sides, norm(c.color)])
    rec("circle.repr", lambda: repr(c))
    rec("circle.bounds", lambda: [c.min_x(), c.min_y(), c.max_x(), c.max_y()])

    def sides():
        q = Circle(Vec2(0, 0), 0, 5, Color.RED)
        q.sides = 3
        q.radius = -2
        return [q.sides, q.radius, q.min_x(), q.max_x()]

    rec("circle.setters", sides)

    # Collisions: a fixed table of shape pairs.
    shapes = {
        "r0": Rectangle(Vec2(0, 0), 0.0, Vec2(10, 10), Color.RED),
        "r_touch": Rectangle(Vec2(10, 0), 0.0, Vec2(10, 10), Color.RED),
        "r_overlap": Rectangle(Vec2(9.5, 9.5), 0.0, Vec2(10, 10), Color.RED),
        "r_rot": Rectangle(Vec2(12, 0), 0.785398, Vec2(10, 10), Color.RED),
        "r_far": Rectangle(Vec2(500, 500), 0.0, Vec2(1, 1), Color.RED),
        "r_neg": Rectangle(Vec2(5, 5), 0.0, Vec2(-10, -10), Color.RED),
        "c0": Circle(Vec2(5, 5), 0.0, 3.0, Color.RED),
        "c_edge": Circle(Vec2(23, 5), 0.0, 3.0, Color.RED),
        "c_corner": Circle(Vec2(12.5, 12.5), 0.0, 3.0, Color.RED),
        "c_big": Circle(Vec2(0, 0), 0.0, 100.0, Color.RED),
        "c_zero": Circle(Vec2(5, 5), 0.0, 0.0, Color.RED),
    }
    names = list(shapes)
    for a in names:
        for b in names:
            rec(f"collision.{a}~{b}", lambda: shapes[a].collides_with(shapes[b]))
    lst = list(shapes.values())
    rec("collides_with_list", lambda: [names[lst.index(x)] if x in lst else "?"
                                       for x in shapes["r0"].collides_with_list(lst)])
    rec("eq", lambda: [shapes["r0"] == shapes["r0"], shapes["r0"] == shapes["r_touch"], shapes["c0"] == 5])
    rec("hash_stable", lambda: hash(shapes["r0"]) == hash(shapes["r0"]))
    rec("collides_bad_type", lambda: shapes["r0"].collides_with(5))

    # tick(): count calls across frames, and order relative to next_frame.
    calls = []
    t1 = Rectangle()
    t1.tick(lambda o: calls.append(("r", round(o.position.x))))
    t1.position = Vec2(7, 0)
    t2 = Circle(Vec2(1, 1), 0, 1, Color.RED)
    t2.tick(lambda o: (calls.append(("c", o.radius)), setattr(o, "radius", o.radius + 1)))
    rec("tick.has_tick", lambda: [t1.has_tick(), t2.has_tick()])
    for _ in range(3):
        p.next_frame(None)
    t1.remove_tick()
    p.next_frame(None)
    rec("tick.calls", calls)
    rec("tick.after_remove", lambda: [t1.has_tick(), t2.radius])

    def deleted_tick():
        hits = []
        q = Rectangle()
        q.tick(lambda o: hits.append(1))
        p.next_frame(None)
        del q
        import gc
        gc.collect()
        p.next_frame(None)
        p.next_frame(None)
        return len(hits)

    rec("tick.deleted_object_stops", deleted_tick)
    t2.remove_tick()

    def bad_tick():
        q = Rectangle()
        q.tick(lambda: None)  # wrong arity
        try:
            p.next_frame(None)
            return "no error"
        finally:
            q.remove_tick()

    rec("tick.wrong_arity", bad_tick)


def probe_cameras(p):
    Camera2D, Camera3D, Vec2, Vec3 = p.Camera2D, p.Camera3D, p.Vec2, p.Vec3
    c = Camera2D()
    rec("2d.default", lambda: [c.rotation, norm(c.zoom), norm(c.target), norm(c.offset), c.render_target,
                               c.viewport])
    rec("2d.matrix", c.matrix, scr=True)
    d = Camera2D.from_display_rect(0, 0, W, H)
    rec("2d.display_rect", lambda: [d.rotation, norm(d.zoom), norm(d.target), norm(d.offset)])
    rec("2d.display_rect.matrix", d.matrix, scr=True)
    for pt in ((0, 0), (W, H), (17.5, 99.25)):
        rec(f"2d.display_rect.s2w{pt}", lambda: d.screen_to_world(Vec2(*pt)), abs=1e-4, scr=True)
        rec(f"2d.display_rect.w2s{pt}", lambda: d.world_to_screen(Vec2(*pt)), abs=1e-4, scr=True)
    e = Camera2D(rotation=30.0, zoom=Vec2(0.01, -0.02), target=Vec2(50, 40), offset=Vec2(0.1, -0.2))
    rec("2d.custom.matrix", e.matrix, scr=True)
    rec("2d.custom.s2w", lambda: e.screen_to_world(Vec2(100, 50)), abs=1e-3, scr=True)
    rec("2d.custom.w2s", lambda: e.world_to_screen(Vec2(100, 50)), abs=1e-3, scr=True)
    rec("2d.roundtrip", lambda: e.screen_to_world(e.world_to_screen(Vec2(12, 34))), abs=1e-3)

    def setters():
        q = Camera2D()
        q.rotation = 45.0
        q.zoom = Vec2(2, 3)
        q.target = Vec2(4, 5)
        q.offset = Vec2(6, 7)
        q.viewport = (1, 2, 3, 4)
        return [q.rotation, norm(q.zoom), norm(q.target), norm(q.offset), q.viewport]

    rec("2d.setters", setters)
    c3 = Camera3D()
    rec("3d.default", lambda: [norm(c3.position), norm(c3.target), c3.aspect, norm(c3.up), c3.fovy,
                               repr(c3.projection), c3.render_target, c3.viewport, c3.z_near, c3.z_far])
    rec("camera_font_scale", lambda: p.camera_font_scale(20.0), abs=1e-4, scr=True)
    rec("push_pop", lambda: (p.push_camera_state(), p.pop_camera_state()))
    rec("pop_empty", lambda: p.pop_camera_state())


def probe_files(p):
    name = "_consistency_tmp.bin"
    payload = bytes(range(256)) * 3 + b"\x00end"
    try:
        rec("write", lambda: p.write_to_file(p.FileData.from_raw_bytes(payload), name))
        rec("read_back", lambda: p.load_file(name).bytes)
        rec("read_back_equal", lambda: bytes(p.load_file(name).bytes) == payload)
        rec("filedata_ctor", lambda: p.FileData(name).bytes)
        rec("future.result", lambda: p.load_file_future(name).result())
        rec("future.result_nowait_timeout", lambda: p.load_file_future(name).result_nowait_timeout(10.0))
        rec("load_multiple", lambda: {k: len(v.bytes) for k, v in
                                      p.Loading.load_multiple_files({"a": name, "b": name}).items()})
    finally:
        try:
            os.remove(name)
        except OSError:
            pass
    rec("missing", lambda: p.load_file("does/not/exist.bin"))
    rec("missing_future", lambda: p.load_file_future("does/not/exist.bin").result())
    rec("missing_image", lambda: p.Image("does/not/exist.png"))
    rec("bytes_setter", lambda: _bytes_set(p))


def _bytes_set(p):
    f = p.FileData.from_raw_bytes(b"abc")
    f.bytes = b"xyz!"
    return f.bytes


def probe_audio(p):
    """No backend lets us listen to the output, so this maps what the engine's
    audio pipeline accepts (decode + format conversion), how it fails, and that
    playback commands are accepted. Playback uses silent clips / volume 0."""
    Sound, FileData = p.Sound, p.FileData
    variants = {
        "pcm16_mono_44k": wav_bytes(2000),
        "pcm16_stereo_44k": wav_bytes(2000, channels=2),
        "pcm16_mono_22k": wav_bytes(1000, rate=22050),
        "pcm16_mono_8k": wav_bytes(400, rate=8000),
        "pcm16_mono_96k": wav_bytes(4000, rate=96000),
        "pcm16_6ch": wav_bytes(500, channels=6),
        "pcm8_mono": wav_bytes(2000, bits=8),
        "pcm24_mono": wav_bytes(2000, bits=24),
        "pcm32_mono": wav_bytes(2000, bits=32),
        "float32_mono": wav_bytes(2000, bits=32, fmt=3),
        "pcm16_one_frame": wav_bytes(1),
        "pcm16_zero_frames": wav_bytes(0),
        "silent_16": wav_bytes(2000, amp=0.0),
        "truncated_header": wav_bytes(2000)[:30],
        "truncated_data": wav_bytes(2000)[:1000],
        "garbage": b"definitely not audio" * 10,
        "empty": b"",
    }
    for name, data in variants.items():
        rec(f"load_bytes.{name}", lambda: type(Sound.load_sound_from_bytes(list(data))).__name__)
        rec(f"filedata.{name}", lambda: type(FileData.from_raw_bytes(data).to_Sound()).__name__)

    for f in ("sound_example.mp3", "sound_example_wav.wav"):
        data = asset(f)
        if data is not None:
            rec.info(f"{f}.head", data[:64])
            rec(f"bytes.{f}", lambda: type(Sound.load_sound_from_bytes(data)).__name__)
            rec(f"path.load_sound.{f}", lambda: type(Sound.load_sound(f)).__name__)
            rec(f"path.load_file.{f}", lambda: type(p.load_file(f).to_Sound()).__name__)
    rec("load_sound.missing", lambda: Sound.load_sound("does_not_exist.wav"))
    rec("load_sound.wrong_type", lambda: Sound.load_sound_from_bytes("abc"))

    silent = Sound.load_sound_from_bytes(list(variants["silent_16"]))
    rec("play_once", silent.play_sound_once)
    rec("play_params", lambda: silent.play_sound(p.PlaySoundParams(looped=True, volume=0.0)))
    rec("set_volume", lambda: silent.set_sound_volume(0.0))
    rec("set_volume_negative", lambda: silent.set_sound_volume(-1.0))
    rec("set_volume_loud", lambda: silent.set_sound_volume(0.0) or silent.set_sound_volume(10.0))
    p.next_frame(None)
    rec("stop", silent.stop_sound)
    rec("stop_twice", silent.stop_sound)
    rec("play_after_stop", lambda: silent.play_sound(p.PlaySoundParams(volume=0.0)))
    p.next_frame(None)
    silent.stop_sound()
    rec("params_roundtrip", lambda: _params(p))


def _params(p):
    s = p.PlaySoundParams()
    s.looped = True
    s.volume = 0.3
    return [s.looped, s.volume]


# ---------------------------------------------------------------------------
# Rendered scenes
# ---------------------------------------------------------------------------

def run_scenes(p, font):
    Color, Vec2, Vec3 = p.Color, p.Vec2, p.Vec3
    BG = Color(0.1, 0.1, 0.15, 1.0)

    def scene(name, draw, bg=BG, probes=None, screen=False, **tol):
        rec.screen_dependent = screen
        try:
            _scene(name, draw, bg, probes, **tol)
        finally:
            rec.screen_dependent = False

    def _scene(name, draw, bg, probes, **tol):
        p.set_default_camera()
        p.clear_background(bg)
        try:
            draw()
            rec(f"{name}.draw", "ok")
        except (KeyboardInterrupt, SystemExit):
            raise
        except BaseException as e:
            rec(f"{name}.draw", e)
        capture(name, bg, probes, **tol)
        p.next_frame(None)

    # -- basics -------------------------------------------------------------
    def clear_only():
        from pyroquad import get_screen_data
        img = get_screen_data()
        # raw first pixel incl. alpha and the image size, before any cropping
        first = list(bytes(img.bytes)[:4])
        rec("raw_first_pixel_rgb", first[:3], abs=1)
        # A native framebuffer reports opaque alpha; the web canvas target is an
        # RGBA texture holding blended alpha. A surface property, not behaviour.
        rec("raw_first_pixel_alpha", first[3], abs=1, canvas="skip")
        rec.info("raw_size", [img.width, img.height])

    scene("clear", clear_only, bg=Color(0.2, 0.4, 0.6, 1.0), probes={"c": (160, 120), "tl": (0, 0)}, probe_tol=1)

    def orientation():
        p.draw_rectangle(0, 0, 40, 20, Color.RED)       # top-left
        p.draw_rectangle(W - 40, H - 20, 40, 20, Color(0, 1, 0, 1))  # bottom-right

    scene("orientation", orientation, probes={"tl": (5, 5), "br": (W - 5, H - 5), "tr": (W - 5, 5),
                                              "bl": (5, H - 5)}, probe_tol=1)

    def quantize():
        for i in range(8):
            v = i / 7
            p.draw_rectangle(i * 40, 0, 40, 60, Color(v, 0.5, 1 - v, 1.0))
            p.draw_rectangle(i * 40, 60, 40, 60, Color(0.123 * i, 0.333, 0.777, 1.0))
            p.draw_rectangle(i * 40, 120, 40, 60, Color(1, 1, 1, v))      # alpha over BG
            p.draw_rectangle(i * 40, 180, 40, 60, Color(1.0, 0.0, 0.0, 0.5))
            p.draw_rectangle(i * 40, 180, 40, 30, Color(0.0, 0.0, 1.0, (i + 1) / 9))

    scene("quantize", quantize,
          probes={f"r{r}c{i}": (i * 40 + 20, r * 60 + (15 if r == 3 else 30)) for r in range(4) for i in range(8)},
          probe_tol=2)

    def rects():
        p.draw_rectangle(10.5, 10.5, 50, 30, Color.ORANGE)
        p.draw_rectangle(120, 60, -40, -30, Color.CYAN)           # negative size
        p.draw_rectangle(300, 220, 100, 100, Color.YELLOW)         # partially offscreen
        p.draw_rectangle(70, 10, 1, 1, Color.WHITE)
        p.draw_rectangle(80, 10, 0.4, 0.4, Color.WHITE)
        p.draw_rectangle_lines(140, 10, 60, 40, 1, Color.WHITE)
        p.draw_rectangle_lines(210, 10, 60, 40, 3, Color.MAGENTA)
        p.draw_rectangle_lines(10, 100, 80, 60, 12, Color.LIME)
        p.draw_rectangle_lines(110, 100, 20, 20, 30, Color.PINK)   # thickness > size

    scene("rects", rects, probes={"filled": (35, 25), "neg": (100, 45), "offscr": (310, 230),
                                  "thick": (15, 130), "inside_thick": (50, 130), "huge_lines": (120, 110)})

    def lines():
        p.draw_line(10, 10, 310, 10, 1, Color.WHITE)
        p.draw_line(10, 20.5, 310, 20.5, 1, Color.WHITE)
        p.draw_line(10, 40, 310, 40, 2.5, Color.YELLOW)
        p.draw_line(10, 60, 310, 230, 8, Color.RED)
        p.draw_line(300, 60, 20, 230, 3, Color.GREEN)
        p.draw_line(160, 120, 160, 120, 10, Color.BLUE)          # zero length
        p.draw_line(50, 200, 50, 80, 16, Color.ORANGE)           # vertical, bottom to top

    scene("lines", lines, probes={"thick_mid": (160, 145), "vertical": (50, 140)})

    def circles():
        p.draw_circle(40, 40, 30, Color.RED)
        p.draw_circle(100, 40, 3, Color.WHITE)
        p.draw_circle(160.5, 40.5, 20.25, Color.BLUE)
        p.draw_circle_lines(240, 50, 40, 1, Color.WHITE)
        p.draw_circle_lines(240, 50, 25, 6, Color.YELLOW)
        p.draw_ellipse(60, 160, 50, 25, 0, Color.GREEN)
        p.draw_ellipse(170, 160, 50, 25, 30, Color.CYAN)
        p.draw_ellipse_lines(270, 170, 40, 60, 45, 3, Color.ORANGE)

    scene("circles", circles, probes={"c1": (40, 40), "c3": (160, 40), "e1": (60, 160), "e2": (170, 160),
                                      "ring": (240 + 25, 50)})

    def polys():
        for i, sides in enumerate((3, 4, 5, 6, 8, 20)):
            p.draw_poly(30 + i * 50, 35, sides, 22, 0, Color.WHITE)
            p.draw_poly(30 + i * 50, 95, sides, 22, 45, Color.SKY_BLUE if hasattr(Color, "SKY_BLUE") else Color.BLUE)
            p.draw_poly_lines(30 + i * 50, 150, sides, 22, 10 * i, 2 + i, Color.YELLOW)
        p.draw_hexagon(40, 205, 25, 4, False, Color.RED, Color.GREEN)
        p.draw_hexagon(110, 205, 25, 4, True, Color.WHITE, Color.BLUE)
        p.draw_arc(190, 205, 16, 25, 0, 6, 90, Color.ORANGE)
        p.draw_arc(260, 205, 32, 25, 45, 3, 270, Color.MAGENTA)

    scene("polys", polys, probes={f"poly{i}": (30 + i * 50, 35) for i in range(6)} | {
        "hex_fill": (40, 205), "hex_v_fill": (110, 205)})

    def triangles():
        p.draw_triangle(Vec2(10, 10), Vec2(150, 10), Vec2(80, 110), Color.RED)     # clockwise on screen
        p.draw_triangle(Vec2(170, 10), Vec2(240, 110), Vec2(310, 10), Color.GREEN)  # counter-clockwise
        p.draw_triangle(Vec2(10, 130), Vec2(10, 130), Vec2(100, 200), Color.WHITE)  # degenerate
        p.draw_triangle_lines(Vec2(120, 130), Vec2(310, 140), Vec2(200, 230), 4, Color.YELLOW)

    scene("triangles", triangles, probes={"cw": (80, 40), "ccw": (240, 40)})

    def alpha():
        p.draw_rectangle(20, 20, 150, 150, Color(1, 0, 0, 0.5))
        p.draw_rectangle(90, 70, 150, 150, Color(0, 0, 1, 0.5))
        p.draw_circle(200, 80, 60, Color(0, 1, 0, 0.25))
        p.draw_rectangle(250, 150, 60, 60, Color(1, 1, 1, 0.0))   # invisible
        p.draw_rectangle(250, 20, 60, 60, Color(1, 1, 1, 2.0))    # alpha > 1

    scene("alpha", alpha, probes={"red": (40, 40), "both": (130, 120), "blue": (220, 200), "three": (160, 100),
                                  "invisible": (280, 180), "over1": (280, 50)})

    # -- textures -----------------------------------------------------------
    checker = p.Texture2D.from_rgba8(4, 4, checker_rgba(4, 4))
    grad = p.Texture2D(p.Image.from_bytes(gradient_rgba(16, 16), 16, 16))

    def textures():
        p.draw_texture(checker, 5, 5, Color.WHITE)               # 1:1
        checker.draw(15, 5, Color(1, 1, 1, 0.5))
        checker.set_filter(p.FilterMode.Nearest)
        p.Rectangle(Vec2(10, 30), 0.0, Vec2(64, 64), Color.WHITE, checker).draw()
        p.Rectangle(Vec2(90, 30), 0.0, Vec2(64, 64), Color(1, 1, 0, 1), checker).draw()  # tinted
        grad.set_filter(p.FilterMode.Nearest)
        p.Rectangle(Vec2(170, 30), 0.0, Vec2(64, 64), Color.WHITE, grad).draw()
        p.Rectangle(Vec2(250, 30), 0.6, Vec2(50, 50), Color.WHITE, grad).draw()
        p.draw_texture(grad, 10, 120, Color.WHITE)
        p.Circle(Vec2(120, 170), 0.3, 40, Color.WHITE, grad).draw()

    scene("textures", textures, probes={"tex_1to1": (5, 5), "nearest_a": (14, 34), "nearest_b": (30, 34),
                                        "tint_red": (64, 6), "tint_blue": (80, 6), "grad_tl": (172, 32), "grad_br": (231, 91),
                                        "grad_raw": (17, 127)})

    def textures_linear():
        checker.set_filter(p.FilterMode.Linear)
        grad.set_filter(p.FilterMode.Linear)
        p.Rectangle(Vec2(10, 10), 0.0, Vec2(140, 140), Color.WHITE, checker).draw()
        p.Rectangle(Vec2(170, 10), 0.0, Vec2(140, 140), Color.WHITE, grad).draw()
        checker.set_filter(p.FilterMode.Nearest)

    scene("textures_linear", textures_linear, probes={"mid_checker": (80, 80), "mid_grad": (240, 80)}, probe_tol=4)

    png_img = p.FileData.from_raw_bytes(png_bytes(32, 24, gradient_rgba(32, 24))).to_Image()
    png_tex = png_img.to_texture()
    png_tex.set_filter(p.FilterMode.Nearest)

    def decoded_png():
        p.draw_texture(png_tex, 0, 0, Color.WHITE)
        p.Rectangle(Vec2(40, 0), 0.0, Vec2(280, 240), Color.WHITE, png_tex).draw()

    scene("decoded_png", decoded_png, probes={"raw_tl": (0, 0), "raw_br": (31, 23), "big": (180, 120)})

    jpg_bytes = asset("example_image.jpg")
    if jpg_bytes is not None:
        jpg_tex = p.FileData.from_raw_bytes(jpg_bytes).to_Texture2D()

        def jpg():
            p.Rectangle(Vec2(0, 0), 0.0, Vec2(W, H), Color.WHITE, jpg_tex).draw()

        scene("jpg", jpg, grid_tol=12)

    # -- text ---------------------------------------------------------------
    def text_default():
        p.draw_text("Hello pyroquad", 5, 20, Color.WHITE)
        p.draw_text("size 10 - gjpqy", 5, 40, Color.YELLOW, font_size=10)
        p.draw_text("BIG", 5, 100, Color.RED, font_size=60)
        p.draw_text("scaled", 150, 100, Color.GREEN, font_size=20, font_scale=2.0, font_scale_aspect=0.5)
        p.draw_text("rotated", 180, 160, Color.CYAN, font_size=24, rotation=0.6)
        rec("text_default.dims", lambda: p.draw_text("dims?", 5, 230, Color.WHITE, font_size=16), abs=1e-3)
        p.draw_multiline_text("multi\nline\ntext", 100, 150, 16, Color.ORANGE)
        p.draw_multiline_text("spaced\nlines", 240, 200, 14, Color.WHITE, line_distance_factor=2.0)

    scene("text_default", text_default, grid_tol=14, count_rel=0.08, bbox_tol=3)

    if font is not None:
        def text_ttf():
            p.draw_text("TTF font AaBbGg", 5, 30, Color.WHITE, font_size=24, font=font)
            p.draw_text("ÄÖÜ ß éè", 5, 70, Color.YELLOW, font_size=30, font=font)
            p.draw_text("tiny", 5, 90, Color.WHITE, font_size=8, font=font)
            p.draw_text("rot", 200, 150, Color.RED, font_size=40, font=font, rotation=-0.4)
            p.draw_multiline_text("two\nlines", 20, 160, 28, Color.GREEN, font=font)
            rec("text_ttf.dims", lambda: p.draw_text("x", 300, 230, Color.WHITE, font_size=30, font=font), abs=1e-3)

        scene("text_ttf", text_ttf, grid_tol=14, count_rel=0.08, bbox_tol=3)

    # -- 2D objects ---------------------------------------------------------
    def objects2d():
        p.Rectangle(Vec2(10, 10), 0.0, Vec2(60, 40), Color.RED).draw()
        p.Rectangle(Vec2(100, 10), 0.5, Vec2(60, 40), Color.GREEN).draw()
        p.Rectangle(Vec2(200, 10), -1.0, Vec2(-60, 40), Color.BLUE).draw()
        p.Rectangle.from_xy(10, 80, 90, 150, Color.YELLOW).draw()
        p.Circle(Vec2(150, 115), 0.0, 35, Color.ORANGE).draw()
        q = p.Circle(Vec2(250, 115), 0.0, 35, Color.WHITE)
        q.sides = 5
        q.draw()
        shapes = [p.Rectangle(Vec2(10 + i * 30, 180), 0.2 * i, Vec2(20, 20), Color(i / 9, 1 - i / 9, 0.5, 1))
                  for i in range(10)]
        shapes.append(p.Circle(Vec2(300, 220), 0, 10, Color.MAGENTA))
        p.batch_draw_shapes(shapes)

    scene("objects2d", objects2d, probes={"rect": (40, 30), "rot": (130, 30), "from_xy": (50, 115),
                                          "circle": (150, 115), "pentagon": (250, 115), "batch0": (20, 190),
                                          "batch_circle": (300, 220)})

    # -- cameras ------------------------------------------------------------
    def cam_display_rect():
        p.Camera2D.from_display_rect(0, 0, W, H).set_camera()
        p.draw_rectangle(0, 0, 40, 20, Color.RED)
        p.draw_circle(160, 120, 30, Color.WHITE)

    scene("cam_display_rect", cam_display_rect, screen=True, probes={"rect": (10, 10), "rect_flipped": (10, H - 10),
                                                        "circle": (160, 120)})

    def cam_zoom_rot():
        cam = p.Camera2D(rotation=20.0, zoom=Vec2(2 / W * 2, 2 / H * 2), target=Vec2(80, 60))
        cam.set_camera()
        p.draw_rectangle(60, 40, 40, 40, Color.GREEN)
        p.draw_line(0, 0, 160, 120, 2, Color.WHITE)
        p.set_default_camera()
        p.draw_rectangle(0, 0, 20, 20, Color.RED)   # back to pixels

    scene("cam_zoom_rot", cam_zoom_rot, screen=True, probes={"center": (160, 120), "pixel_rect": (10, 10)})

    def cam_push_pop():
        p.Camera2D(zoom=Vec2(0.01, 0.01)).set_camera()
        p.push_camera_state()
        p.set_default_camera()
        p.draw_rectangle(0, 0, 30, 30, Color.RED)
        p.pop_camera_state()
        p.draw_rectangle(-20, -20, 40, 40, Color.BLUE)

    scene("cam_push_pop", cam_push_pop, screen=True, probes={"default": (10, 10), "restored": (160, 120)})

    def render_target_isolation():
        rt = p.render_target(64, 64)
        rec("rt.type", type(rt).__name__)
        cam = p.Camera2D.from_display_rect(0, 0, 64, 64)
        cam.render_target = rt
        rec("rt.camera_has_rt", cam.render_target is not None)
        cam.set_camera()
        p.clear_background(Color.RED)          # must only hit the target
        p.draw_rectangle(0, 0, 64, 64, Color.YELLOW)
        p.set_default_camera()
        p.draw_rectangle(100, 100, 30, 30, Color.GREEN)
        rec("rt.msaa", lambda: type(p.render_target_msaa(32, 32)).__name__)
        rec("rt.depth", lambda: type(p.render_target(32, 32, p.RenderTargetParams(1, True))).__name__)

    scene("render_target", render_target_isolation, probes={"screen_bg": (10, 10), "green": (110, 110)})

    def grab():
        p.draw_rectangle(0, 0, 50, 50, Color.RED)
        p.draw_circle(200, 150, 40, Color.BLUE)
        t = p.Texture2D(p.Image.gen_image_color(W, H, Color.BLACK))
        t.grab_screen()
        img = t.get_texture_data()
        rec("grab_screen.size", [img.width, img.height], canvas="skip")  # physical canvas size
        b = bytes(img.bytes)
        iw = img.width

        def px(x, y):
            i = (y * iw + x) * 4
            return list(b[i:i + 3]) if i + 3 <= len(b) else None

        # raw texture rows, no flipping: records which way up grab_screen stores them
        rec("grab_screen.px_row0", px(10, 10), abs=2)
        rec("grab_screen.px_rowlast", px(10, img.height - 10), abs=2)

    scene("grab_screen", grab, screen=True)

    # nothing drawn and no clear: what does a fresh frame contain?
    p.next_frame(None)
    capture("post_frame", None, {"c": (160, 120)}, probe_tol=1)
    p.next_frame(None)

    # -- 3D -----------------------------------------------------------------
    def cam3d(pos=Vec3(4, 3, 5), target=Vec3(0, 0.5, 0), proj=None, fovy=0.9):
        kw = dict(position=pos, target=target, aspect=W / H, up=Vec3(0, 1, 0), fovy=fovy)
        if proj is not None:
            kw["projection"] = proj
        p.Camera3D(**kw).set_camera()

    def prims3d():
        cam3d()
        p.draw_grid(10, 1.0, Color.RED, Color.GRAY)
        p.draw_cube(Vec3(0, 0.5, 0), Vec3(1, 1, 1), Color.ORANGE, None)
        p.draw_cube_wires(Vec3(2, 0.5, 0), Vec3(1, 1, 1), Vec3(0, 0.5, 0), Color.WHITE)
        p.draw_cube(Vec3(-2, 0.5, 0), Vec3(1, 2, 0.5), Color.WHITE, checker)
        p.draw_plane(Vec3(0, 0.01, 2.5), Vec2(1.5, 1), Color.BLUE, None)
        p.draw_line_3d(Vec3(0, 0, 0), Vec3(0, 3, 0), Color.GREEN)

    scene("3d_prims", prims3d, screen=True, bg=Color(0.05, 0.05, 0.1, 1), grid_tol=12)

    def cyl3d():
        cam3d()
        p.draw_cylinder(Vec3(-1.5, 0, 0), 0.5, 0.8, 1.5, None, Color.YELLOW)
        p.draw_cylinder(Vec3(0.5, 0, 0), 0.0, 0.6, 1.2, None, Color.RED)
        p.draw_cylinder_wires(Vec3(2, 0, -1), 0.4, 0.4, 2.0, None, Color.WHITE)
        p.draw_affine_parallelepiped(Vec3(-1, 0, 2), Vec3(1, 0, 0), Vec3(0.3, 1, 0), Vec3(0, 0, 0.7), None,
                                     Color.GREEN)
        p.draw_affine_parallelogram(Vec3(1, 0.01, 1.5), Vec3(1, 0, 0), Vec3(0, 0, 1), grad, Color.WHITE)

    scene("3d_cylinders_affine", cyl3d, screen=True, bg=Color(0.05, 0.05, 0.1, 1), grid_tol=12)

    def ortho3d():
        cam3d(pos=Vec3(0, 5, 0.001), target=Vec3(0, 0, 0), proj=p.Projection.Orthographics, fovy=6.0)
        p.draw_cube(Vec3(0, 0, 0), Vec3(1, 1, 1), Color.RED, None)
        p.draw_cube(Vec3(2, 0, 1), Vec3(1, 1, 1), Color.GREEN, None)

    scene("3d_ortho", ortho3d, screen=True, bg=Color(0.05, 0.05, 0.1, 1), grid_tol=12)

    def depth3d():
        cam3d(pos=Vec3(0, 0, 5), target=Vec3(0, 0, 0))
        # drawn far-to-near and near-to-far: the depth buffer decides, not the order
        p.draw_cube(Vec3(0, 0, 1), Vec3(1, 1, 1), Color.RED, None)     # near
        p.draw_cube(Vec3(0.4, 0.4, -1), Vec3(2, 2, 1), Color.BLUE, None)  # far, drawn after

    scene("3d_depth", depth3d, screen=True, bg=Color(0.05, 0.05, 0.1, 1), probes={"near_wins": (160, 120)}, grid_tol=12)

    sky = p.Texture2D(p.Image.from_bytes(gradient_rgba(64, 32), 64, 32))

    def skybox():
        cam3d(pos=Vec3(0, 0, 0), target=Vec3(1, 0.2, 0.3))
        p.draw_skybox(sky, Color.WHITE)
        p.draw_cube(Vec3(3, 0.5, 1), Vec3(0.5, 0.5, 0.5), Color.RED, None)

    scene("3d_skybox", skybox, screen=True, grid_tol=14)

    def mixed():
        cam3d()
        p.draw_cube(Vec3(0, 0.5, 0), Vec3(1, 1, 1), Color.ORANGE, None)
        p.set_default_camera()
        p.draw_rectangle(140, 100, 40, 40, Color(0, 1, 1, 0.5))   # 2D over 3D
        p.draw_text("HUD", 5, 20, Color.WHITE)

    scene("3d_then_2d", mixed, screen=True, bg=Color(0.05, 0.05, 0.1, 1), grid_tol=12)

    return checker, grad


OBJ_TETRA = b"""# tetrahedron
v 0 0 0
v 1 0 0
v 0 1 0
v 0 0 1
vt 0 0
vt 1 0
vt 0 1
f 1/1 3/3 2/2
f 1/1 2/2 4/3
f 1/1 4/3 3/2
f 2/1 3/2 4/3
"""


def _objects3d_state(p, classes):
    import gc
    Vec3, Color, Collider = p.Vec3, p.Color, p.Collider
    if True:
        for n, C in classes.items():
            o = C()
            rec(f"{n}.default", lambda: [norm(o.pos), norm(o.rot), norm(o.scale), o.physics])
            rec(f"{n}.str", lambda: str(o))
            rec(f"{n}.repr", lambda: repr(o))
            o2 = C(Vec3(1, 2, 3), Vec3(0.1, 0.2, 0.3), Vec3(2, 1, 0.5), Color.RED, None, Collider.NONE())
            rec(f"{n}.ctor", lambda: [norm(o2.pos), norm(o2.rot), norm(o2.scale)])

            def setters(o2=o2):
                o2.pos = Vec3(-1, 0.5, 9)
                o2.rot = Vec3(7.0, -1.0, 0.25)       # beyond 2*pi on purpose
                o2.scale = Vec3(0, -1, 3)
                return [norm(o2.pos), norm(o2.rot), norm(o2.scale)]

            rec(f"{n}.setters", setters)

            def orbit(C=C):
                q = C(position=Vec3(2, 0, 0))
                q.rotate_around(Vec3(0, 0, 0), Vec3(0, math.pi / 2, 0), True)
                a = [norm(q.pos), norm(q.rot)]
                q.rotate_around(Vec3(1, 1, 1), Vec3(0.3, 0, 0.2), False)
                return a + [norm(q.pos), norm(q.rot)]

            rec(f"{n}.rotate_around", orbit, abs=1e-5)
            rec(f"{n}.eq_hash", lambda: [o == o, o == o2, o == None, hash(o) == hash(o)])  # noqa: E711
            del o, o2
        c = p.Cube.from_xyz(1, 2, 3, 4, 6, 8, Color.BLUE)
        rec("Cube.from_xyz", lambda: [norm(c.pos), norm(c.scale)])
        rec("Cube.from_xyz_inverted", lambda: norm(p.Cube.from_xyz(4, 6, 8, 1, 2, 3).scale))
        del c
        m = p.FileData.from_raw_bytes(OBJ_TETRA).to_mesh_data()
        rec("Mesh.obj", lambda: [norm(m.pos), norm(m.rot), norm(m.scale), str(m)])
        rec("Mesh.garbage", lambda: p.Mesh.from_file_data(p.FileData.from_raw_bytes(b"\xff\xfe garbage")))
        rec("Mesh.empty", lambda: p.Mesh.from_file_data(p.FileData.from_raw_bytes(b"")))
        rec("Mesh.bad_gltf", lambda: p.Mesh.from_file_data(p.FileData.from_raw_bytes(b"glTF\x02\x00\x00\x00")))
        del m
        gc.collect()


def probe_objects3d(p, checker):
    import gc
    Vec3, Color, Collider = p.Vec3, p.Color, p.Collider
    classes = {"Cube": p.Cube, "Sphere": p.Sphere, "Cylinder": p.Cylinder, "Pill": p.Pill}

    with rec.section("state"):
        _objects3d_state(p, classes)
    gc.collect()

    with rec.section("render"):
        rec.screen_dependent = True
        bg = Color(0.05, 0.05, 0.1, 1)
        objs = [
            p.Cube(Vec3(-2.5, 0.5, 0), Vec3(0, 0.6, 0), Vec3(1, 1, 1), Color.ORANGE),
            p.Sphere(Vec3(-0.8, 0.5, 0), Vec3(0, 0, 0), Vec3(1, 1, 1), Color.RED),
            p.Cylinder(Vec3(0.8, 0.5, 0), Vec3(0.3, 0, 0), Vec3(0.8, 1, 0.8), Color.GREEN),
            p.Pill(Vec3(2.5, 0.5, 0), Vec3(0, 0, 0.5), Vec3(0.6, 1, 0.6), Color.YELLOW),
            p.Cube(Vec3(0, -0.05, 0), Vec3(0, 0, 0), Vec3(8, 0.1, 4), Color.GRAY),
            p.Cube(Vec3(0, 1.2, -1.5), Vec3(0.2, 0.3, 0.4), Vec3(0.7, 0.7, 0.7), Color.WHITE, checker),
            p.FileData.from_raw_bytes(OBJ_TETRA).to_mesh_data(),
        ]
        objs[-1].pos = Vec3(-1.5, 1.5, 1)
        hidden = p.Cube(Vec3(0, 3, 0), Vec3(0, 0, 0), Vec3(1, 1, 1), Color.MAGENTA)
        hidden.set_draw_each_frame(False)

        def draw_objs():
            p.Camera3D(position=Vec3(0, 3, 7), target=Vec3(0, 0.5, 0), aspect=W / H, fovy=0.8).set_camera()
            p.draw_all_objects()

        p.set_default_camera()
        p.clear_background(bg)
        draw_objs()
        capture("objects", bg, {"cube": (85, 135)}, grid_tol=12)
        p.next_frame(None)

        # manual draw ignores set_draw_each_frame(False)
        p.clear_background(bg)
        p.Camera3D(position=Vec3(0, 3, 7), target=Vec3(0, 0.5, 0), aspect=W / H, fovy=0.8).set_camera()
        hidden.draw()
        capture("manual_draw_hidden", bg, grid_tol=12)
        p.next_frame(None)

        # deleting objects removes them from draw_all_objects
        del objs[1:4]
        del hidden
        gc.collect()
        p.clear_background(bg)
        draw_objs()
        capture("after_delete", bg, grid_tol=12)
        p.next_frame(None)
        del objs
        gc.collect()

        # nothing left: draw_all_objects draws nothing
        p.clear_background(bg)
        draw_objs()
        capture("empty_world", bg, grid_tol=12)
        p.next_frame(None)
        rec.screen_dependent = False

    with rec.section("collision"):
        a = p.Cube(Vec3(0, 0, 0), collider_type=Collider.FIXED())
        b = p.Cube(Vec3(0.5, 0, 0), collider_type=Collider.FIXED())
        c = p.Sphere(Vec3(10, 0, 0), collider_type=Collider.FIXED())
        d = p.Sphere(Vec3(10.9, 0, 0), collider_type=Collider.DYNAMIC(gravity_scale=0.0))
        s = p.Cube(Vec3(0, 0.6, 0), collider_type=Collider.FIXED(is_sensor=True))
        n = p.Cube(Vec3(0, 0, 0), collider_type=Collider.NONE())
        named = {"a": a, "b": b, "c": c, "d": d, "s": s, "n": n}
        rec("before_frame", lambda: {k: o.does_collide() for k, o in named.items()})
        p.next_frame(0.0)
        p.next_frame(0.0)

        def who(o):
            return sorted(k for k, v in named.items() for x in o.check_collision() if x == v)

        rec("does_collide", lambda: {k: o.does_collide() for k, o in named.items()})
        rec("check_collision", lambda: {k: who(o) for k, o in named.items()})
        rec("physics_present", lambda: {k: o.physics is not None for k, o in named.items()})

        def switch():
            n.set_collider(Collider.FIXED())
            p.next_frame(0.0)
            p.next_frame(0.0)
            r = [n.does_collide(), who(n), n.physics is not None]
            a.set_collider(Collider.NONE())
            p.next_frame(0.0)
            p.next_frame(0.0)
            return r + [a.does_collide(), who(b)]

        rec("set_collider", switch)
        del a, b, c, d, s, n, named
        gc.collect()
        p.next_frame(0.0)

    with rec.section("physics"):
        floor = p.Cube(Vec3(0, -0.5, 0), Vec3(0, 0, 0), Vec3(20, 1, 20), Color.GRAY, None, Collider.FIXED())
        box = p.Cube(Vec3(0, 3, 0), Vec3(0.3, 0.2, 0.1), Vec3(1, 1, 1), Color.RED, None, Collider.DYNAMIC())
        ball = p.Sphere(Vec3(2, 2, 0), Vec3(0, 0, 0), Vec3(1, 1, 1), Color.BLUE, None,
                        Collider.DYNAMIC(restitution=0.9, friction=0.1))
        floaty = p.Cube(Vec3(-3, 2, 0), collider_type=Collider.DYNAMIC(gravity_scale=0.0))
        ph = box.physics
        props = ("get_angular_damping", "get_angular_velocity", "get_density", "get_friction", "get_gravity_scale",
                 "get_linear_damping", "get_linear_velocity", "get_mass", "get_restitution", "get_rotation_locks",
                 "is_ccd_enabled")
        rec("defaults", lambda: {k: norm(getattr(ph, k)()) for k in props}, abs=1e-5)
        rec("ball_defaults", lambda: {k: norm(getattr(ball.physics, k)()) for k in props}, abs=1e-5)

        def setters():
            q = floaty.physics
            q.set_angular_damping(0.5)
            q.set_linear_damping(0.25)
            q.set_density(2.0)
            q.set_friction(0.3)
            q.set_gravity_scale(0.0)
            q.set_restitution(0.1)
            q.enable_ccd(True)
            q.lock_rotation_x_Axes(True)
            q.lock_rotation_z_Axes(True)
            q.set_linear_velocity(Vec3(1, 0, 0))
            q.set_angular_velocity(Vec3(0, 2, 0))
            return {k: norm(getattr(q, k)()) for k in props}

        rec("setters", setters, abs=1e-5)
        progress("physics impulses")
        floaty.physics.apply_impulse(Vec3(0, 0, 1))
        floaty.physics.add_torque_impulse(Vec3(0, 0.5, 0))
        ball.physics.add_force(Vec3(5, 0, 0))
        ball.physics.add_torque(Vec3(0, 0, 1))

        # Fixed time step: positions must follow the same trajectory everywhere.
        # (rapier runs with simd/parallel features, so allow small drift.)
        traj = {"box": [], "ball": [], "floaty": []}
        # 60 frames: the box lands and the ball bounces.
        for i in range(1, 61):
            progress(f"physics frame {i}")
            p.next_frame(1 / 60)
            if i % 10 == 0:
                traj["box"].append(norm(box.pos) + norm(box.rot))
                traj["ball"].append(norm(ball.pos))
                traj["floaty"].append(norm(floaty.pos) + norm(floaty.rot))
        for k, v in traj.items():
            rec(f"trajectory.{k}", v, abs=2e-2, rel=1e-3)
        rec("box_resting_on_floor", lambda: box.does_collide())
        rec("velocity_after", lambda: norm(ball.physics.get_linear_velocity()), abs=5e-2, rel=1e-2)

        def manual_step():
            q = p.Sphere(Vec3(0, 10, 5), collider_type=Collider.DYNAMIC())
            p.next_frame(None)
            y0 = q.pos.y
            p.step_physics(0.5)
            p.next_frame(None)
            return [y0, q.pos.y]

        rec("step_physics", manual_step, abs=1e-3, rel=1e-3)

        def physics_after_delete():
            q = p.Cube(collider_type=Collider.DYNAMIC())
            handle = q.physics
            del q
            gc.collect()
            return handle.get_mass()

        rec("handle_after_delete", physics_after_delete)

        # draw the final physics state
        bg = Color(0.05, 0.05, 0.1, 1)
        p.clear_background(bg)
        p.Camera3D(position=Vec3(0, 3, 9), target=Vec3(0, 0.5, 0), aspect=W / H, fovy=0.8).set_camera()
        p.draw_all_objects()
        rec.screen_dependent = True
        capture("final_frame", bg, grid_tol=14, count_rel=0.06)
        rec.screen_dependent = False
        p.next_frame(None)
        del floor, box, ball, floaty, ph
        gc.collect()
        p.next_frame(None)

    with rec.section("ticks"):
        hits = []
        q = p.Cube()
        q.tick(lambda o: (hits.append(round(o.pos.x, 3)), setattr(o, "pos", o.pos + Vec3(1, 0, 0))))
        for _ in range(4):
            p.next_frame(None)
        q.remove_tick()
        p.next_frame(None)
        rec("calls", hits)
        rec("final_pos", lambda: norm(q.pos))
        del q
        gc.collect()


# ===========================================================================
# Main
# ===========================================================================

def _web_canvas():
    """In the browser, force the canvas to the test resolution."""
    try:
        import js  # type: ignore
        c = js.document.getElementById("glcanvas")
        c.style.width = f"{W}px"
        c.style.height = f"{H}px"
        # miniquad only re-reads the canvas size on a window resize event
        js.window.dispatchEvent(js.Event.new("resize"))
    except Exception as e:  # pragma: no cover
        print("could not resize canvas:", e)


def run_all() -> dict:
    os.chdir(HERE)  # assets are loaded relative to the working directory
    import pyroquad as p

    start_watchdog()
    capture_rust_panics()

    t0 = time.time()
    # Pure / pre-engine behaviour first.
    with rec.section("math"):
        probe_math(p)
    with rec.section("color"):
        probe_color(p)
    with rec.section("enums"):
        probe_enums(p)
    with rec.section("config"):
        probe_config(p)

    rec("pre_engine.screen_width", lambda: p.screen_width())

    # Not probes, but a crash report has to be able to name them.
    progress("activate_engine()")
    try:
        p.activate_engine(p.Config(window_title="pyroquad consistency test", window_width=W, window_height=H,
                                   fullscreen=False, window_resizable=False, high_dpi=False, swap_interval=0,
                                   sample_count=MSAA))
    except Exception as e:
        # stderr is teed through a pipe that can lose its last lines when the process
        # ends (see capture_rust_panics), so put the reason on stdout as well.
        print(f"\nactivate_engine() failed: {type(e).__name__}: {e}", flush=True)
        raise
    if IS_WEB:
        _web_canvas()
    progress("first frames after activate_engine()")
    for _ in range(5):
        p.next_frame(None)
    if (p.screen_width(), p.screen_height()) != (W, H):
        p.request_new_screen_size(W, H)
        for _ in range(10):
            p.next_frame(None)

    _screen_now[:] = [p.screen_width(), p.screen_height()]
    stream_write({"screen": list(_screen_now)})

    with rec.section("env"):
        probe_environment(p)
    with rec.section("images"):
        probe_images(p)
    font = None
    with rec.section("text"):
        font = probe_text(p)
    with rec.section("obj2d"):
        probe_objects2d(p)
    with rec.section("camera"):
        probe_cameras(p)
    with rec.section("files"):
        probe_files(p)
    with rec.section("audio"):
        probe_audio(p)
    checker = None
    with rec.section("scene"):
        checker, _ = run_scenes(p, font)
    with rec.section("obj3d"):
        probe_objects3d(p, checker)

    rec.info("runtime_s", round(time.time() - t0, 1))
    screen = [p.screen_width(), p.screen_height()]
    sizes = sorted(set(_screen_info.values()))
    rec.info("screen_data_sizes", sizes)
    _screen_now[:] = screen
    merge_rust_panics()
    out = dict(rec.data)
    out["__meta__"] = make_meta()
    return out


def write_json(path: str, data: dict):
    meta = data.get("__meta__", {})
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write("{\n")
        f.write(f'"__meta__": {json.dumps(meta)}')
        for k in sorted(k for k in data if k != "__meta__"):
            f.write(f",\n{json.dumps(k)}: {json.dumps(data[k], ensure_ascii=False, separators=(',', ':'))}")
        f.write("\n}\n")


def read_json(path: str) -> dict:
    with open(path, encoding="utf-8") as f:
        return json.load(f)


# ---------------------------------------------------------------------------
# Metadata, crash handling, GitHub output
# ---------------------------------------------------------------------------

OPTS = {"warn_only": False, "github": False, "dump": None}
_screen_now: list = []
LAST_RESULT_JSON = ""  # the browser runner reads the fingerprint from here

PLATFORM_NAMES = {"win32": "Windows", "linux": "Linux", "darwin": "macOS", "emscripten": "Browser (wasm)"}


def make_meta(crashed: str | None = None, crash_detail: list[str] | None = None) -> dict:
    import platform as _platform
    meta = {"format": FORMAT_VERSION, "platform": sys.platform, "python": sys.version.split()[0],
            "machine": _platform.machine(), "w": W, "h": H,
            "msaa": 4 if IS_WEB else MSAA,  # the web canvas target is always 4x
            "screen": list(_screen_now) or None}
    if os.environ.get("CONSISTENCY_LABEL"):
        meta["label"] = os.environ["CONSISTENCY_LABEL"]
    if crashed:
        meta["crashed"] = crashed
    if crash_detail:
        meta["crash_detail"] = crash_detail  # stderr tail / OS crash report, see supervise
    return meta


def platform_label(meta: dict) -> str:
    if meta.get("label"):
        return meta["label"]
    name = PLATFORM_NAMES.get(meta.get("platform"), meta.get("platform") or "?")
    if meta.get("msaa", 1) != 1 and meta.get("platform") != "emscripten":
        name += f" (msaa {meta['msaa']})"
    return name


def partial_result(reason: str, detail: list[str] | None = None) -> dict:
    """Whatever was recorded before a crash/hang, marked as such."""
    out = dict(rec.data)
    out["__meta__"] = make_meta(crashed=reason, crash_detail=detail)
    return out


def partial_result_json(reason: str) -> str:
    return json.dumps(partial_result(reason))


def report_crash(reason: str, detail: list[str] | None = None):
    global LAST_RESULT_JSON
    data = partial_result(reason, detail)
    if IS_WEB:
        LAST_RESULT_JSON = json.dumps(data)
    if OPTS["dump"]:
        try:
            write_json(OPTS["dump"], data)
            print(f"wrote partial fingerprint ({len(data) - 1} entries) to {OPTS['dump']}")
        except OSError as e:
            print("could not write partial fingerprint:", e)
    if OPTS["github"]:
        gh_annotation("warning", f"Consistency test crashed on {platform_label(data['__meta__'])}",
                      f"{reason} ({len(data) - 1} entries were recorded before that). "
                      "Not a build failure - see the job log.")


def _gh_data(text: str) -> str:
    return text.replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")


def gh_annotation(level: str, title: str, msg: str):
    """A GitHub Actions annotation (shown on the run page and the commit)."""
    title = _gh_data(title).replace(":", "%3A").replace(",", "%2C")
    print(f"::{level} title={title}::{_gh_data(msg)}", flush=True)


def gh_group(title: str, lines: list[str]):
    print(f"::group::{title}", flush=True)
    for line in lines:
        say(line)
    print("::endgroup::", flush=True)


def gh_summary(markdown: str):
    path = os.environ.get("GITHUB_STEP_SUMMARY")
    if path:
        with open(path, "a", encoding="utf-8") as f:
            f.write(markdown + "\n")


def areas(keys: list[str]) -> str:
    """'scene (7), audio (2)' - where the differences are."""
    counts: dict[str, int] = {}
    for k in keys:
        a = k.split(".")[0]
        counts[a] = counts.get(a, 0) + 1
    return ", ".join(f"{a} ({n})" for a, n in sorted(counts.items(), key=lambda x: -x[1]))


def evaluate(base: dict, actual: dict, canvas=None, github=False) -> int:
    """Compare one run against the baseline and print the result. Returns the failure count."""
    bm, am = base.get("__meta__", {}), actual.get("__meta__", {})
    crashed = am.get("crashed")
    n, lines, failed = compare(base, actual, canvas=canvas, only_present=bool(crashed))
    bl, al = platform_label(bm), platform_label(am)
    entries = len(base) - 1
    panics = [(k, actual[k]["panic"] + (f" [{actual[k]['at']}]" if actual[k].get("at") else ""))
              for k in failed if "panic" in actual.get(k, {})]
    diffs = [k for k in failed if "panic" not in actual.get(k, {})]
    if crashed:
        status = (f"CRASHED on {al}: {crashed}\n{len(actual) - 1} of {entries} entries were recorded, "
                  f"{n} of those differ from the baseline ({bl}).")
    elif n:
        status = (f"FAILED: {len(diffs)} of {entries} entries differ from the baseline (recorded on {bl} / "
                  f"py{bm.get('python')}), {len(panics)} probe(s) panicked.\n"
                  "Panics are always bugs. If the engine changed on purpose, re-record with --record on one platform.")
    else:
        status = f"PASSED: all {entries} entries match the baseline (recorded on {bl})."
    if github:
        gh_group(f"Consistency differences: {al} vs {bl} ({n})", lines or ["(none)"])
        say(status)
        if panics:
            gh_annotation("warning", f"Consistency: {len(panics)} Rust panic(s) on {al}",
                          "A panic is never intended behaviour. Not a build failure.\n"
                          + "\n".join(f"{k}: {m}" for k, m in panics[:8])
                          + (f"\n(+{len(panics) - 8} more)" if len(panics) > 8 else ""))
        if crashed:
            gh_annotation("warning", f"Consistency test crashed on {al}",
                          f"{crashed}. {n} of the {len(actual) - 1} recorded entries differ from {bl}. "
                          "Not a build failure.")
        elif diffs:
            shown = []
            for k in diffs[:6]:
                e, a = base.get(k, {}).get("v"), actual.get(k, {}).get("v")
                if isinstance(e, list) and len(e) > 4:  # grids: just name them
                    shown.append(k)
                else:
                    shown.append(f"{k}: {_short(e, 40)} -> {_short(a, 40)}")
            more = f"\n(+{len(diffs) - len(shown)} more)" if len(diffs) > len(shown) else ""
            gh_annotation("warning", f"Consistency: {al} differs from {bl}",
                          f"{len(diffs)} of {entries} entries differ - {areas(diffs)}. Not a build failure; "
                          "the 'Consistency report' job summary lists every difference.\n"
                          + "\n".join(shown) + more)
        elif not panics:
            gh_annotation("notice", f"Consistency: {al} matches {bl}",
                          f"All {entries} entries match the baseline.")
    else:
        say("\n".join(lines))
        say("\n" + status)
    return n


def report(baseline_path: str, runs: list[tuple[str, str]], github: bool) -> str:
    """Markdown overview of several runs (label, dump path) against the baseline."""
    base = read_json(baseline_path)
    bm = base.get("__meta__", {})
    entries = len(base) - 1
    rows, details = [], []
    labels = [label for label, _ in runs]
    outcome: dict[str, tuple] = {}  # label -> (data or None, failed keys)
    for label, path in runs:
        if not os.path.exists(path):
            outcome[label] = (None, set())
            rows.append(f"| {label} | ⛔ no result | – | – | the job failed before the test could run (see its log) |")
            continue
        data = read_json(path)
        am = data.get("__meta__", {})
        crashed = am.get("crashed")
        canvas = is_canvas_pair(bm, am)
        n, lines, failed = compare(base, data, only_present=bool(crashed))
        outcome[label] = (data, set(failed))
        recorded = len(data) - 1
        mode = " (canvas mode)" if canvas else ""
        if crashed:
            result = f"💥 crashed{mode}"
            note = f"`{crashed[:120]}`"
        elif n:
            npanic = sum(1 for k in failed if "panic" in data.get(k, {}))
            result = f"⚠️ differs{mode}" if npanic < n else f"💥 panics{mode}"
            note = "; ".join(x for x in (f"**{npanic} panic(s)**" if npanic else "",
                                         areas([k for k in failed if "panic" not in data.get(k, {})])) if x)
        else:
            result = f"✅ matches{mode}"
            note = ""
        rows.append(f"| {label} | {result} | {recorded} | {n} | {note} |")
        if n or crashed:
            if am.get("crash_detail"):
                lines = [f"CRASH    {crashed}", *("  " + ln for ln in am["crash_detail"]), "", *lines]
            body = "\n".join(lines).replace("```", "'" * 3)
            details.append(f"<details><summary><b>{label}</b> – {n} difference(s)</summary>\n\n"
                           f"```text\n{body}\n```\n</details>\n")

    def cell(label: str, key: str) -> str:
        data, failed = outcome[label]
        if data is None:
            return "⛔"
        if key in failed:
            return "💥" if "panic" in data.get(key, {}) else "⚠️"
        if data["__meta__"].get("crashed") and key not in data and key != "<screen size>":
            return "–"
        return "✅"

    matrix = {k: {label: cell(label, k) for label in labels}
              for k in sorted(set().union(*(f for _, f in outcome.values())))}
    md = [
        "## Cross-platform consistency",
        "",
        f"> Warning-only check: differences are reported here but never fail the build. "
        f"Baseline: **{platform_label(bm)}** (py{bm.get('python')}, {entries} entries). "
        "Re-record it with `python tests/consistency_test.py --record` when behaviour changes on purpose.",
        "",
        "| Platform | Result | Entries recorded | Differences | Where |",
        "|---|---|---:|---:|---|",
        *rows,
        "",
    ]
    if matrix:
        keys = sorted(matrix)
        shown = keys[:80]
        md += ["### Entries that differ from the baseline or panic", "",
               "| Entry | " + " | ".join(labels) + " |",
               "|---|" + "|".join(":-:" for _ in labels) + "|"]
        md += [f"| `{k}` | " + " | ".join(matrix[k][label] for label in labels) + " |" for k in shown]
        if len(keys) > len(shown):
            md.append(f"| … {len(keys) - len(shown)} more | " + " | ".join("" for _ in labels) + " |")
        md.append("")
        md.append("✅ matches · ⚠️ differs · 💥 Rust panic (never intended) · – not recorded "
                  "(the run crashed before it) · ⛔ no result")
        md.append("")
    if details:
        md += ["### Details", "", *details]
    text = "\n".join(md)
    if github:
        gh_summary(text)
    return text


def supervise(args: list[str]) -> int:
    """--ci: run the test in a child process. A native crash (abort, segfault)
    kills the process before Python can report anything, so the child streams
    every entry to a file and this parent turns a dead child into a partial
    fingerprint + a warning instead of a bare non-zero exit."""
    import signal
    import subprocess
    import tempfile
    child_args = [a for a in args if a != "--ci"] + ["--warn-only", "--github"]
    fd, stream_path = tempfile.mkstemp(prefix="consistency_", suffix=".jsonl")
    os.close(fd)
    log_path = stream_path[:-len(".jsonl")] + ".stderr.log"
    cmd = ([sys.executable, os.path.abspath(__file__)] + child_args
           + ["--stream", stream_path, "--stderr-log", log_path])
    sys.stdout.flush()
    started = time.time()
    child = subprocess.Popen(cmd)
    code = child.wait()
    if code == 0:
        for path in (stream_path, log_path):
            try:
                os.remove(path)
            except OSError:
                pass
        return 0
    try:
        sig = signal.Signals(-code).name if code < 0 else (signal.Signals(code - 128).name if code > 128 else None)
    except ValueError:
        sig = None
    data, last = {}, "<before the first probe>"
    with open(stream_path, encoding="utf-8") as f:
        for line in f:
            try:
                obj = json.loads(line)
            except ValueError:
                continue  # a line cut off by the crash
            if "at" in obj:
                last = obj["at"]
            elif "k" in obj:
                data[obj["k"]] = obj["e"]
            elif "screen" in obj:
                _screen_now[:] = obj["screen"]
    os.remove(stream_path)
    how = f"signal {sig}" if sig else (f"exit code {code:#x}" if code > 0xFFFF else f"exit code {code}")

    # What the child printed last (it is gone, so its pump never forwarded that).
    tail = read_stderr_tail(log_path)
    try:
        os.remove(log_path)
    except OSError:
        pass
    cause = crash_cause(tail)
    mac_report = macos_crash_report(child.pid, started) if sys.platform == "darwin" else []
    if mac_report and (cause is None or cause.startswith("Fatal Python error")):
        cause = macos_report_cause(mac_report) or cause

    reason = f"the process died ({how}) during '{last}'"
    if cause:
        reason += f": {cause}"
    print(f"\nCRASH: {reason} - {len(data)} entries were recorded before that", flush=True)
    if tail:
        gh_group(f"Last {len(tail)} stderr line(s) before the crash", tail)
    if mac_report:
        gh_group("macOS crash report", mac_report)
    detail = tail[-CRASH_DETAIL_LINES:] + ([""] if tail and mac_report else []) + mac_report[:CRASH_DETAIL_LINES]
    rec.data = data
    OPTS["github"] = True
    if "--dump" in args:
        OPTS["dump"] = os.path.abspath(args[args.index("--dump") + 1])
    report_crash(reason, detail)
    return 0


CRASH_DETAIL_LINES = 40  # per source, kept in the fingerprint and shown in the report


def read_stderr_tail(path: str, max_lines: int = 60) -> list[str]:
    """The last lines a crashed child wrote to stderr, without the probe markers."""
    try:
        with open(path, "rb") as f:
            f.seek(0, os.SEEK_END)
            f.seek(max(0, f.tell() - 256 * 1024))
            raw = f.read()
    except OSError:
        return []
    lines = [ln.decode("utf-8", "replace").rstrip() for ln in raw.splitlines() if not ln.startswith(_MARK)]
    return [ln for ln in lines if ln.strip()][-max_lines:]


def crash_cause(lines: list[str]) -> str | None:
    """Pick the line that best names why the process died, most specific first."""
    import re

    def panic_at(i: int) -> str:
        m = re.match(r"thread '([^']*)'(?: \(\d+\))? panicked at (.+?):\s*$", lines[i])
        msg = lines[i + 1].strip() if i + 1 < len(lines) else ""
        return f"Rust panic: {msg} (at {m.group(2)}, thread '{m.group(1)}')" if m else lines[i].strip()

    def last(pattern: str, before: int | None = None) -> int | None:
        for i in range(len(lines) if before is None else before, 0, -1):
            if re.search(pattern, lines[i - 1]):
                return i - 1
        return None

    def short(text: str) -> str:
        return text.strip()[:300]

    # an uncaught Objective-C exception (macOS AppKit) carries its own reason
    i = last(r"\*\*\* Terminating app due to uncaught exception")
    if i is not None:
        return short(lines[i])
    # an Objective-C exception that unwound into Rust's catch_unwind: Rust aborts without
    # naming it, but the engine logs every throw (src/engine/MacExceptionLog.rs)
    i = last(r"Rust cannot catch foreign exceptions")
    if i is not None:
        j = last(r"^Objective-C exception thrown on ", before=i)
        return short(lines[j] if j is not None else lines[i])
    # Rust aborts the process: the panic right before that is the actual error
    i = last(r"panic in a function that cannot unwind|non-unwinding panic|fatal runtime error|"
             r"panicked while processing panic|failed to initiate panic")
    if i is not None:
        j = last(r"panicked at ", before=i)
        return short(panic_at(j) if j is not None else lines[i])
    i = last(r"panicked at ")
    if i is not None:
        return short("last one before the crash - " + panic_at(i))
    # faulthandler only names the signal, but it is better than nothing
    i = last(r"^Fatal Python error")
    if i is not None:
        return short(lines[i])
    return None


def macos_crash_report(pid: int, since: float, wait: float = 20.0) -> list[str]:
    """macOS writes a crash report (.ips, JSON) for every process that dies of a signal.
    It names an uncaught Objective-C exception and has a symbolicated backtrace of the
    crashed thread - also when nothing reached stderr. ReportCrash writes it a few
    seconds after the process died, so wait a little for it."""
    import glob
    dirs = [os.path.expanduser("~/Library/Logs/DiagnosticReports"), "/Library/Logs/DiagnosticReports"]
    deadline = time.time() + wait
    while True:
        paths = [p for d in dirs for p in glob.glob(os.path.join(d, "*.ips"))]
        for path in sorted(paths, key=os.path.getmtime, reverse=True):
            try:
                if os.path.getmtime(path) < since - 5:
                    continue
                with open(path, encoding="utf-8", errors="replace") as f:
                    header_line, _, body_text = f.read().partition("\n")
                body = json.loads(body_text)
            except (OSError, ValueError):
                continue
            if body.get("pid") == pid:
                return summarize_ips(path, body)
        if time.time() > deadline:
            return [f"(no crash report for pid {pid} showed up in {', '.join(dirs)} within {wait:.0f}s)"]
        time.sleep(1)


def summarize_ips(path: str, body: dict) -> list[str]:
    images = body.get("usedImages") or []

    def frame(fr: dict) -> str:
        idx = fr.get("imageIndex", -1)
        image = images[idx].get("name", "?") if isinstance(idx, int) and 0 <= idx < len(images) else "?"
        sym = fr.get("symbol") or f"+{fr.get('imageOffset', 0):#x}"
        src = f"  ({fr['sourceFile']}:{fr.get('sourceLine')})" if fr.get("sourceFile") else ""
        return f"    {image:<28} {sym}{src}"

    out = [f"report: {path}"]
    exc = body.get("exception") or {}
    if exc:
        out.append("exception: " + " ".join(str(exc[k]) for k in ("type", "signal", "subtype") if exc.get(k)))
    for image, messages in (body.get("asi") or {}).items():
        for msg in messages if isinstance(messages, list) else [messages]:
            out.append(f"message ({image}): {msg}")
    backtrace = body.get("lastExceptionBacktrace")
    if backtrace:
        out.append("last exception backtrace:")
        out += [frame(fr) for fr in backtrace[:30]]
    threads = body.get("threads") or []
    ft = body.get("faultingThread")
    if isinstance(ft, int) and 0 <= ft < len(threads):
        t = threads[ft]
        label = t.get("name") or t.get("queue") or ""
        out.append(f"crashed thread: #{ft}{' (the main thread)' if ft == 0 else ' (NOT the main thread)'}"
                   + (f" {label}" if label else ""))
        out += [frame(fr) for fr in (t.get("frames") or [])[:30]]
    return out


def macos_report_cause(report: list[str]) -> str | None:
    """The most telling message of a crash report: an exception reason over `abort() called`."""
    messages = [ln.split(": ", 1)[1] for ln in report if ln.startswith("message (")]
    for msg in messages:
        if "abort() called" not in msg:
            return msg.strip()[:300]
    return messages[0].strip()[:300] if messages else None


def _canvas_flag(args):
    if "--canvas" in args:
        return True
    if "--no-canvas" in args:
        return False
    return None  # automatic


def main(argv: list[str]) -> int:
    global IMAGES_DIR, LAST_RESULT_JSON, _stream, _stderr_log
    args = list(argv)
    if "--ci" in args and not IS_WEB:
        return supervise(args)
    if "--stream" in args:
        _stream = open(args[args.index("--stream") + 1], "w", encoding="utf-8")
    if "--stderr-log" in args:
        _stderr_log = args[args.index("--stderr-log") + 1]
    OPTS["warn_only"] = "--warn-only" in args
    OPTS["github"] = "--github" in args
    fail_code = (lambda code: 0) if OPTS["warn_only"] else (lambda code: code)

    if "--report" in args:
        # --report BASELINE LABEL=DUMP [LABEL=DUMP ...]   (no engine needed)
        i = args.index("--report")
        rest = [a for a in args[i + 1:] if not a.startswith("--")]
        runs = [tuple(r.split("=", 1)) if "=" in r else (os.path.basename(r), r) for r in rest[1:]]
        say(report(rest[0], runs, OPTS["github"]))
        return 0

    if "--compare" in args:
        i = args.index("--compare")
        a, b = args[i + 1], args[i + 2]
        A, B = read_json(a), read_json(b)
        n, lines, _ = compare(A, B, os.path.basename(a), os.path.basename(b), canvas=_canvas_flag(args))
        say("\n".join(lines))
        print(f"\n{n} difference(s) between {a} ({platform_label(A.get('__meta__', {}))}) "
              f"and {b} ({platform_label(B.get('__meta__', {}))})")
        return fail_code(1) if n else 0

    record = "--record" in args
    if "--dump" in args:
        OPTS["dump"] = os.path.abspath(args[args.index("--dump") + 1])
    if "--images" in args:
        IMAGES_DIR = os.path.abspath(args[args.index("--images") + 1])
        os.makedirs(IMAGES_DIR, exist_ok=True)

    try:
        data = run_all()
    except (KeyboardInterrupt, SystemExit):
        raise
    except BaseException as e:  # e.g. no OpenGL on this machine, or a panic
        traceback.print_exc()
        first = (str(e).strip().splitlines() or [""])[0][:300]
        report_crash(f"{type(e).__name__}: {first}" if first else type(e).__name__)
        return fail_code(2)
    if IS_WEB:
        LAST_RESULT_JSON = json.dumps(data)
    if rec.errors:
        lines = ["Sections that raised (recorded as behaviour):"] + ["    " + e for e in rec.errors]
        gh_group("Sections that raised", lines) if OPTS["github"] else say("\n".join(lines))
    entries = len(data) - 1
    sizes = data.get("screen_data_sizes", {}).get("info")
    if sizes and sizes != [[W, H]]:
        print(f"WARNING: screen data was {sizes}, expected {W}x{H}. Frames are cropped to the top-left "
              f"{W}x{H} area; camera scenes will differ.")

    if OPTS["dump"]:
        write_json(OPTS["dump"], data)
        print(f"wrote fingerprint ({entries} entries) to {OPTS['dump']}")

    if record or not os.path.exists(BASELINE):
        if IS_WEB:
            print("refusing to record a baseline inside the browser; record it natively")
            return fail_code(1)
        write_json(BASELINE, data)
        print(f"{'recorded' if record else 'no baseline found - recorded'} {entries} entries to {BASELINE}")
        return 0

    base = read_json(BASELINE)
    # round-trip through JSON so both sides have identical types
    actual = json.loads(json.dumps(data))
    n = evaluate(base, actual, canvas=_canvas_flag(args), github=OPTS["github"])
    return fail_code(1) if n else 0


if __name__ == "__main__":
    code = main(sys.argv[1:])
    if not IS_WEB:
        sys.stdout.flush()
        sys.stderr.flush()
        os._exit(code)  # the engine thread would otherwise keep the process alive / swallow the code
