"""Summarise web/offscreen_bench/out/ after driver.html?plan=demos / plan=bench.

    python web/offscreen_bench/compare.py demos
    python web/offscreen_bench/compare.py bench
    python web/offscreen_bench/compare.py gpu
"""

from __future__ import annotations

import json
import re
import statistics
import sys
from collections import defaultdict
from pathlib import Path

OUT = Path(__file__).resolve().parent / "out"


def demos() -> None:
    import numpy as np
    from PIL import Image

    def load(name):
        p = OUT / f"{name}.png"
        return np.asarray(Image.open(p).convert("RGBA")).astype(int) if p.exists() else None

    pairs = [("base", "split2"), ("split2", "rt4"), ("base", "rt1"), ("base", "rt4")]
    for demo in ["shapes", "camfont", "3d", "skybox", "usert", "screendata"]:
        for dpr in (1, 2):
            row = [f"{demo:10s} dpr{dpr}"]
            for a, b in pairs:
                ia, ib = load(f"demo_{demo}_{a}_dpr{dpr}"), load(f"demo_{demo}_{b}_dpr{dpr}")
                if ia is None or ib is None or ia.shape != ib.shape:
                    row.append(f"{a}-{b}: n/a")
                    continue
                d = np.abs(ia - ib).max(axis=2)
                row.append(f"{a}-{b}: {int((d > 0).sum()):>6} px ({int((d > 32).sum()):>5} >32)")
            print("  ".join(row))

    print()
    for demo in ["camfont", "screendata"]:
        for tag in ("base", "split2", "rt4", "rt1"):
            p = OUT / f"demo_{demo}_{tag}_dpr1.json"
            if p.exists():
                r = json.loads(p.read_text())
                print(demo, tag, json.dumps(r.get("camera_font_scale") or r.get("screen_data")))


def bench(prefix: str = "bench") -> None:
    runs = defaultdict(list)
    for p in sorted(OUT.glob(f"{prefix}_*.json")):
        r = json.loads(p.read_text())
        key = re.sub(r"_r\d+$", "", r["name"])
        runs[key].append(r)

    print(f"{'run':28s} {'screen':>11s}  {'median':>7s} {'p95':>7s}  | draw   next_f  finish (medians, ms)")
    for key in sorted(runs):
        rs = runs[key]
        med = statistics.median(r["total"]["median_ms"] for r in rs)
        p95 = statistics.median(r["total"]["p95_ms"] for r in rs)
        dr = statistics.median(r["draw_phase"]["median_ms"] for r in rs)
        nf = statistics.median(r["next_frame"]["median_ms"] for r in rs)
        fi = statistics.median(r["finish"]["median_ms"] for r in rs)
        w, h = rs[0]["screen"]
        dpi = rs[0]["dpi"]
        print(f"{key:28s} {int(w*dpi):>5d}x{int(h*dpi):<5d}  {med:7.2f} {p95:7.2f}  | {dr:6.2f} {nf:6.2f} {fi:6.2f}  (n={len(rs)})")


if __name__ == "__main__":
    {"demos": demos, "bench": bench, "gpu": lambda: bench("gpu")}[sys.argv[1] if len(sys.argv) > 1 else "demos"]()
