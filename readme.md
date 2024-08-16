# PWM fan control for raspberry pi

## Cross compiling
cross can be install by using `cargo install cross`

I use it for easier cross compiling. 

`cross build --release  --target aarch64-unknown-linux-gnu` for pi5 64bit and respectively `cross build --release --target armv7-unknown-linux-gnueabihf` for pi4 32bit.

# Installation and Usage
Download the *.deb package for your architecture and install it with `sudo dpkg -i *.deb`. Then enable the service by `systemctl enable fan-control --user` and reboot. This program is intended to run as regular user, not as system or root.

The fan's PWM control cable needs to be connected to PIN12 (GPIO18 - PWM0)

A configuration file for the pwm controll will be created in `~/.config/fan-control.config` and can be modified.

You can check the current state by running `systemctl status fan-control --user` or by `journalctl -b | grep fan-control` or by `journalctl -b | grep fan-control`

