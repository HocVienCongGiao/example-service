pub struct Config {
    pub(crate) db_user: String,
    pub(crate) db_password: String,
    pub(crate) db_name: String,
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
            db_user: "postgres".to_string(),
            db_password: "password".to_string(),
            db_name: "database_name".to_string(),
            os_type: ostype,
        }
    }
}
