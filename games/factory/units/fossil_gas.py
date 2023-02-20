from game_systems import Asset
from game_types import Amount
from game_types import Resource
from units.simple import SimpleConsumer
from units.simple import SimpleProducer


class FossilGasMine(Asset):
    name = "Fossil Gas Mine"
    cost = Amount({Resource.MONEY: 3})
    consumer = SimpleConsumer(Amount({Resource.MONEY: 1}))
    producer = SimpleProducer(Amount({Resource.FOSSIL_GAS: 2}))


class PowerPlantCoal(Asset):
    name = "Fossil Gas Burning Plant"
    cost = Amount({Resource.MONEY: 5})
    consumer = SimpleConsumer(Amount({Resource.FOSSIL_GAS: 2}))
    producer = SimpleProducer(Amount({Resource.STEAM: 4}))
