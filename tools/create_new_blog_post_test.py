#!/usr/bin/python3
import os
import random
import string
import unittest
from datetime import date
from pathlib import Path

from fs import open_fs

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
        self.assertTrue(path_to_file.exists())


if __name__ == "__main__":
    unittest.main()
