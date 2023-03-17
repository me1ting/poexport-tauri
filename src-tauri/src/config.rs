use std::{mem, path::PathBuf};

use crate::utils::paths;
use anyhow::Result;
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

const CONFIG_FILE_NAME: &str = "config.json";

const DEFAULT_LISTENING_PORT: u16 = 8655;

static CONFIG_MANAGER: OnceCell<ConfigManager> = OnceCell::new();

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    poe_session_id: String,
    #[serde(default)]
    pob_path: String,
    #[serde(default = "default_listening_port")]
    listening_port: u16,
    #[serde(default)]
    pob_proxy_supported: bool,
}

fn default_listening_port() -> u16 {
    DEFAULT_LISTENING_PORT
}

impl Default for Config {
    fn default() -> Config {
        Config {
            poe_session_id: Default::default(),
            pob_path: Default::default(),
            listening_port: DEFAULT_LISTENING_PORT,
            pob_proxy_supported: Default::default(),
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    lock: RwLock<Config>,
}

impl ConfigManager {
    /// The global [ConfigManager]
    ///
    /// initilize by `config::init()`, can only be accessed after initialization
    pub fn global() -> &'static ConfigManager {
        CONFIG_MANAGER
            .get()
            .expect("config manager is not initialized")
    }

    /// Load config from path, or use default config if some wrongs happen
    fn load_or_default_config(path: &PathBuf) -> Config {
        match paths::read_yaml::<Config>(&path) {
            Ok(config) => config,
            Err(err) => {
                log::error!("{:#}", err);
                log::info!("use default config");
                let config: Config = Default::default();
                if let Err(err) = paths::write_yaml(path, &config) {
                    log::error!("{:#}", err);
                }
                config
            }
        }
    }

    pub fn save_config(&self, config: &Config) -> Result<()> {
        paths::write_yaml(&self.config_path, config)
    }

    pub fn get_config(&self) -> Config {
        self.lock.read().clone()
    }

    pub fn reset_config(&self) -> Result<Config> {
        let mut config = self.lock.write();
        mem::take(&mut (*config));
        self.save_config(&config)?;
        Ok(config.clone())
    }

    pub fn set_poe_session_id(&self, id: &str) -> Result<()> {
        let mut config = self.lock.write();
        if config.poe_session_id == id {
            return Ok(());
        }
        config.poe_session_id = String::from(id);
        self.save_config(&config)
    }

    pub fn set_pob_path(&self, path: &str) -> Result<()> {
        let mut config = self.lock.write();
        if config.pob_path == path {
            return Ok(());
        }
        config.pob_path = String::from(path);
        self.save_config(&config)
    }

    pub fn set_listening_port(&self, port: u16) -> Result<()> {
        let mut config = self.lock.write();
        if config.listening_port == port {
            return Ok(());
        }
        config.listening_port = port;
        self.save_config(&config)
    }

    pub fn set_pob_proxy_supported(&self, supported: bool) -> Result<()> {
        let mut config = self.lock.write();
        if config.pob_proxy_supported == supported {
            return Ok(());
        }
        config.pob_proxy_supported = supported;
        self.save_config(&config)
    }
}

pub fn init(config_dir: &PathBuf) -> Result<()> {
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    let config = ConfigManager::load_or_default_config(&config_path);
    let manager = ConfigManager {
        config_path,
        lock: RwLock::new(config),
    };
    match CONFIG_MANAGER.set(manager) {
        Ok(()) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("it should never happen")),
    }
}
