#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="fastpath-route-priority-table"
CARD="docs/development/current/main/phases/phase-296x/296x-983-FASTPATH-ROUTE-PRIORITY-TABLE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-982-FASTPATH-UNREACHABLE-CONSUMER-GUARD-001.md"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/hako_check/fastpath_route_priority_table.py"
VOCAB="tools/hako_check/fastpath_route_priority.py"
TEST="tools/hako_check/tests/test_fastpath_route_priority_table.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_fastpath_route_priority_table_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$TOOL" "$VOCAB" "$TEST"; do
  [[ -f "$file" ]] || { echo "[$TAG] missing file: $file" >&2; exit 1; }
done

grep -q '^Status: Landed$' "$CARD" || {
  echo "[$TAG] card must be Landed" >&2
  exit 1
}

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
  "output_contract=hako-fastpath-route-priority-table-v0" \
  "source_evidence=296x-982" \
  "row_kind=design_guard" \
  "route_priority_table_version=v0" \
  "lowest_priority_wins=1" \
  "exact_seed_priority=10" \
  "local_fastpath_fact_priority=20" \
  "generic_metadata_consumer_priority=30" \
  "runtime_helper_fallback_priority=90" \
  "route_priority_changes_backend_lowering=0" \
  "route_priority_retires_exact_seed=0" \
  "forced_reachability_allowed=0" \
  "winner_claim_from_priority_table_allowed=0" \
  "next_task=FASTPATH-REACHABILITY-LEDGER-V1-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "FASTPATH-ROUTE-PRIORITY-TABLE-001" "$PREV_CARD" || {
  echo "[$TAG] previous card does not hand off to priority table" >&2
  exit 1
}

python3 -m unittest tools.hako_check.tests.test_fastpath_route_priority_table >/tmp/"$TAG".unittest.out
python3 "$TOOL" >"/tmp/$TAG.kv"

for expected in \
  "output_contract=hako-fastpath-route-priority-table-v0" \
  "entry_count=4" \
  "priority_unique=1" \
  "lowest_priority_wins=1" \
  "route_priority_changes_backend_lowering=0" \
  "route_priority_retires_exact_seed=0" \
  "entry_0_family=exact_seed" \
  "entry_0_priority=10" \
  "entry_1_family=local_fastpath_fact" \
  "entry_1_priority=20" \
  "entry_2_family=string_dead_text_region" \
  "entry_2_priority=30" \
  "entry_3_family=runtime_helper_fallback" \
  "entry_3_priority=90"; do
  grep -F -x -q "$expected" "/tmp/$TAG.kv" || {
    echo "[$TAG] missing tool output: $expected" >&2
    exit 1
  }
done

echo "[$TAG] ok"
