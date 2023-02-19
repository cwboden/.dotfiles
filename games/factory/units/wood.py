from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class Forest(Asset):
    name = "Forest"
    consumer = SimpleConsumer(Amount({(Resource.MONEY, 1)}))
    producer = SimpleProducer(Amount({(Resource.WOOD, 2)}))


class PowerPlantWood(Asset):
    name = "Power Plant (Wood)"
    consumer = SimpleConsumer(Amount({(Resource.WOOD, 2)}))
    producer = SimpleProducer(Amount({(Resource.ENERGY, 2)}))
