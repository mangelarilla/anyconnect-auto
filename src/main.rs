use std::error::Error;

mod installer;
mod connect;
mod config;
mod dialog;

fn main() {
    if let Some(settings) = config::read_settings() {
        if let Err(e) = connect(settings) {
            eprintln!("{}", e);
        }
    } else if let Err(e) = install() {
        eprintln!("{}", e);
    }
}

fn install() -> Result<(), Box<dyn Error>> {
    let installation_path = dialog::client_location::check_install()?;
    let (host, user, password) = dialog::login_info::ask_for_login()?;
    let config_path = config::save(config::Settings {
        user, password, host,
        cisco_path: installation_path
    })?;
    println!("Settings saved to {:?}", config_path);
    let exe_path = installer::copy_executable()?;
    println!("Application installed in {:?}", exe_path);
    installer::link_to_windows_startup()?;
    println!("Added to windows startup");
    println!("Installation completed! Press any key to exit");
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}

fn connect(settings: config::Settings) -> std::io::Result<()> {
    connect::kill_vpn_ui();
    connect::connect(&settings)?;
    connect::start_vpn_ui(&settings)?;
    Ok(())
}