use std::fs::File;
use std::io::Read;
use serde_json::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub docker_api_version_major: usize,
    pub docker_api_version_minor: usize,
}

/// Read in config file
pub fn read_config() -> Result<Config, String> {
    match read_json("config.json") {
        Ok(config) => Ok(config),
        Err(e) => Err(format!("Error: {}", e))
    }
}

/// Read json file
fn read_json(file: &str) -> Result<Config, Error> {
    let mut file = File::open(&file).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let net: Config = serde_json::from_str(&data)?;
    return Ok(net);
}
