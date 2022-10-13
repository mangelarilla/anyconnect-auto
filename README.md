# Auto-login for Cisco AnyConnect Windows client
I made this helper application to provide user/password auto-login to the AnyConnect Windows client on system startup.
It asks a few initial questions during setup to retrieve the right configuration and adds itself to the Windows startup.
## DISCLAIMER
This only works for the Windows Cisco AnyConnect client, I believe macOS and Linux already have user/password auto-login sorted out in their respective clients.
This is an open side project unrelated to Cisco, use it at your own risk.
The AnyConnect password is stored unencrypted along with the configuration, for the moment I don't have a more secure solution that doesn't involve prompting the user every time, which ruins the whole idea of auto-login.
## Installation
Download the latest release, and run it.
## How it works
During the Installation, it will ask for your configuration, most of it is inferred from the defaults and the AnyConnect preferences file, so you just need to confirm is alright.
Then, it will copy the executable and the config to your user home, in the `.anyconnect-auto` directory, and create a shortcut in the startup directory so that it auto-starts with Windows.
This auto-login functionality is provided using the `vpncli.exe` that AnyConnect client has available. This is the only known way to do it in Windows [afaik](https://stackoverflow.com/questions/21682121/cisco-vpn-client-automatic-login).
## I want to change the existing configuration
Either delete the config file in `%USERPROFILE%\.anyconnect-auto\config` and run the exe there to recreate the config, or simply change this config file values (it's just an xml).