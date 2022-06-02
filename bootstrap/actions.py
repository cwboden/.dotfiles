import os
import subprocess
from subprocess import CalledProcessError
from typing import List
from typing import Protocol


class ActionException(Exception):
    pass


class BuildAction(Protocol):
    """Used by a BuildUnit to run some code, create some files, etc."""

    def execute(self) -> None:
        return

    def __str__(self) -> str:
        return self.__class__.__name__


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
        completed_process = subprocess.run(self.command, capture_output=True)

        # Add stdout and stderr output if the call fails
        try:
            completed_process.check_returncode()
        except CalledProcessError as e:
            raise ActionException(
                f"{completed_process.args!r}\n"
                f"stdout: {completed_process.stdout!r}\n"
                f"stderr: {completed_process.stderr!r}"
            ) from e


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
