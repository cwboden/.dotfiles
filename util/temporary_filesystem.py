import os
import shutil
from pathlib import Path


class TemporaryFilesystem:
    """
    A temporary filesystem for reading and writing to, often within tests. The
    advantages here are that:
     - You will avoid naming collisions between other code/tests
     - You can clean up created files/directories easily
     - Cleanup occurs even if the test fails
    """

    def __init__(self):
        # XXX: Add random suffix to avoid collisions between multiple instances
        self.root = "/tmp/temporary_filesystem"
        os.mkdir(self.root)

    def __del__(self):
        shutil.rmtree(self.root)

    def exists(self, path: str) -> bool:
        """Determines if a given path exists within the TemporaryFilesystem"""
        return os.path.exists(self.translate(path))

    def mkdir(self, path: str) -> None:
        """Creates a directory within the TemporaryFilesystem"""
        os.mkdir(self.translate(path))

    def translate(self, path: str) -> str:
        """
        Translates a "fake" relative path into the "real" absolute path within
        the temporary filesystem
        """
        temp_fs_path = os.path.join(self.root, path)
        return os.path.abspath(temp_fs_path)

    def touch(self, path: str) -> None:
        """Touch the file at the given path"""
        Path(self.translate(path)).touch()
