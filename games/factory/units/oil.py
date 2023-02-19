from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class OilDerrick(Asset):
    name = "Oil Derrick"
    consumer = SimpleConsumer(Amount({(Resource.MONEY, 1)}))
    producer = SimpleProducer(Amount({(Resource.OIL, 1)}))


class PowerPlantOil(Asset):
    name = "Power Plant (Oil)"
    consumer = SimpleConsumer(Amount({(Resource.OIL, 1)}))
    producer = SimpleProducer(Amount({(Resource.ENERGY, 4)}))
