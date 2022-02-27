#!/usr/bin/python3
import os
import sys
from typing import List

from colorama import Fore
from colorama import Style

from bootstrap.actions import MakeSymlinkBuildAction
from bootstrap.actions import RunShellCommandBuildAction
from bootstrap.predicates import AlwaysRunBuildPredicate
from bootstrap.predicates import DirectoryExistsBuildPredicate
from bootstrap.predicates import FileExistsBuildPredicate
from bootstrap.units import BuildUnit
from bootstrap.units import InstallSystemPackagesBuildUnit
from bootstrap.units import MakeDirectoryBuildUnit


class Builder:
    def __init__(self, io_out=sys.stdout):
        self.units = []
        self.errors = []
        self.io_out = io_out

    def add_unit(self, unit: BuildUnit) -> None:
        self.units.append(unit)

    def build(self) -> None:
        for unit in self.units:
            try:
                unit.build()
            except NotImplementedError as e:
                print(
                    Fore.YELLOW
                    + f"Missing Implementation for BuildUnit '{str(unit)}'"
                    + Style.RESET_ALL,
                    file=self.io_out,
                )
                self.errors.append(e)
            except Exception as e:
                print(
                    Fore.RED + f"Failure for BuildUnit '{str(unit)}'" + Style.RESET_ALL,
                    file=self.io_out,
                )
                self.errors.append(e)

        if self.errors:
            print(
                Fore.RED + "\nBuildUnit Failures:" + Style.RESET_ALL,
                file=self.io_out,
            )
            raise Exception(self.errors)


def install_common_dependencies(builder: Builder) -> None:
    """Installs common dependencies for Linux, Python, Git, etc."""

    builder.add_unit(InstallSystemPackagesBuildUnit())

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


def install_tmux(builder: Builder) -> None:
    # Create Tmux folders
    home_dir = os.path.expanduser("~")
    builder.add_unit(MakeDirectoryBuildUnit(f"{home_dir}/.tmux/"))
    builder.add_unit(MakeDirectoryBuildUnit(f"{home_dir}/.tmux/plugins"))

    # Install Tmux Plugin Manager (TPM)
    builder.add_unit(
        BuildUnit(
            DirectoryExistsBuildPredicate(f"{home_dir}/.tmux/plugins/tpm/"),
            RunShellCommandBuildAction(
                [
                    "git",
                    "clone",
                    "https://github.com/tmux-plugins/tpm",
                    f"{home_dir}/.tmux/plugins/tpm",
                ]
            ),
        ),
    )

    # Install Plugins
    builder.add_unit(
        BuildUnit(
            AlwaysRunBuildPredicate(),
            RunShellCommandBuildAction(
                [
                    f"{home_dir}/.tmux/plugins/tpm/bin/install_plugins",
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
    install_tmux(builder)

    builder.build()


if __name__ == "__main__":
    main()
