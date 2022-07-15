import json
from dataclasses import dataclass
from pathlib import Path
from typing import List
from typing import Optional

from dacite import from_dict


@dataclass
class Player:
    bggUsername: Optional[str]
    id: int
    isAnonymous: bool
    modificationDate: str
    name: str
    uuid: str


@dataclass
class Location:
    id: int
    modificationDate: str
    name: str
    uuid: str


@dataclass
class Game:
    cooperative: bool
    highestWins: bool
    id: int
    maxPlayTime: int
    maxPlayerCount: int
    minPlayTime: int
    minPlayerCount: int
    modificationDate: str
    name: str
    noPoints: bool
    rating: int
    usesTeams: bool
    uuid: str


@dataclass
class PlayerScore:
    newPlayer: bool
    playerRefId: int
    rank: int
    role: Optional[str]
    score: Optional[str]
    startPlayer: bool
    winner: bool


@dataclass
class Play:
    board: Optional[str]
    comments: Optional[str]
    durationMin: int
    entryDate: str
    gameRefId: int
    ignored: bool
    locationRefId: int
    modificationDate: str
    playDate: str
    playerScores: List[PlayerScore]
    rounds: int
    scoringSetting: int
    usesTeams: bool
    uuid: str

    @property
    def player_ids(self) -> List[int]:
        return [score.playerRefId for score in self.playerScores]


@dataclass
class BgStats:
    games: List[Game]
    locations: List[Location]
    players: List[Player]
    plays: List[Play]

    @staticmethod
    def from_file(filename: Path) -> "BgStats":
        with open(filename) as raw_data:
            return from_dict(BgStats, json.load(raw_data))

    def get_plays_for_player(self, player_id: int) -> List[Play]:
        return [play for play in self.plays if player_id in play.player_ids]
