#!/bin/bash
# phase29bq_harness_arraybox_birth_ternary_basic_vm.sh
# Lane-B exact blocker pin:
# - llvmlite harness must accept ArrayBox.birth() after newbox ArrayBox
# - ternary_basic must still exit 10 through the harness keep lane

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="${SMOKE_NAME_OVERRIDE:-phase29bq_harness_arraybox_birth_ternary_basic_vm}"
FIXTURE="${1:-$NYASH_ROOT/apps/tests/ternary_basic.hako}"
OUT_DIR="$(mktemp -d /tmp/phase29bq_harness_arraybox_birth.XXXXXX)"
MIR_JSON="$OUT_DIR/ternary_basic.mir.json"
OUT_EXE="$OUT_DIR/ternary_basic.harness.exe"
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

"$NYASH_BIN" --emit-mir-json "$MIR_JSON" --backend mir "$FIXTURE" >/dev/null

"$NYASH_LLVMC_BIN" \
  --driver harness \
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

test_pass "$SMOKE_NAME: PASS (ArrayBox.birth harness compat locked)"
