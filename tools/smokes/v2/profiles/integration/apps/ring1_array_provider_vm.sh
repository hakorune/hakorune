#!/bin/bash
# RING1-CORE-06-min2: array provider smoke (VM)
#
# Contract pin:
# - ArrayBox push/size/get baseline stays stable while ring1 array provider is wired.
# - Exit code must be 0.
# - Output must match one-line contract.

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/ring1_array_provider/array_size_push_min.hako"

if [ ! -f "$FIXTURE" ]; then
  test_fail "ring1_array_provider_vm: Fixture not found: $FIXTURE"
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
  test_fail "ring1_array_provider_vm: expected rc=0, got $rc"
  exit 1
fi

expected="ARRAY_PROVIDER_OK size=2 get0=11"
compare_outputs "$expected" "$output" "ring1_array_provider_vm" || exit 1

test_pass "ring1_array_provider_vm"
