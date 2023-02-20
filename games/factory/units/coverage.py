from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class SmallSubstation(Asset):
    # TODO: Worth 1 coverage
    name = "Small Substation"
    cost = Amount({Resource.MONEY: 5})
    consumer = SimpleConsumer(Amount({Resource.ENERGY: 1}))
    producer = SimpleProducer(Amount({Resource.MONEY: 5}))
