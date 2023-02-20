from enum import auto
from enum import Enum
from typing import Iterator
from typing import MutableMapping
from typing import Tuple

import emoji


class Resource(Enum):
    # Core resources
    MONEY = auto()
    COVERAGE = auto()
    ENERGY = auto()
    # TODO: Eventually add pollution values to all producers
    POLLUTION = auto()

    # Materials
    COAL = auto()
    FOSSIL_GAS = auto()
    METAL = auto()
    STEAM = auto()
    WOOD = auto()

    # Research
    PUBLICATIONS = auto()
    SCIENTISTS = auto()

    # TODO
    RADIOACTIVE = auto()

    # Abandoned Types
    ## Oil isn't a major part of power generation
    CRUDE_OIL = auto()
    REFINED_OIL = auto()
    PLASTICS = auto()

    def __str__(self) -> str:
        emoji_eqivalent = {
            Resource.MONEY: ":coin:",
            Resource.COVERAGE: ":electric_plug:",
            Resource.ENERGY: ":high_voltage:",
            Resource.POLLUTION: ":no_smoking:",
            Resource.COAL: ":rock:",
            Resource.FOSSIL_GAS: ":dashing_away:",
            Resource.METAL: ":building_construction:",
            Resource.CRUDE_OIL: ":droplet:",
            Resource.REFINED_OIL: ":oil_drum:",
            Resource.RADIOACTIVE: ":radioactive:",
            Resource.STEAM: ":wind_face:",
            Resource.WOOD: ":wood:",
            Resource.PLASTICS: ":water_pistol:",
            Resource.PUBLICATIONS: ":page_facing_up:",
            Resource.SCIENTISTS: ":scientist:",
        }

        return emoji.emojize(emoji_eqivalent[self])

    def __lt__(self, other: "Resource") -> bool:
        return self.value < other.value


class Amount:
    amount: MutableMapping[Resource, int]

    def __init__(self, amount: MutableMapping[Resource, int]) -> None:
        self.amount = amount

    def __iter__(self) -> Iterator[Tuple[Resource, int]]:
        return iter(self.amount.items())

    def __getitem__(self, resource: Resource) -> int:
        return self.amount[resource]

    def __str__(self) -> str:
        resources = list(self.amount.items())
        resources.sort(key=lambda kvp: kvp[0])
        resource_strings = ", ".join(
            [f"{quantity}{resource}" for (resource, quantity) in resources]
        )
        return f"[ {resource_strings} ]"

    def __format__(self, format_spec: str) -> str:
        return format(str(self), format_spec)

    def add(self, other: "Amount"):
        for (resource, quantity) in other:
            self.amount[resource] += quantity

    def sub(self, other: "Amount"):
        for (resource, quantity) in other:
            self.amount[resource] -= quantity


class NewAssetLocation(Enum):
    START = auto()
    END = auto()
