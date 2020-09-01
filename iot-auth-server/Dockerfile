FROM rustlang/rust:nightly-stretch-slim

RUN apt-get update && \
  apt-get install libssl-dev pkg-config lsb-release -y && \
  apt-get autoremove

# install the watcher script we need for dev
RUN cargo install cargo-watch

WORKDIR /app

# should only be exposed to internal network. prelim tests seemed to confirm
EXPOSE 8001

# DEV ONLY COMMAND
CMD [ "cargo", "watch", "-x", "run" ]
