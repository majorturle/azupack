
class UniversalPackage:
    """
    Universal Package for defining the properties and download options of a
    universal package from Azure DevOps.
    """
    organization: str = None
    feed: str = None
    name: str = None
    path: str = None
    version: str = None

    def __init__(self, organization: str, feed: str, name: str, path: str, version: str):
        # type validations for the input
        if type(organization) is not str:
            raise TypeError("The field 'organization' has to be a 'str' type.")
        if type(feed) is not str:
            raise TypeError("The field 'feed' has to be a 'str' type.")
        if type(name) is not str:
            raise TypeError("The field 'name' has to be a 'str' type.")
        if type(version) is not str:
            raise TypeError("The field 'version' has to be a 'str' or 'None' type.")

        self.organization = organization
        self.feed = feed
        self.name = name
        self.path = path
        self.version = version