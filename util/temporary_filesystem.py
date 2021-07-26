import os
import shutil


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
        self.root = "temporary_filesystem"
        os.mkdir(self.root)

    def __del__(self):
        shutil.rmtree(self.root)
