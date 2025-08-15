from Docker.Serializer import Serializer as SZR
from Docker.DockerMan import DockerContainer, DockerMan

class DockerSerializer:
    def __init__(self):
        self.DM = DockerMan()

    def list_containers(self):
        JE_LIST = self.DM.list_JE_containers()
        t = DockerSerializer.jsonify_docker_list(JE_LIST)
        return t

    @staticmethod
    def jsonify_docker_list(containers: list[DockerContainer] ):
        return SZR.jsonify(containers)

    def __del__(self):
        try:
            self.DM.client.close()
        except Exception as e:
            print(f"Error closing Docker Client, {e}")
