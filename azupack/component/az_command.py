from subprocess import Popen, PIPE
from sys import stdout
from typing import List
from collections import namedtuple

CommandResult = namedtuple("CommandResult", ["stdout", "stderr", "exit_code"])

class AZCommand:
    def __init__(self):
        pass

    def run(self, pipein: str, args: List[str]) -> CommandResult:
        process = Popen(args, executable='az', stdin=PIPE, stdout=PIPE, stderr=PIPE)
        stdout, stderr = process.communicate(pipein.encode("ascii"))
        exit_code = process.returncode
        return CommandResult(stdout, stderr, exit_code)