"""Assemble the browser test bundle.

Takes the Emscripten side module produced by

    cargo build --target wasm32-unknown-emscripten --release --features abi_314

(with ``RUSTFLAGS=-C link-arg=-sSIDE_MODULE=2``) plus the pure-Python half of the
package in ``src/python/pyroquad`` and zips them into ``web/pyroquad_pkg.zip``,
which ``index.html`` unpacks into Pyodide's home directory.

Run from anywhere:  python web/build_web.py
"""

from __future__ import annotations

import shutil
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
WEB = ROOT / "web"
WASM = ROOT / "target" / "wasm32-unknown-emscripten" / "release" / "_pyroquad.wasm"
PYSRC = ROOT / "src" / "python" / "pyroquad"
OUT = WEB / "pyroquad_pkg.zip"

# Pyodide names extension modules with the full ABI tag, but a bare `.so` is
# also importable and keeps this independent of the exact CPython build.
SO_NAME = "_pyroquad.so"


def main() -> int:
    if not WASM.is_file():
        print(f"missing {WASM}\nbuild it first:", file=sys.stderr)
        print(
            '  RUSTFLAGS="-C link-arg=-sSIDE_MODULE=2" cargo build '
            "--target wasm32-unknown-emscripten --release --features abi_314",
            file=sys.stderr,
        )
        return 1
    if not PYSRC.is_dir():
        print(f"missing {PYSRC}", file=sys.stderr)
        return 1

    if OUT.exists():
        OUT.unlink()

    with zipfile.ZipFile(OUT, "w", zipfile.ZIP_DEFLATED) as z:
        for path in sorted(PYSRC.rglob("*")):
            if path.is_dir():
                continue
            if "__pycache__" in path.parts or path.suffix in {".pyc", ".pyd", ".so"}:
                continue
            z.write(path, Path("pyroquad") / path.relative_to(PYSRC))
        z.write(WASM, Path("pyroquad") / SO_NAME)

    size = OUT.stat().st_size
    print(f"wrote {OUT.relative_to(ROOT)}  ({size / 1e6:.1f} MB)")
    print(f"  native module: {WASM.stat().st_size / 1e6:.1f} MB -> pyroquad/{SO_NAME}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
