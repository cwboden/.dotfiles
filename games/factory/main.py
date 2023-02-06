import sys
import textwrap
from typing import Sequence

from game_state import GameState
from game_types import Resource
from units.simple import CoalMine
from units.simple import Forest
from units.simple import Kiln
from units.simple import PowerPlant


def main(args: Sequence[str]) -> None:
    state = GameState()

    # Start with 5 initial gold
    state.resources[Resource.GOLD] = 5

    while True:
        print("\n#############################################")

        state.print()
        choice = input(
            textwrap.dedent(
                """
            (1) Forest      | -[$1] -> +[2 wood]
            (2) CoalMine    | -[$1] -> +[2 coal]
            (3) Kiln        | -[1 coal, 1 wood] -> +[4 coal]
            (4) Power Plant | -[2 coal] -> +[$4]

            Choose an asset: """
            )
        )

        if choice == "1":
            state.assets.append(Forest())
        elif choice == "2":
            state.assets.append(CoalMine())
        elif choice == "3":
            state.assets.append(Kiln())
        elif choice == "4":
            state.assets.append(PowerPlant())

        for asset in state.assets:
            asset.execute(state)


if __name__ == "__main__":
    main(sys.argv[1:])
