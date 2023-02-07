from typing import Dict
from typing import List

from common import Printable
from game_systems import Asset
from game_types import Resource


class GameState(Printable):
    resources: Dict[Resource, int] = dict()
    assets: List[Asset] = list()

    def __init__(self) -> None:
        for resource in list(Resource):
            self.resources[resource] = 0

    def __str__(self) -> str:
        statements = list()
        statements.append("\nLiquid funds:")
        for resource, amount in self.resources.items():
            statements.append(f"{resource.name} ({resource}): {amount}")

        statements.append("\nYour pipeline:")
        for asset in self.assets:
            statements.append(f"\t{asset}")

        return "\n".join(statements)
