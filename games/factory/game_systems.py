from typing import Dict
from typing import Protocol

from game_types import Resource


class GameState:
    resources: Dict[Resource, int] = dict()

    def __init__(self) -> None:
        for resource in list(Resource):
            self.resources[resource] = 0

    def print(self) -> None:
        for resource, amount in self.resources.items():
            print(f"{resource.name}: {amount}")


class Consumer(Protocol):
    def try_consume(self, game_state: GameState) -> bool:
        pass


class Producer(Protocol):
    def produce(self, game_state: GameState) -> None:
        pass


class PipelineUnit(Protocol):
    consumer: Consumer
    producer: Producer

    def execute(self, game_state: GameState) -> None:
        if self.consumer.try_consume(game_state):
            self.producer.produce(game_state)
