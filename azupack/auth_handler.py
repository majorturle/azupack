"""
Authentication handler for azure logins (support for multiple organizations)
with PAT (personal access token).
"""

import re
from dataclasses import dataclass
from typing import List
from .exceptions import AuthInstanceNotFoundError


@dataclass
class AuthInstance:
    organization: str
    token: str
    feed: str

class AuthHandler:
    instances: List[AuthInstance] = None

    def __init__(self) -> None:
        self.instances = []

    def add_auth(self, organization: str, token: str, feed: str = None):
        # type validation for the auth
        if type(organization) is not str:
            raise TypeError("The argument 'organization' has to be of type 'str'.")
        if type(token) is not str:
            raise TypeError("The argument 'token' has to be of type 'str'.")
        if not (type(feed) is str or feed is None):
            raise TypeError("The argument 'feed' has to be of type 'str' or 'None'.")

        # further validate the configurations
        url_expr = r"^https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&\/=]*)$"
        if re.match(url_expr, organization) is None:
            raise ValueError(f"The organization '{organization}' is not an URL!")

        # todo: further validate the values

        # if the feed is empty, leave it as is
        self.instances.append(
            AuthInstance(organization, token, feed=feed)
        )

    def get_auth(self, organization: str, feed:str) -> AuthInstance:
        # find the instances based on the organization
        matching_auth_instances = list(filter(
            lambda instance: instance.organization == organization, self.instances))

        if len(matching_auth_instances) == 0:
            raise AuthInstanceNotFoundError(
                f"No authentication instance found for organization '{organization}'!")

        # look for feed-specific data
        feed_auth_instances = list(filter(
            lambda instance: (instance.feed is not None) and (instance.feed == feed), matching_auth_instances))

        # one-single instance found
        if len(feed_auth_instances) == 1:
            return feed_auth_instances[0]
        elif len(feed_auth_instances) > 1:
            raise AuthInstanceNotFoundError(
                f"Multiple authentication instances were found for organizaition={organization}, feed={feed}."
                )
        else:
            # check for only-organization specific settings
            org_auth_instances = list(filter(
                lambda instance: (instance.feed is None), matching_auth_instances))

            if len(org_auth_instances) == 1:
                return org_auth_instances[0]
            else:
                # if none are found, that already covered with `if len(matching_auth_instances)`.
                raise AuthInstanceNotFoundError(
                    f"Multiple authentication instances were found for organizaition={organization} feed=None."
                    )