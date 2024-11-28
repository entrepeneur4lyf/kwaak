# Always build against latest stable
ARG RUST_VERSION=1.82-slim
ARG RUST_PROFILE=dev
ARG RUST_TARGET=debug
FROM rust:${RUST_VERSION} as builder
# ARG APP_NAME
# ARG RUST_PROFILE
# ARG RUST_TARGET


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




# Add github.com to known hosts
# RUN mkdir /root/.ssh/; ssh-keyscan github.com >> /root/.ssh/known_hosts \
#   && git config --global user.name "Fluyt" \
#   && git config --global user.email "fluyt@bosun.ai"
#
#
# RUN --mount=type=bind,source=crates,target=crates \
#   --mount=type=bind,source=tests,target=tests \
#   --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
#   --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
#   --mount=type=cache,target=/app/target/ \
#   --mount=type=cache,target=/usr/local/cargo/registry/ \
#   cargo build --profile ${RUST_PROFILE} && \
#   cp ./target/$RUST_TARGET/fluyt-server /bin/app && \
#   cp ./target/$RUST_TARGET/fluyt-cli /bin/fluyt-cli
#
# ARG PORT=8080
# # Expose the port that the application listens on.
# EXPOSE $PORT
# CMD ["/bin/app"]
