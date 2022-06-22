#!/usr/bin/python3
import glob

from stitch import CardImageStitcher


def main():
    decklists = glob.glob("decklists/*.decklist")
    print("==> Found", len(decklists), "decklists to stitch")

    for decklist in decklists:
        with open(decklist, "r") as decklist_contents:
            decklist_raw = decklist_contents.readlines()

        decklist_card_paths = [
            "out/" + card_raw.strip() + ".bmp" for card_raw in decklist_raw
        ]

        output_path = decklist.replace(".decklist", ".png")
        print("Decklist Card Paths:", decklist_card_paths)
        print("Output Path:", output_path)
        stitcher = CardImageStitcher(decklist_card_paths, output_path)
        stitcher.stitch()


if __name__ == "__main__":
    main()
