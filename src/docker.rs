use bollard::Docker;
use bollard::container::{CreateContainerOptions, Config};
use bollard::container::StartContainerOptions;
use bollard::container::RemoveContainerOptions;
use bollard::network::CreateNetworkOptions;
use bollard::network::ConnectNetworkOptions;
use bollard::network::DisconnectNetworkOptions;
use bollard::models::{EndpointSettings, EndpointIpamConfig, Ipam};

use std::default::Default;
use std::collections::HashMap;

/// Create a new docker container
pub async fn create_docker_container(docker: &Docker, image_name: &str, container_name: &str){
    let options = Some(CreateContainerOptions{
        name: container_name,
    });
    
    let config = Config {
        image: Some(image_name),
        ..Default::default()
    };
    
    docker.create_container(options, config).await.expect("container failed to be created");
}

/// Remove an existing docker container
pub async fn remove_docker_container(docker: &Docker, container_name: &str){
    let options = Some(RemoveContainerOptions{
        force: true,
        ..Default::default()
    });

    docker.remove_container(container_name, options).await.expect("container failed to be removed");
}

pub async fn start_docker_container(docker: &Docker, container_name: &str){
    docker.start_container(container_name, None::<StartContainerOptions<String>>).await.expect("container failed to start");
}

pub async fn stop_docker_container(docker: &Docker, container_name: &str){
    docker.stop_container(container_name, None).await.expect("container failed to stop");
}
//
// pub async fn start_docker_network(docker: Docker, network_name:&str, subnet:String, is_attachable:bool){
//     let mut subnet_config = HashMap::new();
//     let subnet_string = "Subnet".to_string();
//     subnet_config.insert(subnet_string, subnet);
//     let ipam_configs = vec![subnet_config];
//
//     let config = CreateNetworkOptions {
//         name: network_name,
//         ipam: Ipam{
//             config: Some(ipam_configs),
//             ..Default::default()
//         },
//         attachable: is_attachable,
//         ..Default::default()
//     };
//
//     docker.create_network(config).await.expect("network failed to start");
// }
//
// pub async fn connect_docker_network(network_name:&str, docker:Docker, container_name:&str, ip_address:&str){
//     let config = ConnectNetworkOptions {
//         container: container_name,
//         endpoint_config: EndpointSettings {
//             ipam_config: Some(EndpointIpamConfig {
//                 ipv4_address: Some(String::from(ip_address)),
//                 ..Default::default()
//             }),
//             ..Default::default()
//         }
//     };
//
//     docker.connect_network(network_name, config).await.expect("failed to connect to network");
// }
//
// pub async fn disconnect_docker_network(docker:Docker, network_name:&str, container_name: &str){
//     let config = DisconnectNetworkOptions {
//         container: container_name,
//         force: true
//     };
//
//     docker.disconnect_network(network_name, config).await.expect("failed to disconnnect from network");
// }
