import subprocess
import sys
from pathlib import Path


def log(string: str) -> None:
    print(f"# CI #: {string}")


branch_sha = sys.argv[1]
target_sha = sys.argv[2]

merge_base = subprocess.check_output(["git", "merge-base", branch_sha, target_sha])
changed_files = subprocess.check_output(["git", "diff", "--name-only", branch_sha,
                                         target_sha]).decode('utf-8').split('\n')
directories = [ Path(file) for file in changed_files ]

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


