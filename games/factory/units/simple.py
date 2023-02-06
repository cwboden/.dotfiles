from game_state import GameState
from game_systems import Asset
from game_systems import Consumer
from game_systems import Producer
from game_types import Amount
from game_types import Resource


class SimpleConsumer(Consumer):
    def __init__(self, payment: Amount) -> None:
        self.payment = payment

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

    def produce(self, game_state: GameState) -> None:
        for (resource, amount) in self.production:
            game_state.resources[resource] += amount


class Forest(Asset):
    consumer = SimpleConsumer({(Resource.GOLD, 1)})
    producer = SimpleProducer({(Resource.WOOD, 2)})

    def __str__(self) -> str:
        return f"Forest | [$1] -> [2 wood]"


class CoalMine(Asset):
    consumer = SimpleConsumer({(Resource.GOLD, 1)})
    producer = SimpleProducer({(Resource.COAL, 2)})

    def __str__(self) -> str:
        return f"Coal Mine | [$1] -> [2 coal]"


class Kiln(Asset):
    consumer = SimpleConsumer({(Resource.WOOD, 1), (Resource.COAL, 1)})
    producer = SimpleProducer({(Resource.COAL, 4)})

    def __str__(self) -> str:
        return f"Kiln | [1 wood, 1 coal] -> [4 coal]"


class PowerPlant(Asset):
    consumer = SimpleConsumer({(Resource.COAL, 2)})
    producer = SimpleProducer({(Resource.GOLD, 4)})

    def __str__(self) -> str:
        return f"Power Plant | [2 coal] -> [$4]"
