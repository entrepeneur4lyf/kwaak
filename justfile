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
