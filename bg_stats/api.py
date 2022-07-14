import json
from pathlib import Path


class BgStats:
    def __init__(self, filename: Path):
        with open(filename) as raw_data:
            self.root = json.load(raw_data)
