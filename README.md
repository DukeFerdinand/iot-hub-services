# IOT Hub Services and DB

None of this is going to be publicly accessible, but I'm adding it here both for my use and for inspiration in case anyone else wants to follow what I'm doing :)

## General setup
1. Make sure you've got `docker` and `docker-compose`.
2. If you don't have tool chains (for IDE features) for any of these services installed, you can either:
    * Do what I do and use VS Code's remote ssh extension to remote into a central PC that has all the toolchains -or- remote into the docker containers with the same extension
    * Install the toolchains locally (I'm not going to add documentation)

## Connectivity
The idea here is to do some development locally with volume mounts and the target language's equivalent of `cargo-watch` or `nodemon` to rebuild code. Containers are then exposed locally and (TODO: in production) through just ONE "public" image - the `proxy` image containing the stack's nginx config.

For example, let's say you want to get the data pulled from the plant bluetooth monitors:

```
On timer
--------------------------------
Data created ->
  Monitors publish to proxy container, via mqtt -> route to mqtt broker

Broker sends data to IOT Plant Hub ->
  looks something like "POST <container-ip>/mqtt BODY: {...plant_data}

Proxy image ->
  sends data to service, which accepts + stores in DB or sends error

LATER
--------------------------------
Web client ->
  Netlify/Cordova app sends GET to service route (<container_ip>/service-route?params)

Proxy image ->
  routes request to target service where request is handled

ERRORS
--------------------------------
TODO
Ultimately all errors from all containers will be standardized and stashed in
another DB with some sort of alert/visualizer

```

## Running the stack
TL;DR: `docker-compose up`

~ More info coming soon ~