from game_systems import Consumer
from game_systems import GameState
from game_types import Amount


class GeneralConsumer(Consumer):
    def __init__(self, payment: Amount):
        self.payment = payment

    def try_consume(self, game_state: GameState) -> bool:
        for resource in self.payment.resources:
            return True
        return False
