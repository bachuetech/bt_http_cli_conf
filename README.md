# Project Title
BT HTTP CLI CONFIG

## Description
The get_http_client_config function is used to retrieve the HTTP client configuration for a given environment. 
It loads the configuration from a YAML file specified by an environment variable or specific path if the environment variable is nout found 
and returns the configuration as a vector of tuples containing key-value pairs.

## Usage
```

const API_MGR_CONFIG_YML: &str = "config/http/client-config.yml";
const API_MGR_CONFIG_YML_ENV_VAR_NAME: &str = "BT_HTTPCLI_YMLCONFIGFILE";

let environment = "dev";

let conf = get_http_client_bool_config(environment.to_owned()
                                        , API_MGR_CONFIG_YML_ENV_VAR_NAME.to_string()
                                        , API_MGR_CONFIG_YML.to_string());
```

## Version History
* 0.1.0
    * Initial Release

## License
GPL-3.0-only