#!/usr/bin/python
import os
import unittest

from temporary_filesystem import TemporaryFilesystem


class TemporaryFilesystemTest(unittest.TestCase):
    def test_creates_folder_on_init(self) -> None:
        fs = TemporaryFilesystem()
        self.assertTrue(os.path.exists("temporary_filesystem"))

    def test_removes_folder_on_free(self) -> None:
        fs = TemporaryFilesystem()
        del fs

        self.assertFalse(os.path.exists("temporary_filesystem"))


if __name__ == "__main__":
    unittest.main()
