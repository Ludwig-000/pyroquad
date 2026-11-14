"""
Built-in materials, written in plain Python so they can be read and copied.

Each module in this folder describes one material: its GLSL source, the uniforms and
textures it declares, its pipeline settings and the starting value of every uniform.
Each one is an attribute of `Material` (`Material.COLOR_GRADING`, ...). Nothing is compiled
when pyroquad is imported: a material is compiled the first time its attribute is read, and
every later read returns that same material:

    Material.COLOR_GRADING.set_uniform("saturation", UniformType.Float1([0.5]))
    Material.COLOR_GRADING.start_using()
    draw_rectangle(0, 0, 1, 1, Color.WHITE)
    use_default_material()

How a material is meant to be drawn is described at the top of its module.
The material is shared by the whole program, so a uniform set on it stays set for everyone.

To write your own material, copy one of these modules and pass its parts to `Material(...)`,
the same way `_compile` below does.

Adding a built-in: write its module here, then add a `#[classattr]` for it next to
`COLOR_GRADING` in `src/py_abstractions/Material.rs`. It is declared there, not here,
so the generated type stubs know about it.
"""

import importlib

from .._pyroquad import Material  # type: ignore


def _compile(module: str) -> Material:
    """Builds the material described in the module `module` of this folder.
    Called by `Material.<NAME>` on its first access."""
    m = importlib.import_module(f".{module}", __package__)
    mat = Material(m.VERTEX, m.FRAGMENT, module.upper(), m.UNIFORMS, m.TEXTURES, m.PIPELINE)
    for name, value in m.DEFAULTS.items():
        mat.set_uniform(name, value)
    return mat
