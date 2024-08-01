mod main_tests;
mod config;

use config::readconfig;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;


const MIN_TEMP: f64 = 30.0;
const MAX_TEMP: f64 = 70.0;
const FREQUENCY: f64 = 50.0;
const SLEEP1: u64 = 2000;
const SLEEPLOOP: u64 = 3000;
const MIN_FAN_SPEED: f64 = 0.0;
const MAX_FAN_SPEED: f64 = 1.0;





fn main() {
    // ctrlc crate for handling SIGTERM
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let my_pwm = Pwm::with_frequency(Channel::Pwm0, FREQUENCY, 1.0, Polarity::Normal, false);
    let my_pwm = match my_pwm {
		Ok(object) => object,
		Err(error) => panic!("no pwm: {:?} \n You might need to enable access to the PWM pins. Typically add \'dtoverlay=pwm\' to \'/boot/firmware/config.txt\' ",error),
	};

    let myconfig = readconfig();

    thread::sleep(Duration::from_millis(SLEEP1));
    if let Err(error) = my_pwm.enable() {
        panic!("pwm could not be enabled. {:?}", error)
    }

    thread::sleep(Duration::from_millis(SLEEP1));
    while running.load(Ordering::SeqCst) {
        // wait for SIGTERM
        let duty_cycle = myconfig.get_value(read_temp());

        if let Err(error) = my_pwm.set_duty_cycle(duty_cycle) {
            panic!("Could not set duty cycle.{:?}", error)
        }

        thread::sleep(Duration::from_millis(SLEEPLOOP));
    }

    println!("Shutting down fan_control. This will set the duty cycle to 0.0");
    if let Err(error) = my_pwm.set_duty_cycle(0.0) {
        panic!("Could not set duty cycle.{:?}", error)
    }
}

fn read_temp() -> f64 {
    let output = Command::new("vcgencmd")
        .arg("measure_temp")
        .output()
        .expect("Failed to execute command");

    let mystring = String::from_utf8(output.stdout).expect("Found invalid UTF-8");
    let temp = match mystring[5..9].parse::<f64>() {
        Ok(result) => result,
        Err(error) => panic!("could not parse.{:?}", error),
    };

    //println!("{}",temp);
    return temp.min(MAX_TEMP).max(MIN_TEMP);
}
