#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
cd "$ROOT_DIR"

exec python3 tools/checks/guard_spec_runner.py \
  --root "$ROOT_DIR" \
  --spec tools/checks/specs/hako_alloc_osvm_fast_path_route_closeout.toml
