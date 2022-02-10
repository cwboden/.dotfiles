import subprocess
from pathlib import Path
from typing import Sequence
from unittest import TestCase

from parameterized import parameterized


def find_test_files(glob: str) -> Sequence[Path]:
    file_path = Path(__file__).parent.absolute()
    test_files = [
        test_file for test_file in file_path.joinpath("test_input").glob(glob)
    ]
    return sorted(test_files)


def get_test_name(path: Path) -> str:
    return path.parts[-1].split(".")[0]


TEST_FILES = find_test_files("*.maze")
SOLUTION_FILES = find_test_files("*.ron")
TEST_NAMES = [get_test_name(test_file) for test_file in TEST_FILES]


class PathFindingIntegrationTest(TestCase):
    def test_solution_files_match_test_files_one_to_one(self) -> None:
        self.assertGreater(len(TEST_FILES), 0)
        self.assertGreater(len(SOLUTION_FILES), 0)
        self.assertEqual(len(TEST_FILES), len(SOLUTION_FILES))

        for test_file, solution_file in zip(TEST_FILES, SOLUTION_FILES):
            test_name = test_file.parts[-1].split(".")[0]
            solution_name = solution_file.parts[-1].split(".")[0]

            # Solutions end with '-solution', so trim it off
            stripped_solution_name = solution_name[: -len("-solution")]

            self.assertEqual(test_name, stripped_solution_name)

    def do_integration_test(self, test_file: Path, solution_file: Path, algorithm: str):
        with test_file.open("r") as test_contents:
            subprocess.check_call(
                ["cargo", "run", "--example", "path_finding", "--", "-r", algorithm],
                stdin=test_contents,
            )

    @parameterized.expand(zip(TEST_NAMES, TEST_FILES, SOLUTION_FILES))
    def test_integration_queue(
        self,
        _test_name: str,
        test_file: Path,
        solution_file: Path,
    ):
        self.do_integration_test(test_file, solution_file, "--queue")

    @parameterized.expand(zip(TEST_NAMES, TEST_FILES, SOLUTION_FILES))
    def test_integration_stack(
        self,
        _test_name: str,
        test_file: Path,
        solution_file: Path,
    ):
        self.do_integration_test(test_file, solution_file, "--stack")
