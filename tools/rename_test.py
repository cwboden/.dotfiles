#!/usr/bin/python3
import os
import random
import string
import unittest
from io import StringIO
from typing import List
from typing import Union

from fs import open_fs
from fs.base import FS
from fs.base import SubFS

FILE_1_CONTENTS = """
old_name
oldName
OldName
OLD_NAME
"""

FILE_2_CONTENTS = """
new_name
newName
NewName
NEW_NAME
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


if __name__ == "__main__":
    unittest.main()
