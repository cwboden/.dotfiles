#!/usr/bin/python3
import os
import shutil
import unittest
from pathlib import Path

from builder import Builder
from main import crawl_for_symlink_sources
from main import create_symlinks
from main import translate_symlink_to_destination

DEVNULL = open(os.devnull, "w")


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
