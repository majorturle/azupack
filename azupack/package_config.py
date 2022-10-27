from dataclasses import dataclass, field, MISSING
from email.policy import default
from typing import List, Optional

class PackageInstance:
    organization: str = field(default=MISSING, metadata={"required": True})
    feed: str = field(default=MISSING, metadata={"required": True})
    name: str = field(default=MISSING, metadata={"required": True})
    path: str = field(default=MISSING, metadata={"required": True})
    version: Optional[str] = field(default=None, metadata={"required": False, "allow_none": True})

@dataclass
class PackageConfig:
    """
    Package configuration of a project directory containing all the dependencies.
    """
    schema: str = field(default=MISSING)
    packages: List[PackageInstance] = field(default_factory=list)