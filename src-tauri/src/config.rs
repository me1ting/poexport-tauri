use std::{mem, path::PathBuf};

use crate::utils;
use anyhow::Result;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(rename = "poeSessId")]
    pub poesessid: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            poesessid: Default::default(),
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    lock: RwLock<Config>,
}

impl ConfigManager {
    pub fn new(config_path: PathBuf) -> ConfigManager {
        let config = ConfigManager::load_or_default(&config_path);
        return ConfigManager {
            config_path,
            lock: RwLock::new(config),
        };
    }

    /// Load config from path, or use default config if some wrongs happened
    pub fn load_or_default(path: &PathBuf) -> Config {
        match utils::files::read_json::<Config>(&path) {
            Ok(config) => config,
            Err(err) => {
                log::error!("{:#}", err);
                log::info!("use default config");
                let config: Config = Default::default();
                if let Err(err) = utils::files::write_json(path, &config) {
                    log::error!("{:#}", err);
                }
                config
            }
        }
    }

    pub fn save(&self, config: &Config) -> Result<()> {
        utils::files::write_json(&self.config_path, config)
    }

    pub fn get(&self) -> Config {
        self.lock.read().clone()
    }

    pub fn reset(&self) -> Result<Config> {
        let mut config = self.lock.write();
        mem::take(&mut (*config));
        self.save(&config)?;
        Ok(config.clone())
    }

    pub fn set_poesessid(&self, id: &str) -> Result<()> {
        let mut config = self.lock.write();
        if config.poesessid == id {
            return Ok(());
        }
        config.poesessid = String::from(id);
        self.save(&config)
    }
}
