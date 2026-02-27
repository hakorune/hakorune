#!/bin/bash
# Phase 29cc PLG-07-min6: FileBox binary retire readiness lock (VM)
# Contract:
# - PLG-07-min6 is docs-first readiness lock; behavior is unchanged.
# - Readiness evidence remains reproducible:
#   1) default switch contract guard
#   2) dual-run parity guard
#   3) .hako route smoke
# - rollback knobs (compat/dualrun toggles) stay documented and available.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg07_filebox_binary_retire_readiness_lock_vm"
DOC="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-183-plg07-min6-filebox-retire-readiness-lock-ssot.md"

if [ ! -f "$DOC" ]; then
  test_fail "$SMOKE_NAME: lock doc missing ($DOC)"
  exit 1
fi

for needle in \
  "PLG-07-min6" \
  "retire readiness" \
  "NYASH_PLG07_COMPAT_RUST" \
  "NYASH_PLG07_DUALRUN" \
  "dev_gate.sh portability"; do
  if ! grep -q "$needle" "$DOC"; then
    test_fail "$SMOKE_NAME: missing keyword in lock doc: $needle"
    exit 1
  fi
done

bash "$NYASH_ROOT/tools/checks/phase29cc_plg07_filebox_binary_default_switch_guard.sh"
bash "$NYASH_ROOT/tools/checks/phase29cc_plg07_filebox_binary_dualrun_guard.sh"
bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_hako_route_vm.sh"

test_pass "$SMOKE_NAME: PASS (PLG-07-min6 retire readiness lock)"
