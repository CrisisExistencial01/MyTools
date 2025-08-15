#!/usr/bin/env python3

from Docker.DockerSerializer import DockerSerializer
import sys

class OptionHandler:
    def __init__(self, args: list[str]):
        self.args = list(args) or []
        self.dockerman = DockerSerializer()
        self.options = {
            "--list" : self.get_containers,
        }

        print(self.standard_response())

    def standard_response(self):
        data = self.match_option()
        return {
            "status" : "success" if data else "failure",
            "data" : data
        }

    def exec(self, func):
        return func(self.args[1:])

    def get_containers(self, args):
        all_containers = self.dockerman.list_containers()
        return all_containers

    def match_option(self):
        match = self.options[self.args[1]] if len(self.args) > 1 else None
        if match:
            return self.exec(match)
        elif len(self.args) < 2:
            return "Error: Arguments not provided"
        else:
            return "Error: Option not found o.O"

if __name__ == "__main__":
    handler = OptionHandler(sys.argv)
