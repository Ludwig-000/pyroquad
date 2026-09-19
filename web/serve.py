"""Static dev server for the browser build.

    python web/serve.py            # http://127.0.0.1:8000/web/index.html

Serves the repository root so that `web/index.html` can reach both `web/` and
the test scripts at the top level. Sends no-cache headers (the .wasm changes on
every rebuild) and the WebAssembly mime type.

Cross-origin isolation headers are sent as well. They are not required by this
port - it uses JSPI, not SharedArrayBuffer - but they are harmless and keep the
door open for measuring against a threaded variant.
"""

from __future__ import annotations

import http.server
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8000


class Handler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {
        **http.server.SimpleHTTPRequestHandler.extensions_map,
        ".wasm": "application/wasm",
        ".mjs": "text/javascript",
        ".js": "text/javascript",
    }

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        self.send_header("Cross-Origin-Resource-Policy", "cross-origin")
        super().end_headers()

    protocol_version = "HTTP/1.1"

    def log_message(self, fmt, *args):  # only complain about failures
        msg = fmt % args
        if " 200 " not in msg and " 304 " not in msg:
            print(msg, file=sys.stderr, flush=True)


def main() -> int:
    os.chdir(ROOT)
    http.server.ThreadingHTTPServer.allow_reuse_address = True
    with http.server.ThreadingHTTPServer(("127.0.0.1", PORT), Handler) as httpd:
        print(f"serving {ROOT} at http://127.0.0.1:{PORT}/web/index.html")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            pass
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
