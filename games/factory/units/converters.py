from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class Kiln(Asset):
    name = "Kiln"
    consumer = SimpleConsumer(Amount({(Resource.WOOD, 1), (Resource.COAL, 1)}))
    producer = SimpleProducer(Amount({(Resource.COAL, 4)}))


class CoalLiquifactor(Asset):
    name = "Coal Liquifactor"
    consumer = SimpleConsumer(Amount({(Resource.MONEY, 1), (Resource.COAL, 1)}))
    producer = SimpleProducer(Amount({(Resource.OIL, 1)}))
