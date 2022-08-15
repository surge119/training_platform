import docker
from pathlib import Path


def collect_dockerfile(path: str) -> list:
    """Get all the dockerfiles that need to be built

    Args:
        path: The path to the docker source directory

    Returns: A list of the paths to dockerfiles

    """
    path_list = list()

    for path in Path(path).rglob("Dockerfile"):
        path_list.append(path.parent)

    return path_list


def build_containers() -> None:
    """Build all docker containers under docker/

    Returns: None

    """
    # Get current containers
    docker_path = "docker/"
    path_list = collect_dockerfile(docker_path)

    # Get docker daemon
    docker_client = docker.from_env()

    for path in path_list:
        docker_client.images.build(path=str(path), tag=path.name)


if __name__ == "__main__":
    build_containers()
