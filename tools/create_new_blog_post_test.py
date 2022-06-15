#!/usr/bin/python3
import os
import random
import string
import unittest
from argparse import Namespace
from datetime import date
from pathlib import Path

from create_new_blog_post import TEMPLATE_PATH
from fs import open_fs
from parameterized import parameterized

from git import Repo
from tools import create_new_blog_post


class CreateNewBlogPostTest(unittest.TestCase):
    def setUp(self) -> None:
        # TODO: Extract into `TestFs` class or something
        random_suffix = "".join(random.choice(string.ascii_letters) for _ in range(10))
        self.fs_dir_path = Path.home().joinpath(f"tmp_{random_suffix}")
        self.fs = open_fs(str(self.fs_dir_path), create=True)

        self.fs.makedir("repo")
        self.repo_path = self.fs_dir_path.joinpath("repo")
        self.repo = Repo.init(self.repo_path)
        self.repo.index.commit("Initial commit")
        self.repo.create_head("main")

    def tearDown(self) -> None:
        self.fs.removetree("/")
        self.fs.close()
        os.rmdir(self.fs_dir_path)

    def parse_args(self, title: str) -> Namespace:
        return create_new_blog_post.parse_args(
            [
                title,
                str(self.fs_dir_path),
            ]
        )

    def test_create_file(self) -> None:
        post_title = "my-blog-post"
        args = self.parse_args(post_title)

        today = date.today().strftime("%Y-%m-%d")
        create_new_blog_post.main(args, self.repo)

        path_to_file = self.fs_dir_path.joinpath(f"{today}-{post_title}.md")
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
        args = self.parse_args(title_input)

        today = date.today().strftime("%Y-%m-%d")
        create_new_blog_post.main(args, self.repo)

        path_to_file = self.fs_dir_path.joinpath(f"{today}-{title_expected}.md")
        self.assertTrue(path_to_file.exists(), f"{path_to_file} does not exist!")

    def test_cannot_create_file_with_same_name(self) -> None:
        post_title = "my-blog-post"
        args = self.parse_args(post_title)

        create_new_blog_post.main(args, self.repo)

        with self.assertRaisesRegex(Exception, "Cannot create new post"):
            create_new_blog_post.main(args, self.repo)

    def test_created_file_contains_blog_post_template(self) -> None:
        post_title = "my-blog-post"
        args = self.parse_args(post_title)

        new_post_path = create_new_blog_post.main(args, self.repo)

        with open(TEMPLATE_PATH) as template, open(new_post_path) as new_post:
            self.assertListEqual(list(template), list(new_post))

    def test_cannot_create_branch_if_outstanding_changes_exist(self) -> None:
        post_title = "my-blog-post"
        args = self.parse_args(post_title)

        new_file_path = self.repo_path.joinpath("my-file.txt")
        new_file_path.touch()
        self.repo.index.add(str(new_file_path))

        with self.assertRaisesRegex(Exception, "Cannot create git branch"):
            create_new_blog_post.main(args, self.repo)

    def test_creates_branch_with_post_name(self) -> None:
        post_title = "my-blog-post"
        args = self.parse_args(post_title)

        create_new_blog_post.main(args, self.repo)

        today = date.today().strftime("%Y-%m-%d")
        self.assertEqual(self.repo.active_branch.name, f"posts/{today}-{post_title}")


if __name__ == "__main__":
    unittest.main()
