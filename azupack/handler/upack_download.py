from ..model import UniversalPackage
from ..auth_handler import AuthHandler
from subprocess import Popen, PIPE

# TODO: make a command executor function
def download_upack(package: UniversalPackage)

p = Popen(['program', 'arg1'], stdin=PIPE, stdout=PIPE, stderr=PIPE)
output, err = p.communicate(b"input data that is passed to subprocess' stdin")
rc = p.returncode