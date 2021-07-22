#!/usr/bin/python3

import os
import unittest

from bootstrap import BuildAction, BuildPredicate, BuildUnit, FileExistsBuildPredicate, MakeDirectoryBuildAction

class AlwaysTrueBuildPredicate(BuildPredicate):
    def check(self) -> bool:
        return True


class FileExistsBuildPredicateTest(unittest.TestCase):
    def setUp(self) -> None:
        self.path = "./foobar.txt"

    def test_false_if_file_does_not_exist(self) -> None:
        predicate = FileExistsBuildPredicate(self.path)
        self.assertFalse(predicate.check())

    def test_true_if_file_exists(self) -> None:
        predicate = FileExistsBuildPredicate(self.path)
        open(self.path, "w").close()
        self.assertTrue(predicate.check())
        os.remove(self.path)


class MakeDirectoryBuildActionTest(unittest.TestCase):
    def test_creates_directory(self) -> None:
        path = "foobar"
        action = MakeDirectoryBuildAction(path)
        action.execute()

        self.assertTrue(os.path.exists(path))

        os.rmdir(path)


class SpyBuildAction(BuildAction):
    def __init__(self):
        self.called = False

    def execute(self) -> None:
        if not self.called:
            self.called = True
        else:
            raise AssertionError("SpyBuildAction already called")

    def assert_called(self) -> None:
        if not self.called:
            raise AssertionError("SpyBuildAction never called")


class SpyBuildActionTest(unittest.TestCase):
    def test_spy_build_action_not_called_by_default(self) -> None:
        action = SpyBuildAction()
        self.assertRaises(AssertionError, action.assert_called)

    def test_spy_build_action_called_once(self) -> None:
        action = SpyBuildAction()
        action.execute()
        action.assert_called()

    def test_spy_build_can_be_called_at_most_once(self) -> None:
        action = SpyBuildAction()
        action.execute()
        self.assertRaises(AssertionError, action.execute)


class BuildUnitTest(unittest.TestCase):
    def test_action_triggers_if_conditional_is_true(self) -> None:
        spy_action = SpyBuildAction()
        unit = BuildUnit(AlwaysTrueBuildPredicate(), spy_action)

        unit.build()

        spy_action.assert_called()


if __name__ == "__main__":
    unittest.main()
