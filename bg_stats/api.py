import json
from dataclasses import dataclass
from datetime import date as Date
from datetime import datetime as DateTime
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

    @property
    def play_date(self) -> Date:
        return DateTime.strptime(self.playDate, "%Y-%m-%d %H:%M:%S").date()


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

    def get_plays_at_location(self, location_id: int) -> List[Play]:
        return [play for play in self.plays if location_id == play.locationRefId]

    def get_plays_from_dates(
        self, start: Date = Date.min, end: Date = Date.max
    ) -> List[Play]:
        plays = []
        for play in self.plays:
            play_date = play.play_date
            if start <= play_date and play_date <= end:
                plays.append(play)

        return plays

    def get_play_date_earliest(self) -> Date:
        return self.plays[0].play_date

    def get_play_date_latest(self) -> Date:
        return self.plays[-1].play_date
