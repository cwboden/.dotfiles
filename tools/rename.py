#!/usr/bin/python3
import subprocess
import sys
from argparse import ArgumentParser
from argparse import Namespace
from pathlib import Path
from subprocess import CalledProcessError
from subprocess import CompletedProcess
from typing import List


def parse_args(args: List[str]) -> Namespace:
    parser = ArgumentParser(
        description="Rename via string-matching recursively across a directory or file."
    )

    parser.add_argument("old_name", type=str, help="the string you want to be replaced")
    parser.add_argument(
        "new_name", type=str, help="the string you want to replace with"
    )
    parser.add_argument(
        "path",
        type=Path,
        nargs="?",
        default=".",
        help="the path to recursively search through",
    )

    return parser.parse_args(args)


class CalledProcessWithStackError(Exception):
    def __init__(self, process: CompletedProcess) -> None:
        super().__init__(
            f"{process.args!r}\n"
            f"stdout: {process.stdout}\n"
            f"stderr: {process.stderr}"
        )


def main(args: Namespace):
    completed_rg = subprocess.run(
        ["rg", "-l", args.old_name, args.path], capture_output=True
    )
    try:
        completed_rg.check_returncode()
    except CalledProcessError as e:
        raise CalledProcessWithStackError(completed_rg) from e

    files_to_check = completed_rg.stdout.decode("utf-8").split("\n")[:-1]
    for file_name in files_to_check:
        completed_sed = subprocess.run(
            ["sed", "-i", f"s/{args.old_name}/{args.new_name}/g", file_name],
            capture_output=True,
        )
        try:
            completed_sed.check_returncode()
        except CalledProcessError as e:
            raise CalledProcessWithStackError(completed_sed) from e


if __name__ == "__main__":
    args = parse_args(sys.argv[1:])
    main(args)
