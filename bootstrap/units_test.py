#!/usr/bin/python3
import unittest
from unittest.mock import patch

from parameterized import parameterized

from bootstrap.actions import SpyBuildAction
from bootstrap.predicates import AlwaysRunBuildPredicate
from bootstrap.units import BuildUnit
from bootstrap.units import InstallSystemPackagesBuildUnit


class BuildUnitTest(unittest.TestCase):
    def test_action_triggers_if_conditional_is_true(self) -> None:
        spy_action = SpyBuildAction()
        unit = BuildUnit(AlwaysRunBuildPredicate(), spy_action)

        unit.build()

        spy_action.assert_called()

    def test_to_str(self) -> None:
        spy_action = SpyBuildAction()
        unit = BuildUnit(AlwaysRunBuildPredicate(), spy_action)

        self.assertEqual(
            str(unit), "BuildUnit: { AlwaysRunBuildPredicate -> SpyBuildAction }"
        )


class InstallSystemPackagesBuildUnitTest(unittest.TestCase):
    def test_build_possible_on_current_platform(self) -> None:
        unit = InstallSystemPackagesBuildUnit()
        unit.build()

    @parameterized.expand(["Windows", "OSX", "non-existant-system"])
    def test_build_not_possible_on_system(self, system: str) -> None:
        unit = InstallSystemPackagesBuildUnit(system=system)
        self.assertRaises(NotImplementedError, unit.build)

    @parameterized.expand(["Arch", "Kali", "Red Hat"])
    def test_build_not_possible_on_linux_distribution(
        self, linux_distribution: str
    ) -> None:
        unit = InstallSystemPackagesBuildUnit(linux_distribution=linux_distribution)
        self.assertRaises(NotImplementedError, unit.build)

    @patch("subprocess.check_call")
    def test_skips_already_installed_dependencies(
        self, subprocess_mock: unittest.mock.MagicMock
    ) -> None:
        unit = InstallSystemPackagesBuildUnit()

        # Definitely installed already
        unit.dependencies = ["python"]

        unit.build()
        subprocess_mock.assert_not_called()


if __name__ == "__main__":
    unittest.main()
