import docker
from docker.models.containers import Container as DockerContainer
class Aux:
    @staticmethod
    def to_dict(data: DockerContainer) -> dict:
        return data.attrs

    @staticmethod
    def handle_docker_image(container: DockerContainer) -> list:
        img = container.image
        return img.tags if img else []

    @staticmethod
    def just_enougth(data: DockerContainer) -> dict:
        return {
            "id": data.id,
            "name": data.name,
            "status": data.status,
            "image": Aux.handle_docker_image(data)[0],
        }

    @staticmethod
    def JE_HEADERS() -> list:
        return [ "name", "id", "image", "status" ]

    @staticmethod
    def to_string(data: DockerContainer) -> str:
        return f"""
                name: \033[032m {data.name} \033[0m
                id: \033[035m {data.id} \033[0m
                status: {data.status}
                image: {Aux.handle_docker_image(data)[0]}
                """


class DockerMan:
    def __init__(self):
        self.client = docker.DockerClient(base_url='unix://var/run/docker.sock', version='1.35')

    def list_containers(self) -> list:
        try:
            containers = self.client.containers.list(all=True)
            return containers
        except Exception as e:
            print(f"Error listing containers: {e}")
            return []

    def get_container(self, container_id: str) -> DockerContainer:
        try:
            container = self.client.containers.get(container_id)
            return container
        except Exception as e:
            print(f"Error getting container {container_id}: {e}")
            raise e
