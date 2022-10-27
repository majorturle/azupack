import unittest
from azupack import AuthHandler
from azupack.exceptions import AuthInstanceNotFoundError


class Test(unittest.TestCase):
    def test_auth_invalid_org_url(self):
        auth = AuthHandler()

        with self.assertRaises(ValueError) as context:
            auth.add_auth(organization="", token="something funny", feed=None)

        with self.assertRaises(ValueError) as context:
            auth.add_auth(organization="this is not an URL", token="something funny", feed=None)
    
    def test_auth_org_match(self):
        auth = AuthHandler()
        auth.add_auth()