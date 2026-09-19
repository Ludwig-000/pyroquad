from ._pyroquad import * #type: ignore
from ._pyroquad import InternalGL  #type: ignore

# we export Color seperately, to keep the module file readable.
from ._pyroquad import ColorMod #type: ignore
Color = ColorMod.Color
del ColorMod

from . import examples #imports the python-half of the library




"""
if you are looking for documentation, note there exists (AI.md)[https://github.com/Ludwig-000/pyroquad/blob/main/AI.md]
and (README.md)[https://github.com/Ludwig-000/pyroquad/blob/main/README.md]
"""