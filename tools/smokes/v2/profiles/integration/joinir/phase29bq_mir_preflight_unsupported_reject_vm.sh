#!/bin/bash
# phase29bq_mir_preflight_unsupported_reject_vm.sh
# L2/L4 pin: strict/dev VM preflight must reject unsupported MIR instructions.
# Uses MIR unit fixture and runs targeted cargo tests.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../../../.." && pwd)" # tools/
source "$ROOT_DIR/smokes/v2/lib/test_runner.sh"
require_env || exit 2

TEST_FILTER="vm_preflight_rejects_unsupported_throw_under_strict_gate"
CARGO_ARGS=(test -q "$TEST_FILTER" -- --test-threads=1)
PRELOAD_LIB="$ROOT_DIR/tmp/exdev/librename_copy_fallback.so"

set +e
if [ -f "$PRELOAD_LIB" ]; then
  LD_PRELOAD="$PRELOAD_LIB" cargo "${CARGO_ARGS[@]}" >/tmp/${TEST_FILTER}.out 2>&1
  RC=$?
else
  cargo "${CARGO_ARGS[@]}" >/tmp/${TEST_FILTER}.out 2>&1
  RC=$?
fi
set -e

if [ "$RC" -ne 0 ]; then
  echo "[FAIL] mir_preflight_unsupported reject pin: cargo test failed (rc=$RC)" >&2
  echo "[FAIL] test_filter=$TEST_FILTER" >&2
  echo "[FAIL] log=/tmp/${TEST_FILTER}.out" >&2
  tail -n 80 "/tmp/${TEST_FILTER}.out" >&2 || true
  exit 1
fi

if ! rg -q "^running [1-9][0-9]* test" "/tmp/${TEST_FILTER}.out"; then
  echo "[FAIL] mir_preflight_unsupported reject pin: no tests executed" >&2
  echo "[FAIL] test_filter=$TEST_FILTER" >&2
  echo "[FAIL] log=/tmp/${TEST_FILTER}.out" >&2
  tail -n 80 "/tmp/${TEST_FILTER}.out" >&2 || true
  exit 1
fi

echo "[PASS] mir_preflight_unsupported reject pin: PASS"
