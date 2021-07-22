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


class AlwaysTrueBuildPredicate(BuildPredicate):
    def check(self) -> bool:
        return True


class FileExistsBuildPredicate(BuildPredicate):
    """Checks whether the given file exists"""

    def __init__(self, path: str):
        self.path = path

    def check(self) -> bool:
        return os.path.exists(self.path)


class DirectoryExistsBuildPredicate(BuildPredicate):
    """Checks whether the given directory exists"""

    def __init__(self, path: str):
        self.path = path

    def check(self) -> bool:
        return os.path.isdir(self.path)


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
        subprocess.check_call(self.command)


class BuildUnit:
    def __init__(self, predicate: BuildPredicate, action: BuildAction):
        self.predicate = predicate
        self.action = action

    def build(self) -> None:
        if self.predicate.check():
            self.action.execute()


class MakeDirectoryBuildUnit(BuildUnit):
    """Creates the given directory if it does not exist"""

    def __init__(self, path: str):
        self.predicate = DirectoryExistsBuildPredicate(path)
        self.action = MakeDirectoryBuildAction(path)


class Builder:
    def __init__(self):
        self.units = []

    def add_unit(self, unit: BuildUnit) -> None:
        self.units.append(unit)

    def build(self) -> None:
        for unit in self.units:
            unit.build()


def main() -> None:
    builder = Builder()

    # Install Python dependencies
    builder.add_unit(
        BuildUnit(
            AlwaysTrueBuildPredicate(),
            RunShellCommandBuildAction(["pip", "install", "pre-commit"]),
        ),
    )

    # Install Git Hook for pre-commit
    builder.add_unit(
        BuildUnit(
            FileExistsBuildPredicate(".git/hooks/pre-commit"),
            RunShellCommandBuildAction(["pre-commit", "install"]),
        ),
    )

    # Create Vim folders
    home_dir = os.path.expanduser("~")
    builder.add_unit(MakeDirectoryBuildUnit(f"{home_dir}/.vim/"))
    for folder in ["swapfiles", "backups", "undodir"]:
        builder.add_unit(MakeDirectoryBuildUnit(f"{home_dir}/.vim/{folder}"))

    # Install VimPlug
    builder.add_unit(
        BuildUnit(
            FileExistsBuildPredicate(f"{home_dir}/.vim/autoload/plug.vim"),
            RunShellCommandBuildAction(
                [
                    "curl",
                    "-fLo",
                    f"{home_dir}/.vim/autoload/plug.vim",
                    "--create-dirs",
                    "https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim",
                ]
            ),
        ),
    )

    builder.build()


if __name__ == "__main__":
    main()
