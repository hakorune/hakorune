#!/bin/bash
# RING1-CORE-06-min3: array string-store provider smoke (VM)
#
# Contract pin:
# - ArrayBox string-handle set stays owned by ArrayCoreBox on the VM route.
# - Exit code must be 0.
# - Output must match one-line contract.

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/ring1_array_provider/array_string_set_min.hako"

if [ ! -f "$FIXTURE" ]; then
  test_fail "ring1_array_string_provider_vm: Fixture not found: $FIXTURE"
  exit 2
fi

set +e
output=$(NYASH_DISABLE_PLUGINS=1 \
         NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
         NYASH_VM_USE_FALLBACK=0 \
         NYASH_JOINIR_DEV=0 \
         NYASH_JOINIR_STRICT=0 \
         run_nyash_vm "$FIXTURE")
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "ring1_array_string_provider_vm: expected rc=0, got $rc"
  exit 1
fi

expected="ARRAY_STRING_SET_OK size=1"
compare_outputs "$expected" "$output" "ring1_array_string_provider_vm" || exit 1

test_pass "ring1_array_string_provider_vm"
