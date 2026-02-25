#!/bin/bash
# APP-1: gate-log-summarizer (VM)
#
# Contract pin:
# - Reads log file and outputs SUMMARY line with counts
# - Extracts FAIL lines in input order
# - Exits 0 on success, non-zero on invalid input
#
# Note: Force Rust VM lane (not vm-hako) because FileBox is required

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

APP="$NYASH_ROOT/apps/tools/gate_log_summarizer/main.hako"
FIXTURE="$NYASH_ROOT/apps/tests/gate_log_summarizer/sample_mixed.log"

# Pre-flight checks
if [ ! -f "$APP" ]; then
  test_fail "gate_log_summarizer_vm: App not found: $APP"
  exit 2
fi
if [ ! -f "$FIXTURE" ]; then
  test_fail "gate_log_summarizer_vm: Fixture not found: $FIXTURE"
  exit 2
fi

# Test 1: Valid input (exact match)
# Force Rust VM lane (not vm-hako)
output=$(NYASH_VM_HAKO_PREFER_STRICT_DEV=0 NYASH_VM_USE_FALLBACK=0 \
         NYASH_JOINIR_DEV=0 NYASH_JOINIR_STRICT=0 \
         GATE_LOG_FILE="$FIXTURE" \
         run_nyash_vm "$APP")

expected=$(cat << 'TXT'
SUMMARY pass=7 fail=2 skip=1
FAIL_LINES 2
[FAIL] phase29y_handle_abi_borrowed_owned_vm: rc=1
[FAIL] phase29y_lane_gate_vm: contract mismatch
TXT
)

compare_outputs "$expected" "$output" "gate_log_summarizer_vm" || exit 1

# Test 2: Invalid input (non-zero exit code)
set +e
error_output=$(NYASH_VM_HAKO_PREFER_STRICT_DEV=0 NYASH_VM_USE_FALLBACK=0 \
               NYASH_JOINIR_DEV=0 NYASH_JOINIR_STRICT=0 \
               GATE_LOG_FILE="/nonexistent.log" \
               run_nyash_vm "$APP" 2>&1)
error_rc=$?
set -e

if [ "$error_rc" -eq 0 ]; then
  test_fail "gate_log_summarizer_vm: should fail on missing file"
  exit 1
fi

test_pass "gate_log_summarizer_vm"
