#!/usr/bin/python3
import sys
from argparse import ArgumentParser
from argparse import Namespace
from datetime import timedelta as TimeDelta
from pathlib import Path
from typing import List

from matplotlib import dates as Dates
from matplotlib import pyplot

from bg_stats.api import BgStats


def parse_args(args: List[str]) -> Namespace:
    parser = ArgumentParser(
        description="Show graphs of board game insights based on BGStats (bgstatsapp.com) data"
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


def main(args: Namespace) -> None:
    stats = BgStats.from_file(args.path_to_data)

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

    pyplot.tight_layout()
    pyplot.savefig(args.output_path)


if __name__ == "__main__":
    args = parse_args(sys.argv[1:])
    main(args)
