import json
import textwrap
from dataclasses import dataclass
from datetime import date as Date
from datetime import datetime as DateTime
from pathlib import Path
from typing import List
from typing import Optional
from typing import Protocol

from dacite import from_dict


class SqlTable(Protocol):
    TABLE_NAME: str
    SQL_SCHEMA: str


@dataclass
class Player(SqlTable):
    bggUsername: Optional[str]
    id: int
    isAnonymous: bool
    modificationDate: str
    name: str
    uuid: str

    TABLE_NAME = "players"
    SQL_SCHEMA = textwrap.dedent(
        """
        bgg_username VARCHAR(64),
        id INT PRIMARY KEY,
        is_anonymous BOOL,
        modification_date DATE,
        name VARCHAR(128),
        uuid VARCHAR(64)
    """
    )


@dataclass
class Location(SqlTable):
    id: int
    modificationDate: str
    name: str
    uuid: str

    TABLE_NAME = "locations"
    SQL_SCHEMA = textwrap.dedent(
        """
        id INT PRIMARY KEY,
        modification_date DATE,
        name VARCHAR(128),
        uuid VARCHAR(64)
    """
    )


@dataclass
class Game(SqlTable):
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

    TABLE_NAME = "games"
    SQL_SCHEMA = textwrap.dedent(
        """
        cooperative BOOL,
        highest_wins BOOL,
        id INT PRIMARY KEY,
        max_play_time INT,
        max_player_count INT,
        min_play_time INT,
        min_player_count INT,
        modification_date DATE,
        name VARCHAR(128),
        no_points BOOL,
        rating INT,
        uses_teams BOOL,
        uuid VARCHAR(64)
    """
    )


@dataclass
class Play(SqlTable):
    board: Optional[str]
    comments: Optional[str]
    durationMin: int
    entryDate: str
    gameRefId: int
    ignored: bool
    locationRefId: int
    modificationDate: str
    playDate: str
    playerScores: List["PlayerScore"]
    rounds: int
    scoringSetting: int
    usesTeams: bool
    uuid: str

    TABLE_NAME = "plays"
    SQL_SCHEMA = textwrap.dedent(
        f"""
        board VARCHAR(128),
        comments VARCHAR(256),
        duration_min INT,
        entry_date DATE,
        game_ref_id INT,
        FOREIGN KEY(game_ref_id) REFERENCES {Game.TABLE_NAME}(id),
        id INT AUTO_INCREMENT PRIMARY KEY,
        ignored BOOL,
        location_ref_id INT,
        FOREIGN KEY(location_ref_id) REFERENCES {Location.TABLE_NAME}(id),
        modification_date DATE,
        play_date DATE,
        rounds INT,
        scoring_setting INT,
        uses_teams BOOL,
        uuid VARCHAR(64)
    """
    )

    @property
    def player_ids(self) -> List[int]:
        return [score.playerRefId for score in self.playerScores]

    @property
    def play_date(self) -> Date:
        return DateTime.strptime(self.playDate, "%Y-%m-%d %H:%M:%S").date()


@dataclass
class PlayerScore(SqlTable):
    newPlayer: bool
    playerRefId: int
    role: Optional[str]
    score: Optional[str]
    startPlayer: bool
    winner: bool

    TABLE_NAME = "player_scores"
    SQL_SCHEMA = textwrap.dedent(
        f"""
        id INT AUTO_INCREMENT PRIMARY KEY,
        new_player BOOL,
        play_ref_id INT,
        FOREIGN KEY(play_ref_id) REFERENCES {Play.TABLE_NAME}(id),
        player_ref_id INT,
        FOREIGN KEY(player_ref_id) REFERENCES {Player.TABLE_NAME}(id),
        role VARCHAR(64),
        score INT,
        start_player BOOL,
        winner BOOL
    """
    )


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
