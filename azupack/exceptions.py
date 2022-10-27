
class ParseConfigError(Exception):
    """
    Exception type for reporting error while parsing the package configuration.
    """
    pass

class AuthInstanceNotFoundError(Exception):
    """
    No authentication isntance found for a organization / (feed).
    """
    pass