_default:
  @just --choose

watch-dev:
  cargo watch --clear -x "run"

checks:
  #!/usr/bin/env bash
  set -x
  cargo check
  cargo check --tests
  cargo clippy --all-targets
  cargo fmt --all -- --check
