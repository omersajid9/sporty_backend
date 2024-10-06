#!/bin/bash

# Stop all running containers
docker stop $(docker ps -aq)

# Remove all containers
docker rm $(docker ps -aq)

# Remove all volumes
docker volume rm $(docker volume ls -q)

# Remove all images
docker rmi $(docker images -q) -f

echo "All Docker containers, volumes, and images have been removed."