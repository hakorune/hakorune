#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

exec python3 "$ROOT_DIR/tools/checks/lib/manifest_runner.py" \
  --root "$ROOT_DIR" \
  --manifest tools/checks/guard_rows.toml \
  --table rows \
  --tag row-guard \
  --item-name row \
  "$@"
