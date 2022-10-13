use std::{process::{Command, Stdio}, io::{Write, self}};
use crate::config::Settings;

pub fn kill_vpn_ui() {
    println!("Ending the UI application to prevent conflicts");
    match Command::new("taskkill").args(["/IM","vpnui.exe","/F"]).output() {
        Ok(output) => println!("{}", String::from_utf8(output.stdout).unwrap_or("-----".to_string())),
        Err(e) => eprintln!("{}", e)
    }
}

pub fn connect(settings: &Settings) -> io::Result<()> {
    let mut vpncli = Command::new(settings.cisco_path.join("vpncli.exe"))
        .arg("-s")
        .stdin(Stdio::piped())
        .spawn()?;
    let vpncli_input = vpncli.stdin.as_mut().expect("Could not borrow stdin as mutable");
    vpncli_input.write_all(&format!("connect {}\n{}\n{}", settings.host, settings.user, settings.password).into_bytes())?;
    vpncli.wait()?;
    Ok(())
}

pub fn start_vpn_ui(settings: &Settings) -> io::Result<()> {
    let path = settings.cisco_path.join("vpnui.exe");
    Command::new("powershell")
        .arg(format!("start {:?}", path))
        .status()
        .map(|_| ())
}