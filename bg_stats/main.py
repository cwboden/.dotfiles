#!/usr/bin/python3
import sys
import textwrap
from argparse import ArgumentParser, Namespace
from datetime import timedelta as TimeDelta
from enum import Enum
from pathlib import Path
from typing import List

from matplotlib import dates as Dates
from matplotlib import pyplot

from api import BgStats


class OutputFormat(Enum):
    GAMES_BY_MONTH = "GAMES_BY_MONTH"
    GAMES_BY_PLAYS = "GAMES_BY_PLAYS"


def parse_args(args: List[str]) -> Namespace:
    parser = ArgumentParser(
        description=textwrap.dedent("""
            Show graphs of board game insights based on BGStats (bgstatsapp.com)
            path_to_data
        """)
    )

    parser.add_argument(
        "output_format",
        type=OutputFormat,
        choices=list(OutputFormat),
        help="which type of graph to create",
    )

    parser.add_argument(
        "path_to_data",
        type=Path,
        nargs="?",
        default="BGStatsExport.json",
        help="the exported data, in JSON",
    )

    parser.add_argument(
        "output_path",
        type=Path,
        nargs="?",
        default="figure.svg",
        help="where to save the generated graph",
    )

    return parser.parse_args(args)


def plot_games_by_month(stats: BgStats) -> None:
    month_start = stats.get_play_date_earliest()
    month_start.replace(day=1)

    month_end = stats.get_play_date_latest()
    month_end.replace(day=1)

    plays_by_month = []
    months = []
    month_current = month_start
    while month_current < month_end:
        # Since months are not fixed length, we have to reset days each time
        month_next = month_current + TimeDelta(days=30)
        month_next.replace(day=1)

        num_plays = len(stats.get_plays_from_dates(month_current, month_next))
        plays_by_month.append(num_plays)
        months.append(month_current)

        month_current = month_next

    pyplot.gca().xaxis.set_major_formatter(Dates.DateFormatter("%Y-%m"))
    pyplot.gca().xaxis.set_major_locator(Dates.MonthLocator(interval=3))
    pyplot.xticks(rotation=45, ha="right")

    pyplot.plot(months, plays_by_month)
    pyplot.ylabel("Plays by Month")


def plot_games_by_plays(stats: BgStats) -> None:
    num_games = len(stats.games)
    num_plays = [0] * num_games
    for play in stats.plays:
        # `gameRefId`s are 1-indexed
        num_plays[play.gameRefId - 1] += 1

    # Sort bar chart positions by number of plays
    positions = [
        index
        for _, index in sorted(
            zip(num_plays, range(num_games)), key=lambda pair: pair[0]
        )
    ]

    #  for game, num_plays_for_game, position in zip(stats.games, num_plays, positions):
    #      print(game.name, num_plays_for_game, position)

    pyplot.barh(list(range(num_games)), num_plays)
    pyplot.yticks(ticks=positions, labels=[game.name for game in stats.games])


def main(args: Namespace) -> None:
    stats = BgStats.from_file(args.path_to_data)

    if args.output_format == OutputFormat.GAMES_BY_MONTH:
        plot_games_by_month(stats)
    if args.output_format == OutputFormat.GAMES_BY_PLAYS:
        plot_games_by_plays(stats)

    pyplot.tight_layout()
    pyplot.savefig(args.output_path)


if __name__ == "__main__":
    args = parse_args(sys.argv[1:])
    main(args)
