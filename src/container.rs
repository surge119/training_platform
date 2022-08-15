use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::{DirEntry, File, ReadDir};
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json;

use crate::docker;
use crate::docker::DockerController;

/// Struct that represents a docker network - read in from json file
#[derive(Deserialize, Debug)]
struct Network {
    labs: HashMap<String, Machine>,
    subnet: String,
}

/// Struct that represents a docker machine - used by Network
#[derive(Deserialize, Debug)]
struct Machine {
    container_name: String,
    ip: String,
    path: String,
    description: String,
}

/// Test network for now
pub async fn init_containers() {
    let network: Network =
        json_to_network("docker/pentesting/lin_net/linux_network.json").unwrap();
    let docker_controller: DockerController = DockerController::new();
}

/// Convert the json file provided into a Network struct
fn json_to_network(file: &str) -> Result<Network, serde_json::Error> {
    let mut file: File = File::open(&file).unwrap();
    let mut data: String = String::new();

    file.read_to_string(&mut data).unwrap();

    let network: Network = serde_json::from_str(&data)?;
    return Ok(network)
}
