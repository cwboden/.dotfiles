import unittest
from pathlib import Path

from bg_stats.api import BgStats


class BgStatsTest(unittest.TestCase):
    def test_data_is_loaded(self) -> None:
        stats = BgStats(Path("BGStatsExport.json"))
        self.assertIsNotNone(stats.root)


if __name__ == "__main__":
    unittest.main()
