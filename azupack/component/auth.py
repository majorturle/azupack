"""
Authentication handler for azure logins (support for multiple organizations)
with PAT (personal access token).
"""

import re
import click
from dataclasses import dataclass
from typing import List
from .exceptions import AuthError, AuthInstanceNotFoundError
from .az_command import AZCommand


@dataclass
class AuthInstance:
    organization: str  # Organization URL
    token: str         # Personal Access Token (PATH)
    logged_in: bool    # indicates whether the instance was already used for login

class Auth:
    instances: List[AuthInstance] = None

    def __init__(self) -> None:
        self.instances = []

    def _normalize_org(self, org: str) -> str:
        norm_org = org
        if not org.endswith('/'):
            norm_org += '/'
        return norm_org

    def add_login(self, organization: str, token: str):
        # type validation for the auth
        if type(organization) is not str:
            raise TypeError("The argument 'organization' has to be of type 'str'.")
        if type(token) is not str:
            raise TypeError("The argument 'token' has to be of type 'str'.")

        # further validate the configurations
        url_expr = r"^https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&\/=]*)$"
        if re.match(url_expr, organization) is None:
            raise ValueError(f"The organization '{organization}' is not an URL!")

        # always add the '/' at the end of the organization
        organization = self._normalize_org(organization)

        # TODO: further validate the token type

        # if the feed is empty, leave it as is
        self.instances.append(
            AuthInstance(organization=organization, token=token, logged_in=False)
        )

    def _get_login(self, organization: str) -> List[AuthInstance]:
        """
        Get the list of authentication instances for a DevOps organization.

        Args:
            organization (str): Organization URL.

        Raises:
            AuthInstanceNotFoundError: No auth instance was found for the given organization.

        Returns:
            List[AuthInstance]: List of auth. instances for the given organization. All of them should be
               used for login by the login handler.
        """
        organization = self._normalize_org(organization)

        # find the instances based on the organization
        matching_auth_instances = list(filter(
            lambda instance: instance.organization == organization, self.instances))

        if len(matching_auth_instances) == 0:
            raise AuthInstanceNotFoundError(
                f"No authentication instance found for organization '{organization}'!")
        else:
            return matching_auth_instances

    def _perform_login(self, inst: AuthInstance) -> None:
        # TODO: command wrapper should be passed by the CLI
        command = AZCommand()
        args = [f"devops login --organization {inst.organization}"]
        result = command.run(inst.token, args)

        if result.exit_code == 0:
            click.echo(f"Successful login to '{inst.organization}'.")
        else:
            raise AuthError(f"Cannot login to '{inst.organization}'. Reason: {result.stderr}")

    def login(self, organization:str = None):
        logins = self._get_login(organization=organization)
        for inst in logins:
            self._perform_login(inst)

    def login_all(self):
        for inst in self.instances:
            self._perform_login(inst)