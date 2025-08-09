# docker-utils 🐋
A collection of Docker-related utilities and scripts to streamline your container management tasks.

## Features
- List all containers
- Remove dangling images

## Installation
### Clone the repository
You can clone the repository using HTTPS:
```bash
git clone -b tool/docker-utils https://github.com/CrisisExistencial01/MyTools
```
or ssh:
```bash
git clone -b tool/docker-utils git@github.com:CrisisExistencial01/MyTools.git
```
### Compile the script
After cloning the repository, you need to compile the script. First, ensure you have `make` installed on your system. Then, navigate to the `docker-utils` directory and run the `make` command to compile the script.

```bash
cd MyTools/docker-utils
make
```
The script will be compiled and placed in the `bin` directory.

## Usage
### List your containers

This command lists all Docker containers, showing their names and IDs. If any container is detected as malformed, it will be flagged and automatically removed to keep your environment clean.

```bash
./bin/docker-utils
```
