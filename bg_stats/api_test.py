import unittest
from pathlib import Path

from bg_stats.api import BgStats


PATH_TO_DATA: Path = Path("BGStatsExport-for-test.json")


class BgStatsTest(unittest.TestCase):
    def setUp(self) -> None:
        self.stats = BgStats.from_file(PATH_TO_DATA)

    def test_data_is_loaded(self) -> None:
        self.assertIsNotNone(self.stats)
        self.assertIsNotNone(self.stats.players)
        self.assertIsNotNone(self.stats.locations)
        self.assertIsNotNone(self.stats.games)
        self.assertIsNotNone(self.stats.plays)


if __name__ == "__main__":
    unittest.main()
