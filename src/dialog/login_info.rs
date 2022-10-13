use inquire::{Confirm, Password, PasswordDisplayMode, required, Text, error::InquireResult};
use crate::dialog::preferences;

const HOST_LABEL: &str = "VPN host";
const USER_LABEL: &str = "User";

pub fn ask_for_login() -> InquireResult<(String, String, String)> {
    if let Some(preferences) = preferences::read_preferences() {
        let host = check_default(preferences.default_host, HOST_LABEL)?;
        let user = check_default(preferences.default_user, USER_LABEL)?;
        Ok((host, user, ask_for_password()?))
    } else {
        Ok((ask_for_text(HOST_LABEL)?, ask_for_text(USER_LABEL)?, ask_for_password()?))
    }
}

fn check_default(default_value: String, label: &str) -> InquireResult<String> {
    if !default_value.is_empty() && confirm_default_text(&default_value, label)? {
        Ok(default_value)
    } else {
        ask_for_text(label)
    }
}

fn ask_for_text(label: &str) -> InquireResult<String> {
    Text::new(&format!("{}:", label))
        .with_validator(required!())
        .prompt()
}

fn ask_for_password() -> InquireResult<String> {
    Password::new("Password:")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Masked)
        .with_validator(required!())
        .with_formatter(&|_| String::from("Input received"))
        .prompt()
}

fn confirm_default_text(default_text: &str, label: &str) -> InquireResult<bool> {
    let question = format!("Detected default {}: '{}'. Is this correct?", label, default_text);
    Confirm::new(&question)
        .with_default(true)
        .prompt()
}