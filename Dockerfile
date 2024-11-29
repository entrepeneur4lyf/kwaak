# Always build against latest stable
ARG RUST_VERSION=1.83-slim
ARG RUST_PROFILE=dev
ARG RUST_TARGET=debug
FROM rust:${RUST_VERSION} as builder


# Install tool dependencies for app and git/ssh for the workspace
RUN apt-get update && apt-get install -y --no-install-recommends \
  ripgrep fd-find git ssh curl  \
  protobuf-compiler \
  pkg-config libssl-dev iputils-ping \
  && rm -rf /var/lib/apt/lists/* \
  && cp /usr/bin/fdfind /usr/bin/fd

RUN cargo install cargo-tarpaulin
COPY . /app

WORKDIR /app
