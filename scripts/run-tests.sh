#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail

docker compose up postgres --wait -d

# Run tests
cargo test
