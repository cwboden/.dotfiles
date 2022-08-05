"""
Creates a MySQL database from the BGStats export.
This can then be queried for that juicy, juicy data.
"""
import sys
from argparse import ArgumentParser
from argparse import Namespace
from typing import List

from mysql import connector


def parse_args(args: List[str]) -> Namespace:
    parser = ArgumentParser(
        description="Create a MySQL database from a BGStats export."
    )

    parser.add_argument(
        "-U",
        "--username",
        type=str,
        required=True,
        help="username of the MySQL database",
    )

    parser.add_argument(
        "-P",
        "--password",
        type=str,
        required=True,
        help="password of the MySQL database",
    )

    return parser.parse_args(args)


def main(args: Namespace) -> None:
    db = connector.connect(
        host="localhost",
        user=args.username,
        passwd=args.password,
    )


if __name__ == "__main__":
    args = parse_args(sys.argv[1:])
    main(args)
