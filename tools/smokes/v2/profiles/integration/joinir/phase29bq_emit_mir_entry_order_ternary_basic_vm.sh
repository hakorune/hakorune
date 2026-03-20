#!/bin/bash
# phase29bq_emit_mir_entry_order_ternary_basic_vm.sh
# Lane-B blocker pin:
# - Rust `--emit-mir-json` must serialize `main` as functions[0]
# - ny-llvmc EXE built from that MIR must exit 10 for ternary_basic

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="${SMOKE_NAME_OVERRIDE:-phase29bq_emit_mir_entry_order_ternary_basic_vm}"
FIXTURE="${1:-$NYASH_ROOT/apps/tests/ternary_basic.hako}"
OUT_DIR="$(mktemp -d /tmp/phase29bq_emit_mir_entry.XXXXXX)"
MIR_JSON="$OUT_DIR/ternary_basic.mir.json"
OUT_EXE="$OUT_DIR/ternary_basic.exe"
cleanup() {
  rm -rf "$OUT_DIR"
}
trap cleanup EXIT

if [[ "$FIXTURE" != /* ]]; then
  FIXTURE="$NYASH_ROOT/$FIXTURE"
fi

if [ ! -f "$FIXTURE" ]; then
  test_fail "$SMOKE_NAME: fixture missing: $FIXTURE"
  exit 1
fi

NYASH_BIN="${NYASH_BIN:-$NYASH_ROOT/target/release/hakorune}"
NYASH_LLVMC_BIN="${NYASH_LLVMC_BIN:-$NYASH_ROOT/target/release/ny-llvmc}"
NYRT_DIR="${NYASH_LLVM_NYRT:-$NYASH_ROOT/target/release}"

if [ ! -x "$NYASH_BIN" ]; then
  test_fail "$SMOKE_NAME: hakorune missing: $NYASH_BIN"
  exit 1
fi
if [ ! -x "$NYASH_LLVMC_BIN" ]; then
  test_fail "$SMOKE_NAME: ny-llvmc missing: $NYASH_LLVMC_BIN"
  exit 1
fi

"$NYASH_BIN" --emit-mir-json "$MIR_JSON" --backend mir "$FIXTURE" >/dev/null

FIRST_NAME="$(jq -r '.functions[0].name' "$MIR_JSON")"
if [ "$FIRST_NAME" != "main" ]; then
  echo "[INFO] functions=$(jq -c '.functions | map(.name)' "$MIR_JSON")"
  test_fail "$SMOKE_NAME: functions[0] must be main, got $FIRST_NAME"
  exit 1
fi

"$NYASH_LLVMC_BIN" \
  --in "$MIR_JSON" \
  --emit exe \
  --nyrt "$NYRT_DIR" \
  --out "$OUT_EXE" >/dev/null

set +e
"$OUT_EXE" >/dev/null 2>&1
CODE=$?
set -e

if [ "$CODE" -ne 10 ]; then
  test_fail "$SMOKE_NAME: expected exit 10, got $CODE"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (main-first MIR JSON entry order locked)"
