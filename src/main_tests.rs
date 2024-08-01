#![cfg(test)]

use super::*;

#[test]
fn check_xdg() {
    let xdg_base = xdg::BaseDirectories::new().unwrap();
    let path = xdg_base.get_config_home();
    println!("{:?}", path);
}

#[test]
fn check_config_file() {
    let cf = readconfig();
    println!("{:?}", cf);
}
