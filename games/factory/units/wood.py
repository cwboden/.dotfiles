from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class Forest(Asset):
    name = "Forest"
    cost = Amount({Resource.MONEY: 1})
    consumer = SimpleConsumer(Amount({Resource.MONEY: 1}))
    producer = SimpleProducer(Amount({Resource.WOOD: 2}))


class PowerPlantWood(Asset):
    name = "Wood Burning Plant"
    cost = Amount({Resource.MONEY: 3})
    consumer = SimpleConsumer(Amount({Resource.WOOD: 2}))
    producer = SimpleProducer(Amount({Resource.STEAM: 2}))
