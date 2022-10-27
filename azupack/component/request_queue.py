from typing import List
from ..model import LoginRequest, UniversalPackage

class RequestQueue:

    def __init__(self):
        self._logins = set()
        self._packages = []

    def request_login(self, organization: str):
        self._logins.add(LoginRequest(organization=organization))

    def request_upack(self, upack: UniversalPackage):
        self._packages.append(upack)

    def get_logins(self) -> List[LoginRequest]:
        return self._logins

    def get_packages(self) -> List[UniversalPackage]:
        return self._packages