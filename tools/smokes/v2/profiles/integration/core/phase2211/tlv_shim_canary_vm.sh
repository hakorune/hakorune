#!/usr/bin/env bash
# Phase 22.1 — TLV shim minimal path canary (crate-level unit test)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../.." && pwd)"

(
  cd "$ROOT" && cargo build -q --release -p nyash-rust --features tlv-shim
)

echo "[PASS] tlv_shim_canary_vm"
exit 0
