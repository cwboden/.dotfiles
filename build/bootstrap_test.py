#!/usr/bin/python3
import os
import shutil
import subprocess
import unittest
from pathlib import Path

from parameterized import parameterized

from build.bootstrap import AlwaysRunBuildPredicate
from build.bootstrap import BuildAction
from build.bootstrap import Builder
from build.bootstrap import BuildPredicate
from build.bootstrap import BuildUnit
from build.bootstrap import crawl_for_symlink_sources
from build.bootstrap import create_symlinks
from build.bootstrap import DirectoryExistsBuildPredicate
from build.bootstrap import FileExistsBuildPredicate
from build.bootstrap import InstallSystemPackagesBuildUnit
from build.bootstrap import MakeDirectoryBuildAction
from build.bootstrap import PythonModuleInstalledBuildPredicate
from build.bootstrap import RunShellCommandBuildAction
from build.bootstrap import translate_symlink_to_destination


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


class PythonModuleInstalledBuildPredicateTest(unittest.TestCase):
    def test_false_if_module_not_installed(self) -> None:
        predicate = PythonModuleInstalledBuildPredicate("foobar")
        self.assertFalse(predicate.check())

    def test_true_if_module_installed(self) -> None:
        predicate = PythonModuleInstalledBuildPredicate("pre-commit")
        self.assertTrue(predicate.check())


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
        unit = BuildUnit(AlwaysRunBuildPredicate(), spy_action)

        unit.build()

        spy_action.assert_called()


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


class BuilderTest(unittest.TestCase):
    def test_all_build_units_complete(self) -> None:
        builder = Builder()
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


class SymlinkDotFilesTest(unittest.TestCase):
    def test_crawl_for_symlink_sources(self) -> None:
        os.mkdir("foo")
        Path("foo/bar.symlink").touch()
        Path("foo/baz.txt").touch()
        Path("foo/bat.wav").touch()

        os.mkdir("foo/fah")
        Path("foo/fah/bir.symlink").touch()

        actual_sources = crawl_for_symlink_sources("foo")
        self.assertEqual(
            [
                "foo/bar.symlink",
                "foo/fah/bir.symlink",
            ],
            actual_sources,
        )

        shutil.rmtree("foo")

    def test_translate_symlink_to_destination(self) -> None:
        dest_dir = "/root"
        self.assertEqual(
            translate_symlink_to_destination("/home/user/file.symlink", dest_dir),
            f"{dest_dir}/.file",
        )

    def test_create_symlinks(self) -> None:
        os.mkdir("fiz")
        Path("fiz/bar.symlink").touch()
        os.mkdir("fiz/fah")
        Path("fiz/fah/bir.symlink").touch()

        dest_dir = "test_home"
        os.mkdir(dest_dir)

        builder = Builder()
        create_symlinks(builder, "fiz", dest_dir)
        builder.build()

        for root, dirs, files in os.walk(dest_dir):
            self.assertIn(".bar", files)
            self.assertIn(".bir", files)

        shutil.rmtree("fiz")
        shutil.rmtree(dest_dir)


if __name__ == "__main__":
    unittest.main()
