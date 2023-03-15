use std::{
    fs::{self, File},
    io::{Read, Write},
    mem,
    path::Path,
    sync::RwLock,
};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

const BUNDLE_IDENTIFIER: &str = "cn-poe-community";
const CONFIG_FILE_NAME: &str = "config.json";

const DEFAULT_LISTENING_PORT: u16 = 8655;

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

impl Config {
    pub fn new(json: &str) -> Config {
        serde_json::from_str(json).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
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
    config_path: String,
    lock: RwLock<Config>,
}

impl ConfigManager {
    pub fn global() -> &'static ConfigManager {
        static CONFIG_MANAGER: OnceCell<ConfigManager> = OnceCell::new();

        CONFIG_MANAGER.get_or_init(|| {
            let config_path = Self::get_config_path();
            let config = Self::load_or_default(&config_path);

            ConfigManager {
                config_path,
                lock: RwLock::new(config),
            }
        })
    }

    fn get_config_path() -> String {
        let home_dir = tauri::api::path::config_dir().unwrap();
        let config_path = home_dir.join(BUNDLE_IDENTIFIER).join(CONFIG_FILE_NAME);
        String::from(config_path.as_path().to_str().unwrap())
    }

    /// Load config from path, or use default config if the file not exist.
    fn load_or_default(path: &str) -> Config {
        if let Ok(mut file) = File::open(path) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            return serde_json::from_str(&contents).unwrap();
        } else {
            let config = Default::default();
            Self::save(path, &config);
            return config;
        }
    }

    fn save(path_str: &str, config: &Config) {
        let path = Path::new(path_str);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut file = File::create(path).unwrap();
        file.write_all(config.to_json().as_bytes()).unwrap();
    }

    pub fn get_config(&self) -> Config {
        self.lock.read().unwrap().clone()
    }

    pub async fn reset_config(&mut self) {
        let mut config = self.lock.write().unwrap();
        mem::take(&mut (*config));
        Self::save(&self.config_path, &config);
    }

    pub async fn set_poe_session_id(&mut self, id: &str) {
        let config = self.lock.read().unwrap();
        if config.poe_session_id == id {
            return;
        }
        let mut config = self.lock.write().unwrap();
        config.poe_session_id = String::from(id);
        Self::save(&self.config_path, &config);
    }

    pub async fn set_pob_path(&mut self, path: &str) {
        let config = self.lock.read().unwrap();
        if config.pob_path == path {
            return;
        }
        let mut config = self.lock.write().unwrap();
        config.pob_path = String::from(path);
        Self::save(&self.config_path, &config);
    }

    pub async fn set_listening_port(&mut self, port: u16) {
        let config = self.lock.read().unwrap();
        if config.listening_port == port {
            return;
        }
        let mut config = self.lock.write().unwrap();
        config.listening_port = port;
        Self::save(&self.config_path, &config);
    }

    pub async fn set_pob_proxy_supported(&mut self, supported: bool) {
        let config = self.lock.read().unwrap();
        if config.pob_proxy_supported == supported {
            return;
        }
        let mut config = self.lock.write().unwrap();
        config.pob_proxy_supported = supported;
        Self::save(&self.config_path, &config);
    }
}
