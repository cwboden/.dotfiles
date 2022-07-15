import json
from pathlib import Path
from typing import Any
from typing import List
from typing import TypedDict


class Player(TypedDict):
    uuid: str
    id: int
    name: str
    isAnonymous: bool
    modificationDate: str
    bggUsername: str


class Location(TypedDict):
    uuid: str
    id: int
    name: str
    modificationDate: str


class Game(TypedDict):
    uuid: str
    id: int
    name: str
    modificationDate: str
    cooperative: bool
    highestWins: bool
    noPoints: bool
    usesTeams: bool
    urlThumb: str
    urlImage: str
    bggName: str
    bggYear: int
    bggId: int
    designers: str
    metadata: str
    isBaseGame: int
    isExpansion: int
    rating: int
    minPlayerCount: int
    maxPlayerCount: int
    minPlayTime: int
    maxPlayTime: int
    minAge: int
    preferredImage: int
    copies: List[Any]


class Play(TypedDict):
    uuid: str
    modificationDate: str
    entryDate: str
    playDate: str
    usesTeams: bool
    durationMin: int
    ignored: bool
    manualWinner: bool
    rounds: int
    bggId: int
    bggLastSync: str
    importPlayId: int
    locationRefId: int
    gameRefId: int
    board: str
    comments: str
    rating: int
    nemestatsId: int
    scoringSetting: int
    metaData: str
    playerScores: List[Any]
    expansionPlays: List[Any]


class BgStats:
    def __init__(self, filename: Path):
        with open(filename) as raw_data:
            self.root = json.load(raw_data)

    @property
    def players(self) -> List[Player]:
        return self.root["players"]

    @property
    def locations(self) -> List[Location]:
        return self.root["locations"]

    @property
    def games(self) -> List[Game]:
        return self.root["games"]

    @property
    def plays(self) -> List[Play]:
        return self.root["plays"]
