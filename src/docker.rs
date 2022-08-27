use std::default::Default;
use std::fs::File;
use std::io::Read;
use std::path;

use actix_web::dev::ResourcePath;
use actix_web::web::Path;
use bollard::container::{Config, CreateContainerOptions, RestartContainerOptions};
use bollard::container::RemoveContainerOptions;
use bollard::container::StartContainerOptions;
use bollard::Docker;
use bollard::image::BuildImageOptions;
use bollard::models::{EndpointIpamConfig, EndpointSettings, Ipam};
use bollard::models::IpamConfig;
use bollard::network::ConnectNetworkOptions;
use bollard::network::CreateNetworkOptions;
use bollard::network::DisconnectNetworkOptions;

#[derive(Debug, Clone)]
pub struct DockerController {
    docker_daemon: Docker,
}

impl DockerController {
    pub fn new() -> DockerController {
        DockerController {
            docker_daemon: Docker::connect_with_socket_defaults().expect("Docker daemon failed to connect. Check if Docker is running")
        }
    }

    /// Start a docker container
    pub async fn start_docker_container(&self, container_name: &str) -> Result<(), String> {
        match self.docker_daemon
            .start_container(container_name, None::<StartContainerOptions<String>>)
            .await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Container '{}' failed to start", container_name))
        }
    }

    /// Stop a docker container
    pub async fn stop_docker_container(&self, container_name: &str) -> Result<(), String> {
        match self.docker_daemon
            .stop_container(container_name, None)
            .await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Container '{}' failed to stop", container_name))
        }
    }

    /// Reset a docker container
    pub async fn reset_docker_container(&self, container_name: &str) -> Result<(), String> {
        let options = Some(RestartContainerOptions{
            t: 0,
        });

        match self.docker_daemon
            .restart_container(container_name, options)
            .await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Container '{}' failed to restart", container_name))
        }
    }
}

//docker container must be running for this to work
pub async fn connect_docker_network(
    docker: &Docker,
    network_name: &str,
    container_name: &str,
    ip_address: &str,
) {
    let config = ConnectNetworkOptions {
        container: container_name,
        endpoint_config: EndpointSettings {
            ipam_config: Some(EndpointIpamConfig {
                ipv4_address: Some(String::from(ip_address)),
                ..Default::default()
            }),
            ..Default::default()
        },
    };

    docker
        .connect_network(network_name, config)
        .await
        .expect("failed to connect to network");
}

pub async fn disconnect_docker_network(docker: &Docker, network_name: &str, container_name: &str) {
    let config = DisconnectNetworkOptions {
        container: container_name,
        force: true,
    };

    docker
        .disconnect_network(network_name, config)
        .await
        .expect("failed to disconnect from network");
}
