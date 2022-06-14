import platform
import subprocess

import distro

from bootstrap.actions import BuildAction
from bootstrap.actions import MakeDirectoryBuildAction
from bootstrap.actions import RunShellCommandBuildAction
from bootstrap.predicates import BuildPredicate
from bootstrap.predicates import DirectoryExistsBuildPredicate
from bootstrap.predicates import PythonModuleInstalledBuildPredicate


class BuildUnit:
    def __init__(self, predicate: BuildPredicate, action: BuildAction):
        self.predicate = predicate
        self.action = action

    def build(self) -> None:
        if not self.predicate.check():
            self.action.execute()

    def __str__(self) -> str:
        return f"{self.__class__.__name__}: {{ {str(self.predicate)} -> {str(self.action)} }}"


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

    def __str__(self) -> str:
        return (
            f"{self.__class__.__name__}: {{ {self.system}-{self.linux_distribution} }}"
        )

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
                    )
            else:
                raise NotImplementedError(
                    f"Bootstrap not yet supported on {self.linux_distribution}!"
                )
        else:
            raise NotImplementedError(f"Bootstrap not yet supported on {self.system}!")


class SaboteurBuildUnitException(Exception):
    def __init__(self) -> None:
        super().__init__("SaboteurBuildUnit called")


class SaboteurBuildUnit(BuildUnit):
    """Raises an Exception, no matter what"""

    def __init__(self) -> None:
        pass

    def __str__(self) -> str:
        return self.__class__.__name__

    def build(self) -> None:
        raise SaboteurBuildUnitException()
