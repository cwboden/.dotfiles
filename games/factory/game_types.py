from enum import Enum
from typing import Set
from typing import Tuple


class Resource(Enum):
    GOLD = 1
    COAL = 2
    WOOD = 3


Amount = Set[Tuple[Resource, int]]
