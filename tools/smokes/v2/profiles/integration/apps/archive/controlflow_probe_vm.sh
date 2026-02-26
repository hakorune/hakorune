#!/bin/bash
# APP-2: controlflow_probe (VM)
#
# Contract pin:
# - Tests conditional loop variable update (early exit)
# - Tests continue in loop
# - Tests nested if value carry
# - Output is single line with exact format

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

APP="$NYASH_ROOT/apps/tools/controlflow_probe/main.hako"
EXPECTED="$NYASH_ROOT/apps/tests/controlflow_probe/expected_summary.txt"

# Pre-flight checks
if [ ! -f "$APP" ]; then
  test_fail "controlflow_probe_vm: App not found: $APP"
  exit 2
fi
if [ ! -f "$EXPECTED" ]; then
  test_fail "controlflow_probe_vm: Expected file not found: $EXPECTED"
  exit 2
fi

# Force Rust VM lane (with explicit exit code check)
set +e
output=$(NYASH_DISABLE_PLUGINS=1 \
         NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
         NYASH_VM_USE_FALLBACK=0 \
         NYASH_JOINIR_DEV=0 \
         NYASH_JOINIR_STRICT=0 \
         HAKO_JOINIR_STRICT=0 \
         HAKO_JOINIR_PLANNER_REQUIRED=0 \
         run_nyash_vm "$APP")
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "controlflow_probe_vm: expected rc=0, got $rc"
  exit 1
fi

# Check for error patterns
if echo "$output" | grep -q "JoinIR does not support this pattern"; then
  test_fail "controlflow_probe_vm: JoinIR pattern not supported"
  exit 1
fi
if echo "$output" | grep -q "vm-hako/unimplemented"; then
  test_fail "controlflow_probe_vm: vm-hako unimplemented"
  exit 1
fi

# Exact match verification
expected=$(cat "$EXPECTED")
compare_outputs "$expected" "$output" "controlflow_probe_vm" || exit 1

test_pass "controlflow_probe_vm"
