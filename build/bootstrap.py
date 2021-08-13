#!/usr/bin/python3
import distro
import platform
import os
import subprocess
import sys
from typing import List
from typing import Protocol


class BuildPredicate(Protocol):
    """Used by a BuildUnit to check if it should build"""

    def check(self) -> bool:
        return False


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

    def __init__(self, system: str = platform.system(), linux_distribution: str = distro.id()):
        self.system = system
        self.linux_distribution = linux_distribution

    def build(self) -> None:
        if self.system == 'Linux':
            if self.linux_distribution == 'Ubuntu':
                subprocess.check_call(["sudo", "apt", "install", "`cat dependencies.txt`"])
            else:
                raise NotImplemented("Bootstrap only supported on Ubuntu!")
        else:
            raise NotImplemented("Bootstrap only supported on Linux!")


class Builder:
    def __init__(self):
        self.units = []

    def add_unit(self, unit: BuildUnit) -> None:
        self.units.append(unit)

    def build(self) -> None:
        for unit in self.units:
            unit.build()


def install_common_dependencies(builder: Builder) -> None:
    """Installs common dependencies for Linux, Python, Git, etc."""

    builder.add_unit(InstallSystemPackagesBuildUnit())

    # Install Python dependencies
    for module in [
        "mypy",
        "pre-commit",
        "types-setuptools",
    ]:
        builder.add_unit(InstallPythonModuleBuildUnit(module))

    # Have mypy install type stubs for any external libraries
    builder.add_unit(
        BuildUnit(
            AlwaysRunBuildPredicate(),
            RunShellCommandBuildAction(
                ["python", "-m", "mypy", "--install-types", "./"]
            ),
        ),
    )

    # Install Git Hook for pre-commit
    builder.add_unit(
        BuildUnit(
            FileExistsBuildPredicate(".git/hooks/pre-commit"),
            RunShellCommandBuildAction(["pre-commit", "install"]),
        ),
    )


def install_vim(builder: Builder) -> None:
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
    builder.add_unit(
        BuildUnit(
            DirectoryExistsBuildPredicate(f"{home_dir}/.vim/plugged"),
            RunShellCommandBuildAction(
                [
                    "vim",
                    "+PlugInstall",
                    "+qa",
                ]
            ),
        ),
    )


def install_zsh(builder: Builder, home_dir: str) -> None:
    zsh_installer_path = "/tmp/zsh_installer.sh"

    # Download installer
    builder.add_unit(
        BuildUnit(
            FileExistsBuildPredicate(zsh_installer_path),
            RunShellCommandBuildAction(
                [
                    "wget",
                    "-O",
                    zsh_installer_path,
                    "https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh",
                ]
            ),
        ),
    )

    # Run installer
    builder.add_unit(
        BuildUnit(
            DirectoryExistsBuildPredicate(f"{home_dir}/.oh-my-zsh"),
            RunShellCommandBuildAction(
                [
                    "sh",
                    zsh_installer_path,
                    "--unattended",
                    "--keep-zshrc",
                ]
            ),
        ),
    )

    # Install powerlevel10k theme
    builder.add_unit(
        BuildUnit(
            DirectoryExistsBuildPredicate(
                f"{home_dir}/.oh-my-zsh/custom/themes/powerlevel10k"
            ),
            RunShellCommandBuildAction(
                [
                    "git",
                    "clone",
                    "--depth=1",
                    "https://github.com/romkatv/powerlevel10k.git",
                    f"{home_dir}/.oh-my-zsh/custom/themes/powerlevel10k",
                ]
            ),
        ),
    )


def crawl_for_symlink_sources(start_directory: str) -> List[str]:
    """
    Crawls through the start directory and all subdirectories for files that
    end with `.symlink`
    """

    sources = set()
    for root, dirs, files in os.walk(start_directory):
        for name in files:
            if name.endswith(".symlink"):
                sources.add(os.path.join(root, name))

    sorted_order = list(sources)
    sorted_order.sort()
    return sorted_order


def translate_symlink_to_destination(symlink: str, destination: str) -> str:
    """
    Finds the translation from a *.symlink file to where its symlink should
    live in the home directory. This follows a straightforward pattern:
      1. The `.symlink` suffix will be removed
      2. It will start with a `.`
    """

    file_name = symlink.split("/")[-1].replace(".symlink", "")
    return f"{destination}/.{file_name}"


def create_symlinks(builder: Builder, source_dir: str, dest_dir: str) -> None:
    """
    Crawl through the source_dir for any files that end in ".symlink" and
    create symlinks to them in the dest_dir
    """

    sources = crawl_for_symlink_sources(source_dir)
    destinations = [
        translate_symlink_to_destination(source, dest_dir) for source in sources
    ]

    for source, destination in zip(sources, destinations):
        builder.add_unit(
            BuildUnit(
                FileExistsBuildPredicate(destination),
                MakeSymlinkBuildAction(os.path.abspath(source), destination),
            ),
        )


def main() -> None:
    builder = Builder()

    install_common_dependencies(builder)

    home_dir = os.path.expanduser("~")
    install_zsh(builder, home_dir)
    create_symlinks(builder, "./", home_dir)

    install_vim(builder)

    builder.build()


if __name__ == "__main__":
    main()
