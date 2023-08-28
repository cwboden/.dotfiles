import textwrap

from actions import (BuildAction, MakeDirectoryBuildAction,
                     RunShellCommandBuildAction)
from predicates import (BuildPredicate, DirectoryExistsBuildPredicate,
                        PythonModuleInstalledBuildPredicate)


class BuildUnit:
    def __init__(self, predicate: BuildPredicate, action: BuildAction):
        self.predicate = predicate
        self.action = action

    def build(self) -> None:
        if not self.predicate.check():
            self.action.execute()

    def __str__(self) -> str:
        return textwrap.dedent(f"""
            {self.__class__.__name__}: {{ {str(self.predicate)} -> {str(self.action)} }}
        """)


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
