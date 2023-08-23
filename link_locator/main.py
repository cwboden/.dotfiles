import sys
from argparse import ArgumentParser
from argparse import Namespace
from dataclasses import dataclass
from math import sqrt
from pathlib import Path
from typing import Self

import yaml
from apiclient import APIClient


@dataclass
class LatLong:
    latitude: float
    longitude: float

    def distance_to(self, other: Self) -> float:
        return sqrt(
            (abs(self.latitude - other.latitude)) ** 2
            + (abs(self.longitude - other.longitude)) ** 2
        )


@dataclass
class Station:
    name: str
    address: str
    latlong: LatLong


class GeocodeMapsClient(APIClient):
    # See: https://geocode.maps.co/

    def encode_address(self, address: str) -> LatLong:
        response = self.get(f"https://geocode.maps.co/search?q={address}")
        return LatLong(
            float(response.json()[0]["lat"]), float(response.json()[0]["lon"])
        )


def parse_args(args: list[str]) -> Namespace:
    parser = ArgumentParser(
        description="Determines the nearest (SoundTransit) Link stop and how long it would take to walk there."
    )

    parser.add_argument(
        "address",
        type=str,
        help="The address to query.",
    )

    parser.add_argument(
        "-s",
        "--stations",
        type=Path,
        default="stations.yaml",
        help="A list of files, in YAML format, to use as the train stations.",
    )

    return parser.parse_args(args)


def load_stations(path: Path) -> list[Station]:
    with open(path, "r") as stations_file:
        stations_raw = yaml.safe_load(stations_file)
        return [
            Station(
                station_raw["name"],
                station_raw["address"],
                LatLong(
                    station_raw["latlong"]["latitude"],
                    station_raw["latlong"]["longitude"],
                ),
            )
            for station_raw in stations_raw["stations"]
        ]


def main(args: Namespace):
    stations = load_stations(args.stations)

    client = GeocodeMapsClient()
    input_latlong = client.encode_address(args.address)
    print(f"\nINFO - Input location: {input_latlong}")

    closest_station = min(
        stations, key=lambda station: station.latlong.distance_to(input_latlong)
    )

    print(f"\n RESULT: Closest station is: {closest_station.name}")


if __name__ == "__main__":
    args = parse_args(sys.argv[1:])
    main(args)
