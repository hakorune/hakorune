#!/bin/bash
# phase29cc_wsm_p10_min10_native_promotion_closeout_lock_vm.sh
# Contract pin:
# - WSM-P10-min10 closes out fixed4 native promotion family (warn/info/error/debug)
#   while keeping min5 fixed3 inventory bridge-only.

set -euo pipefail

source "$(dirname "$0")/phase29cc_wsm_p10_common.sh"

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-203-wsm-p10-min10-native-promotion-closeout-lock-ssot.md"
require_p10_doc_keywords \
  "phase29cc_wsm_p10_min10_native_promotion_closeout_lock_vm" \
  "$doc" \
  "WSM-P10-min10" \
  "wsm.p10.main_loop_extern_call.warn.fixed4.v0" \
  "wsm.p10.main_loop_extern_call.info.fixed4.v0" \
  "wsm.p10.main_loop_extern_call.error.fixed4.v0" \
  "wsm.p10.main_loop_extern_call.debug.fixed4.v0" \
  "warn.fixed3.inventory.v0" \
  "info.fixed3.inventory.v0" \
  "error.fixed3.inventory.v0" \
  "debug.fixed3.inventory.v0" \
  "monitor-only"

run_p10_contract_tests \
  "bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min6_warn_native_promotion_lock_vm.sh" \
  "bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min7_info_native_promotion_lock_vm.sh" \
  "bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min8_error_native_promotion_lock_vm.sh" \
  "bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min9_debug_native_promotion_lock_vm.sh"

test_pass "phase29cc_wsm_p10_min10_native_promotion_closeout_lock_vm: PASS (WSM-P10-min10 native promotion closeout lock)"
