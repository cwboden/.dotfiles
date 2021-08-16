import os
import subprocess
from typing import List
from typing import Protocol


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


class MakeSymlinkBuildAction(BuildAction):
    """Creates a link from source to destination"""

    def __init__(self, source_path: str, dest_path: str):
        self.source_path = source_path
        self.dest_path = dest_path

    def execute(self) -> None:
        os.symlink(self.source_path, self.dest_path)


class SpyBuildAction(BuildAction):
    def __init__(self):
        self.called = False

    def execute(self) -> None:
        if not self.called:
            self.called = True
        else:
            raise AssertionError("SpyBuildAction already called")

    def assert_called(self) -> None:
        if not self.called:
            raise AssertionError("SpyBuildAction never called")
