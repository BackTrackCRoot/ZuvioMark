use failure::Error;
use serde_json;
use std::fs::{self, File};
use std::path::Path;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub user_id: String,
    pub access_token: String,
}

impl UserConfig {
    pub fn gnerate_config(&self) -> Result<(), Error> {
        let path = Path::new("./config.json");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(Path::new("./config.json"))?;
        serde_json::to_writer_pretty(&mut file, self)?;
        Ok(())
    }
    pub fn load_config(&self) -> Result<Self, Error> {
        let file = File::open(Path::new("./config.json"))?;
        Ok(serde_json::from_reader(&file)?)
    }
}