import json
from typing import Any

from Docker.DockerMan import DockerContainer, Aux

class Serializer:
    @staticmethod
    def serialize(obj: Any) -> dict:
        return obj.__dict__

    @staticmethod
    def jsonify(obj: Any):
        return json.dumps(obj)

    @staticmethod
    def jsonify_docker_list(containers: list[DockerContainer] ):
        info_list = list(map(Aux.just_enougth, containers))
        return json.dumps(info_list)
