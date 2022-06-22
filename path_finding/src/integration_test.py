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

    def do_integration_test(
        self, test_name: str, test_file: Path, solution_file: Path, algorithm: str
    ):
        output_file = Path(f"tmp/{test_name}-output.ron")
        with test_file.open("r") as test_contents, output_file.open(
            "w"
        ) as output_contents:
            subprocess.check_call(
                ["cargo", "run", "--example", "path_finding", "--", "-r", algorithm],
                stdin=test_contents,
                stdout=output_contents,
                stderr=subprocess.DEVNULL,
                timeout=10,
            )

        with output_file.open("r") as output_contents:
            output_lines = output_contents.readlines()
        with solution_file.open("r") as solution_contents:
            solution_lines = solution_contents.readlines()

        # Strip extraneous whitespace
        output_lines = [line.strip() for line in output_lines]
        solution_lines = [line.strip() for line in solution_lines]
        self.assertEqual(output_lines, solution_lines)

    @parameterized.expand(zip(TEST_NAMES, TEST_FILES, SOLUTION_FILES))
    def test_integration_queue(
        self,
        test_name: str,
        test_file: Path,
        solution_file: Path,
    ):
        self.do_integration_test(test_name, test_file, solution_file, "--queue")

    @parameterized.expand(zip(TEST_NAMES, TEST_FILES, SOLUTION_FILES))
    def test_integration_stack(
        self,
        test_name: str,
        test_file: Path,
        solution_file: Path,
    ):
        self.do_integration_test(test_name, test_file, solution_file, "--stack")
