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


class SqlTableEntry(Protocol):
    TABLE_NAME: str
    SQL_SCHEMA: str

    def into_schema(self) -> str:
        raise NotImplementedError


@dataclass
class Player(SqlTableEntry):
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
        modification_date DATETIME,
        name VARCHAR(128),
        uuid VARCHAR(64)
    """
    )

    def into_schema(self) -> str:
        return textwrap.dedent(
            f"""(
            '{self.bggUsername}',
            {self.id},
            {self.isAnonymous},
            '{self.modificationDate}',
            \"{self.name}\",
            '{self.uuid}'
        )"""
        )


@dataclass
class Location(SqlTableEntry):
    id: int
    modificationDate: str
    name: str
    uuid: str

    TABLE_NAME = "locations"
    SQL_SCHEMA = textwrap.dedent(
        """
        id INT PRIMARY KEY,
        modification_date DATETIME,
        name VARCHAR(128),
        uuid VARCHAR(64)
    """
    )

    def into_schema(self) -> str:
        return textwrap.dedent(
            f"""(
            {self.id},
            '{self.modificationDate}',
            \"{self.name}\",
            '{self.uuid}'
        )"""
        )


@dataclass
class Game(SqlTableEntry):
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

    def into_schema(self) -> str:
        return textwrap.dedent(
            f"""(
            {self.cooperative},
            {self.highestWins},
            {self.id},
            {self.maxPlayTime},
            {self.maxPlayerCount},
            {self.minPlayTime},
            {self.minPlayerCount},
            '{self.modificationDate}',
            \"{self.name}\",
            {self.noPoints},
            {self.rating},
            {self.usesTeams},
            '{self.uuid}'
        )"""
        )


@dataclass
class Play(SqlTableEntry):
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
        entry_date DATETIME,
        game_ref_id INT,
        FOREIGN KEY(game_ref_id) REFERENCES {Game.TABLE_NAME}(id),
        id INT AUTO_INCREMENT PRIMARY KEY,
        ignored BOOL,
        location_ref_id INT,
        FOREIGN KEY(location_ref_id) REFERENCES {Location.TABLE_NAME}(id),
        modification_date DATETIME,
        play_date DATETIME,
        rounds INT,
        scoring_setting INT,
        uses_teams BOOL,
        uuid VARCHAR(64)
    """
    )

    def into_schema(self) -> str:
        return textwrap.dedent(
            f"""(
            \"{self.board}\",
            \"{self.comments}\",
            {self.durationMin},
            '{self.entryDate}',
            {self.gameRefId},
            {self.ignored},
            {self.locationRefId},
            '{self.modificationDate}',
            '{self.playDate}',
            {self.rounds},
            {self.scoringSetting},
            {self.usesTeams},
            '{self.uuid}'
        )"""
        )

    @property
    def player_ids(self) -> List[int]:
        return [score.playerRefId for score in self.playerScores]

    @property
    def play_date(self) -> Date:
        return DateTime.strptime(self.playDate, "%Y-%m-%d %H:%M:%S").date()


@dataclass
class PlayerScore(SqlTableEntry):
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

    def into_schema(self) -> str:
        role_sanitized = (
            '"' + self.role.replace('"', "'") + '"' if self.role else "NULL"
        )

        score_sanitized = self.score
        if score_sanitized:
            # Clear off leading 0's, since Python can't interpret them.
            # Add "+0" in case we have trailing add/subtract symbols or we strip off the only zero.
            score_sanitized = score_sanitized.lstrip("0") + "+0"

            # XXX: It's typically a bad idea to use `eval`, but since we're using data created by
            # BGStats, I'm going to assume it's not malicious.
            score_sanitized = eval(score_sanitized)
        else:
            score_sanitized = "NULL"

        return textwrap.dedent(
            f"""(
            {self.newPlayer},
            {{}},
            {self.playerRefId},
            {role_sanitized},
            {score_sanitized},
            {self.startPlayer},
            {self.winner}
        )"""
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
