#!/usr/bin/python3
import os
import shutil
import unittest
from pathlib import Path

from bootstrap.actions import SpyBuildAction
from bootstrap.bootstrap import Builder
from bootstrap.bootstrap import crawl_for_symlink_sources
from bootstrap.bootstrap import create_symlinks
from bootstrap.bootstrap import translate_symlink_to_destination
from bootstrap.predicates import AlwaysRunBuildPredicate
from bootstrap.units import BuildUnit
from bootstrap.units import SaboteurBuildUnit

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

        builder = Builder(io_out=DEVNULL)
        create_symlinks(builder, "fiz", dest_dir)
        builder.build()

        for root, dirs, files in os.walk(dest_dir):
            self.assertIn(".bar", files)
            self.assertIn(".bir", files)

        shutil.rmtree("fiz")
        shutil.rmtree(dest_dir)


if __name__ == "__main__":
    unittest.main()
