from game_systems import Asset
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class CoalMine(Asset):
    name = "Coal Mine"
    consumer = SimpleConsumer({(Resource.GOLD, 1)})
    producer = SimpleProducer({(Resource.COAL, 2)})


class PowerPlantCoal(Asset):
    name = "Power Plant (Coal)"
    consumer = SimpleConsumer({(Resource.COAL, 2)})
    producer = SimpleProducer({(Resource.ENERGY, 4)})
