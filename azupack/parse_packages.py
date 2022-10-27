import json
from typing import Optional
from dataclasses import dataclass, MISSING, field

@dataclass
class PackageInstance:
    organization: str = field(default=MISSING, metadata={"required": True})
    feed: str = field(default=MISSING, metadata={"required": True})
    name: str = field(default=MISSING, metadata={"required": True})
    path: str = field(default=MISSING, metadata={"required": True})
    version: Optional[str] = field(default=None, metadata={"required": False, "allow_none": True})