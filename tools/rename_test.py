#!/usr/bin/python3
import os
import random
import string
import textwrap
import unittest
from io import StringIO
from typing import List
from typing import Union

from fs import open_fs
from fs.base import FS
from fs.subfs import SubFS

from tools import rename

FILE_1_CONTENTS = """
old_name
oldName
OldName
OLD_NAME
prefixed_old_name
old_name_suffixed
"""

FILE_1_CONTENTS_EXPECTED = """
old_name
newName
OldName
OLD_NAME
prefixed_old_name
old_name_suffixed
"""

FILE_2_CONTENTS = """
new_name
newName
NewName
NEW_NAME
prefixed_new_name
new_name_suffixed
"""

FILE_3_CONTENTS = FILE_1_CONTENTS + FILE_2_CONTENTS


class RenameTest(unittest.TestCase):
    def setUp(self) -> None:
        random_suffix = "".join(random.choice(string.ascii_letters) for _ in range(10))
        self.fs_dir_name = f"tmp_{random_suffix}"
        self.fs = open_fs(f"./{self.fs_dir_name}", create=True)

        # Layout for Test:
        #
        # tmp_{random_suffix}
        #  ├── bar
        #  │   ├── 1.txt
        #  │   ├── 2.txt
        #  │   └── 3.txt
        #  ├── foo
        #  │   ├── baz
        #  │   │   ├── 1.txt
        #  │   │   ├── 2.txt
        #  │   │   └── 3.txt
        #  │   ├── 1.txt
        #  │   ├── 2.txt
        #  │   └── 3.txt
        #  ├── 1.txt
        #  ├── 2.txt
        #  └── 3.txt

        file_dirs: List[Union[FS, SubFS[FS]]] = [
            self.fs.makedir(dir_name) for dir_name in ["foo/", "bar/", "foo/baz"]
        ]
        file_dirs.append(self.fs)

        for file_dir in file_dirs:
            for file_name, contents in zip(
                ["1.txt", "2.txt", "3.txt"],
                [FILE_1_CONTENTS, FILE_2_CONTENTS, FILE_3_CONTENTS],
            ):
                file_dir.writetext(file_name, contents)

    def tearDown(self) -> None:
        self.fs.removetree("/")
        self.fs.close()
        os.rmdir(self.fs_dir_name)

    def test_setup(self) -> None:
        output_tree = StringIO()
        self.fs.tree(file=output_tree)

        output_tree.seek(0)
        expected_tree = output_tree.read()

        self.assertEqual(
            expected_tree,
            # XXX: I couldn't get this to work using `textwrap.dedent()`, but this looks
            # kinda gross, not readable
            "|-- bar\n|   |-- 1.txt\n|   |-- 2.txt\n|   `-- 3.txt\n|-- foo\n"
            "|   |-- baz\n|   |   |-- 1.txt\n|   |   |-- 2.txt\n|   |   `-- 3.txt\n"
            "|   |-- 1.txt\n|   |-- 2.txt\n|   `-- 3.txt\n|-- 1.txt\n|-- 2.txt\n"
            "`-- 3.txt\n",
        )

    def test_rename_within_one_file(self) -> None:
        file_path = "1.txt"
        args = rename.parse_args(
            ["oldName", "newName", f"{self.fs_dir_name}/{file_path}"]
        )
        rename.main(args)

        self.assertEqual(self.fs.readtext(file_path), FILE_1_CONTENTS_EXPECTED)

    def test_rename_ignores_prefixes_and_suffixes(self) -> None:
        file_path = "1.txt"
        args = rename.parse_args(
            ["old_name", "new_name", f"{self.fs_dir_name}/{file_path}"]
        )
        rename.main(args)

        self.assertEqual(
            self.fs.readtext(file_path),
            textwrap.dedent(
                """
                new_name
                oldName
                OldName
                OLD_NAME
                prefixed_old_name
                old_name_suffixed
            """
            ),
        )

    def test_rename_matches_case(self) -> None:
        file_path = "1.txt"
        args = rename.parse_args(
            ["oldName", "newName", f"{self.fs_dir_name}/{file_path}"]
        )
        rename.main(args)

        self.assertEqual(
            self.fs.readtext(file_path),
            FILE_1_CONTENTS_EXPECTED,
        )

    def test_rename_searches_recursively(self) -> None:
        args = rename.parse_args(["oldName", "newName", f"{self.fs_dir_name}/"])
        rename.main(args)

        for file_path in ["1.txt", "foo/1.txt", "foo/baz/1.txt", "bar/1.txt"]:
            self.assertEqual(
                self.fs.readtext(file_path),
                FILE_1_CONTENTS_EXPECTED,
            )


if __name__ == "__main__":
    unittest.main()
