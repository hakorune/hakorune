#!/bin/bash
# RING1-CORE-06-min4: array string shadow guard smoke (VM)
#
# Contract pin:
# - shadow-only handle text must not be reinterpreted as live `nyash.array.set_his`
# - strict array-core mode must stay silent on this path
# - output must match one-line contract

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/ring1_array_provider/array_string_shadow_guard_min.hako"

if [ ! -f "$FIXTURE" ]; then
  test_fail "ring1_array_string_shadow_guard_vm: Fixture not found: $FIXTURE"
  exit 2
fi

set +e
output=$(NYASH_DISABLE_PLUGINS=1 \
         NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
         NYASH_VM_USE_FALLBACK=0 \
         NYASH_JOINIR_DEV=0 \
         NYASH_JOINIR_STRICT=0 \
         HAKO_VM_ARRAY_CORE_STRICT=1 \
         run_nyash_vm "$FIXTURE")
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "ring1_array_string_shadow_guard_vm: expected rc=0, got $rc"
  exit 1
fi

expected="ARRAY_STRING_SHADOW_GUARD_OK handled=1 got=1"
compare_outputs "$expected" "$output" "ring1_array_string_shadow_guard_vm" || exit 1

test_pass "ring1_array_string_shadow_guard_vm"
