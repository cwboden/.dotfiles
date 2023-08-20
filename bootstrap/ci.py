import subprocess
import sys
from argparse import ArgumentDefaultsHelpFormatter, ArgumentParser, Namespace
from pathlib import Path


def log(string: str) -> None:
    print(f"# CI #: {string}")


def parse_args(args: list[str]) -> Namespace:
    parser = ArgumentParser(
        description="Run CI environments based on changes between two Git branches.",
        formatter_class=ArgumentDefaultsHelpFormatter,
    )

    parser.add_argument(
        "branch_sha",
        type=str,
        help="the branch you're working on.",
    )
    parser.add_argument(
        "target_sha",
        type=str,
        help="the branch you're targeting.",
    )
    parser.add_argument(
        "--root",
        "-r",
        type=Path,
        default=Path.home().joinpath(".dotfiles/"),
        help="the root of where to search for tests.",
    )

    return parser.parse_args(args)

args = parse_args(sys.argv[1:])
branch_sha = args.branch_sha
target_sha = args.target_sha
root = args.root

merge_base = subprocess.check_output(
    ["git", "merge-base", branch_sha, target_sha],
    cwd = root
)
changed_files = subprocess.check_output(
    ["git", "diff", "--name-only", branch_sha, target_sha],
    cwd = root
).decode('utf-8').split('\n')

log("Changed files:")
for file in changed_files:
    log(f" => {file}")

directories = [ (root / file) for file in changed_files ]

ci_environments = set()
while directories:
    directories = set([
        path.parents[0]
        for path in filter(lambda p: p.parents, directories)
    ])
    for directory in directories:
        possible_devenv = directory / "devenv.nix"
        if possible_devenv.is_file():
            ci_environments.add(directory)

environments = []
for directory in sorted(ci_environments):
    log(f"Executing `devenv ci` from {directory}/...")
    environments.append(
        (
            str(directory),
            subprocess.Popen(
                ["devenv", "ci"],
                cwd = directory,
            )
        )
    )


print('\n' + ('=' * 88) + '\n')
log("Waiting for CI environments:\n")
for (directory, process) in environments:
    if process.wait() == 0:
        log(f"{directory:40}: success")
    else:
        log(f"{directory:40}: FAILURE")
        print(process.stderr)


