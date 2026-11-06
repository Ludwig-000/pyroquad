# pyroquad vs pygame — 2D rendering benchmark

**Result:** pyroquad is roughly **3.8× faster** than pygame-ce in typical 2D drawing
(geometric mean over 8 scenes; **5.4×** including the rotate/scale outlier).

## Setup

| | |
|---|---|
| OS | Windows 10 Pro (10.0.19045) |
| CPU | Intel Core, Family 6 Model 158 (8 logical cores) |
| GPU | NVIDIA GeForce RTX 3070 |
| Python | 3.14.0 (CPython) |
| pygame | pygame-ce 2.5.8 (SDL 2.32.10, software surface) |
| pyroquad | 0.2.1 (macroquad, OpenGL) |
| Window | 1280×720, vsync off, not resizable, high-DPI off |
| Measurement | 1.5 s warm-up discarded, then 5 s measured; 3 runs per library per scene, median reported |

## Results

| Scene | Count | pygame-ce FPS | median ms | p99 ms | pyroquad FPS | median ms | p99 ms | Speedup |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| empty (clear + present) | – | 2028 | 0.46 | 1.12 | 6730 | 0.13 | 0.43 | **3.3×** |
| filled rectangles 24×24 | 5000 | 94 | 10.58 | 12.21 | 718 | 1.38 | 1.70 | **7.6×** |
| filled circles r=10 | 2000 | 421 | 2.29 | 3.43 | 754 | 1.30 | 1.80 | **1.8×** |
| lines, 2 px | 3000 | 272 | 3.60 | 4.59 | 1076 | 0.92 | 1.22 | **4.0×** |
| 32×32 alpha sprites, one call each | 5000 | 155 | 6.23 | 10.08 | 723 | 1.38 | 1.80 | **4.7×** |
| 32×32 alpha sprites, bulk-draw API | 10000 | 128 | 7.55 | 10.79 | 654 | 1.50 | 2.10 | **5.1×** |
| sprites rotated + scaled every frame | 500 | 52 | 19.03 | 22.19 | 4834 | 0.19 | 0.49 | **93×** |
| text changing every frame, size 20 | 200 | 506 | 1.90 | 2.96 | 1579 | 0.62 | 0.91 | **3.1×** |
| "game": Python physics + sprites + HUD + text | 2000 | 260 | 3.63 | 5.37 | 850 | 1.16 | 1.65 | **3.3×** |

- Geometric mean speedup, all 9 scenes: **5.4×**
- Geometric mean speedup, excluding rotate/scale: **3.8×**
- Run-to-run spread: within ~3% (the empty scene varies more, since its frames are ~0.1 ms)

### CPU usage (all threads of the process; 100% = one core)

| | pygame-ce | pyroquad |
|---|---:|---:|
| Typical drawing scene | ~100% | ~150–225% |

## How the comparison is kept fair

- Every library/scene run is a fresh Python process; library order alternates between repeats.
- Same window size and settings; both framebuffers confirmed 1280×720.
- Scene data (positions, colours, sizes, strings, sprite pixels) comes from one fixed seed and is
  identical for both. The Python code around the draw calls is the same; only the draw call differs.
- pygame uses its fastest idioms: `Surface.fill(color, rect)` for solid rects, `convert_alpha()`
  sprites, `fblits` for bulk drawing. pyroquad uses `draw_*` calls and `batch_draw_shapes`.
- A frame is timed until the present call returns: pygame `display.flip()`, pyroquad `next_frame()`
  (which blocks until the frame is drawn). pygame pumps window events each frame, as it must.
- Screenshots taken at a fixed frame index were compared: both libraries draw the same picture
  (the rotate/scale scene is pixel-identical, including sprite angles).

## Caveats

- **pygame-ce, not classic pygame:** classic pygame has no Python 3.14 wheel. pygame-ce is the
  faster fork, so it is the tougher baseline.
- **Threads and GPU:** pygame rasterizes on one CPU core; pyroquad renders on the GPU and runs a
  separate render thread (~2 cores). Part of the speedup comes from that architecture.
- **Rotate/scale is an outlier:** pygame re-rasterizes each sprite on the CPU with
  `transform.rotozoom` every frame, while pyroquad only changes a quad's vertices.
- **Circles differ slightly:** pyroquad's `draw_circle` is a 20-sided polygon; pygame's circle is
  pixel-exact.
- **Text:** pygame re-renders each changing string with `font.render`; pyroquad caches glyphs.

## Reproduce

```bash
pip install pyroquad pygame-ce
python pyroquad_vs_pygame.py --seconds 5 --repeat 3
```
