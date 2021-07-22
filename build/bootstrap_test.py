#!/usr/bin/python3
import os
import subprocess
import unittest

from bootstrap import AlwaysTrueBuildPredicate
from bootstrap import BuildAction
from bootstrap import Builder
from bootstrap import BuildPredicate
from bootstrap import BuildUnit
from bootstrap import DirectoryExistsBuildPredicate
from bootstrap import FileExistsBuildPredicate
from bootstrap import MakeDirectoryBuildAction
from bootstrap import RunShellCommandBuildAction


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


class DirectoryExistsBuildPredicateTest(unittest.TestCase):
    def setUp(self) -> None:
        self.path = "foobar/"

    def test_false_if_file_does_not_exist(self) -> None:
        predicate = DirectoryExistsBuildPredicate(self.path)
        self.assertFalse(predicate.check())

    def test_true_if_file_exists(self) -> None:
        predicate = DirectoryExistsBuildPredicate(self.path)
        os.mkdir(self.path)
        self.assertTrue(predicate.check())
        os.rmdir(self.path)


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


class BuildUnitTest(unittest.TestCase):
    def test_action_triggers_if_conditional_is_true(self) -> None:
        spy_action = SpyBuildAction()
        unit = BuildUnit(AlwaysTrueBuildPredicate(), spy_action)

        unit.build()

        spy_action.assert_called()


class BuilderTest(unittest.TestCase):
    def test_all_build_units_complete(self) -> None:
        builder = Builder()
        spy_actions = []

        for _ in range(3):
            spy_action = SpyBuildAction()
            spy_actions.append(spy_action)
            builder.add_unit(
                BuildUnit(
                    AlwaysTrueBuildPredicate(),
                    spy_action,
                ),
            )

        builder.build()

        for spy_action in spy_actions:
            spy_action.assert_called()


class BootstrapIntegrationTest(unittest.TestCase):
    """
    Assumes Bootstrap has been run on the system. Will check to ensure certain
    dependencies are available.
    """

    def setUp(self) -> None:
        self.home_dir = os.path.expanduser("~")

    def test_pre_commit_git_hook_installed(self) -> None:
        subprocess.check_call(["pre-commit", "run", "--all-files"])

    def test_vim_folders_are_created(self) -> None:
        self.assertTrue(os.path.isdir(f"{self.home_dir}/.vim/swapfiles"))
        self.assertTrue(os.path.isdir(f"{self.home_dir}/.vim/backups"))
        self.assertTrue(os.path.isdir(f"{self.home_dir}/.vim/undodir"))

    def test_vim_plug_installed(self) -> None:
        self.assertTrue(os.path.exists(f"{self.home_dir}/.vim/autoload/plug.vim"))
        self.assertTrue(os.path.exists(f"{self.home_dir}/.vim/plugged"))


if __name__ == "__main__":
    unittest.main()