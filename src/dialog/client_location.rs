use std::path::PathBuf;
use inquire::error::InquireResult;
use inquire::{Confirm, CustomType};
use inquire::validator::Validation;

const DEFAULT_INSTALL: &str = r"C:\Program Files (x86)\Cisco\Cisco AnyConnect Secure Mobility Client";

pub fn check_install() -> InquireResult<PathBuf> {
    let default_install: PathBuf = DEFAULT_INSTALL.into();
    if default_install.exists() && confirm_default_installation()? {
        Ok(default_install)
    } else {
        ask_for_custom_installation()
    }
}

fn confirm_default_installation() -> InquireResult<bool> {
    let question = format!("Detected installation on '{}'. Is this correct?", DEFAULT_INSTALL);
    Confirm::new(&question)
        .with_default(true)
        .prompt()
}

fn ask_for_custom_installation() -> InquireResult<PathBuf> {
    let question = "Default installation not found. What is your installation folder?";
    CustomType::<String>::new(question)
        .with_help_message(&format!("Example: '{}'", DEFAULT_INSTALL))
        .with_validator(|value: &String| {
            let value: PathBuf = value.into();
            if value.exists() &&
                value.is_dir() &&
                value.join("vpncli.exe").exists() &&
                value.join("vpnui.exe").exists() {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("You must provide a valid installation path".into()))
            }
        }).prompt().map(|value| value.into())
}