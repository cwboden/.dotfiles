#!/usr/bin/python3
import os
import shutil
import subprocess
import unittest
from pathlib import Path

from build.bootstrap import AlwaysRunBuildPredicate
from build.bootstrap import BuildAction
from build.bootstrap import Builder
from build.bootstrap import BuildPredicate
from build.bootstrap import BuildUnit
from build.bootstrap import crawl_for_symlink_sources
from build.bootstrap import create_symlinks
from build.bootstrap import DirectoryExistsBuildPredicate
from build.bootstrap import FileExistsBuildPredicate
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

    def test_zsh_installed(self) -> None:
        self.assertTrue(os.path.exists("/tmp/zsh_installer.sh"))
        self.assertTrue(os.path.isdir(f"{self.home_dir}/.oh-my-zsh"))
        self.assertTrue(
            os.path.isdir(f"{self.home_dir}/.oh-my-zsh/custom/themes/powerlevel10k")
        )

    def test_symlinks_created(self) -> None:
        self.assertTrue(os.path.exists(f"{self.home_dir}/.hgrc"))
        self.assertTrue(os.path.exists(f"{self.home_dir}/.vimrc"))
        self.assertTrue(os.path.exists(f"{self.home_dir}/.zshrc"))
        self.assertTrue(os.path.exists(f"{self.home_dir}/.gitconfig"))
        self.assertTrue(os.path.exists(f"{self.home_dir}/.tmux.conf"))


if __name__ == "__main__":
    unittest.main()
