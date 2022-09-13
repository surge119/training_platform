import argparse
import os
import shutil
import subprocess
import yaml

from setup import rust_server


def remove_custom_properties(yaml_dict: dict) -> dict:
    """Remove the custom properties that were added to the docker-compose.yml file

    Args:
            yaml_dict: A dict of the docker-compose.yml file

    Returns: An updated dict without the custom properties

    """
    # Loop to find 'services
    for key in yaml_dict:
        if key == "services":
            # Loop to find each service
            for service_key in yaml_dict[key]:
                # Remove properties from service
                yaml_dict[key][service_key].pop("description", None)

    return yaml_dict


def generate_docker_compose(path: str) -> str:
    """Take docker-compose.yml.tp and create proper docker compose file without extra fields

    Args:
            path: The path to the docker compose file

    Returns: The path of the new file created

    """
    with open(path, "r") as tp_file:
        yaml_dict = yaml.safe_load(tp_file)
        yaml_dict = remove_custom_properties(yaml_dict)

        compose_path = str(path)[:-3]

        with open(compose_path, "w") as compose_file:
            yaml.safe_dump(yaml_dict, compose_file)

        return compose_path


def docker_compose_create(path: str) -> None:
    """Run `docker compose create`

    Args:
            path: The path to the docker compose file

    Returns: None

    """
    exec_docker_compose(path, "create")


def docker_compose_up(path: str) -> None:
    """Run `docker compose up`

    Args:
            path: The path to the docker compose file

    Returns: None

    """
    exec_docker_compose(path, "up")


def exec_docker_compose(path: str, cmd: str) -> None:
    """Run `docker compose -f {path} {cmd}`

    Args:
            path: The path to the docker compose file
            cmd: The command to run

    Returns: None

    """
    subprocess.run(["docker", "compose", "-f", path, cmd])


def delete_compose_file(path: str) -> None:
    """Delete the generated docker-compose.yml file

    Args:
            path: The path to the docker compose file

    Returns: None

    """
    os.remove(path)


def build_containers(path: str) -> None:
    """Build all docker containers that are ctf challenges

    Args:
            path: The path to the docker compose file

    Returns: None

    """
    compose_path = generate_docker_compose(path)
    docker_compose_create(compose_path)
    delete_compose_file(compose_path)


def build_training_platform(path: str, challenges_path: str) -> None:
    """Build the training platform

    Args:
            path: The path to the training platform docker compose file
            challenges_path: The path to the challenges' docker compose file

    Returns: None

    """
    rust_server.setup_rust_server(challenges_path)
    docker_compose_up(path)


def setup_args() -> argparse.Namespace:
    """Setup and parse the cmdline arguments

        Returns: The arguments parsed

    """
    parser = argparse.ArgumentParser(
        description="Training Platform Setup Script")

    parser.add_argument("target", action="store", choices=[
        "all", "ctfd", "rust-server", "challenges", "rust-server-config"],
        help="Specify which docker containers should be set up")

    parser.add_argument("-cf", "--challenge-file", action="store",
                        required=False,
                        default="./docker/docker-compose.yml.tp",
                        type=str,
                        help="Specify where to find the `docker-compose.yml` "
                             "file to build. (Default: ./docker/docker-compose"
                             ".yml.tp")

    return parser.parse_args()


if __name__ == "__main__":
    challenges_path = "./docker/docker-compose.yml.tp"
    platform_path = "./docker-compose.yml"
    args = setup_args()
    if args.target == "all":
        build_containers(challenges_path)
        build_training_platform(platform_path, args.challenge_file)
    elif args.target == "rust-server":
        rust_server.setup_rust_server(args.challenge_file)
    elif args.target == "challenges":
        build_containers(challenges_path)
    elif args.target == "rust-server-config":
        rust_server.setup_rust_server(challenges_path)
