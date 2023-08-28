#!/usr/bin/python3
import os
from typing import List

from actions import MakeSymlinkBuildAction
from builder import Builder
from predicates import FileExistsBuildPredicate
from units import BuildUnit, MakeDirectoryBuildUnit


def install_nvim(builder: Builder) -> None:
    # Symlink `init.vim` into `nvim/`
    home_dir = os.path.expanduser("~")
    config_dir = f"{home_dir}/.config/nvim"
    builder.add_unit(MakeDirectoryBuildUnit(config_dir))
    vim_init_install_path = f"{config_dir}/init.vim"
    builder.add_unit(
        BuildUnit(
            FileExistsBuildPredicate(vim_init_install_path),
            MakeSymlinkBuildAction(
                os.path.abspath("../nvim/init.vim"),
                vim_init_install_path,
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

    home_dir = os.path.expanduser("~")
    create_symlinks(builder, "../", home_dir)

    install_nvim(builder)

    builder.build()


if __name__ == "__main__":
    main()
