#!/usr/bin/python3
import re
import shutil
import sys
from argparse import ArgumentDefaultsHelpFormatter, ArgumentParser, Namespace
from datetime import date
from pathlib import Path
from typing import List

from git import Repo

TEMPLATE_PATH: Path = Path(__file__).parent.joinpath("blog-post-template.md")


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


def kebab_case(s: str) -> str:
    # Split "snake_case" apart into "snake case"
    underscore = re.compile(r"_")
    s = underscore.sub(" ", s)

    # Remove punctuation
    punctuation = re.compile(r"[?!.,\'\"]")
    s = punctuation.sub(" ", s)

    # Shorten multiple spaces into one
    multispace = re.compile(r"\s+")
    s = multispace.sub(" ", s)

    spaces = re.compile(r" ")
    return spaces.sub("-", s.strip()).lower()


def main(args: Namespace, repo: Repo) -> Path:
    today = date.today().strftime("%Y-%m-%d")
    title = kebab_case(args.title)

    path_to_file = Path(f"{args.path}/{today}-{title}.md")

    if path_to_file.exists():
        raise Exception(f"Cannot create new post '{path_to_file}'. Post exists.")

    branch_name = f"posts/{today}-{title}"
    if repo.is_dirty():
        raise Exception(
            f"Cannot create git branch: '{branch_name}'. Repo has outstanding changes."
        )

    repo.heads.main.checkout()
    repo.create_head(branch_name).checkout()

    shutil.copyfile(TEMPLATE_PATH, path_to_file)
    return path_to_file


if __name__ == "__main__":
    args = parse_args(sys.argv[1:])
    repo = Repo(Path.home().joinpath(".dotfiles"))
    main(args, repo)
