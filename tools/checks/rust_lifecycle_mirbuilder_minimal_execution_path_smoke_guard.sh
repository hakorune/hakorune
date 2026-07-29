#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 tools/rust_lifecycle/mirbuilder_minimal_execution_path_smoke.py --check
cargo test -q --lib \
  mir::builder::module_lifecycle_capture_tests::mirbuilder_minimal_literal_integer_path_smoke \
  -- --exact
