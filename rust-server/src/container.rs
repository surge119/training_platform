use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::docker::DockerController;

/// Struct for all of the containers and the docker daemon
#[derive(Debug, Clone)]
pub struct Containers {
    pub docker_controller: DockerController,
    pub networks: HashMap<String, Network>,
}

/// Struct that represents a docker network - read in from docker-compose file
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Network {
    version: String,
    // Key is the name of the container, value is the info of the container
    pub services: HashMap<String, Machine>,
    networks: NetworkInfo,
}

/// Struct that represents a docker machine - used by Network
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Machine {
    build: HashMap<String, String>,
    networks: HashMap<String, HashMap<String, String>>,
    description: String,
}

/// Struct describing the networking info of a docker-compose
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct NetworkInfo {
    #[serde(flatten)]
    network: HashMap<String, NetworkConfig>,
}

/// Struct for the configuration of the networking of a docker-compose
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct NetworkConfig {
    driver: String,
    ipam: HashMap<String, Vec<HashMap<String, String>>>,
}

/// Test network for now
pub fn init_containers() -> Containers {
    let docker_controller: DockerController = DockerController::new();

    let network: Network = yaml_to_network("docker-compose.yml.tp").unwrap();

    let mut networks: HashMap<String, Network> = HashMap::new();
    networks.insert("Challenges".to_owned(), network);

    Containers {
        docker_controller,
        networks,
    }
}

/// Load the yaml files to a struct
fn yaml_to_network(file: &str) -> Result<Network, serde_yaml::Error> {
    let mut file: File = File::open(&file).unwrap();
    let mut data: String = String::new();

    file.read_to_string(&mut data).unwrap();

    let network: Network = serde_yaml::from_str(&data).unwrap();
    return Ok(network);
}
