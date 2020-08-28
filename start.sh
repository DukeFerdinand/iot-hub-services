#!/bin/bash

echo "Starting services --------"

echo "Removing old stack"
/usr/bin/docker stack rm iot-hub-stack

# Make sure your images are uploaded before this
echo "Deploying to swarm"
/usr/bin/docker stack deploy --with-registry-auth --compose-file docker-compose-prod.yml iot-hub-stack

echo "Done! Please wait a bit for changes to take effect throughout swarm."