from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class CoalMine(Asset):
    name = "Coal Mine"
    cost = Amount({Resource.MONEY: 3})
    consumer = SimpleConsumer(Amount({Resource.MONEY: 1}))
    producer = SimpleProducer(Amount({Resource.COAL: 2}))


class PowerPlantCoal(Asset):
    name = "Coal Burning Plant"
    cost = Amount({Resource.MONEY: 5})
    consumer = SimpleConsumer(Amount({Resource.COAL: 2}))
    producer = SimpleProducer(Amount({Resource.STEAM: 4}))
