use std::thread;
use std::time::Duration;
use bollard::Docker;

mod docker;

#[tokio::main]
async fn main() {
    let docker_container = Docker::connect_with_local_defaults().unwrap();
    let image: &str = "ubuntu";
    let network_name: &str = "test_network";
    let container_name: &str = "test_container";
    let subnet: String = "192.168.0.0/16".to_string();
    //docker::create_docker_container(&docker_container, image, container_name).await;
    //docker::start_docker_container(&docker_container, container_name).await;
    //docker::stop_docker_container(&docker_container, container_name).await;
    //docker::start_docker_network(&docker_container, network_name, subnet, true).await;
    //docker::connect_docker_network(&docker_container, network_name, container_name, "192.168.0.2").await;
    //docker::disconnect_docker_network(&docker_container, network_name, container_name).await;
}
