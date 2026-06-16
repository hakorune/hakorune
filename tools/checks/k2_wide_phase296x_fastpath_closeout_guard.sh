#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="fastpath-closeout"
CARD="docs/development/current/main/phases/phase-296x/296x-988-FASTPATH-CLOSEOUT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_fastpath_closeout_guard.sh"

cards=(
  "docs/development/current/main/phases/phase-296x/296x-981-FASTPATH-REACHABILITY-LEDGER-001.md"
  "docs/development/current/main/phases/phase-296x/296x-982-FASTPATH-UNREACHABLE-CONSUMER-GUARD-001.md"
  "docs/development/current/main/phases/phase-296x/296x-983-FASTPATH-ROUTE-PRIORITY-TABLE-001.md"
  "docs/development/current/main/phases/phase-296x/296x-984-FASTPATH-REACHABILITY-LEDGER-V1-001.md"
  "docs/development/current/main/phases/phase-296x/296x-985-EXACT-SEED-RETIRE-INVENTORY-001.md"
  "docs/development/current/main/phases/phase-296x/296x-986-FASTPATH-CONSUMER-REACHABILITY-GATE-001.md"
  "docs/development/current/main/phases/phase-296x/296x-987-FASTPATH-CONSUMER-INVENTORY-001.md"
  "$CARD"
)

for file in "${cards[@]}" "$INDEX"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

for file in "${cards[@]}"; do
  grep -q '^Status: Landed$' "$file" || {
    echo "[$TAG] card must be Landed: $file" >&2
    exit 1
  }
done

grep -F -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[$TAG] check index missing guard entry" >&2
  exit 1
}

require_card_line() {
  local expected="$1"
  if ! grep -F -x -q "$expected" "$CARD"; then
    echo "[$TAG] missing card line: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-fastpath-closeout-v0" \
  "source_evidence=296x-981..987" \
  "row_kind=closeout" \
  "route_priority_table_landed=1" \
  "reachability_ledger_v1_landed=1" \
  "unreachable_consumer_guard_landed=1" \
  "exact_seed_retire_inventory_landed=1" \
  "consumer_reachability_gate_landed=1" \
  "consumer_inventory_landed=1" \
  "backend_lowering_changed=0" \
  "route_priority_changed_runtime=0" \
  "exact_seed_retired=0" \
  "forced_reachability_allowed=0" \
  "winner_claim_from_unreachable_consumer_allowed=0" \
  "fastpath_infra_closeout=1" \
  "next_task=MIMALLOC-FRESH-FRONT-SELECTION-AFTER-FASTPATH-CLOSEOUT-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

bash tools/checks/k2_wide_phase296x_fastpath_unreachable_consumer_guard.sh >/tmp/"$TAG".unreachable.out
bash tools/checks/k2_wide_phase296x_fastpath_route_priority_table_guard.sh >/tmp/"$TAG".priority.out
bash tools/checks/k2_wide_phase296x_fastpath_reachability_ledger_v1_guard.sh >/tmp/"$TAG".ledger.out
bash tools/checks/k2_wide_phase296x_exact_seed_retire_inventory_guard.sh >/tmp/"$TAG".exact_seed.out
bash tools/checks/k2_wide_phase296x_fastpath_consumer_reachability_gate_guard.sh >/tmp/"$TAG".consumer_gate.out
bash tools/checks/k2_wide_phase296x_fastpath_consumer_inventory_guard.sh >/tmp/"$TAG".consumer_inventory.out

python3 -m unittest \
  tools.hako_check.tests.test_fastpath_reachability_ledger \
  tools.hako_check.tests.test_fastpath_route_priority_table \
  tools.hako_check.tests.test_exact_seed_retire_inventory \
  tools.hako_check.tests.test_fastpath_consumer_inventory \
  >/tmp/"$TAG".unittest.out

echo "[$TAG] ok"
