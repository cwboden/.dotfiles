import sys

from colorama import Fore, Style

from units import BuildUnit


class Builder:
    def __init__(self, io_out=sys.stdout):
        self.units = []
        self.errors = []
        self.io_out = io_out

    def add_unit(self, unit: BuildUnit) -> None:
        self.units.append(unit)

    def build(self) -> None:
        for unit in self.units:
            try:
                unit.build()
            except NotImplementedError as e:
                print(
                    Fore.YELLOW
                    + f"Missing Implementation for BuildUnit '{str(unit)}'"
                    + Style.RESET_ALL,
                    file=self.io_out,
                )
                self.errors.append(e)
            except Exception as e:
                print(
                    Fore.RED + f"Failure for BuildUnit '{str(unit)}'" + Style.RESET_ALL,
                    file=self.io_out,
                )
                self.errors.append(e)

        if self.errors:
            print(
                Fore.RED + "\nBuildUnit Failures:" + Style.RESET_ALL,
                file=self.io_out,
            )
            raise Exception(self.errors)
