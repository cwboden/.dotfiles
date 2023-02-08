import sys
from typing import Sequence

from game_state import GameConfiguration
from game_state import GameState
from units.simple import CoalMine
from units.simple import Forest
from units.simple import Kiln
from units.simple import PowerPlant


ALL_ASSETS = [Forest(), CoalMine(), Kiln(), PowerPlant()]
NUM_OPTIONS = 3
NUM_SEASONS = 4

REQUIRED_ENERGY = [4, 6, 10, 16, 26, 42, 68, 111]


def main(args: Sequence[str]) -> None:
    config = GameConfiguration(
        5,  # starting gold
        NUM_OPTIONS,
        NUM_SEASONS,
        ALL_ASSETS,
        REQUIRED_ENERGY,
    )
    state = GameState(config)
    state.run()


if __name__ == "__main__":
    main(sys.argv[1:])
