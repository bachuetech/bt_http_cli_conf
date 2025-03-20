use bt_logger::{log_error, log_warning};
use bt_yaml_utils::get_yaml;
use yaml_rust2::Yaml;

///This function reads configuration data from a YAML file based on an environment variable or 
///a specific path and returns a vector of tuples containing keys and their corresponding boolean values.
/// Parameters:
/// environment (String): The name of the environment to look for in the configuration (e.g. dev, prod).
/// confi_env_var_name (String): The name of the environment variable that contains the YAML file path.
/// config_yaml_file (String): The path to the YAML configuration file. It is used only if the environment variable is not found
/// Returns:
/// Returns an 'Option', which is a vector containing tuples where each tuple consists of:
///     A string key
///     A boolean value corresponding to that key.
///     If no valid entries are found, or if there's an error during processing (the configuration is not found or invalid), the function returns None.
    pub fn get_http_client_bool_config(environment: &String, confi_env_var_name: &String, config_yaml_file: &String) -> Option<Vec<(String, bool)>>{
        let api_config: Yaml;
        
        match get_yaml(confi_env_var_name.as_str(), config_yaml_file.as_str()) {
            Ok(y_file_conf) => api_config = y_file_conf,
            Err(e) => {
                log_error!("get_http_client_config","Error Reading HTTP Client configuration ('{}'). Will use default values and continue. Error: {}",&environment, e); 
                return None
            }
        }

        let mut int_config: Vec<(String, bool)> = Vec::new();

        if let Some(env_config) = api_config[environment.as_str()].as_hash().take() {
            for tuple in env_config{
                if let Some(key) =  tuple.0.as_str(){
                    if let Some(value) = tuple.1.as_bool(){
                        int_config.push((key.to_owned(),value));
                    }else{
                        log_warning!("get_http_client_config","Invalid boolean value for key {}. Ignoring line",&key);
                    }
                }else{
                    log_warning!("get_http_client_config","Invalid str value in {:?}. Ignoring line",&tuple);
                }
            }
        }else{
            log_warning!("get_http_client_config","Invalid Environment {}. Returning None",&environment);
            return None;
        }

        if int_config.len() > 0{
            return Some(int_config)
        }else{
            log_warning!("get_http_client_config","No Configurations found. Returning None");
            return None
        }
    }

//***********/
// UNIT TEST 
//***********/
    #[cfg(test)]
    mod config_tests {
        use bt_logger::{build_logger, LogLevel, LogTarget};

        use crate::get_http_client_bool_config;

        #[cfg(test)]
        const API_MGR_CONFIG_YML: &str = "config/http/client-config.yml";
        #[cfg(test)]
        const API_MGR_CONFIG_YML_ENV_VAR_NAME: &str = "BT_HTTPCLI_YMLCONFIGFILE";

        #[test]
        pub fn read_config_test_empty() {
            build_logger("BACHUETECH", "HTTP_CONFIG", LogLevel::VERBOSE, LogTarget::STD_OUT);
            let environment = "empty";
            let conf = get_http_client_bool_config(&environment.to_owned(), &API_MGR_CONFIG_YML_ENV_VAR_NAME.to_owned()
                                                                , &API_MGR_CONFIG_YML.to_owned());
            //let resp = vec!(("danger_accept_invalid_hostnames".to_string(), true), ("danger_accept_invalid_certs".to_string(), false));
            println!("Config {:?}",conf);
            assert_eq!(conf,None);
        }

        #[test]
        pub fn read_config_test_wrong_path() {
            build_logger("BACHUETECH", "HTTP_CONFIG", LogLevel::VERBOSE, LogTarget::STD_OUT);
            let environment = "empty";
            let conf = get_http_client_bool_config(&environment.to_owned()
                                                                ,&"wrong".to_string()
                                                                , &"wrong".to_string());
            //let resp = vec!(("danger_accept_invalid_hostnames".to_string(), true), ("danger_accept_invalid_certs".to_string(), false));
            println!("Config {:?}",conf);
            assert_eq!(conf,None);
        }

        #[test]
        pub fn read_config_test_success() {
            build_logger("BACHUETECH", "HTTP_CONFIG", LogLevel::VERBOSE, LogTarget::STD_OUT);
            let environment = "dev";
            let conf = get_http_client_bool_config(&environment.to_owned(), &API_MGR_CONFIG_YML_ENV_VAR_NAME.to_string()
                                                                , &API_MGR_CONFIG_YML.to_string());
            let resp = vec!(("danger_accept_invalid_hostnames".to_string(), true), ("danger_accept_invalid_certs".to_string(), false));
            println!("Config {:?}",conf);
            assert_eq!(conf,Some(resp));
        }

        #[test]
        pub fn read_config_test_invalid_env() {
            build_logger("BACHUETECH", "HTTP_CONFIG", LogLevel::VERBOSE, LogTarget::STD_OUT);
            let environment = "INVALID";
            let conf = get_http_client_bool_config(&environment.to_owned(), &API_MGR_CONFIG_YML_ENV_VAR_NAME.to_string(),
                                                & API_MGR_CONFIG_YML.to_string());
            println!("Config {:?}",conf);
            assert_eq!(conf,None);
        }

        #[test]
        pub fn read_config_test_wrong_bool() {
            build_logger("BACHUETECH", "HTTP_CONFIG", LogLevel::VERBOSE, LogTarget::STD_OUT);
            let environment = "err_wrong";
            let conf = get_http_client_bool_config(&environment.to_owned(), &API_MGR_CONFIG_YML_ENV_VAR_NAME.to_string(), 
                                            &API_MGR_CONFIG_YML.to_string());
            println!("Config {:?}",conf);
            assert_eq!(conf,None);
        }
    }   