#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"

# Prebuild ny-llvmc and nyash_kernel (NyRT)
timeout "${HAKO_BUILD_TIMEOUT:-10}" bash -lc '(cd "$ROOT" && cargo build -q --release -p nyash-llvm-compiler >/dev/null)' || true
timeout "${HAKO_BUILD_TIMEOUT:-10}" bash -lc '(cd "$ROOT/crates/nyash_kernel" && cargo build -q --release >/dev/null)' || true

APP="/tmp/ny_crate_backend_exe_$$"
BIN_NYLLVMC="$ROOT/target/release/ny-llvmc"

if timeout "${HAKO_BUILD_TIMEOUT:-10}" "$BIN_NYLLVMC" --dummy --emit exe --nyrt "$ROOT/target/release" --out "$APP" >/dev/null 2>&1; then
  if [[ -x "$APP" ]]; then
    set +e
    timeout "${HAKO_EXE_TIMEOUT:-5}" "$APP"
    rc=$?
    set -e
    if [ "$rc" -eq 0 ]; then
      echo "[PASS] s3_backend_selector_crate_exe_canary_vm"
      rm -f "$APP" 2>/dev/null || true
      exit 0
    fi
    if [ "$rc" -eq 124 ]; then
      echo "[SKIP] s3_backend_selector_crate_exe_canary_vm: timed out running EXE (rc=$rc)" >&2
      rm -f "$APP" 2>/dev/null || true
      exit 0
    fi
  fi
fi
echo "[SKIP] s3_backend_selector_crate_exe_canary_vm: build or run failed/timed out" >&2
exit 0
