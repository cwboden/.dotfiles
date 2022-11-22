#!/usr/bin/python
import os
import unittest

from temporary_filesystem import TemporaryFilesystem


class TemporaryFilesystemTest(unittest.TestCase):
    def setUp(self) -> None:
        self.fs = TemporaryFilesystem()

    def test_creates_folder_on_init(self) -> None:
        self.assertTrue(os.path.exists("/tmp/temporary_filesystem"))

    def test_removes_folder_on_free(self) -> None:
        del self.fs
        self.assertFalse(os.path.exists("/tmp/temporary_filesystem"))

    def test_translate_prepends_temp_fs_root(self) -> None:
        self.assertEqual(
            self.fs.translate("foo/bar"),
            "/tmp/temporary_filesystem/foo/bar",
        )

    def test_touch_creates_file(self) -> None:
        filename = "foobar.txt"
        self.fs.touch(filename)

        self.assertTrue(os.path.exists(self.fs.translate(filename)))

    def test_touch_no_error_if_file_exists(self) -> None:
        filename = "foobar.txt"
        self.fs.touch(filename)

        # We should not see an error, we are just touching it again
        self.fs.touch(filename)

    def test_exists_true_if_file_present(self) -> None:
        filename = "foobar.txt"
        self.fs.touch(filename)

        self.assertTrue(self.fs.exists(filename))

    def test_mkdir_creates_directory(self) -> None:
        dirname = "foobar"
        self.fs.mkdir(dirname)

        self.assertTrue(self.fs.exists(dirname))


if __name__ == "__main__":
    unittest.main()
