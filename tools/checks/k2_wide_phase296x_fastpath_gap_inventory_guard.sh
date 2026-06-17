#!/usr/bin/env bash
set -euo pipefail

TAG="fastpath-gap-inventory"
CARD="docs/development/current/main/phases/phase-296x/296x-1042-FASTPATH-GAP-INVENTORY-001.md"
TOOL="tools/hako_check/fastpath_gap_inventory.py"
TEST="tools/hako_check/tests/test_fastpath_gap_inventory.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_fastpath_gap_inventory_guard.sh"

echo "[$TAG] checking FastPath gap inventory"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -F -q "$needle" "$path"; then
    echo "[$TAG] missing '$needle' in $path" >&2
    exit 1
  fi
}

[[ -f "$CARD" ]] || { echo "[$TAG] missing card: $CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[$TAG] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$TEST" ]] || { echo "[$TAG] missing test: $TEST" >&2; exit 1; }

require_text "$CARD" "Status: Landed"
require_text "$CARD" "output_contract=hako-fastpath-gap-inventory-v0"
require_text "$CARD" "fallback_evidence_fact_enabled=0"
require_text "$CARD" "backend_lowering_changed=0"
require_text "$CARD" "winner_claim_allowed=0"

require_text "$TOOL" "hako-fastpath-gap-inventory-v0"
require_text "$TOOL" "known_receiver_direct_method_without_fact_count"
require_text "$TOOL" "fallback_evidence_fact_enabled"
require_text "$TOOL" "winner_claim_allowed"
require_text "$TOOL" "backend_lowering_changed"

require_text "$TEST" "test_reports_known_receiver_routes_without_local_fastpath_fact"
require_text "$TEST" "test_matching_fact_closes_the_gap_for_that_site"
require_text "$INDEX" "$SELF_SCRIPT"
require_text "$INDEX" "$TOOL"

python3 -m unittest tools.hako_check.tests.test_fastpath_gap_inventory >/tmp/"$TAG".unittest.out

echo "[$TAG] ok"
