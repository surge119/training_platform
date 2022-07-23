use bollard::container::RemoveContainerOptions;
use bollard::container::StartContainerOptions;
use bollard::container::RestartContainerOptions;
use bollard::container::{Config, CreateContainerOptions};
use bollard::models::IpamConfig;
use bollard::models::{EndpointIpamConfig, EndpointSettings, Ipam};
use bollard::network::ConnectNetworkOptions;
use bollard::network::CreateNetworkOptions;
use bollard::network::DisconnectNetworkOptions;
use bollard::Docker;

use std::default::Default;

/// Create a new docker container
pub async fn create_docker_container(docker: &Docker, image_name: &str, container_name: &str) {
    let options = Some(CreateContainerOptions {
        name: container_name,
    });

    let config = Config {
        image: Some(image_name),
        ..Default::default()
    };

    docker
        .create_container(options, config)
        .await
        .expect("container failed to be created");
}

/// Remove an existing docker container
pub async fn remove_docker_container(docker: &Docker, container_name: &str) {
    let options = Some(RemoveContainerOptions {
        force: true,
        ..Default::default()
    });

    docker
        .remove_container(container_name, options)
        .await
        .expect("container failed to be removed");
}

pub async fn start_docker_container(docker: &Docker, container_name: &str) {
    docker
        .start_container(container_name, None::<StartContainerOptions<String>>)
        .await
        .expect("container failed to start");
}
//was able to test by running docker run -t -d IMAGENAME and then running this function
pub async fn stop_docker_container(docker: &Docker, container_name: &str) {
    docker
        .stop_container(container_name, None)
        .await
        .expect("container failed to stop");
}

pub async fn reset_docker_container(docker: &Docker, container_name: &str) {
    let options = Some(RestartContainerOptions{
        t: 0,
    });
    
    docker
        .restart_container(container_name, options)
        .await
        .expect("container failed to reset");
}

pub async fn start_docker_network(
    docker: &Docker,
    network_name: &str,
    subnet: String,
    is_attachable: bool,
) {
    let ipam = IpamConfig {
        subnet: Some(subnet),
        ..Default::default()
    };
    let ipam_configs = vec![ipam];
    let config = CreateNetworkOptions {
        name: network_name,
        ipam: Ipam {
            config: Some(ipam_configs),
            ..Default::default()
        },
        attachable: is_attachable,
        ..Default::default()
    };

    docker
        .create_network(config)
        .await
        .expect("network failed to start");
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
