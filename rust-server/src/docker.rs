use bollard::container::{RestartContainerOptions, StartContainerOptions};
use bollard::ClientVersion;
use bollard::Docker;
use crate::config::{Config, read_config};

#[derive(Debug, Clone)]
pub struct DockerController {
    docker_daemon: Docker,
}

fn get_docker_api_version() -> ClientVersion {
    let config_info: Config = read_config().unwrap();

    ClientVersion {
        major_version: config_info.docker_api_version_major,
        minor_version: config_info.docker_api_version_minor,
    }
}

impl DockerController {
    pub fn new() -> DockerController {
        DockerController {
            docker_daemon: Docker::connect_with_http(
                "http://172.17.0.1:2375",
                4,
                &get_docker_api_version(),
            )
            .expect("Failed to connect to remote Docker daemon"),
        }
    }

    /// Start a docker container
    pub async fn start_docker_container(&self, container_name: &str) -> Result<(), String> {
        match self
            .docker_daemon
            .start_container(container_name, None::<StartContainerOptions<String>>)
            .await
        {
            Ok(_) => Ok(()),
            Err(_e) => Err(format!("Container '{}' failed to start", container_name)),
        }
    }

    /// Stop a docker container
    pub async fn stop_docker_container(&self, container_name: &str) -> Result<(), String> {
        match self
            .docker_daemon
            .stop_container(container_name, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(_e) => Err(format!("Container '{}' failed to stop", container_name)),
        }
    }

    /// Reset a docker container
    pub async fn reset_docker_container(&self, container_name: &str) -> Result<(), String> {
        let options = Some(RestartContainerOptions { t: 0 });

        match self
            .docker_daemon
            .restart_container(container_name, options)
            .await
        {
            Ok(_) => Ok(()),
            Err(_e) => Err(format!("Container '{}' failed to restart", container_name)),
        }
    }
}
