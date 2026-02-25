#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

# Quick profile: heavy ny-llvmc build/obj emit is skipped after env consolidation.
echo "[SKIP] s3_backend_selector_crate_obj_canary_vm (disabled in quick profile after env consolidation)"
exit 0

# Prebuild ny-llvmc
(cd "$ROOT" && cargo build -q --release -p nyash-llvm-compiler >/dev/null)

OBJ="/tmp/ny_crate_backend_$$.o"
# Use ny-llvmc dummy mode directly (crate backend)
BIN_NYLLVMC="$ROOT/target/release/ny-llvmc"
if "$BIN_NYLLVMC" --dummy --emit obj --out "$OBJ" >/dev/null 2>&1; then
  if [[ -f "$OBJ" ]]; then
    echo "[PASS] s3_backend_selector_crate_obj_canary_vm"
    rm -f "$OBJ" 2>/dev/null || true
    exit 0
  fi
fi
echo "[FAIL] s3_backend_selector_crate_obj_canary_vm" >&2
exit 1
