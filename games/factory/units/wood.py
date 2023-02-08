from game_systems import Asset
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class Forest(Asset):
    name = "Forest"
    consumer = SimpleConsumer({(Resource.GOLD, 1)})
    producer = SimpleProducer({(Resource.WOOD, 2)})


class PowerPlantWood(Asset):
    name = "Power Plant (Wood)"
    consumer = SimpleConsumer({(Resource.WOOD, 2)})
    producer = SimpleProducer({(Resource.ENERGY, 2)})
