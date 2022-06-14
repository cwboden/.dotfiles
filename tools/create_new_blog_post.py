#!/usr/bin/python3
import sys
from argparse import ArgumentDefaultsHelpFormatter
from argparse import ArgumentParser
from argparse import Namespace
from datetime import date
from pathlib import Path
from typing import List


def parse_args(args: List[str]) -> Namespace:
    parser = ArgumentParser(
        description="Create a new blog post for GitHub Pages in Jekyll format",
        formatter_class=ArgumentDefaultsHelpFormatter,
    )

    parser.add_argument("title", type=str, help="the name of the blog post")
    parser.add_argument(
        "path",
        type=Path,
        nargs="?",
        default=Path.home().joinpath(".dotfiles/docs/_posts/"),
        help="where to create the new post",
    )

    return parser.parse_args(args)


def main(args: Namespace):
    today = date.today().strftime("%Y-%m-%d")
    path_to_file = Path(f"{args.path}/{today}-{args.title}.md")
    path_to_file.touch()


if __name__ == "__main__":
    args = parse_args(sys.argv[1:])
    main(args)
