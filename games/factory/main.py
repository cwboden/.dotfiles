import sys
from typing import Sequence

from game_systems import GameState
from game_types import Resource


def main(args: Sequence[str]) -> None:
    state = GameState()

    # Start with 5 initial gold
    state.resources[Resource.GOLD] = 5

    state.print()


if __name__ == "__main__":
    main(sys.argv[1:])
