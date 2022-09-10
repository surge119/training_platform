use std::borrow::Cow;
use std::process::Command;

use bollard::container::{RestartContainerOptions, StartContainerOptions};
use bollard::ClientVersion;
use bollard::Docker;

#[derive(Debug, Clone)]
pub struct DockerController {
    docker_daemon: Docker,
}

/// Get the version of Docker Engine API for connecting to remote Docker Daemon
pub fn get_docker_api_version() -> ClientVersion {
    let output: Vec<u8> = Command::new("sh")
        .arg("docker")
        .arg("version")
        .output()
        .expect("docker version failed to execute")
        .stdout;

    // Find the index of where the Docker Engine API version is listed
    let api_index: usize = String::from_utf8(output.clone())
        .expect("Invalid UTF8")
        .find("API version")
        .unwrap();

    // Get minor version first, since major requires move of `output`
    let minor_version_str: Cow<str> =
        String::from_utf8_lossy(&output[api_index + 21..api_index + 21 + 2]);
    let minor_version: usize = String::from(minor_version_str).parse::<usize>().unwrap();

    // Get major version
    let major_version_char: char = output[api_index + 19] as char;
    let major_version: usize = major_version_char.to_digit(10).unwrap() as usize;

    ClientVersion {
        major_version,
        minor_version,
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
