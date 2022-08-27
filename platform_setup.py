import os
from pathlib import Path
import subprocess
import yaml


def collect_docker_compose(path: str) -> list:
    """Get all the docker compose files

    Args:
        path: The path to the docker source directory

    Returns: A list of the paths to the docker compose files

    """
    path_list = list()

    # Read in docker compose files with extra fields (*.yml.tp)
    for path in Path(path).rglob("*.yml.tp"):
        path_list.append(path)

    return path_list


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
    """Take docker-compose.yml.tp and create propper docker compose file without extra fields

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


def create_containers(path: str) -> None:
    """Run `docker compose create` on all docker compose files

    Args:
        path: The path to the docker compose file
    
    Returns: None

    """
    subprocess.run(["docker", "compose", "-f", path, "create"])


def delete_compose_file(path: str) -> None:
    """Delete the generated docker-compose.yml file

    Args:
        path: The path to the docker compose file

    Returns: None

    """
    os.remove(path)


def build_containers() -> None:
    """Build all docker containers under docker/

    Returns: None

    """
    # Get current containers
    docker_path = "docker/"
    path_list = collect_docker_compose(docker_path)

    for path in path_list:
        compose_path = generate_docker_compose(path)
        create_containers(compose_path)
        delete_compose_file(compose_path)


if __name__ == "__main__":
    build_containers()
