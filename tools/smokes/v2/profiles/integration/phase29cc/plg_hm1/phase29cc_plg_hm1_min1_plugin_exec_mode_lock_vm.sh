#!/bin/bash
# phase29cc_plg_hm1_min1_plugin_exec_mode_lock_vm.sh
# Contract pin:
# - PLG-HM1-min1: NYASH_PLUGIN_EXEC_MODE accepts module_first|dynamic_only|dynamic_first.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test plugin_exec_mode_ -- --nocapture 2>&1)
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_plg_hm1_min1_plugin_exec_mode_lock_vm: plugin exec mode tests failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,200p'
  exit 1
fi

for marker in \
  "plugin_exec_mode_defaults_to_module_first" \
  "plugin_exec_mode_accepts_all_modes" \
  "plugin_exec_mode_rejects_invalid"; do
  if ! printf '%s\n' "$output" | grep -q "$marker"; then
    test_fail "phase29cc_plg_hm1_min1_plugin_exec_mode_lock_vm: missing marker: $marker"
    printf '%s\n' "$output" | sed -n '1,200p'
    exit 1
  fi
done

test_pass "phase29cc_plg_hm1_min1_plugin_exec_mode_lock_vm: PASS (PLG-HM1-min1 plugin exec mode contract)"
