import imp
import json

from azupack.component.exceptions import ParseConfigError
from azupack.model.package_config import PackageConfig
from ..model import UniversalPackage

def parse_package_config(package: dict):
    if "organization" not in package.keys():
        raise ParseConfigError(f"Package instance does not contain field 'organization'. {str(package)}")
    if "feed" not in package.keys():
        raise ParseConfigError(f"Package instance does not contain field 'feed'. {str(package)}")
    if "name" not in package.keys():
        raise ParseConfigError(f"Package instance does not contain field 'name'. {str(package)}")
    if "path" not in package.keys():
        raise ParseConfigError(f"Package instance does not contain field 'path'. {str(package)}")

    organization = package["organization"]
    feed = package["feed"]
    name = package["name"]
    path = package["path"]
    version = package["version"]

    return UniversalPackage(organization, feed, name, path, version)


def parse_config(file: str) -> None:
    with open(file, "r") as f:
        data = json.load(f)

    # check for the list of packages
    if "packages" not in data.keys():
        raise ParseConfigError(f"Field 'packages' not found in the package configuration {file}")

    new_config = PackageConfig()

    package_list = data["packages"]
    if not type(package_list) == list:
        raise ParseConfigError(f"The field 'packages' has to be a list of objects.")

    # deserialize the json into python objects
    for package_dict in package_list:
        new_config.packages.append(parse_package_config(package_dict))

    return new_config