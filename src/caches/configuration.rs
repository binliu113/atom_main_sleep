use super::config_parser::ConfigParser;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub config: ConfigParser,
}

impl Configuration {
    pub fn new() -> Self {
        let config = ConfigParser::new();
        Self {
            config: config.clone(),
        }
    }
}