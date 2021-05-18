pub struct Config {
    pub(crate) db_user: Option<String>,
    pub(crate) db_password: Option<Vec<u8>>,
    pub(crate) db_name: Option<String>,
    pub(crate) db_options: Option<String>,
    pub(crate) application_name: Option<String>,
    pub(crate) os_type: String,
}

impl Default for Config {
    fn default() -> Config {
        Config::new()
    }
}

impl Config {
    pub fn new() -> Config {
        let ostype_env_result = std::env::var("OSTYPE");
        let mut ostype: String = "windows".to_string();
        let ostype_env_lowercase = ostype_env_result
            .unwrap_or_else(|_| String::from("windows"))
            .to_lowercase();
        match &ostype_env_lowercase[0..5] {
            "linux" => {
                ostype = String::from("linux");
            }
            "darwi" => {
                ostype = String::from("darwin");
            }
            _ => {}
        }

        Config {
            db_user: None,
            db_password: None,
            db_name: None,
            db_options: None,
            application_name: None,
            os_type: ostype,
        }
    }
}
