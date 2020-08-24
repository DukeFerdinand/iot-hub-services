FROM rustlang/rust:nightly
RUN rustup override set nightly

# install the watcher script we need for dev
RUN cargo install cargo-watch

WORKDIR /app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# Framework specific toml
COPY ./Rocket.toml ./Rocket.toml

# copy your source tree
COPY ./src ./src

# cache deps
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

# DEV ONLY COMMAND
CMD [ "cargo", "watch", "-x", "run" ]
