import random
from dataclasses import dataclass
from typing import List

from common import Printable
from game_systems import Asset
from game_types import Amount
from game_types import NewAssetLocation
from game_types import Resource

DELIMITER = (
    "\n###############################################################################"
)


@dataclass
class GameConfiguration:
    starting_resources: Amount
    num_asset_choices: int
    num_seasons: int
    asset_options: List[Asset]
    required_resources: List[Amount]


class GameState(Printable):
    resources: Amount
    assets: List[Asset] = list()

    asset_options: List[Asset]
    required_resources: List[Amount]
    num_asset_choices: int
    num_seasons: int

    current_season: int = 0
    current_level: int = 0

    def __init__(self, config: GameConfiguration) -> None:
        self.resources = config.starting_resources
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
        for resource, amount in self.resources:
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

    def _get_asset_choice(self, asset_options: List[Asset]) -> Asset:
        while True:
            choice = input("Choose an asset: ")
            try:
                parsed_choice = int(choice)
                if parsed_choice < 0 or parsed_choice > self.num_asset_choices:
                    raise ValueError(f"Invalid asset choice: {parsed_choice}")
                # TODO: Payments should become its own system
                if (
                    self.resources[Resource.MONEY]
                    < asset_options[parsed_choice].cost[Resource.MONEY]
                ):
                    raise ValueError(f"Not enough {Resource.MONEY}")

                # Account for 1-indexing
                return asset_options[parsed_choice - 1]

            except Exception as err:
                print(f"INVALID INPUT: {err}\n")
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
        if not self.can_pay_for(self._required_resources()):
            print("Unsufficient energy.")
            print("\nGAME OVER")
            exit(1)
        else:
            self.pay_for(self._required_resources())

        self.current_season = 0
        self.current_level += 1

    # XXX: Abandoned in lieu of player choice
    #  def _sell_resources(self) -> None:
    #      gained_funds = 0
    #      for resource, sell_rate in SELL_RATES.items():
    #          gained_funds += sell_rate * self.resources[resource]
    #          self.resources[resource] = 0
    #
    #      print(f"Excess resources were sold for {gained_funds}{Resource.MONEY}")
    #      self.resources[Resource.MONEY] += gained_funds

    def can_pay_for(self, amount: Amount) -> bool:
        for (resource, quantity) in amount:
            if self.resources[resource] < quantity:
                return False

        return True

    def pay_for(self, amount: Amount) -> None:
        if not self.can_pay_for(amount):
            raise Exception(f"Cannot pay: {amount}")

        self.resources.sub(amount)

    def run(self) -> None:
        while True:
            self._print_header()
            asset_options = self._roll_and_print_asset_options()
            selected_asset = self._get_asset_choice(asset_options)

            location = self._choose_asset_location()
            if location == NewAssetLocation.START:
                self.assets = [selected_asset] + self.assets
            elif location == NewAssetLocation.END:
                self.assets.append(selected_asset)

            self._execute_pipeline()
            self._end_season()
