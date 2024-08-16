# PWM fan control for raspberry pi

## Cross compiling
cross can be install by using `cargo install cross`

I use it for easier cross compiling. 

`cross build --release  --target aarch64-unknown-linux-gnu` for pi5 64bit and respectively `cross build --release --target armv7-unknown-linux-gnueabihf` for pi4 32bit.

# Usage
Copy `fancontrol.service` to `/etc/systemd/user/` on your raspberry pi. Then enable the service by `systemctl enable fancontrol --user` and reboot. This program is intended to run as regular user, not as system or root.

A configuration file for the pwm controll will be created in `~/.config/fancontrol.config` and can be modified.

You can check the current state by running `systemctl status fancontrol --user` or by `journalctl -b | grep fancontrol` or by `journalctl -b | grep fan_control`

# Changelog
## Version 0.2.0
This version supports now a configuration file. And can be gracefully shutdown via `systemctl stop`.
### Version 0.2.1
updated dependencies
