#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
cd "$ROOT_DIR"

echo "[build] hakorune (vm)"
cargo build --release

echo "[build] core plugins (subset)"
cargo build -p nyash-counter-plugin --release

APP="apps/tests/vm-plugin-smoke-counter/main.hako"
echo "[run] VM plugin-first strict: $APP"
BIN=${NYASH_BIN:-./target/release/hakorune}
NYASH_VM_PLUGIN_STRICT=1 "$BIN" --backend vm "$APP"
