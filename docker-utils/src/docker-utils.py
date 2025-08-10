from UI.ui import DockerListUI
from Docker.DockerMan import DockerMan, Aux

def main():
    dockerman = DockerMan()
    containers = dockerman.list_containers()
    if not containers:
        print("No containers found.")
        return

    data = [
        Aux.just_enougth(container)
        for container in containers
    ]

    headers = Aux.JE_HEADERS()
    ui = DockerListUI(data, headers)
    ui.start()
    dockerman.client.close()

if __name__ == "__main__":
    main()
