#!/bin/bash
# Phase 29cc PLG-07-min7: FileBox binary retire execution lock (VM)
# Contract:
# - vm_plugin_smoke keeps FileBox binary on .hako route only.
# - compat/dual-run toggles are removed from default entrypoint.
# - retire execution guard + .hako route smoke remain green.

set -euo pipefail

source "$(dirname "$0")/phase29cc_plg07_filebox_binary_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg07_filebox_binary_retire_execution_lock_vm"
DOC="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-204-plg07-min7-filebox-retire-execution-lock-ssot.md"
plg07_require_doc_keywords \
  "$SMOKE_NAME" \
  "$DOC" \
  "PLG-07-min7" \
  "retire execution" \
  "phase29cc_plg07_filebox_binary_hako_route_vm.sh" \
  "NYASH_PLG07_COMPAT_RUST" \
  "NYASH_PLG07_DUALRUN"

bash "$NYASH_ROOT/tools/checks/phase29cc_plg07_filebox_binary_retire_execution_guard.sh"
bash "$NYASH_ROOT/tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_hako_route_vm.sh"

test_pass "$SMOKE_NAME: PASS (PLG-07-min7 retire execution lock)"
