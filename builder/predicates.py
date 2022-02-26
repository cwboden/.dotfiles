import os
import subprocess
from typing import Protocol


class BuildPredicate(Protocol):
    """Used by a BuildUnit to check if it should build"""

    def check(self) -> bool:
        return False

    def __str__(self) -> str:
        return self.__class__.__name__


class AlwaysRunBuildPredicate(BuildPredicate):
    def check(self) -> bool:
        return False


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


class PythonModuleInstalledBuildPredicate(BuildPredicate):
    """Checks whether a provided Python module is installed"""

    def __init__(self, module: str):
        self.module = module

    def check(self) -> bool:
        installed_packages = subprocess.check_output(["pip", "list"]).decode("utf-8")
        return self.module in installed_packages
