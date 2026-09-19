"""Report which imports of an Emscripten side module Pyodide can resolve.

    python web/check_imports.py target/wasm32-unknown-emscripten/release/_pyroquad.wasm

Parses the side module's wasm import section and cross-references every name
against the Pyodide distribution in `web/pyodide/`: the exports of
`pyodide.asm.wasm` plus the JS-library functions in `pyodide.asm.mjs`'s
`wasmImports` object (which is what Emscripten's `resolveGlobalSymbol` actually
consults at dlopen time). Anything left over must be supplied by
`src/web/shim.c`, either as a defined function or as an `__em_js__` body.

Exits non-zero if anything is unresolved, so it can be used as a build gate.
"""
import re
import sys
import os

DIST = os.path.join(os.path.dirname(os.path.abspath(__file__)), "pyodide")
PY_WASM = os.path.join(DIST, "pyodide.asm.wasm")
PY_MJS = os.path.join(DIST, "pyodide.asm.mjs")


def uleb(d, p):
    r = s = 0
    while True:
        b = d[p]
        p += 1
        r |= (b & 0x7F) << s
        if not b & 0x80:
            return r, p
        s += 7


def sections(data):
    p = 8
    while p < len(data):
        sid = data[p]
        p += 1
        size, p = uleb(data, p)
        yield sid, p, p + size
        p += size


def exports(path):
    data = open(path, "rb").read()
    out = set()
    for sid, p, end in sections(data):
        if sid != 7:
            continue
        n, p = uleb(data, p)
        for _ in range(n):
            l, p = uleb(data, p)
            out.add(data[p:p + l].decode())
            p += l + 1
            _, p = uleb(data, p)
    return out


def imports(path):
    data = open(path, "rb").read()
    out = []
    for sid, p, end in sections(data):
        if sid != 2:
            continue
        n, p = uleb(data, p)
        for _ in range(n):
            l, p = uleb(data, p)
            mod = data[p:p + l].decode()
            p += l
            l, p = uleb(data, p)
            nm = data[p:p + l].decode()
            p += l
            kind = data[p]
            p += 1
            if kind == 0:
                _, p = uleb(data, p)
            elif kind == 1:
                p += 1
                fl = data[p]
                p += 1
                _, p = uleb(data, p)
                if fl:
                    _, p = uleb(data, p)
            elif kind == 2:
                fl = data[p]
                p += 1
                _, p = uleb(data, p)
                if fl & 1:
                    _, p = uleb(data, p)
            elif kind == 3:
                p += 2
            out.append((mod, nm, kind))
    return out


def pyodide_symbols():
    wasm_exports = exports(PY_WASM)
    s = open(PY_MJS, encoding="utf8").read()
    i = s.index("wasmImports={")
    j = i + len("wasmImports=")
    depth = 0
    for k in range(j, len(s)):
        if s[k] == "{":
            depth += 1
        elif s[k] == "}":
            depth -= 1
            if depth == 0:
                end = k + 1
                break
    js = set(re.findall(r"[,{]([A-Za-z_$][\w$]*)\s*[:,}]", s[j:end]))
    return js | {x[1:] for x in wasm_exports if x.startswith("_")} | wasm_exports


def main(path):
    avail = pyodide_symbols()
    mine_imports = imports(path)
    mine_exports = exports(path)

    env_funcs = sorted({n for m, n, k in mine_imports if k == 0 and m == "env"})
    em_js = {x.replace("__em_js__", "") for x in mine_exports if x.startswith("__em_js__")}

    from_pyodide = [n for n in env_funcs if n in avail]
    from_self = [n for n in env_funcs if n not in avail and (n in mine_exports or n in em_js)]
    unresolved = [n for n in env_funcs if n not in avail and n not in mine_exports and n not in em_js]

    print(f"{path}")
    print(f"  env function imports : {len(env_funcs)}")
    print(f"    resolved by Pyodide: {len(from_pyodide)}")
    print(f"    supplied by shim   : {len(from_self)}  {sorted(from_self)}")
    print(f"    UNRESOLVED         : {len(unresolved)}")
    for n in unresolved:
        print(f"      ! {n}")
    # PIC relocations. Most point at the module's own symbols; the loader only
    # needs a real address, which EM_JS-only functions do not have - that is the
    # trap `src/web/shim.c` avoids by defining every imported name in C.
    got = [(m, n) for m, n, k in mine_imports if m.startswith("GOT.")]
    got_bad = [n for m, n in got if n not in mine_exports and n not in avail]
    print(f"  GOT relocations      : {len(got)}")
    print(f"    UNRESOLVED         : {len(got_bad)}")
    for n in sorted(set(got_bad)):
        print(f"      ! {n}")

    return 1 if (unresolved or got_bad) else 0


DEFAULT_WASM = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "target", "wasm32-unknown-emscripten", "release", "_pyroquad.wasm",
)

if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else DEFAULT_WASM))
