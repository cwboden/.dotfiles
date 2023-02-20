import sys
from typing import Sequence

from game_state import GameConfiguration
from game_state import GameState
from game_types import Amount
from game_types import Resource
from units.coal import CoalMine
from units.coal import PowerPlantCoal
from units.converters import Kiln
from units.coverage import SmallSubstation
from units.fossil_gas import FossilGasMine
from units.fossil_gas import PowerPlantFossilGas
from units.headquarters import Headquarters
from units.science import ResearchLab
from units.science import ThinkTank
from units.steam import SmallTurbine
from units.wood import Forest
from units.wood import PowerPlantWood


ALL_ASSETS = [
    CoalMine(),
    Forest(),
    FossilGasMine(),
    Kiln(),
    PowerPlantCoal(),
    PowerPlantFossilGas(),
    PowerPlantWood(),
    SmallSubstation(),
    SmallTurbine(),
    ThinkTank(),
    ResearchLab(),
]
NUM_OPTIONS = 5
NUM_SEASONS = 4

REQUIRED_RESOURCES = [
    Amount({Resource.ENERGY: 4, Resource.COVERAGE: 1}),
    Amount({Resource.ENERGY: 12, Resource.COVERAGE: 2}),
    Amount({Resource.ENERGY: 36, Resource.COVERAGE: 3}),
    Amount({Resource.ENERGY: 72, Resource.COVERAGE: 5}),
    Amount({Resource.ENERGY: 216, Resource.COVERAGE: 8}),
]


def main(args: Sequence[str]) -> None:
    config = GameConfiguration(
        Amount({Resource.MONEY: 10}),  # starting gold
        [Headquarters()],
        NUM_OPTIONS,
        NUM_SEASONS,
        ALL_ASSETS,
        REQUIRED_RESOURCES,
    )
    state = GameState(config)
    state.run()


if __name__ == "__main__":
    main(sys.argv[1:])
