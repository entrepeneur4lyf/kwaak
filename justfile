test:
  RUST_LOG=swiftide=debug RUST_BACKTRACE=1 cargo nextest run --all-features --all-targets

lint:
  cargo clippy --all-features -- -D warnings
  cargo fmt --all -- --check
  typos

lint_fix:
  cargo fmt --all
  cargo fix --all-features --allow-dirty --allow-staged
  typos -w

docker-build:
  docker build -t kwaak .

# Mac and Linux have slightly different behaviour when it comes to git/docker/filesystems.
# This ensures a fast feedback loop on macs.
test-in-docker TEST="": docker-build
  docker volume create kwaak-target-cache
  docker volume create kwaak-cargo-cache
  docker run --rm -it \
      -v /var/run/docker.sock:/var/run/docker.sock \
      -v "$(pwd)":/usr/src/myapp \
      -v kwaak-target-cache:/usr/src/myapp/target \
      -v kwaak-cargo-cache:/usr/local/cargo \
      -w /usr/src/myapp \
      -e RUST_LOG=debug \
      -e RUST_BACKTRACE=1 \
      kwaak \
      bash -c "cargo nextest run --no-fail-fast {{TEST}}"

build-in-docker PROFILE="release": docker-build
  docker volume create kwaak-target-cache
  docker volume create kwaak-cargo-cache
  docker run --rm -it \
      -v /var/run/docker.sock:/var/run/docker.sock \
      -v "$(pwd)":/usr/src/myapp \
      -v kwaak-target-cache:/usr/src/myapp/target \
      -v kwaak-cargo-cache:/usr/local/cargo \
      -w /usr/src/myapp \
      -e RUST_LOG=debug \
      -e RUST_BACKTRACE=1 \
      kwaak \
      bash -c "cargo build --profile {{PROFILE}}"

# Runs all or a single instance of a swe benchmark. Only runs benchmarks that are not in the results directory
[group('benchmarks')]
[working-directory: 'benchmarks/swe']
benchmark-swe INSTANCE="":
  uv run kwaak-bench-swe {{ if INSTANCE != "" {"--instance " + INSTANCE } else { ""} }}

# Cleans up all failed benchmarks
[group('benchmarks')]
[working-directory: 'benchmarks/swe']
benchmark-swe-remove-results:
  uv run kwaak-bench-swe --remove-failed
