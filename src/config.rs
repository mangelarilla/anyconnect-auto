use std::{path::PathBuf, env, fs, error::Error, fmt};
use serde::{Serialize, Deserialize};

const CONFIG_PATH: &str = r".anyconnect-auto\config";

#[derive(Debug)]
pub enum SystemDir {
    USERPROFILE, APPDATA
}

impl fmt::Display for SystemDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn from_system_dir(path: &str, dir: SystemDir) -> PathBuf {
    let home_dir: PathBuf = env::var(dir.to_string())
        .expect("Can't find system variable USERPROFILE")
        .into();
    home_dir.join(path)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "HostName")]
    pub host: String,
    #[serde(rename = "CiscoPath")]
    pub cisco_path: PathBuf
}

pub fn save(settings: Settings) -> Result<PathBuf, Box<dyn Error>> {
    let serialized = serde_xml_rs::to_string(&settings)?;
    let path = from_system_dir(CONFIG_PATH, SystemDir::USERPROFILE);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, serialized)?;
    Ok(path)
}

pub fn read_settings() -> Option<Settings> {
    let path = from_system_dir(CONFIG_PATH, SystemDir::USERPROFILE);
    match fs::read_to_string(path) {
        Ok(file) => serde_xml_rs::from_str(&file).ok(),
        _ => None
    }
}