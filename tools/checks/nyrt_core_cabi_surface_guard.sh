#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[nyrt-core-cabi-surface-guard] checking route/verifier/safety/lifecycle surface sync"

if ! command -v rg >/dev/null 2>&1; then
  echo "[nyrt-core-cabi-surface-guard] ERROR: rg is required" >&2
  exit 2
fi

HEADER="include/nyrt.h"
SHIM="src/abi/nyrt_shim.rs"
DOC="docs/reference/abi/nyrt_c_abi_v0.md"

if [[ ! -f "$HEADER" || ! -f "$SHIM" || ! -f "$DOC" ]]; then
  echo "[nyrt-core-cabi-surface-guard] ERROR: required file missing"
  exit 1
fi

SYMS=(
  nyrt_load_mir_json
  nyrt_exec_main
  nyrt_verify_mir_json
  nyrt_safety_check_mir_json
  nyrt_handle_retain_h
  nyrt_handle_release_h
)

for sym in "${SYMS[@]}"; do
  if ! rg -q "\\b${sym}\\b" "$HEADER"; then
    echo "[nyrt-core-cabi-surface-guard] ERROR: missing symbol in header: $sym"
    exit 1
  fi
  if ! rg -q "\\b${sym}\\b" "$SHIM"; then
    echo "[nyrt-core-cabi-surface-guard] ERROR: missing symbol in shim: $sym"
    exit 1
  fi
  if ! rg -q "\\b${sym}\\b" "$DOC"; then
    echo "[nyrt-core-cabi-surface-guard] ERROR: missing symbol in doc: $sym"
    exit 1
  fi
done

echo "[nyrt-core-cabi-surface-guard] ok"
