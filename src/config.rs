use std::fs::File;

use failure::Error;
use serde_json;

use crate::api::UserInfo;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    #[serde(flatten)]
    pub user_info: UserInfo,
}

impl UserConfig {
    pub fn gnerate_config(&self) -> Result<(), Error> {
        let mut file = File::create("./config.json")?;
        serde_json::to_writer_pretty(&mut file, self)?;
        Ok(())
    }

    pub fn load_config() -> Result<Self, Error> {
        let file = File::open("./config.json")?;
        Ok(serde_json::from_reader(&file)?)
    }
}
