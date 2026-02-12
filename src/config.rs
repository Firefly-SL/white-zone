use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};
use eframe::egui;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub window: Window,
    pub theme: Theme,
    pub dot_grid: DotGrid,
}

// Window
#[derive(Serialize, Deserialize)]
pub struct Window {
    pub size: [f32; 2],
    pub resizable: bool,
    pub corner_radius: u8,
    pub position: [f32; 2],
    pub lock_in_center: bool,
    pub drop_shadow: DropShadow,
}

#[derive(Serialize, Deserialize)]
pub struct DropShadow {
    pub offset: [i8; 2],
    pub blur: u8,
    pub spread: u8,
    pub color: ColorConfig
}

// Theme
#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub background: ColorConfig,
    pub heading: ColorConfig,
}

// Grid
#[derive(Serialize, Deserialize)]
pub struct DotGrid {
    pub column_count: i32,
    pub color_past: ColorConfig,
    pub color_future: ColorConfig,
    pub color_today: ColorConfig,
    pub color_today_glow: ColorConfig,
}


fn parse_hex_color(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some((r, g, b))
}

impl ColorConfig {
    pub fn to_color32(&self) -> egui::Color32 {
        let (r, g, b) = parse_hex_color(&self.hex).unwrap_or((255, 0, 255));
        let a = self.opacity.unwrap_or(100);
        let a = ((a as f32 / 100.0) * 255.0).round() as u8;
        egui::Color32::from_rgba_unmultiplied(r, g, b, a)
    }
        
    pub fn with_alpha(&self, a: u8) -> egui::Color32 {
        let (r, g, b) = parse_hex_color(&self.hex).unwrap_or((255, 0, 255));
        egui::Color32::from_rgba_unmultiplied(r, g, b, a)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ColorConfig {
    pub hex: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<u8>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: Window { 
                size: [325.00, 375.00],
                resizable: false,
                position: [0.0, 0.0],
                corner_radius: 24,
                lock_in_center: true,
                drop_shadow: DropShadow { offset: [1, 1], blur: 4, spread: 2, color: ColorConfig { hex: "#151615".into(), opacity: Some(45) }}
            },
            theme: Theme {
                background: ColorConfig {
                    hex: "#161616".into(),
                    opacity: Some(100),
                },
                heading: ColorConfig {
                    hex: "#FEFEFE".into(),
                    opacity: Some(100),
                },
            },
            dot_grid: DotGrid { 
                column_count: 21,            
                color_past: ColorConfig { 
                    hex: "#525152".into(),
                    opacity: Some(100) 
                },
                color_today: ColorConfig { 
                    hex: "#fefefe".into(),
                    opacity: Some(100) 
                },
                color_future: ColorConfig { 
                    hex: "#d7d7d7".into(),
                    opacity: Some(100) 
                },
                color_today_glow: ColorConfig { 
                    // doesn't seems to work, gotta fix it later
                    hex: "#f0f0f0".into(),
                    opacity: Default::default()
                },
            }
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
