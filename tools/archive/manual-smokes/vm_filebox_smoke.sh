#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/../../.." && pwd)
cd "$ROOT_DIR"

echo "[build] hakorune (vm)"
cargo build --release

echo "[build] plugins: filebox"
cargo build -p nyash-filebox-plugin --release

mkdir -p tmp
echo -n "OK" > tmp/vm_filebox_smoke.txt

APP="apps/tests/vm-plugin-smoke-filebox/main.hako"
echo "[run] VM plugin-first strict: $APP"
BIN=${NYASH_BIN:-./target/release/hakorune}
NYASH_VM_PLUGIN_STRICT=1 "$BIN" --backend vm "$APP"
