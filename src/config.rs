use std::collections::HashMap;
use std::env;
use serde_yaml;

pub struct Config {
    pub map: HashMap<String, serde_yaml::Value>,
}

impl Config {
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        let default_config_path = String::from("./config.yml");
        let file_path = match &args.len() {
            2 => &args[args.len() - 1],
            _ => &default_config_path,
        };
        let file = std::fs::File::open(file_path).unwrap();
        let map: HashMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();
        Config{map}
    }
}
