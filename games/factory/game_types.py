from enum import auto
from enum import Enum
from typing import Set
from typing import Tuple

import emoji


class Resource(Enum):
    # Core resources
    MONEY = auto()
    COVERAGE = auto()
    ENERGY = auto()
    POLLUTION = auto()

    # Materials
    COAL = auto()
    METAL = auto()
    OIL = auto()
    RADIOACTIVE = auto()
    STEAM = auto()
    WOOD = auto()

    # Research
    PUBLICATIONS = auto()
    SCIENTISTS = auto()

    def __str__(self) -> str:
        emoji_eqivalent = {
            Resource.MONEY: ":coin:",
            Resource.COVERAGE: ":electric_plug:",
            Resource.ENERGY: ":high_voltage:",
            Resource.POLLUTION: ":no_smoking:",
            Resource.COAL: ":rock:",
            Resource.METAL: ":building_construction:",
            Resource.OIL: ":oil_drum:",
            Resource.RADIOACTIVE: ":radioactive:",
            Resource.STEAM: ":dashing_away:",
            Resource.WOOD: ":wood:",
            Resource.PUBLICATIONS: ":page_facing_up:",
            Resource.SCIENTISTS: ":scientist:",
        }

        return emoji.emojize(emoji_eqivalent[self])


class Amount:
    amount: Set[Tuple[Resource, int]]

    def __init__(self, amount: Set[Tuple[Resource, int]]) -> None:
        self.amount = amount

    def __str__(self) -> str:
        resources = list(self.amount)
        resources.sort(key=lambda kvp: kvp[0])
        resource_strings = ", ".join(
            [f"{quantity}{resource}" for (resource, quantity) in resources]
        )
        return f"[ {resource_strings} ]"


class NewAssetLocation(Enum):
    START = auto()
    END = auto()
