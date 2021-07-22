#!/usr/bin/python3
import os
import subprocess
import sys
from typing import List
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


class RunShellCommandBuildAction(BuildAction):
    """Runs a command in the console"""

    def __init__(self, command: List[str]):
        self.command = command

    def execute(self) -> None:
        subprocess.run(self.command)


class BuildUnit:
    def __init__(self, predicate: BuildPredicate, action: BuildAction):
        self.predicate = predicate
        self.action = action

    def build(self) -> None:
        if self.predicate.check():
            self.action.execute()


class Builder:
    def __init__(self):
        self.units = []

    def add_unit(self, unit: BuildUnit) -> None:
        self.units.append(unit)

    def build(self) -> None:
        for unit in self.units:
            unit.build()


def main() -> None:
    pass


if __name__ == "__main__":
    main()
