"""
Creates a MySQL database from the BGStats export.
This can then be queried for that juicy, juicy data.
"""
import sys
import textwrap
from argparse import ArgumentParser, Namespace
from pathlib import Path
from typing import List

from api import (BgStats, Game, Location, Play, Player, PlayerScore,
                 SqlTableEntry)
from mysql import connector


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

            for name, schema in zip(
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


def extract_fields(schema: str) -> str:
    fields = [
        field.split(" ")[0]
        for field in schema.split("\n")[1:-1]
        if "FOREIGN KEY" not in field and "AUTO_INCREMENT" not in field
    ]
    return ",".join(fields)


def cook_insert_entry_query(entry: SqlTableEntry) -> str:
    return textwrap.dedent(
        f"""
            INSERT INTO {type(entry).TABLE_NAME}
            ({extract_fields(type(entry).SQL_SCHEMA)})
            VALUES {entry.into_schema()}
        """
    )


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
            for player in bg_stats.players:
                cursor.execute(cook_insert_entry_query(player))
            for location in bg_stats.locations:
                cursor.execute(cook_insert_entry_query(location))
            for game in bg_stats.games:
                cursor.execute(cook_insert_entry_query(game))

            for play_id, play in enumerate(bg_stats.plays, start=1):
                cursor.execute(cook_insert_entry_query(play))

                for player_score in play.playerScores:
                    cursor.execute(
                        # We apply an extra `format()` to `PlayerScore` since we need to
                        # know the `play_id` for the play we've just committed.
                        cook_insert_entry_query(player_score).format(play_id)
                    )

            connection.commit()


if __name__ == "__main__":
    args = parse_args(sys.argv[1:])
    main(args)
