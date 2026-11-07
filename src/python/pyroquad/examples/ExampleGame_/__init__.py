# python/pyroquad/examples/ExampleGame_/__init__.py

__all__ = ["launch_2D_game", "launch_marble_demo"]

def launch_2D_game():
    """Deferred launcher for ExampleGame."""
    from .ExampleGame2D import launch
    return launch()

def launch_marble_demo():
    from .Marbles import launch
    return launch()