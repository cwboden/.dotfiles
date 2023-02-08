import sys
from typing import Sequence

from game_state import GameConfiguration
from game_state import GameState
from units.coal import CoalMine
from units.coal import PowerPlantCoal
from units.converters import CoalLiquifactor
from units.converters import Kiln
from units.oil import OilDerrick
from units.oil import PowerPlantOil
from units.wood import Forest
from units.wood import PowerPlantWood


ALL_ASSETS = [
    CoalLiquifactor(),
    CoalMine(),
    Forest(),
    Kiln(),
    OilDerrick(),
    PowerPlantCoal(),
    PowerPlantOil(),
    PowerPlantWood(),
]
NUM_OPTIONS = 5
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
