use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_width: 325.00,
            window_height: 375.00,
        }
    }
}

pub fn load_or_create_config() -> Result<Config, io::Error> {
    let path = config_path()?;

    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let toml = toml::to_string(&Config::default())
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "toml"))?;

        fs::write(&path, toml)?;
    }

    let contents = fs::read_to_string(&path)?;
    toml::from_str(&contents)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "toml"))
}

fn config_path() -> Result<PathBuf, io::Error> {
    let mut path = base_dir()?;
    path.push("white-zone");
    path.push("config.toml");
    Ok(path)
}

#[cfg(windows)]
fn base_dir() -> Result<PathBuf, io::Error> {
    dirs::document_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "documents dir"))
}

#[cfg(not(windows))]
fn base_dir() -> Result<PathBuf, io::Error> {
    dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "config dir"))
}
