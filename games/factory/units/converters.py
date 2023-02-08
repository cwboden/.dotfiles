from game_systems import Asset
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class Kiln(Asset):
    name = "Kiln"
    consumer = SimpleConsumer({(Resource.WOOD, 1), (Resource.COAL, 1)})
    producer = SimpleProducer({(Resource.COAL, 4)})


class CoalLiquifactor(Asset):
    name = "Coal Liquifactor"
    consumer = SimpleConsumer({(Resource.GOLD, 1), (Resource.COAL, 1)})
    producer = SimpleProducer({(Resource.OIL, 1)})
