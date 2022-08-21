use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::{DirEntry, File, ReadDir};
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::{Serialize,Deserialize};
use serde_json;

use crate::docker;
use crate::docker::DockerController;

/// Struct for all of the containers and the docker daemon
#[derive(Debug,Clone)]
pub struct Containers {
    pub docker_controller: DockerController,
    pub networks: HashMap<String, Network>,
}

/// Struct that represents a docker network - read in from json file
#[derive(Deserialize, Debug,Clone,Serialize)]
pub struct Network {
    pub labs: HashMap<String, Machine>,
    subnet: String,
}

/// Struct that represents a docker machine - used by Network
#[derive(Deserialize, Debug,Clone,Serialize)]
pub struct Machine {
    container_name: String,
    ip: String,
    path: String,
    description: String,
}

/// Test network for now
pub fn init_containers() -> Containers {
    let docker_controller: DockerController = DockerController::new();

    let network: Network =
        json_to_network("docker/pentesting/lin_net/linux_network.json").unwrap();

    let mut networks: HashMap<String, Network> = HashMap::new();
    networks.insert("lin_net".to_owned(), network);

    Containers {
        docker_controller,
        networks
    }
}

/// Convert the json file provided into a Network struct
fn json_to_network(file: &str) -> Result<Network, serde_json::Error> {
    let mut file: File = File::open(&file).unwrap();
    let mut data: String = String::new();

    file.read_to_string(&mut data).unwrap();

    let network: Network = serde_json::from_str(&data)?;
    return Ok(network)
}
