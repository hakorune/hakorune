#!/bin/bash
# phase29cc_wsm_p5_min1_default_cutover_docs_lock_vm.sh
# Contract pin:
# - WSM-P5-min1 docs-first lock for default cutover boundary (default route / legacy route / fail-fast).

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-160-wsm-p5-min1-default-cutover-doc-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p5_min1_default_cutover_docs_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P5-min1" \
  "default route" \
  "legacy-wasm-rust" \
  "fail-fast" \
  "P5-min2 Entry Contract"; do
  if ! grep -q "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p5_min1_default_cutover_docs_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

test_pass "phase29cc_wsm_p5_min1_default_cutover_docs_lock_vm: PASS (WSM-P5-min1 docs-first lock)"
