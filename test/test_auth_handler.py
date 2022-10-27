import unittest
from azupack import AuthHandler
from azupack.exceptions import AuthInstanceNotFoundError


class Test(unittest.TestCase):
    def test_auth_invalid_org_url(self):
        auth = AuthHandler()

        with self.assertRaises(ValueError) as context:
            auth.add_login(organization="", token="something funny", feed=None)

        with self.assertRaises(ValueError) as context:
            auth.add_login(organization="this is not an URL", token="something funny", feed=None)
    
    def test_auth_org_match(self):
        auth = AuthHandler()
        auth.add_login(organization="https://dev.azure.com/myorg/", token="abcd1234")
        auth.add_login(organization="https://dev.azure.com/other-org/", token="abcd1234")

        a = auth.get_login(organization="https://dev.azure.com/myorg/", feed=None)
        self.assertEqual(a.organization, "https://dev.azure.com/myorg/")
        self.assertEqual(a.token, "abcd1234")
        self.assertEqual(a.feed, None)

    def test_auth_org_match_slash_tolerance(self):
        auth = AuthHandler()
        auth.add_login(organization="https://dev.azure.com/myorg", token="abcd1234")
        auth.add_login(organization="https://dev.azure.com/other-org/", token="abcd1234")

        a = auth.get_login(organization="https://dev.azure.com/myorg/", feed=None)
        self.assertEqual(a.organization, "https://dev.azure.com/myorg/")
        self.assertEqual(a.token, "abcd1234")
        self.assertEqual(a.feed, None)