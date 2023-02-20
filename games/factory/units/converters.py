from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class Kiln(Asset):
    name = "Kiln"
    cost = Amount({Resource.MONEY: 4})
    consumer = SimpleConsumer(Amount({Resource.WOOD: 1, Resource.COAL: 1}))
    producer = SimpleProducer(Amount({Resource.COAL: 4}))


# XXX: Abandoned, since oil isn't a big part of energy production.
class CoalLiquifactor(Asset):
    name = "Coal Liquifactor"
    cost = Amount({Resource.MONEY: 6})
    consumer = SimpleConsumer(Amount({Resource.MONEY: 1, Resource.COAL: 1}))
    producer = SimpleProducer(Amount({Resource.CRUDE_OIL: 1}))
