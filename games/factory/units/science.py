import random

from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class ThinkTank(Asset):
    name = "Think Tank"
    cost = Amount({Resource.MONEY: 3})
    consumer = SimpleConsumer(Amount({Resource.MONEY: 2}))
    producer = SimpleProducer(Amount({Resource.SCIENTISTS: 1}))


class ResearchLab(Asset):
    cost = Amount({Resource.MONEY: 3})
    producer = SimpleProducer(Amount({Resource.PUBLICATIONS: 1}))

    def __init__(self) -> None:
        resource = random.choice(
            [
                Resource.COAL,
                Resource.WOOD,
                Resource.FOSSIL_GAS,
                Resource.STEAM,
                Resource.METAL,
            ]
        )
        self.consumer = SimpleConsumer(Amount({Resource.SCIENTISTS: 2, resource: 2}))
        self.name = f"Research Lab ({resource})"
