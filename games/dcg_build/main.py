#!/usr/bin/python3
import csv
import glob
import os.path
import pickle
import shutil
import string
import subprocess
from argparse import ArgumentParser
from typing import Dict
from typing import List
from typing import Sequence
from typing import Union

from google.auth.transport.requests import Request
from google.oauth2 import credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from gsheets import Sheets
from ingest.authenticator import Authenticator
from ingest.sheet_downloader import SheetDownloader
from PIL import Image
from pydrive.auth import GoogleAuth
from pydrive.drive import GoogleDrive
from stitch import CardImageStitcher


def parse_args():
    parser = ArgumentParser(
        description="Generate Tabletop Simulator card sheets via a customer Rust library."
    )
    parser.add_argument(
        "-b",
        "--build-only",
        action="store_true",
        help="[TODO] Build the cards from currently downloaded cards",
    )
    parser.add_argument(
        "-c", "--clean", action="store_true", help="[TODO] Remove all build artifacts"
    )
    parser.add_argument(
        "-t", "--type", type=str, help="[TODO] Build all cards of the given type"
    )
    parser.add_argument(
        "-f",
        "--fetch-only",
        action="store_true",
        help="[TODO] Update the downloaded cards to match Google sheets",
    )
    return parser.parse_args()


def ingest_cards(creds: credentials.Credentials):
    creds = Authenticator.get_credentials()
    downloader = SheetDownloader(creds)
    downloader.ingest_cards()


def setup_directory_structure(card_types: List[str], player_card_classes: List[str]):
    print("Setting up Directory Structure")
    HIGH_LEVEL_DIRECTORIES = ["in", "cards", "out"]

    # Create new directories
    for directory in HIGH_LEVEL_DIRECTORIES:
        if not os.path.exists(directory):
            try:
                os.mkdir(directory)
                for card_type in card_types:
                    path = os.path.join(directory, card_type)
                    os.mkdir(path)
                    print(f"==> Made {path}")
                    if card_type == "player":
                        for player_card_class in player_card_classes:
                            class_path = os.path.join(path, player_card_class)
                            os.mkdir(class_path)
                            print(f"==> Made {class_path}")
            except OSError as e:
                print(f"Error: {directory}: {e.strerror}")


def generate_cards(card_types: List[str], player_card_classes: List[str]) -> None:
    for card_directory_name in card_types:
        if card_directory_name == "player":
            for player_card_class in player_card_classes:
                directory_name = f"{card_directory_name}/{player_card_class}"
                generate_cards_helper(directory_name, card_directory_name)
        else:
            generate_cards_helper(card_directory_name, card_directory_name)


def generate_cards_helper(directory_name: str, card_type: str) -> None:
    target_directory = f"cards-yaml/{directory_name}"
    print(f"Generating card images from {directory_name}")
    target_files = os.listdir(target_directory)
    print(f"==> Found {len(target_files)} cards to generate")
    for filename in target_files:
        input_filename = f"cards-yaml/{directory_name}/{filename}"
        output_filename = f"out/{directory_name}/" + filename.replace(".yaml", ".bmp")

        if os.path.exists(output_filename) and (
            os.path.getmtime(input_filename) < os.path.getmtime(output_filename)
        ):
            # No need to build unless input file is newer than the output
            continue

        with open(os.devnull, "w") as devnull:
            result = subprocess.run(
                [
                    "cargo",
                    "run",
                    "--bin",
                    "card-generator",
                    "-q",
                    "--",
                    input_filename,
                    card_type.capitalize(),
                    output_filename,
                ],
                stdout=devnull,
                stderr=devnull,
            )
            if result.returncode != 0:
                print(f"==> !! Could not generate {output_filename}")


def stitch_cards(start_dir: str) -> None:
    print("Stitching cards together for Tabletop Sim")
    stitch_dict: Dict[str, List[str]] = {}
    for root, dirs, files in os.walk(start_dir):
        for name in files:
            if root not in stitch_dict.keys():
                stitch_dict[root] = []
            stitch_dict[root].append(name)

    for output_dir, items in stitch_dict.items():
        images = [
            os.path.join(output_dir, item) for item in items if "stitched" not in item
        ]
        print(f"==> Found {len(images)} images to stitch in {output_dir}")
        stitcher = CardImageStitcher(images, f"{output_dir}/stitched")
        stitcher.stitch()


def main() -> None:
    args = parse_args()

    CARD_TYPES = [
        "act",
        "agenda",
        "encounter",
        "enemy",
        "location",
        "player",
        "character",
    ]

    PLAYER_CARD_CLASSES = [
        "character",
        "researcher",
        "runner",
        "scavenger",
        "seeker",
        "soldier",
    ]

    setup_directory_structure(CARD_TYPES, PLAYER_CARD_CLASSES)

    if not args.build_only:
        creds = Authenticator.get_credentials()
        ingest_cards(creds)

    if not args.fetch_only:
        generate_cards(CARD_TYPES, PLAYER_CARD_CLASSES)
        stitch_cards("out/")


if __name__ == "__main__":
    main()
