use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use super::{
    MIN_TEMP,
    MAX_TEMP,
    MIN_FAN_SPEED,
    MAX_FAN_SPEED,
};


#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    points: Vec<Point>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            points: vec![Point {
                x: MIN_TEMP,
                y: MIN_FAN_SPEED,
            }],
        }
    }

    pub fn add(&mut self, p: Point) {
        if self.points.len() == 0 {
            self.points.push(p)
        } else {
            let last_point = self.points.last().unwrap();
            if last_point.x == p.x && last_point.y == p.y {
                return;
            }
            if p.x > last_point.x && p.y >= last_point.y {
                self.points.push(p);
                return;
            }
            if p.x > last_point.x && p.y < last_point.y {
                let y = p.y.max(last_point.y);
                self.points.push(Point::new(p.x, y));
                return;
            }
            if p.x == last_point.x {
                let y = p.y.max(last_point.y);
                self.points.pop();
                self.points.push(Point::new(p.x, y));
            }
        }
    }

    pub fn get_value(&self, x: f64) -> f64 {
        if self.points.len() < 2 {
            panic!("config not enough")
        }
        let mut p_left = self.points.first().unwrap();
        let mut p_right = self.points.last().unwrap();
        for element in &self.points {
            if x >= element.x {
                p_left = &element;
            } else {
                p_right = &element;
                break;
            }
        }
        interpolated_value(p_left, p_right, x)
    }

    pub fn finalize(&mut self) {
        self.add(Point {
            x: MAX_TEMP,
            y: MAX_FAN_SPEED,
        });
    }
}

fn interpolated_value(p1: &Point, p2: &Point, x: f64) -> f64 {
    if p1.x == p2.x {
        panic!("divided by 0 is not cool.")
    };
    let m = (p2.y - p1.y) / (p2.x - p1.x);
    let t = p1.y - m * p1.x;
    m * x + t
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x: x.min(MAX_TEMP).max(MIN_TEMP),
            y: y.min(MAX_FAN_SPEED).max(MIN_FAN_SPEED),
        }
    }
}

pub fn readconfig() -> Config {
    let mut default_conf = Config::new();
    default_conf.finalize();
    let default_conf = default_conf;
    let Ok(xdg_base) = xdg::BaseDirectories::new() else {
        panic!("XDG Base Directory is configured wrong.")
    };
    let path = xdg_base.get_config_home().join("fan_control.config");

    let read_config = match read_to_string(&path) {
        Ok(filecontent) => deserialize_file(filecontent),
        Err(_) => create_default_file(default_conf, path),
    };

    let mut myvec = Vec::new();
    for element in read_config.points {
        myvec.push(Point::new(element.x, element.y));
    }

    let mut cf = Config::new();
    for element in myvec {
        cf.add(element);
    }
    cf.finalize();
    cf
}

fn create_default_file(default_conf: Config, path: PathBuf) -> Config {
    let filecontent = toml::to_string_pretty(&default_conf).unwrap(); //default config has to be serializable
    match write(&path, filecontent) {
        Ok(_) => println!("default config file was created. Look at {:?}", path),
        Err(e) => println!(
            "Using the default config an no write access to {:?}, error: {:?}.",
            path, e
        ),
    }
    default_conf
}

fn deserialize_file(filecontent: String) -> Config {
    match toml::from_str(&filecontent) {
        Ok(a) => a,
        Err(e) => panic!("file could not been parsed. {:?}", e),
    }
}

#[cfg(test)]
mod privatetests {
    use super::*;

    #[test]
    fn check_add() {
        let mut conf = Config::new();
        let p1 = Point::new(35.0, 0.2);
        let p2 = Point::new(37.0, 0.2);
        let p3 = Point::new(40.0, 0.1);
        let p4 = Point::new(30.0, 0.3);
    
        conf.add(p1);
        conf.add(p2);
        conf.add(p3);
        conf.add(p4);
        println!("{:?}", conf);
        let p = conf.points.pop().unwrap();
        assert_eq!(p.x, 40.0);
        assert_eq!(p.y, 0.2);
        let p = conf.points.pop().unwrap();
        assert_eq!(p.x, 37.0);
        assert_eq!(p.y, 0.2);
        let p = conf.points.pop().unwrap();
        assert_eq!(p.x, 35.0);
        assert_eq!(p.y, 0.2);
    }
    
    #[test]
    fn check_point_min_max() {
        let p1 = Point::new(MIN_TEMP - 0.1, 0.0);
        assert_eq!(p1.x, MIN_TEMP);
        let p1 = Point::new(MAX_TEMP + 0.1, 0.2);
        assert_eq!(p1.x, MAX_TEMP);
        let p1 = Point::new(100.0, MIN_FAN_SPEED - 0.1);
        assert_eq!(p1.y, MIN_FAN_SPEED);
        let p1 = Point::new(100.0, MAX_FAN_SPEED + 0.1);
        assert_eq!(p1.y, MAX_FAN_SPEED);
    }
}

