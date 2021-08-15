#!/usr/bin/python3
import os
import subprocess
import unittest


class BootstrapIntegrationTest(unittest.TestCase):
    """
    Assumes Bootstrap has been run on the system. Will check to ensure certain
    dependencies are available.
    """

    def setUp(self) -> None:
        self.home_dir = os.path.expanduser("~")

    def test_pre_commit_git_hook_installed(self) -> None:
        subprocess.check_call(
            ["pre-commit", "run", "--all-files"], stdout=subprocess.DEVNULL
        )

    def test_vim_folders_are_created(self) -> None:
        self.assertTrue(os.path.isdir(f"{self.home_dir}/.vim/swapfiles"))
        self.assertTrue(os.path.isdir(f"{self.home_dir}/.vim/backups"))
        self.assertTrue(os.path.isdir(f"{self.home_dir}/.vim/undodir"))

    def test_vim_plug_installed(self) -> None:
        self.assertTrue(os.path.exists(f"{self.home_dir}/.vim/autoload/plug.vim"))
        self.assertTrue(os.path.exists(f"{self.home_dir}/.vim/plugged"))

    def test_zsh_installed(self) -> None:
        self.assertTrue(os.path.exists("/tmp/zsh_installer.sh"))
        self.assertTrue(os.path.isdir(f"{self.home_dir}/.oh-my-zsh"))
        self.assertTrue(
            os.path.isdir(f"{self.home_dir}/.oh-my-zsh/custom/themes/powerlevel10k")
        )

    def test_symlinks_created(self) -> None:
        self.assertTrue(os.path.exists(f"{self.home_dir}/.hgrc"))
        self.assertTrue(os.path.exists(f"{self.home_dir}/.vimrc"))
        self.assertTrue(os.path.exists(f"{self.home_dir}/.zshrc"))
        self.assertTrue(os.path.exists(f"{self.home_dir}/.gitconfig"))
        self.assertTrue(os.path.exists(f"{self.home_dir}/.tmux.conf"))


if __name__ == "__main__":
    unittest.main()
