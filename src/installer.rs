use std::{env, fs, io, path::PathBuf};
use crate::config::{self, SystemDir};

const EXE_PATH: &str = r".anyconnect-auto\anyconnect-auto.exe";

pub fn copy_executable() -> io::Result<PathBuf> {
    let executable = env::current_exe()?;
    let path = config::from_system_dir(EXE_PATH, SystemDir::USERPROFILE);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(executable, &path)?;
    Ok(path)
}

pub fn link_to_windows_startup() -> io::Result<()> {
    let startup_cmd = config::from_system_dir(r"Microsoft\Windows\Start Menu\Programs\Startup\anyconnect-auto.cmd", SystemDir::APPDATA);
    let exe = config::from_system_dir(EXE_PATH, SystemDir::USERPROFILE);
    fs::write(startup_cmd, format!("{}\nexit", exe.to_string_lossy()))
}