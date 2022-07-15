import json
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from typing import List

from dacite import from_dict


@dataclass
class Player:
    uuid: str
    id: int
    name: str
    isAnonymous: bool
    modificationDate: str
    bggUsername: str = ""


@dataclass
class Location:
    uuid: str
    id: int
    name: str
    modificationDate: str


@dataclass
class Game:
    uuid: str
    id: int
    name: str
    modificationDate: str
    cooperative: bool
    highestWins: bool
    noPoints: bool
    usesTeams: bool
    bggYear: int
    bggId: int
    rating: int
    minPlayerCount: int
    maxPlayerCount: int
    minPlayTime: int
    maxPlayTime: int
    minAge: int
    preferredImage: int
    copies: List[Any]
    urlThumb: str = ""
    urlImage: str = ""
    bggName: str = ""
    designers: str = ""
    metadata: str = ""
    isBaseGame: int = 0
    isExpansion: int = 0


@dataclass
class PlayerScore:
    score: str
    winner: bool
    newPlayer: bool
    startPlayer: bool
    playerRefId: int
    role: str
    rank: int
    seatOrder: int


@dataclass
class Play:
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
    importPlayId: int
    locationRefId: int
    gameRefId: int
    rating: int
    nemestatsId: int
    scoringSetting: int
    metaData: str
    playerScores: List[Any]
    expansionPlays: List[Any]
    board: str = ""
    comments: str = ""
    bggLastSync: str = ""


@dataclass
class BgStats:
    players: List[Player]
    locations: List[Location]
    games: List[Game]
    plays: List[Play]

    @staticmethod
    def from_file(filename: Path) -> "BgStats":
        with open(filename) as raw_data:
            return from_dict(BgStats, json.load(raw_data))
