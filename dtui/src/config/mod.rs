use crate::domain::AppError;
use defaults::default_keybindings;
use std::collections::HashMap;
use std::path::PathBuf;

mod defaults;
mod keybind;

pub use keybind::KeyBindingMap;

#[derive(Debug)]
pub struct Config {
    pub keybindings: KeyBindingMap,
    pub config_path: Option<PathBuf>,
}

impl Config {
    pub fn load() -> Result<Self, AppError> {
        let config_path = Self::config_file_path();
        let mut keybindings_map = default_keybindings();

        if let Some(path) = &config_path {
            if path.exists() {
                match std::fs::read_to_string(path) {
                    Ok(content) => match toml::from_str::<TomlConfig>(&content) {
                        Ok(toml_config) => {
                            if let Some(kb) = toml_config.keybindings {
                                keybindings_map.extend(kb);
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: failed to parse config file: {e}");
                        }
                    },
                    Err(e) => {
                        eprintln!("Warning: failed to read config file: {e}");
                    }
                }
            }
        }

        let keybindings = KeyBindingMap::from_config(&keybindings_map);

        Ok(Config {
            keybindings,
            config_path,
        })
    }

    fn config_file_path() -> Option<PathBuf> {
        directories::BaseDirs::new().map(|dirs| dirs.config_dir().join("dtui").join("config.toml"))
    }
}

#[derive(Debug, serde::Deserialize)]
struct TomlConfig {
    keybindings: Option<HashMap<String, String>>,
}
