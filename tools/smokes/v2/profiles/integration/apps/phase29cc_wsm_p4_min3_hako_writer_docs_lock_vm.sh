#!/bin/bash
# phase29cc_wsm_p4_min3_hako_writer_docs_lock_vm.sh
# Contract pin:
# - WSM-P4-min3 docs-first lock for .hako writer entry + parity gate.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

doc="$NYASH_ROOT/docs/development/current/main/phases/phase-29cc/29cc-156-wsm-p4-min3-hako-writer-entry-parity-doc-lock-ssot.md"
if [ ! -f "$doc" ]; then
  test_fail "phase29cc_wsm_p4_min3_hako_writer_docs_lock_vm: lock doc missing"
  exit 1
fi

for needle in \
  "WSM-P4-min3" \
  ".hako" \
  "parity" \
  "byte-level" \
  "fail-fast" \
  "WSM-P4-min4"; do
  if ! grep -q "$needle" "$doc"; then
    test_fail "phase29cc_wsm_p4_min3_hako_writer_docs_lock_vm: missing keyword in lock doc: $needle"
    exit 1
  fi
done

test_pass "phase29cc_wsm_p4_min3_hako_writer_docs_lock_vm: PASS (WSM-P4-min3 docs-first lock)"
