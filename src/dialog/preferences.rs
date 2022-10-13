use std::fs;
use serde::{Deserialize, Serialize};
use crate::config::{self, SystemDir};

const PREFERENCES: &str = r"AppData\Local\Cisco\Cisco AnyConnect Secure Mobility Client\preferences.xml";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AnyConnectPreferences {
    #[serde(rename = "DefaultUser")]
    pub default_user: String,
    #[serde(rename = "DefaultHostName")]
    pub default_host: String
}

pub fn read_preferences() -> Option<AnyConnectPreferences> {
    match fs::read_to_string(config::from_system_dir(PREFERENCES, SystemDir::USERPROFILE)) {
        Ok(file) => serde_xml_rs::from_str(&file).ok(),
        _ => None
    }
}