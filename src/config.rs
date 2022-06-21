use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub secrets: HashMap<String, String>,
}

static CONFIG_NAME: &str = "totp";

pub fn load_configuration() -> Result<Configuration, confy::ConfyError> {
    confy::load(CONFIG_NAME)
}

pub fn save_configuration(config: &Configuration) -> Result<(), confy::ConfyError> {
    confy::store(CONFIG_NAME, config)
}
