from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer

# Idea: Different starting headquarters


class Headquarters(Asset):
    name = "Headquarters"
    cost = Amount({Resource.MONEY: 0})
    consumer = SimpleConsumer(Amount({}))
    producer = SimpleProducer(Amount({Resource.MONEY: 5}))
