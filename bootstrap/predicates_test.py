#!/usr/bin/python3
import os
import unittest

from predicates import AlwaysRunBuildPredicate
from predicates import DirectoryExistsBuildPredicate
from predicates import FileExistsBuildPredicate
from predicates import PythonModuleInstalledBuildPredicate


class BuildPredicateTest(unittest.TestCase):
    def test_to_str(self) -> None:
        predicate = AlwaysRunBuildPredicate()
        self.assertEqual(
            str(predicate),
            "AlwaysRunBuildPredicate",
        )


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


if __name__ == "__main__":
    unittest.main()
