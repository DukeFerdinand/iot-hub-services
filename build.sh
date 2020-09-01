#!/bin/bash

# Break if anything exits poorly
set -e

echo "Building Auth Server"
cd iot-auth-server

echo "Compiling for arm64"
cross build --target aarch64-unknown-linux-gnu --release
echo "Creating docker image and pushing"
docker buildx build -t dougflynn/iot-auth-server:latest --platform linux/arm64 -f Dockerfile.prod --push .

echo "DONE. Swapping back to root dir"
cd ..

echo "Building plant api"
cd iot-api-plants
echo "Compiling for arm64"
cross build --target aarch64-unknown-linux-gnu --release
echo "Creating docker image and pushing"
docker buildx build -t dougflynn/iot-api-plants:latest --platform linux/arm64 -f Dockerfile.prod --push .

echo "DONE. Swapping back to root dir"
cd ..

echo "Building MQTT handler"
cd iot-mqtt-hub
echo "Creating arm64 docker image and pushing"
docker buildx build -t dougflynn/iot-mqtt-hub:latest --platform linux/arm64 -f Dockerfile.prod --push .

echo "Skipping mosquitto image, already supports arm64"
# cd iot-mqtt-broker
# echo "Creating arm64 docker image and pushing"
# docker buildx build -t dougflynn/iot-mqtt-hub:latest --platform linux/arm64 -f Dockerfile.prod --push .

echo "DONE. Swapping back to root dir"
cd ..

echo "NOTHING LEFT TO DO :)"

echo "Run the following to push to prod/swarm:"
echo "git push swarm master"

