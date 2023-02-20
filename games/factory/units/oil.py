from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer

# XXX: These classes are not currently in use, as oil is not a major component of power generation.


class OilDerrick(Asset):
    name = "Oil Derrick"
    cost = Amount({Resource.MONEY: 5})
    consumer = SimpleConsumer(Amount({Resource.MONEY: 1}))
    producer = SimpleProducer(Amount({Resource.CRUDE_OIL: 1}))


class PowerPlantOil(Asset):
    name = "Power Plant (Oil)"
    cost = Amount({Resource.MONEY: 8})
    consumer = SimpleConsumer(Amount({Resource.REFINED_OIL: 1}))
    producer = SimpleProducer(Amount({Resource.ENERGY: 4}))
