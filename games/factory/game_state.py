import random
from dataclasses import dataclass
from typing import Dict
from typing import List

from common import Printable
from game_systems import Asset
from game_types import Amount
from game_types import NewAssetLocation
from game_types import Resource

DELIMITER = "\n##################################################"

SELL_RATES = {
    Resource.WOOD: 1,
    Resource.COAL: 2,
    Resource.ENERGY: 4,
}


@dataclass
class GameConfiguration:
    starting_gold: int
    num_asset_choices: int
    num_seasons: int
    asset_options: List[Asset]
    required_resources: List[Amount]


class GameState(Printable):
    resources: Dict[Resource, int] = dict()
    assets: List[Asset] = list()

    asset_options: List[Asset]
    required_resources: List[Amount]
    num_asset_choices: int
    num_seasons: int

    current_season: int = 0
    current_level: int = 0

    def __init__(self, config: GameConfiguration) -> None:
        for resource in list(Resource):
            self.resources[resource] = 0

        self.resources[Resource.MONEY] = config.starting_gold
        self.num_asset_choices = config.num_asset_choices
        self.num_seasons = config.num_seasons
        self.asset_options = config.asset_options
        self.required_resources = config.required_resources

    def _required_resources(self) -> Amount:
        return self.required_resources[self.current_level]

    def _seasons_remaining(self) -> int:
        return self.num_seasons - self.current_season

    def _print_header(self) -> None:
        print(DELIMITER)
        print(
            f"\nYour town requires {self._required_resources()} in {self._seasons_remaining()} seasons (turns)."
        )

        print("\nResources:")
        for resource, amount in self.resources.items():
            print(f"\t{resource.name} ({resource}): {amount}")

        print("\nPipeline:")
        for asset in self.assets:
            print(f"\t{asset}")

    def _roll_and_print_asset_options(self) -> List[Asset]:
        options = random.sample(self.asset_options, self.num_asset_choices)

        options_text = "\n".join(
            [
                f"\t({i}) {option}"
                for i, option in zip(range(1, self.num_asset_choices + 1), options)
            ]
        )
        print(DELIMITER)
        print(f"\nAVAILABLE ASSETS\n{options_text}\n")

        return options

    def _get_asset_choice(self) -> int:
        while True:
            choice = input("Choose an asset: ")
            try:
                parsed_choice = int(choice)
                if parsed_choice < 0 or parsed_choice > self.num_asset_choices:
                    raise ValueError(f"Invalid asset choice: {parsed_choice}")
                return parsed_choice
            except Exception:
                print("INVALID INPUT\n")
                continue

    def _choose_asset_location(self) -> NewAssetLocation:
        while True:
            choice = input("Choose where to add the asset, (s)tart / (e)nd: ")

            if choice == "s" or choice == "start":
                return NewAssetLocation.START
            if choice == "e" or choice == "end":
                return NewAssetLocation.END
            print("INVALID INPUT\n")

    def _execute_pipeline(self) -> None:
        for asset in self.assets:
            # TODO: Invert these operations
            asset.execute(self)

    def _end_season(self) -> None:
        self.current_season += 1
        if self.current_season == self.num_seasons:
            self._end_year()

    def _end_year(self) -> None:
        print(DELIMITER)
        print(f"\nYEAR'S END: {self._required_resources()}{Resource.ENERGY} due.")
        if self.resources[Resource.ENERGY] < self._required_resources()[Resource.ENERGY]:
            print("Unsufficient energy.")
            print("\nGAME OVER")
            exit(1)
        else:
            self.resources[Resource.ENERGY] -= self._required_resources()[Resource.ENERGY]

        self.current_season = 0
        self.current_level += 1

    def _sell_resources(self) -> None:
        gained_funds = 0
        for resource, sell_rate in SELL_RATES.items():
            gained_funds += sell_rate * self.resources[resource]
            self.resources[resource] = 0

        print(f"Excess resources were sold for {gained_funds}{Resource.MONEY}")
        self.resources[Resource.MONEY] += gained_funds

    def run(self) -> None:
        while True:
            self._print_header()
            asset_options = self._roll_and_print_asset_options()
            asset_choice = self._get_asset_choice()

            selected_asset = asset_options[asset_choice - 1]
            location = self._choose_asset_location()
            if location == NewAssetLocation.START:
                self.assets = [selected_asset] + self.assets
            elif location == NewAssetLocation.END:
                self.assets.append(selected_asset)

            self._execute_pipeline()
            self._end_season()
