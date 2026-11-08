from enum import Enum


class Status(Enum):
    """
    State of a pyroquad future that has no value to hand back (yet).

    `poll()` and `wait()` return either the finished value or one of these members.
    If the work failed, they raise the same exception the blocking function would have.

    ```python
    img = image_future.poll()
    if img is Status.Pending:
        return
    draw(img)  # type checkers know this is an Image now
    ```

    The members are singletons, so compare with `is` (or `==`).
    """

    Pending = "pending"
    """The work has not finished yet, or `wait(timeout)` ran out of time."""

    Done = "done"
    """The work finished. Only futures without a value (`EmptyFuture`) return this."""
