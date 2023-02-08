from enum import auto
from enum import Enum
from typing import Set
from typing import Tuple

import emoji


class Resource(Enum):
    GOLD = auto()
    WOOD = auto()
    COAL = auto()
    OIL = auto()
    ENERGY = auto()

    def __str__(self) -> str:
        if self == Resource.GOLD:
            return emoji.emojize(":coin:")
        elif self == Resource.COAL:
            return emoji.emojize(":rock:")
        elif self == Resource.WOOD:
            return emoji.emojize(":wood:")
        elif self == Resource.OIL:
            return emoji.emojize(":oil_drum:")
        elif self == Resource.ENERGY:
            return emoji.emojize(":high_voltage:")

        raise ValueError(f"Invalid Resource value: {self}")


Amount = Set[Tuple[Resource, int]]


class NewAssetLocation(Enum):
    START = auto()
    END = auto()
