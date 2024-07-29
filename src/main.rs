use std::process::Command;
use std::thread;
use std::time::Duration;
use rppal::pwm::Pwm;
use rppal::pwm::Channel;
use rppal::pwm::Polarity;

const MAX_TEMP: f64 = 70.0;
const MIN_TEMP: f64 = 30.0;
const FREQUENCY: f64 = 50.0;
const SLEEP1: u64 = 2000;
const SLEEPLOOP: u64 = 3000;


fn main() {
    let my_pwm = Pwm::with_frequency(Channel::Pwm0, FREQUENCY, 1.0, Polarity::Normal, false);
	let my_pwm = match my_pwm {
		Ok(object) => object,
		Err(error) => panic!("no pwm: {:?} \n You might need to enable access to the PWM pins. Typically add \'dtoverlay=pwm\' to \'/boot/firmware/config.txt\' ",error),
	};
	
	thread::sleep(Duration::from_millis(SLEEP1));
	let mut _result = my_pwm.enable();
	thread::sleep(Duration::from_millis(SLEEP1));
	loop{
		
		let temp = read_temp() as i32;
		let duty_cycle = match temp{
			0..=47 => 0.1,
			48..=50 => 0.2,
			51..=53 => 0.3,
			54..=56 => 0.4,
			57..=60 => 0.5,
			61 => 0.6,
			62 => 0.7,
			63 => 0.8,
			64 => 0.9,
			_ => 1.0,
		};
		
		_result = my_pwm.set_duty_cycle(duty_cycle);
		//println!("\rtemp: {}°C, duty cycle: {}% ", temp, duty_cycle*100.0);
		thread::sleep(Duration::from_millis(SLEEPLOOP));

	}
	
}

fn read_temp() -> f64{
	let output = Command::new("vcgencmd")
        .arg("measure_temp")
        .output()
        .expect("Failed to execute command");

	let mystring = String::from_utf8(output.stdout).expect("Found invalid UTF-8");
	let temp: f64 = mystring[5..9].parse().unwrap();
	//println!("{}",temp);
	return temp.min(MAX_TEMP).max(MIN_TEMP);
	
}
