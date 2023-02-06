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

    def print(self) -> None:
        print("\nLiquid funds:")
        for resource, amount in self.resources.items():
            print(f"{resource.name}: {amount}")

        print("\nYour pipeline:")
        for asset in self.assets:
            print(f"\t{str(asset)}")
