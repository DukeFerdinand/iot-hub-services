#!/bin/bash

echo "Starting services --------"
# Deploy to swarm registry
echo "Pushing production services to swarm registry..."
/usr/bin/docker-compose -f docker-compose-prod.yml push

echo "Deploying to swarm"
/usr/bin/docker stack deploy --compose-file docker-compose-prod.yml iot-hub-stack

echo "Done!"