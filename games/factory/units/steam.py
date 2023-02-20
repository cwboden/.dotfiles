from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class SmallTurbine(Asset):
    name = "Small Turbine"
    cost = Amount({Resource.MONEY: 3})
    consumer = SimpleConsumer(Amount({Resource.STEAM: 2}))
    producer = SimpleProducer(Amount({Resource.ENERGY: 1}))
