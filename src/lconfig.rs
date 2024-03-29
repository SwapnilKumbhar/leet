use log::{error, info};
use serde::Deserialize;
use serde_yaml;
use std::{fs, path::Path};

use crate::error::ConfigError;

static DEFAULT_HOME_PATH: &str = ".config/leet/config.yaml";

#[derive(Deserialize, Debug)]
pub struct LanguageTemplate {
    pub language: String,
    pub files: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSettings {
    pub template_dir: String,
}

#[derive(Deserialize, Debug)]
pub struct CfgData {
    pub settings: ConfigSettings,
    pub templates: std::collections::HashMap<String, LanguageTemplate>,
}

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub data: CfgData,
}

impl Config {
    pub fn new(path: String, data: CfgData) -> Self {
        Config { path, data }
    }

    pub fn get_templates(&self) -> Vec<String> {
        self.data.templates.keys().cloned().collect::<Vec<String>>()
    }

    pub fn get_template(&self, key: &String) -> Option<&LanguageTemplate> {
        self.data.templates.get(key)
    }
}

fn get_wk_paths() -> Result<String, ConfigError> {
    let home_path = match std::env::var("HOME") {
        Ok(path) => path,
        Err(e) => match e {
            std::env::VarError::NotPresent => {
                return Err(ConfigError::EnvNotFoundError {
                    var: "HOME".to_string(),
                })
            }
            _ => {
                return Err(ConfigError::OsError {
                    msg: "ENV is not Unicode".to_string(),
                })
            }
        },
    };

    let absolute_home_path = Path::new(&home_path).join(Path::new(DEFAULT_HOME_PATH));

    let path = if absolute_home_path.exists() {
        match absolute_home_path.into_os_string().into_string() {
            Ok(path) => path,
            Err(_) => return Err(ConfigError::OsStringError {}),
        }
    } else {
        error!("Failed to locate `config.yaml`.");
        error!("Please create one in `~/.config/leet` or pass an explicit path");
        return Err(ConfigError::ConfigNotFoundError {});
    };
    Ok(path)
}

fn validate_config(cfg: Config) -> Result<Config, ConfigError> {
    // write less.
    macro_rules! pathExistsOrErr {
        ($path:expr) => {
            if !Path::new(&$path).exists() {
                return Err(ConfigError::ValidationError {
                    msg: format!("Path [{}] does not exist", $path),
                });
            }
        };
    }
    // Ensure every path exists
    pathExistsOrErr!(cfg.path);

    // Validate CfgData
    pathExistsOrErr!(cfg.data.settings.template_dir);

    Ok(cfg)
}

pub fn parse_config(path: Option<String>) -> Result<Config, ConfigError> {
    let final_path = match path {
        Some(path) => {
            // Some path is specified
            path
        }
        None => {
            // No path specified, we check the well known paths
            get_wk_paths()?
        }
    };

    info!("Parsing YAML at path: {}", final_path);

    let config_file = match fs::File::open(final_path.clone()) {
        Ok(f) => f,
        Err(_) => {
            return Err(ConfigError::FileOpenError {
                file_name: final_path,
            })
        }
    };

    let cfg_data = match serde_yaml::from_reader(config_file) {
        Ok(v) => v,
        Err(e) => {
            return Err(ConfigError::YamlParseError { msg: e.to_string() });
        }
    };

    info!("Successfully parsed the Config file");

    // validate the file
    let cfg = Config::new(final_path, cfg_data);
    validate_config(cfg)
}
