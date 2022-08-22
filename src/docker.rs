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
    pub async fn start_docker_container(&self, container_name: &str) {
        self.docker_daemon
            .start_container(container_name, None::<StartContainerOptions<String>>)
            .await
            .expect("container failed to start");
    }

    //was able to test by running docker run -t -d IMAGENAME and then running this function
    /// Stop a docker container
    pub async fn stop_docker_container(&self, container_name: &str) {
        self.docker_daemon
            .stop_container(container_name, None)
            .await
            .expect("container failed to stop");
    }

    /// Reset a docker container
    pub async fn reset_docker_container(docker: &Docker, container_name: &str) {
        let options = Some(RestartContainerOptions{
            t: 0,
        });

        docker
            .restart_container(container_name, options)
            .await
            .expect("container failed to reset");
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
