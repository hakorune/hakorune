#!/bin/bash
# phase29y_hako_using_resolver_parity_vm.sh
# Contract pin:
# - .hako UsingResolverBox parses modules/usings JSON into map state.
# - UsingResolveSSOTBox honors workspace -> [modules] override -> module_roots fallback.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29y_hako_using_resolver_parity_min.hako"

if [ ! -f "$FIXTURE" ]; then
  test_fail "phase29y_hako_using_resolver_parity_vm: fixture missing: $FIXTURE"
  exit 2
fi

set +e
output=$(NYASH_DISABLE_PLUGINS=1 \
         NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
         NYASH_VM_USE_FALLBACK=0 \
         run_nyash_vm "$FIXTURE" 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "phase29y_hako_using_resolver_parity_vm: rc=$rc"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

expected="OK:phase29y_hako_using_resolver_parity"
compare_outputs "$expected" "$output" "phase29y_hako_using_resolver_parity_vm" || exit 1

test_pass "phase29y_hako_using_resolver_parity_vm: PASS (.hako using resolver parity locked)"
