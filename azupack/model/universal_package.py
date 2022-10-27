from ..exceptions import ParseConfigError
import azure

# forward declare for type annotations
class UniversalPackage:
    pass

class UniversalPackage:
    """
    Universal Package for defining the properties and download options of a
    universal package from Azure DevOps.
    """
    organization: str = None
    feed: str = None
    name: str = None
    path: str = None
    version: None

    def __init__(self, organization: str, feed: str, name: str, path: str, version: str = None):
        # type validations for the input
        if type(organization) is not str:
            raise TypeError("The field 'organization' has to be a 'str' type.")
        if type(feed) is not str:
            raise TypeError("The field 'feed' has to be a 'str' type.")
        if type(name) is not str:
            raise TypeError("The field 'name' has to be a 'str' type.")
        if type(version) is not str and type(version) is not None:
            raise TypeError("The field 'version' has to be a 'str' or 'None' type.")

        self.organization = organization
        self.feed = feed
        self.name = name
        self.path = path
        self.version = version

    def from_dict(package: dict) -> UniversalPackage:
        """
        Create a Package object based on a dictionary containing the configuraiton.

        Args:
            package (dict): Configuraiton dictionary (loaded from json).

        Raises:
            ParseConfigError: Error while parsing the configuration dictionary.

        Returns:
            Package: Package object.
        """

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
