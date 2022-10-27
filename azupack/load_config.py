from .exceptions import (ParseConfigError)

def load_config(config: dict):

    # check for schema version
    if "schema" not in config.keys():
        raise ParseConfigError(
            """Package configuraiton file is missing the field 'schema'. Please
            add 'schema': 'x.y.z' to 'azupack.json'.""")

    # check for a schema handler (use decorators to register schema handlers)
    