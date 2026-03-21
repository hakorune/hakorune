#!/bin/bash
# phase29cc_wsm_p7_min4_compat_retention_lock_vm.sh
# Contract pin:
# - WSM-P7-min4: compat retention is explicit and default route remains unchanged.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-187-wsm-p7-min4-compat-retention-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p7_min4_compat_retention_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P7-min4" \
  "compat route" \
  "default-only" \
  "rollback" \
  "accepted-but-blocked"; do
  if ! grep -Fq "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p7_min4_compat_retention_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/phase29cc_wsm/p7/phase29cc_wsm_p7_min2_default_hako_only_guard_vm.sh"
bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/phase29cc_wsm/p7/phase29cc_wsm_p7_min3_two_demo_lock_vm.sh"

test_pass "phase29cc_wsm_p7_min4_compat_retention_lock_vm: PASS (WSM-P7-min4 compat retention lock)"
