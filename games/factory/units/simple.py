from game_state import GameState
from game_systems import Consumer
from game_systems import Producer
from game_types import Amount


class SimpleConsumer(Consumer):
    def __init__(self, payment: Amount) -> None:
        self.payment = payment

    def __str__(self) -> str:
        return str(self.payment)

    def __format__(self, format_spec: str) -> str:
        return format(str(self), format_spec)

    def try_consume(self, game_state: GameState) -> bool:
        if not game_state.can_pay_for(self.payment):
            return False

        game_state.pay_for(self.payment)
        return True


class SimpleProducer(Producer):
    def __init__(self, production: Amount) -> None:
        self.production = production

    def __str__(self) -> str:
        return str(self.production)

    def __format__(self, format_spec: str) -> str:
        return format(str(self), format_spec)

    def produce(self, game_state: GameState) -> None:
        game_state.resources.add(self.production)
