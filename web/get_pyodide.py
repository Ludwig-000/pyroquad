"""Download the Pyodide runtime the browser build needs into ``web/pyodide/``.

    python web/get_pyodide.py            # the pinned version
    python web/get_pyodide.py 314.0.7    # or any other release

This is the stock ``pyodide-core`` tarball from Pyodide's GitHub releases,
unpacked and otherwise untouched - pyroquad's web build deliberately runs on an
unmodified Pyodide (see ``docs/WASM.md``). Nothing here is needed to build the
module itself; it is the runtime that loads it, plus the reference
``check_imports.py`` resolves symbols against.

Pyodide releases are versioned after the CPython they ship: 314.0.7 is CPython
3.14.2, which is what ``--features abi_314`` targets.
"""

from __future__ import annotations

import io
import json
import shutil
import sys
import tarfile
import urllib.request
from pathlib import Path

DEFAULT_VERSION = "314.0.7"
WEB = Path(__file__).resolve().parent
URL = "https://github.com/pyodide/pyodide/releases/download/{v}/pyodide-core-{v}.tar.bz2"
PYPI = "https://pypi.org/pypi/{name}/{version}/json"


def fetch_micropip(dest: Path) -> None:
    """Put micropip's wheel next to the runtime.

    ``pyodide-core`` lists micropip in ``pyodide-lock.json`` but does not ship
    the file, and ``loadPackage`` resolves package filenames against the page's
    ``indexURL`` - so without this, ``index.html`` cannot install the pyroquad
    wheel the way a user would. micropip is a pure-Python wheel, so the copy on
    PyPI is byte-identical to the one in the full Pyodide distribution.
    """
    lock = json.loads((dest / "pyodide-lock.json").read_text(encoding="utf8"))
    entry = lock["packages"]["micropip"]
    file_name = entry["file_name"]

    if (dest / file_name).is_file():
        print(f"micropip already present ({file_name})")
        return

    # file_name is `micropip-<version>-py3-none-any.whl`.
    version = file_name.split("-")[1]
    with urllib.request.urlopen(PYPI.format(name="micropip", version=version)) as response:
        meta = json.load(response)

    for item in meta["urls"]:
        if item["filename"] == file_name:
            print(f"downloading {file_name}")
            with urllib.request.urlopen(item["url"]) as response:
                (dest / file_name).write_bytes(response.read())
            return

    print(
        f"warning: {file_name} not found on PyPI - index.html will fall back to\n"
        f"         unpacking web/pyroquad_pkg.zip instead of installing a wheel",
        file=sys.stderr,
    )


def main(version: str) -> int:
    dest = WEB / "pyodide"
    url = URL.format(v=version)

    print(f"downloading {url}")
    with urllib.request.urlopen(url) as response:
        blob = response.read()
    print(f"  {len(blob) / 1e6:.1f} MB")

    if dest.exists():
        print(f"replacing {dest}")
        shutil.rmtree(dest)

    print(f"extracting into {WEB}")
    with tarfile.open(fileobj=io.BytesIO(blob), mode="r:bz2") as tar:
        # The tarball's members all live under a top-level `pyodide/`, and this
        # refuses anything that tries to climb out of it.
        for member in tar.getmembers():
            target = (WEB / member.name).resolve()
            if not str(target).startswith(str(WEB)):
                raise RuntimeError(f"refusing unsafe archive path: {member.name}")
        tar.extractall(WEB)

    marker = dest / "pyodide.asm.wasm"
    if not marker.is_file():
        print(f"unexpected archive layout: {marker} is missing", file=sys.stderr)
        return 1

    fetch_micropip(dest)

    print(f"pyodide {version} ready in {dest}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1] if len(sys.argv) > 1 else DEFAULT_VERSION))
