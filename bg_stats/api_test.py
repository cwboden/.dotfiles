#!/bin/usr/python3
import unittest
from datetime import date as Date
from pathlib import Path

from api import BgStats

PATH_TO_DATA: Path = Path("BGStatsExport-for-test.json")


class PlayTest(unittest.TestCase):
    def test_get_player_ids(self) -> None:
        play = BgStats.from_file(PATH_TO_DATA).plays[0]
        self.assertEqual(
            [33, 1, 2],
            play.player_ids,
        )


class BgStatsTest(unittest.TestCase):
    def setUp(self) -> None:
        self.stats = BgStats.from_file(PATH_TO_DATA)

    def test_data_is_loaded(self) -> None:
        self.assertIsNotNone(self.stats)
        self.assertIsNotNone(self.stats.players)
        self.assertIsNotNone(self.stats.locations)
        self.assertIsNotNone(self.stats.games)
        self.assertIsNotNone(self.stats.plays)

    def test_get_plays_for_player(self) -> None:
        player_id = 5
        expected_game_ids = [
            "a51fc10c-bdb1-48dc-9bac-6c6afd9101fd",
            "7c4aa11b-aeae-4fbf-8daa-2d78ebd64684",
            "f2b8dac7-ae0e-432f-af30-9e9bdc3178c2",
            "ba1e377d-86d1-46d7-809b-a7212b5e86b8",
            "8bcf2fce-4793-48a6-8b29-9d3b3a0eda6e",
            "875f81b6-af85-4fbc-9d3d-5c0586b07b19",
            "86e87a88-8a54-492c-a8ce-cfe1627685e1",
            "465f9ead-f766-4c47-add2-a3671a8a26b5",
            "b645bafc-c5b7-45ac-89fb-a7c3f0e2bf28",
            "912406ae-5cd2-4969-b7b3-cf726e07f8b6",
            "24891e12-1f02-4287-9b54-33b0edeab13f",
            "722338d6-2af8-4350-ba51-cd868a0b026f",
            "4862a6c7-6fd5-4d65-8542-fa09b22bebfa",
            "e7137e8a-bff1-44b9-97e7-93e1d009de52",
        ]

        actual_game_ids = [
            game.uuid for game in self.stats.get_plays_for_player(player_id)
        ]

        self.assertEqual(
            expected_game_ids,
            actual_game_ids,
        )

    def test_get_plays_at_location(self) -> None:
        location_id = 3

        actual_game_ids = [
            game.uuid for game in self.stats.get_plays_at_location(location_id)
        ]

        self.assertEqual(
            ["e7137e8a-bff1-44b9-97e7-93e1d009de52"],
            actual_game_ids,
        )

    def test_get_plays_from_dates_start_only(self) -> None:
        start = Date.fromisoformat("2019-09-07")

        actual_game_ids = [
            game.uuid for game in self.stats.get_plays_from_dates(start=start)
        ]

        self.assertEqual(
            actual_game_ids,
            ["e7137e8a-bff1-44b9-97e7-93e1d009de52"],
        )

    def test_get_plays_from_dates_end_only(self) -> None:
        end = Date.fromisoformat("2019-08-01")

        actual_game_ids = [
            game.uuid for game in self.stats.get_plays_from_dates(end=end)
        ]

        self.assertEqual(
            actual_game_ids,
            ["cbe6f05e-6c36-47c9-93c5-99da0fd30fc9"],
        )

    def test_get_plays_from_dates_full_range(self) -> None:
        start = Date.fromisoformat("2019-08-13")
        end = Date.fromisoformat("2019-08-20")

        actual_game_ids = [
            game.uuid for game in self.stats.get_plays_from_dates(start, end)
        ]

        self.assertEqual(
            actual_game_ids,
            [
                "06c7b4ce-e12a-48a7-b087-5a2eb6767bab",
                "8bcf2fce-4793-48a6-8b29-9d3b3a0eda6e",
            ],
        )

    def test_get_play_date_earliest(self) -> None:
        self.assertEqual(
            self.stats.get_play_date_earliest(),
            Date.fromisoformat("2019-08-01"),
        )

    def test_get_play_date_latest(self) -> None:
        self.assertEqual(
            self.stats.get_play_date_latest(),
            Date.fromisoformat("2019-09-07"),
        )


if __name__ == "__main__":
    unittest.main()
