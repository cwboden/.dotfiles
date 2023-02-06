from typing import Protocol
from typing import TYPE_CHECKING

# We only care about this import for ensuring types line up -- it causes a cyclical dependency if
# imported at runtime.
if TYPE_CHECKING:
    from game_state import GameState


class Consumer(Protocol):
    def try_consume(self, game_state: "GameState") -> bool:
        pass


class Producer(Protocol):
    def produce(self, game_state: "GameState") -> None:
        pass


class Asset(Protocol):
    consumer: Consumer
    producer: Producer

    def execute(self, game_state: "GameState") -> None:
        if self.consumer.try_consume(game_state):
            self.producer.produce(game_state)
