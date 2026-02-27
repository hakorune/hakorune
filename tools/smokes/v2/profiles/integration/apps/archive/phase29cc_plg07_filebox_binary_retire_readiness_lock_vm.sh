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

source "$(dirname "$0")/phase29cc_plg07_filebox_binary_common.sh"
require_env || exit 2

SMOKE_NAME="phase29cc_plg07_filebox_binary_retire_readiness_lock_vm"
DOC="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-183-plg07-min6-filebox-retire-readiness-lock-ssot.md"
plg07_require_doc_keywords \
  "$SMOKE_NAME" \
  "$DOC" \
  "PLG-07-min6" \
  "retire readiness" \
  "NYASH_PLG07_COMPAT_RUST" \
  "NYASH_PLG07_DUALRUN" \
  "dev_gate.sh portability"

plg07_run_retire_readiness_evidence

test_pass "$SMOKE_NAME: PASS (PLG-07-min6 retire readiness lock)"
