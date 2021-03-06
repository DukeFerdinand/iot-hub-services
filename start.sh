#!/bin/bash

set -e
echo "Starting services --------"

# Make sure your images are uploaded before this
echo "Deploying to swarm"
/usr/bin/docker stack deploy --with-registry-auth --compose-file docker-compose-prod.yml iot-hub-stack

echo "Done! Please wait a bit for changes to take effect throughout swarm."