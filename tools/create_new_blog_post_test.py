#!/usr/bin/python3
import os
import random
import string
import unittest
from datetime import date
from pathlib import Path

from fs import open_fs
from parameterized import parameterized

from tools import create_new_blog_post


class CreateNewBlogPostTest(unittest.TestCase):
    def setUp(self) -> None:
        # TODO: Extract into `TestFs` class or something
        random_suffix = "".join(random.choice(string.ascii_letters) for _ in range(10))
        self.fs_dir_name = f"tmp_{random_suffix}"
        self.fs = open_fs(f"./{self.fs_dir_name}", create=True)

    def tearDown(self) -> None:
        self.fs.removetree("/")
        self.fs.close()
        os.rmdir(self.fs_dir_name)

    def test_create_file(self) -> None:
        post_title = "my-blog-post"
        args = create_new_blog_post.parse_args([post_title, self.fs_dir_name])

        today = date.today().strftime("%Y-%m-%d")
        create_new_blog_post.main(args)

        path_to_file = Path(f"{self.fs_dir_name}/{today}-{post_title}.md")
        self.assertTrue(path_to_file.exists(), f"{path_to_file} does not exist!")

    @parameterized.expand(
        [
            ("UPPERCASE", "uppercase"),
            ("CamelCase", "camelcase"),
            ("snake_case", "snake-case"),
            ("punctuation...marks!?", "punctuation-marks"),
            ("big      space", "big-space"),
            ("multiple words in sequence", "multiple-words-in-sequence"),
        ]
    )
    def test_create_file_converts_to_kebab_case(
        self, title_input, title_expected
    ) -> None:
        args = create_new_blog_post.parse_args([title_input, self.fs_dir_name])

        today = date.today().strftime("%Y-%m-%d")
        create_new_blog_post.main(args)

        path_to_file = Path(f"{self.fs_dir_name}/{today}-{title_expected}.md")
        self.assertTrue(path_to_file.exists(), f"{path_to_file} does not exist!")

    def test_cannot_create_file_with_same_name(self) -> None:
        post_title = "my-blog-post"
        args = create_new_blog_post.parse_args([post_title, self.fs_dir_name])

        create_new_blog_post.main(args)

        with self.assertRaisesRegex(Exception, "Cannot create new post"):
            create_new_blog_post.main(args)


if __name__ == "__main__":
    unittest.main()
