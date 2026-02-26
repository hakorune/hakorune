#!/bin/bash
# phase29cc_wsm_p3_min1_import_object_lock_vm.sh
# Contract pin:
# - WSM-P3-min1: JS import object generation contract (supported list + fail-fast wording).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

set +e
output=$(cd "$NYASH_ROOT" && cargo test --features wasm-backend runtime_imports_js_object_ -- --nocapture 2>&1)
rc=$?
set -e

if [ "$rc" -ne 0 ]; then
  test_fail "phase29cc_wsm_p3_min1_import_object_lock_vm: import object contract tests failed (rc=$rc)"
  printf '%s\n' "$output" | sed -n '1,220p'
  exit 1
fi

for marker in \
  "runtime_imports_js_object_covers_extern_contract_imports" \
  "runtime_imports_js_object_has_no_unsupported_binding_for_standard_imports" \
  "runtime_imports_js_object_unknown_binding_uses_fail_fast_message"; do
  if ! printf '%s\n' "$output" | grep -q "$marker"; then
    test_fail "phase29cc_wsm_p3_min1_import_object_lock_vm: expected marker missing: $marker"
    printf '%s\n' "$output" | sed -n '1,220p'
    exit 1
  fi
done

test_pass "phase29cc_wsm_p3_min1_import_object_lock_vm: PASS (WSM-P3-min1 import object contract lock)"
