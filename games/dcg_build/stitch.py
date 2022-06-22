from math import ceil
from typing import List

from PIL import Image


class CardImageStitcher:
    """
    Stitches Cards together for use within Tabletop Simulator. Pass it all of the
    sources that need to be stitched together and a destination basename that will
    start the full name of each file. Since each set of imported cards can only
    have a limited number of cards, the stitcher will automatically produce
    multiple files to import, each with the maximum number of images possible.
    """

    IMAGES_PER_ROW = 10
    MIN_ROWS_PER_IMAGE = 7
    MAX_ROWS_PER_IMAGE = 7
    MAX_CARDS_PER_IMAGE = IMAGES_PER_ROW * MAX_ROWS_PER_IMAGE

    def __init__(self, sources: List[str], dest: str) -> None:
        self.sources = sources
        self.dest = dest

    def stitch(self) -> None:
        num_output_images = ceil(
            len(self.sources) / CardImageStitcher.MAX_CARDS_PER_IMAGE
        )
        for idx in range(num_output_images):
            start_idx = idx * CardImageStitcher.MAX_CARDS_PER_IMAGE
            end_idx = (idx + 1) * CardImageStitcher.MAX_CARDS_PER_IMAGE

            output_path = f"{self.dest}-{idx}"
            CardImageStitcher._stitch_helper(
                self.sources[start_idx:end_idx], output_path
            )

    @staticmethod
    def _stitch_helper(sources: List[str], dest: str) -> None:
        image_width, image_height = Image.open(sources[0]).size
        output_width = image_width * CardImageStitcher.IMAGES_PER_ROW
        output_height = image_height * CardImageStitcher.MAX_ROWS_PER_IMAGE

        output_image = Image.new("RGB", (output_width, output_height))

        for i, image in enumerate(sources):
            image = Image.open(image)

            x_coord = (i % CardImageStitcher.IMAGES_PER_ROW) * image_width
            y_coord = (i // CardImageStitcher.IMAGES_PER_ROW) * image_height

            output_image.paste(image, (x_coord, y_coord))

        output_file_name = f"{dest}-{len(sources)}.png"
        output_image.save(output_file_name, "PNG")
