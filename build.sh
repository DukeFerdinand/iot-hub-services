#!/bin/bash

AUTH_SERVER="dougflynn/iot-auth-server:latest"
IOT_PLANTS="dougflynn/iot-api-plants:latest"
IOT_MQTT="dougflynn/iot-mqtt-hub:latest"

# Break if anything exits poorly
set -e

echo "Building Auth Server"
cd iot-auth-server

echo "Compiling for x86_64"
cargo build --release
echo "Creating docker image and pushing"
docker build -t $AUTH_SERVER -f Dockerfile.prod .
docker push $AUTH_SERVER

echo "DONE. Swapping back to root dir"
cd ..

echo "Building plant api"
cd iot-api-plants
echo "Compiling for arm64"
cargo build --release
echo "Creating docker image and pushing"
docker build -t $IOT_PLANTS -f Dockerfile.prod .
docker push $IOT_PLANTS

echo "DONE. Swapping back to root dir"
cd ..

echo "Building MQTT handler"
cd iot-mqtt-hub
echo "Creating docker image and pushing"
docker build -t $IOT_MQTT -f Dockerfile.prod .
docker push $IOT_MQTT

# cd iot-mqtt-broker
# echo "Creating arm64 docker image and pushing"
# docker buildx build -t dougflynn/iot-mqtt-hub:latest --platform linux/arm64 -f Dockerfile.prod --push .

echo "DONE. Swapping back to root dir"
cd ..

echo "NOTHING LEFT TO DO :)"

echo "Run the following to push to prod/swarm:"
echo "git push <target> master"

