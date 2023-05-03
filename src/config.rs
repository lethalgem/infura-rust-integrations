use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use dialoguer::Input;
use dirs::config_dir;

use crate::errors::{self, EssenError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InfuraKeys {
    // add config data here, ex an api key
    pub project_id: String,
}

pub fn get_infura_keys() -> Option<String> {
    let config_path = match get_config_path() {
        Ok(config_path) => config_path,
        Err(e) => {
            errors::handle_error(&e.to_string());
            return None;
        }
    };

    let project_id = if config_path.exists() {
        match read_and_parse_config(&config_path) {
            Ok(keys) => keys,
            Err(e) => {
                errors::handle_error(&e.to_string());
                match prompt_and_store_infura_keys() {
                    Ok(keys) => keys,
                    Err(e) => {
                        errors::handle_error(&e.to_string());
                        return None;
                    }
                }
            }
        }
    } else {
        match prompt_and_store_infura_keys() {
            Ok(keys) => keys,
            Err(e) => {
                errors::handle_error(&e.to_string());
                return None;
            }
        }
    };
    Some(project_id)
}

fn get_config_path() -> Result<PathBuf, EssenError> {
    let mut config_path = match config_dir() {
        Some(path) => path,
        None => {
            return Err(EssenError::ConfigDirNotFound);
        }
    };
    config_path.push("ecli");
    config_path.push("config");
    Ok(config_path)
}

fn prompt_and_store_infura_keys() -> Result<String, EssenError> {
    let project_id: String = Input::new()
        .with_prompt("Enter your Infura project id")
        .interact_text()
        .map_err(|e| EssenError::IOError("Failed to get user database id".to_string(), e))?;

    let key = InfuraKeys {
        project_id: project_id.clone(),
    };

    let keys_json = serde_json::to_string(&key)
        .map_err(|e| EssenError::JsonError("Failed to serialize keys".to_string(), e))?;

    let mut config_path = config_dir().ok_or(EssenError::ConfigDirNotFound)?;

    config_path.push("ecli");
    std::fs::create_dir_all(&config_path)
        .map_err(|e| EssenError::IOError("Failed to create config directory".to_string(), e))?;

    config_path.push("config");
    let mut file = File::create(&config_path)
        .map_err(|e| EssenError::IOError("Failed to create config file".to_string(), e))?;
    file.write_all(keys_json.as_bytes())
        .map_err(|e| EssenError::IOError("Failed to write API key to file".to_string(), e))?;

    Ok(project_id)
}

fn read_and_parse_config(config_path: &Path) -> Result<String, EssenError> {
    // let content = fs::read_to_string(config_path).map_err(|_| EssenError::ConfigReadError)?;
    let content = fs::read_to_string(config_path).map_err(|_| EssenError::ConfigReadError)?;

    let keys: InfuraKeys =
        serde_json::from_str(&content).map_err(|_| EssenError::ConfigParseError)?;

    if keys.project_id.is_empty() {
        Err(EssenError::ConfigReadError)
    } else {
        Ok(keys.project_id)
    }
}
