#!/usr/bin/env bash

# delete migrations
sqlx migrate revert;

# Stop all running containers
docker stop $(docker ps -aq)