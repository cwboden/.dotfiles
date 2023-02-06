from enum import Enum
from typing import Set


class Resource(Enum):
    GOLD = 1
    COAL = 2
    WOOD = 3


class Amount:
    resources: Set[Resource]
