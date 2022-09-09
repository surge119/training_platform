import os
import shutil
import subprocess
import yaml


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
    """Build all docker containers under docker/

    Returns: None

    """
    # Copy challenges compose file to rust-server directorr
    tmp_compose = "./rust-server/docker-compose.yml.tp"
    shutil.copyfile(path, tmp_compose)

    compose_path = generate_docker_compose(path)
    docker_compose_create(compose_path)
    delete_compose_file(compose_path)
    delete_compose_file(tmp_compose)


if __name__ == "__main__":
    build_containers("./docker/docker-compose.yml.tp")
    docker_compose_up("./docker-compose.yml")

