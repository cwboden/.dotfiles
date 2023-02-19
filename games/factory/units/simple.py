from game_state import GameState
from game_systems import Consumer
from game_systems import Producer
from game_types import Amount


class SimpleConsumer(Consumer):
    def __init__(self, payment: Amount) -> None:
        self.payment = payment

    def __str__(self) -> str:
        return str(self.payment)

    def try_consume(self, game_state: GameState) -> bool:
        # TODO: Payments should probably become it's own system.
        for (resource, amount) in self.payment:
            if game_state.resources[resource] < amount:
                return False

        for (resource, amount) in self.payment:
            game_state.resources[resource] -= amount

        return True


class SimpleProducer(Producer):
    def __init__(self, production: Amount) -> None:
        self.production = production

    def __str__(self) -> str:
        return str(self.production)

    def produce(self, game_state: GameState) -> None:
        for (resource, amount) in self.production:
            game_state.resources[resource] += amount
