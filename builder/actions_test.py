#!/usr/bin/python3
import os
import unittest

from builder.actions import BuildAction
from builder.actions import MakeDirectoryBuildAction
from builder.actions import RunShellCommandBuildAction


class MakeDirectoryBuildActionTest(unittest.TestCase):
    def test_creates_directory(self) -> None:
        path = "foobar"
        action = MakeDirectoryBuildAction(path)
        action.execute()

        self.assertTrue(os.path.exists(path))

        os.rmdir(path)


class RunShellCommandBuildActionTest(unittest.TestCase):
    def test_shell_command_runs(self) -> None:
        path = "foobar"
        action = RunShellCommandBuildAction(["touch", path])
        action.execute()

        self.assertTrue(os.path.exists(path))

        os.remove(path)


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


if __name__ == "__main__":
    unittest.main()
