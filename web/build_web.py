"""Stage the browser test bundle for ``index.html``.

There are two things this can stage, matching the two ways the module is built
(see the WebAssembly section of README.md):

* **A wheel** - ``maturin build --target wasm32-unknown-emscripten`` writes a
  PEP 783 ``pyemscripten_*_wasm32`` wheel into ``dist/``. That is the artifact
  that ships to PyPI, so when one exists it is the one staged: copied to
  ``web/pyroquad.whl``, which the page installs with micropip exactly as a user
  would.

* **A bare side module** - ``cargo build --target wasm32-unknown-emscripten
  --release`` writes only ``target/.../_pyroquad.wasm``. Wheels need
  ``pyodide-build``, which does not run natively on Windows, so this shorter
  loop stays supported: the module is zipped together with the pure-Python half
  of the package into ``web/pyroquad_pkg.zip``, which the page unpacks directly.

A wheel wins when both are present. Whichever is staged, the other is deleted,
so the page can never quietly load a stale artifact of the other kind.

Run from anywhere:  python web/build_web.py
"""

from __future__ import annotations

import shutil
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WEB = ROOT / "web"
DIST = ROOT / "dist"
WASM = ROOT / "target" / "wasm32-unknown-emscripten" / "release" / "_pyroquad.wasm"
PYSRC = ROOT / "src" / "python" / "pyroquad"

ZIP_OUT = WEB / "pyroquad_pkg.zip"
WHEEL_OUT = WEB / "pyroquad.whl"

# Pyodide names extension modules with the full ABI tag, but a bare `.so` is
# also importable and keeps this independent of the exact CPython build.
SO_NAME = "_pyroquad.so"

BUILD_HELP = """build one of them first:

  a wheel (what ships; needs pyodide-build, so Linux/macOS/WSL):
    CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_RUSTFLAGS="$(pyodide config get rustflags)" \\
    MATURIN_PYEMSCRIPTEN_PLATFORM_VERSION="$(pyodide config get pyodide_abi_version)" \\
    PYO3_CROSS_PYTHON_VERSION=3.14 \\
    maturin build --release --target wasm32-unknown-emscripten --out dist --features abi_314

  or just the side module (works anywhere):
    RUSTFLAGS="-C link-arg=-sSIDE_MODULE=2" PYO3_CROSS_PYTHON_VERSION=3.14 \\
    cargo build --target wasm32-unknown-emscripten --release --features abi_314"""


def newest_wheel() -> Path | None:
    """The most recently built Emscripten wheel in ``dist/``, if any.

    Matched on the ``*wasm32`` platform tag so that native wheels sitting in the
    same directory are never picked up - that covers ``pyemscripten_*_wasm32``
    as well as the older ``pyodide_*``/``emscripten_*`` spellings.
    """
    if not DIST.is_dir():
        return None
    wheels = sorted(DIST.glob("*wasm32.whl"), key=lambda p: p.stat().st_mtime)
    return wheels[-1] if wheels else None


def stage_wheel(wheel: Path) -> int:
    shutil.copyfile(wheel, WHEEL_OUT)
    ZIP_OUT.unlink(missing_ok=True)

    print(f"staged {wheel.name}  ({wheel.stat().st_size / 1e6:.1f} MB)")
    print(f"  -> {WHEEL_OUT.relative_to(ROOT)}   (index.html installs it with micropip)")
    return 0


def stage_zip() -> int:
    if not PYSRC.is_dir():
        print(f"missing {PYSRC}", file=sys.stderr)
        return 1

    ZIP_OUT.unlink(missing_ok=True)

    with zipfile.ZipFile(ZIP_OUT, "w", zipfile.ZIP_DEFLATED) as z:
        for path in sorted(PYSRC.rglob("*")):
            if path.is_dir():
                continue
            if "__pycache__" in path.parts or path.suffix in {".pyc", ".pyd", ".so"}:
                continue
            z.write(path, Path("pyroquad") / path.relative_to(PYSRC))
        z.write(WASM, Path("pyroquad") / SO_NAME)

    WHEEL_OUT.unlink(missing_ok=True)

    size = ZIP_OUT.stat().st_size
    print(f"wrote {ZIP_OUT.relative_to(ROOT)}  ({size / 1e6:.1f} MB)")
    print(f"  native module: {WASM.stat().st_size / 1e6:.1f} MB -> pyroquad/{SO_NAME}")
    print("  (no wheel in dist/ - staging the bare side module instead)")
    return 0


def main() -> int:
    wheel = newest_wheel()
    if wheel is not None:
        return stage_wheel(wheel)

    if not WASM.is_file():
        print(f"found neither a wheel in {DIST} nor {WASM}", file=sys.stderr)
        print(BUILD_HELP, file=sys.stderr)
        return 1

    return stage_zip()


if __name__ == "__main__":
    raise SystemExit(main())
