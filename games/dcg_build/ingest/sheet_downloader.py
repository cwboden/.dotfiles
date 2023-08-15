import os
import string
import textwrap
import time
from typing import Dict, List, Sequence, Union

import yaml
from google.oauth2.credentials import Credentials
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError


class SheetDownloader:
    # The Folder ID is the part of the URL after */folders/
    FOLDER_ID = "1JyyYPd6q72pX2hOWAzKexnQq0C1fo58_"

    def __init__(self, credentials: Credentials) -> None:
        self.drive_service = build("drive", "v3", credentials=credentials)
        self.sheets_service = build("sheets", "v4", credentials=credentials)

    def _fetch_drive_items(self) -> List[Dict]:
        results = (
            self.drive_service.files()
            .list(
                q=f'"{SheetDownloader.FOLDER_ID}" in parents and trashed=false',
                pageSize=100,  # number of files downloaded
                fields="nextPageToken, files(id, name)",
            )
            .execute()
        )
        return results.get("files", [])

    def _ingest_single_sheet(
        self,
        class_name: str,
        sub_class_name: str,
        subpath: str,
        cards: Sequence[Sequence[str]],
        indices: List[int],
        has_card_class=False,
    ) -> None:
        print(f"==> Ingesting cards from {class_name} - {sub_class_name}")

        # Cannot generate character cards right now
        if class_name == "SKIP":
            print(f"==> !! Skipping {class_name} cards")
            return

        # Clear Decklist File
        class_name = class_name.lower()
        if has_card_class:
            decklist_path = f"decklists/{class_name}_{sub_class_name}.decklist"
        else:
            decklist_path = f"decklists/{subpath}.decklist"
        decklist_file = open(decklist_path, "w+")
        decklist_file.truncate(0)

        header = False
        for card in cards:
            # Create headers
            header_info: Sequence[str]
            if not header:
                header_info = card
                header = True
                continue

            # Remove all newlines
            card = [item.replace("\n", " ").replace("\r", "") for item in card]

            # Create card file
            translated_name = (
                card[0]
                .lower()
                .translate(str.maketrans("", "", string.punctuation))
                .replace(" ", "_")
            )
            if has_card_class:
                card_path = f"cards/{subpath}/{class_name}/{translated_name}.card"
            else:
                card_path = f"cards/{subpath}/{translated_name}.card"

            # Update decklist file for card
            if has_card_class:
                decklist_file.writelines(f"{subpath}/{class_name}/{translated_name}\n")
            else:
                decklist_file.writelines(f"{subpath}/{translated_name}\n")

            if not os.path.exists(os.path.dirname(card_path)):
                os.mkdir(os.path.dirname(card_path))

            with open(card_path, "w") as card_file:
                # Generate card_dict
                card_dict: Dict[str, Union[bool, int, str, Sequence[str]]] = {}
                for idx in indices:
                    val: Union[bool, int, str, Sequence[str]]
                    if idx < len(card):
                        if card[idx].isdigit():
                            val = int(card[idx])
                        elif card[idx].lower() in ["true", "false"]:
                            val = bool(card[idx])
                        elif card[idx].startswith("[") and card[idx].endswith("]"):
                            stripped_arr = card[idx][1:-1]
                            val = [
                                elem.strip()
                                for elem in stripped_arr.split(",")
                                if len(elem.strip()) > 0
                            ]
                        else:
                            val = card[idx]
                    else:
                        val = []
                    card_dict[header_info[idx]] = val

                if has_card_class:
                    card_class = class_name.capitalize()
                    card_dict["class"] = card_class

                yaml.dump(card_dict, card_file, width=1000)

    def ingest_cards(self):
        # Setup card formats
        PLAYER_CARD_INDICES = [
            0,  # Name
            1,  # PlayerCardType
            2,  # Cost
            3,  # Tags
            4,  # Description
            5,  # Slots
        ]
        LOCATION_CARD_INDICES = [
            0,  # Name
            1,  # Shroud
            2,  # Clues
            3,  # Scale Clues
            4,  # Tags
            5,  # Description
        ]
        ENCOUNTER_CARD_INDICES = [
            0,  # Name
            # TODO: 1, # Tags
            2,  # Text
        ]
        ENEMY_CARD_INDICES = [
            0,  # Name
            # TODO: 1, # Tags
            2,  # Health
            3,  # Strength
            4,  # Speed
            5,  # Text
        ]
        ACT_DECK_INDICES = [
            0,  # Title
            1,  # Clues
            2,  # Scale Clues
            3,  # Flavor / Text
            # TODO: 3, # Text
        ]
        AGENDA_DECK_INDICES = [
            0,  # Title
            1,  # Doom
            2,  # Flavor / Text
            # TODO: 3, # Text
        ]
        CARD_TYPE_DICT = {
            "Acts": "act",
            "Agendas": "agenda",
            "Locations": "location",
            "Scavenger": "player",
            "Soldier": "player",
            "Runner": "player",
            "Seeker": "player",
            "Researcher": "player",
            "Player Characters": "SKIP",  # cannot generate player cards
            "Encounters": "encounter",
            "Enemies": "enemy",
        }
        CARD_TYPE_TO_INDICES = {
            "act": ACT_DECK_INDICES,
            "agenda": AGENDA_DECK_INDICES,
            "encounter": ENCOUNTER_CARD_INDICES,
            "enemy": ENEMY_CARD_INDICES,
            "location": LOCATION_CARD_INDICES,
            "player": PLAYER_CARD_INDICES,
        }

        items = self._fetch_drive_items()
        print(f"Fetching {len(items)} items from Google Drive")
        for item in items:
            print(f'==> Downloaded {item["name"]} ({item["id"]})')  # debugging

            # try block will throw an exception for each file that is not a Google Sheet
            try:
                # get meta data for each spreadsheet
                res = (
                    self.sheets_service.spreadsheets()
                    .get(spreadsheetId=item["id"], fields="sheets(properties(title))")
                    .execute()
                )

                # extract type/subtype of card
                class_name = item["name"]
                sub_class_name = None
                card_type = CARD_TYPE_DICT[class_name]

                if card_type == "SKIP":
                    continue

                # get values for each sheet in the spreadsheet
                for i in range(len(res["sheets"])):
                    sheetName = res["sheets"][i]["properties"]["title"]

                    # get subclass for player card
                    if card_type == "player":
                        sub_class_name = sheetName

                    # extract information from sheet
                    still_try_values = True
                    while still_try_values:
                        try:
                            values = (
                                self.sheets_service.spreadsheets()
                                .values()
                                .batchGet(spreadsheetId=item["id"], ranges=(sheetName))
                                .execute()
                            )
                            still_try_values = False
                        except HttpError as err:
                            print("==> !! Could not read values from spreadsheet:")
                            print(f"==>    {err}")
                            retry_time = 10
                            print(textwrap.dedent(f"""
                                !! Waiting for {retry_time} seconds and trying again...
                            """))
                            time.sleep(retry_time)

                    # not sure why this for loop has to be here tbh
                    for value in values["valueRanges"]:
                        cards = value["values"]
                        self._ingest_single_sheet(
                            class_name,
                            sub_class_name,
                            card_type,
                            cards,
                            CARD_TYPE_TO_INDICES[card_type],
                            has_card_class=card_type == "player",
                        )
            except Exception as e:
                raise e
                # print(str(e))
