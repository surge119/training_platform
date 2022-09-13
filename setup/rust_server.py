import docker
import json
import shutil


def setup_rust_server(challenges_path: str):
    """Build the rust server and it's config

    Args:
        challenges_path: The path to the docker compose for the challenges

    Returns: None

    """
    # Generate configuration file
    config_file = "rust-server/config.json"
    generate_config(config_file)

    # Copy challenges compose file to rust-server directory
    tmp_compose = "./rust-server/docker-compose.yml.tp"
    shutil.copyfile(challenges_path, tmp_compose)


def generate_config(config_file: str):
    """Generate a config file for the rust server

    Args:
        config_file: The config file path

    Returns: None

    """
    client = docker.from_env()
    api_version = client.version()["Components"][0]["Details"]["ApiVersion"]
    config = {
        "docker_api_version_major": int(api_version[0]),
        "docker_api_version_minor": int(api_version[2:]),
    }
    with open(config_file, "w") as server_config:
        json.dump(config, server_config)
