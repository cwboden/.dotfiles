#!/usr/bin/python3

import os
import sys

from typing import Protocol


class BuildPredicate(Protocol):
    """Used by a BuildUnit to check if it should build"""
    def check(self) -> bool:
        return False


class FileExistsBuildPredicate(BuildPredicate):
    """Checks whether the given file exists"""
    def __init__(self, path: str):
        self.path = path

    def check(self) -> bool:
        return os.path.exists(self.path)


class BuildAction(Protocol):
    """Used by a BuildUnit to run some code, create some files, etc. """
    def execute(self) -> None:
        return


class MakeDirectoryBuildAction(BuildAction):
    """Creates a directory with the given path"""
    def __init__(self, path: str):
        self.path = path

    def execute(self) -> None:
        os.mkdir(self.path)


class BuildUnit:
    def __init__(self, predicate: BuildPredicate, action: BuildAction):
        self.predicate = predicate
        self.action = action

    def build(self) -> None:
        if self.predicate.check():
            self.action.execute()


def main() -> None:
    pass


if __name__ == '__main__':
    main()
