import random
import sys
import textwrap
from typing import Sequence

from game_state import GameState
from game_types import Resource
from units.simple import CoalMine
from units.simple import Forest
from units.simple import Kiln
from units.simple import PowerPlant


DELIMITER = "#############################################"

ALL_OPTIONS = [Forest(), CoalMine(), Kiln(), PowerPlant()]
NUM_OPTIONS = 3

REQUIRED_ENERGY = [4, 6, 10, 16, 26, 42, 68, 111]
NUM_ROUNDS = 4


def main(args: Sequence[str]) -> None:
    state = GameState()

    # Start with 5 initial gold
    state.resources[Resource.GOLD] = 5

    round_number = 0
    level_number = 0
    required_energy = REQUIRED_ENERGY[level_number]

    while True:
        print(
            textwrap.dedent(
                f"""
            {DELIMITER}

            Your town requires {required_energy}{Resource.ENERGY} in {NUM_ROUNDS - round_number} seasons (turns).
            """
            )
        )

        print(f"{state}")

        options = random.sample(ALL_OPTIONS, NUM_OPTIONS)

        options_text = "\n".join(
            [
                f"\t({i}) {option}"
                for i, option in zip(range(1, NUM_OPTIONS + 1), options)
            ]
        )
        print(f"\nAVAILABLE ASSETS\n{options_text}\n")

        choice = input("Choose an asset: ")

        try:
            parsed_choice = int(choice)
        except Exception:
            print("INVALID INPUT")
            continue

        state.assets.append(options[int(choice) - 1])

        for asset in state.assets:
            asset.execute(state)

        round_number += 1
        if round_number == NUM_ROUNDS:
            print(f"\n{DELIMITER}")
            print(f"\nYEARS END: {required_energy}{Resource.ENERGY} due.")
            if state.resources[Resource.ENERGY] < required_energy:
                print("Unsufficient energy.")
                print("\nGAME OVER")
                break
            else:
                state.resources[Resource.ENERGY] -= required_energy

                round_number = 0
                level_number += 1
                required_energy = REQUIRED_ENERGY[level_number]


if __name__ == "__main__":
    main(sys.argv[1:])
