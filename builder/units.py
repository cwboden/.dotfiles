import os
import platform
import subprocess
import sys
from typing import List
from typing import Protocol

import distro

from builder.actions import BuildAction
from builder.actions import MakeDirectoryBuildAction
from builder.actions import MakeSymlinkBuildAction
from builder.actions import RunShellCommandBuildAction
from builder.predicates import AlwaysRunBuildPredicate
from builder.predicates import BuildPredicate
from builder.predicates import DirectoryExistsBuildPredicate
from builder.predicates import FileExistsBuildPredicate
from builder.predicates import PythonModuleInstalledBuildPredicate


class BuildUnit:
    def __init__(self, predicate: BuildPredicate, action: BuildAction):
        self.predicate = predicate
        self.action = action

    def build(self) -> None:
        if not self.predicate.check():
            self.action.execute()


class MakeDirectoryBuildUnit(BuildUnit):
    """Creates the given directory if it does not exist"""

    def __init__(self, path: str):
        self.predicate = DirectoryExistsBuildPredicate(path)
        self.action = MakeDirectoryBuildAction(path)


class InstallPythonModuleBuildUnit(BuildUnit):
    """Installs a Python module if it isn't yet installed"""

    def __init__(self, module: str):
        self.predicate = PythonModuleInstalledBuildPredicate(module)
        self.action = RunShellCommandBuildAction(["pip", "install", module])


class InstallSystemPackagesBuildUnit(BuildUnit):
    """Installs packages to whichever distro of Linux is being run"""

    def __init__(
        self, system: str = platform.system(), linux_distribution: str = distro.id()
    ):
        self.system = system.lower()
        self.linux_distribution = linux_distribution.lower()

        with open("dependencies.txt", "r") as dependencies_file:
            self.dependencies = [
                dependency.strip() for dependency in dependencies_file.readlines()
            ]

    def build(self) -> None:
        if self.system == "linux":
            if self.linux_distribution == "ubuntu":
                installed_packages = subprocess.check_output(["dpkg", "-l"]).decode(
                    "utf-8"
                )
                uninstalled_dependencies = [
                    dependency
                    for dependency in self.dependencies
                    if dependency not in installed_packages
                ]

                if uninstalled_dependencies:
                    subprocess.check_call(
                        ["sudo", "apt", "install"] + uninstalled_dependencies,
                        stdout=subprocess.DEVNULL,
                        stderr=subprocess.DEVNULL,
                    )
            else:
                raise NotImplementedError(
                    f"Bootstrap not yet supported on {self.linux_distribution}!"
                )
        else:
            raise NotImplementedError(f"Bootstrap not yet supported on {self.system}!")
