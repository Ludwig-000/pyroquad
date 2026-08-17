# python/pyroquad/examples/ExampleGame_/__init__.py

__all__ = ["launch_game"]

def launch_game():
    """Deferred launcher for ExampleGame."""
    from .ExampleGame2D import launch
    return launch()