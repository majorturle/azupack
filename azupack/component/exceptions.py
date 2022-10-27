class AuthInstanceNotFoundError(Exception):
    """
    No authentication isntance found for a organization / (feed).
    """
    pass

class AuthError(Exception):
    """Error while logging in for a DevOps organization."""
    pass

class ParseConfigError(Exception):
    """
    Exception type for reporting error while parsing the package configuration.
    """
    pass