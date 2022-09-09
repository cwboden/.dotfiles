#!/usr/bin/env python3
"""
Tool to take a resume and apply an image to the top of a series of separate
copies. Great for the career fair!

@author: Carson Boden
"""
import os
import tempfile
from optparse import OptionParser

from PyPDF2 import PdfFileReader
from PyPDF2 import PdfFileWriter
from reportlab.pdfgen import canvas


class ResumeImageAdder:
    def main(self):
        parser = self.get_option_parser()
        self.options = parser.parse_args()

        if self.options.image:
            add_image_to_resume(self.options.image)
        else:
            for file_name in os.listdir(self.options.folder):
                add_image_to_resume(file_name)

    def add_image_to_resume(self, image_path):

        temp_file_name = tempfile.gettempdir() + "logo.pdf"
        logo_canvas = canvas.Canvas(temp_file_name)
        logo_canvas.drawImage(image_path, 15, 15)
        logo_canvas.save()

        with open(temp_file_name, "rb") as temp_pdf:
            layer_pdf = PdfFileReader(temp_pdf)

            output_file = PdfFileWriter()
            with open(self.options.resume, "rb") as resume_pdf:
                input_file = PdfFileReader(resume_pdf)
                input_file.mergePage(layer_pdf.getPage(0))

                output_file.write(input_file)

    def get_option_parser(self):
        parser = OptionParser()

        parser.add_option(
            "-i", "--image", dest="image", help="Path to single image to add"
        )
        parser.add_option(
            "-f",
            "--folder",
            dest="folder",
            help="Path to folder of images to add",
            default="logos/",
        )
        parser.add_option(
            "-r",
            "--resume",
            dest="resume",
            help="Path to resume to add images to",
            default="resume.pdf",
        )
        parser.add_option(
            "-o",
            "--output",
            dest="output",
            help="Path to folder where new resumes should be saved",
            default="",
        )

        return parser


if __name__ == "__main__":
    resume_adder = ResumeImageAdder()
    resume_adder.main()
