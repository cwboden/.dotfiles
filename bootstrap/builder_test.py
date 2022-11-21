#!/usr/bin/python3
import os
import unittest

from actions import SpyBuildAction
from builder import Builder
from predicates import AlwaysRunBuildPredicate
from units import BuildUnit
from units import SaboteurBuildUnit

DEVNULL = open(os.devnull, "w")


class BuilderTest(unittest.TestCase):
    def test_all_build_units_complete(self) -> None:
        builder = Builder(io_out=DEVNULL)
        spy_actions = []

        for _ in range(3):
            spy_action = SpyBuildAction()
            spy_actions.append(spy_action)
            builder.add_unit(
                BuildUnit(
                    AlwaysRunBuildPredicate(),
                    spy_action,
                ),
            )

        builder.build()

        for spy_action in spy_actions:
            spy_action.assert_called()

    def test_all_errors_are_thrown_at_end(self) -> None:
        builder = Builder(io_out=DEVNULL)

        num_errors = 3
        for _ in range(num_errors):
            builder.add_unit(SaboteurBuildUnit())

        spy_action = SpyBuildAction()
        builder.add_unit(
            BuildUnit(
                AlwaysRunBuildPredicate(),
                spy_action,
            ),
        )

        try:
            builder.build()
        except Exception as e:
            self.assertEqual(len(e.args[0]), num_errors)

        spy_action.assert_called()


if __name__ == "__main__":
    unittest.main()
