"""
Creates a MySQL database from the BGStats export.
This can then be queried for that juicy, juicy data.
"""
import sys
import textwrap
from argparse import ArgumentParser
from argparse import Namespace
from pathlib import Path
from typing import List

from mysql import connector

from bg_stats.api import BgStats
from bg_stats.api import Game
from bg_stats.api import Location
from bg_stats.api import Play
from bg_stats.api import Player
from bg_stats.api import PlayerScore


def parse_args(args: List[str]) -> Namespace:
    parser = ArgumentParser(
        description="Create a MySQL database from a BGStats export."
    )

    parser.add_argument(
        "path_to_data",
        type=Path,
        nargs="?",
        default="BGStatsExport.json",
        help="the exported data, in JSON",
    )

    parser.add_argument(
        "-u",
        "--username",
        type=str,
        required=True,
        help="username of the MySQL database",
    )

    parser.add_argument(
        "-p",
        "--password",
        type=str,
        required=True,
        help="password of the MySQL database",
    )

    parser.add_argument(
        "-d",
        "--database",
        type=str,
        required=True,
        help="which MySQL database to connect to (or create, if not available)",
    )

    return parser.parse_args(args)


def init_database_if_new(args: Namespace) -> None:
    with connector.connect(
        host="localhost",
        user=args.username,
        passwd=args.password,
    ) as connection:
        with connection.cursor(buffered=True) as cursor:
            cursor.execute("SHOW DATABASES")
            # Need to use tuple, due to form returned by cursor
            if (args.database,) not in cursor:
                cursor.execute(f"CREATE DATABASE {args.database}")

    with connector.connect(
        host="localhost",
        user=args.username,
        passwd=args.password,
        database=args.database,
    ) as connection:
        with connection.cursor(buffered=True) as cursor:
            cursor.execute("SHOW TABLES")
            existing_tables = [table for (table,) in cursor]

            for (name, schema) in zip(
                [
                    Player.TABLE_NAME,
                    Location.TABLE_NAME,
                    Game.TABLE_NAME,
                    Play.TABLE_NAME,
                    PlayerScore.TABLE_NAME,
                ],
                [
                    Player.SQL_SCHEMA,
                    Location.SQL_SCHEMA,
                    Game.SQL_SCHEMA,
                    Play.SQL_SCHEMA,
                    PlayerScore.SQL_SCHEMA,
                ],
            ):
                if name not in existing_tables:
                    create_table_query = textwrap.dedent(
                        f"""
                        CREATE TABLE {name}(
                            {schema}
                        )
                    """
                    )
                    cursor.execute(create_table_query)


def main(args: Namespace) -> None:
    init_database_if_new(args)
    bg_stats = BgStats.from_file(args.path_to_data)

    with connector.connect(
        host="localhost",
        user=args.username,
        passwd=args.password,
        database=args.database,
    ) as connection:
        with connection.cursor(buffered=True) as cursor:
            # Insert Players
            player_fields = [
                field.split(" ")[0] for field in Player.SQL_SCHEMA.split("\n")[1:-1]
            ]
            for player in bg_stats.players:
                insert_player_query = textwrap.dedent(
                    f"""
                    INSERT INTO {Player.TABLE_NAME}
                    ({",".join(player_fields)})
                    VALUES {player.into_schema()}
                """
                )
                cursor.execute(insert_player_query)

            connection.commit()


if __name__ == "__main__":
    args = parse_args(sys.argv[1:])
    main(args)
