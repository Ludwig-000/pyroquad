"""web/serve.py plus a write-only endpoint for the offscreen-frame benchmark.

    python web/offscreen_bench/serve_bench.py [port]
    -> http://127.0.0.1:8000/web/offscreen_bench/driver.html

`POST /__results/<name>` stores the body as web/offscreen_bench/out/<name>
(PNG data URLs are decoded to .png). Nothing else is writable.
"""

from __future__ import annotations

import base64
import http.server
import os
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import serve  # noqa: E402

OUT = HERE / "out"
PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8000


class Handler(serve.Handler):
    def do_POST(self):
        m = re.fullmatch(r"/__results/([A-Za-z0-9_.\-]+)", self.path)
        if not m:
            self.send_error(404)
            return
        body = self.rfile.read(int(self.headers.get("Content-Length", 0)))
        name = m.group(1)
        if body.startswith(b"data:image/png;base64,"):
            body = base64.b64decode(body.split(b",", 1)[1])
        OUT.mkdir(exist_ok=True)
        (OUT / name).write_bytes(body)
        self.send_response(204)
        self.send_header("Content-Length", "0")
        self.end_headers()


def main() -> int:
    os.chdir(serve.ROOT)
    http.server.ThreadingHTTPServer.allow_reuse_address = True
    with http.server.ThreadingHTTPServer(("127.0.0.1", PORT), Handler) as httpd:
        print(f"serving {serve.ROOT} at http://127.0.0.1:{PORT}/web/offscreen_bench/driver.html", flush=True)
        httpd.serve_forever()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
