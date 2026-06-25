#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_minimal_execution_path_smoke.py --check
cargo test --release -q --test mirbuilder_minimal_execution_path_smoke \
  mirbuilder_minimal_literal_integer_path_smoke -- --exact
