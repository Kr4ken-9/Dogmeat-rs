use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize)] // Allows serialization/deserialization from serde
pub struct Config {
    pub token: String
}

pub fn load_config(path: String) -> Config {
    let mut f = File::open(path) // Open config
        .expect("config not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents) // Read config
        .expect("error occurred while reading file");

    let config: Config = toml::from_str(&contents).unwrap(); // Deserialize toml
    return config;
}

pub fn save_config(path: String, _config: Config) {
    let mut _f = File::create(path).expect("error creating config");

    let contents = toml::to_string(&_config).unwrap(); // Serialize toml
    _f.write_all((&contents).as_bytes()) // Write toml
        .expect("error writing config");
}

pub fn check_config(path: &str) -> bool {
    return Path::new(path).exists(); // Return if the given path exists
}
