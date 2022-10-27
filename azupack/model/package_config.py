from typing import List
from typing_extensions import Self

from .universal_package import UniversalPackage

class PackageConfig:
    packages: List[UniversalPackage] = None

    def __init__(self):
        self.packages = []