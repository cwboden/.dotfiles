from typing import Protocol


class Printable(Protocol):
    def print(self) -> None:
        pass
