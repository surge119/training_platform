use std::thread;
use std::time::Duration;
use bollard::Docker;

mod docker;

#[tokio::main]
async fn main() {
    let docker_container = Docker::connect_with_local_defaults().unwrap();
    let image: &str = "ubuntu";
    let name: &str = "test_bollard";
    docker::create_docker_container(&docker_container, image, name).await;
    docker::start_docker_container(&docker_container, name).await;
    // docker::remove_docker_container(&docker_container, name).await;

}
