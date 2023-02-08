from game_systems import Asset
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class OilDerrick(Asset):
    name = "Oil Derrick"
    consumer = SimpleConsumer({(Resource.GOLD, 1)})
    producer = SimpleProducer({(Resource.OIL, 1)})


class PowerPlantOil(Asset):
    name = "Power Plant (Oil)"
    consumer = SimpleConsumer({(Resource.OIL, 1)})
    producer = SimpleProducer({(Resource.ENERGY, 4)})
