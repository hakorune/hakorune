#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="fastpath-reachability-ledger-v1"
CARD="docs/development/current/main/phases/phase-296x/296x-984-FASTPATH-REACHABILITY-LEDGER-V1-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-983-FASTPATH-ROUTE-PRIORITY-TABLE-001.md"
INDEX="docs/tools/check-scripts-index.md"
LEDGER="tools/hako_check/fastpath_reachability_ledger.py"
PRIORITY="tools/hako_check/fastpath_route_priority.py"
README="tools/hako_check/README.md"
TEST="tools/hako_check/tests/test_fastpath_reachability_ledger.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_fastpath_reachability_ledger_v1_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$LEDGER" "$PRIORITY" "$README" "$TEST"; do
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
  "output_contract=hako-fastpath-reachability-ledger-v1" \
  "source_evidence=296x-983" \
  "row_kind=tooling" \
  "route_priority_table_version=v0" \
  "selected_route_priority_source=route_priority_table_v0" \
  "preempted_reason=lower_priority_selected_route" \
  "candidate_only_selected_route=none" \
  "candidate_only_winner_claim_allowed=0" \
  "route_priority_changes_backend_lowering=0" \
  "route_priority_retires_exact_seed=0" \
  "forced_reachability_allowed=0" \
  "next_task=EXACT-SEED-RETIRE-INVENTORY-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=FASTPATH-REACHABILITY-LEDGER-V1-001" "$PREV_CARD" || {
  echo "[$TAG] previous card does not hand off to ledger v1" >&2
  exit 1
}

for text in \
  "priority_value_for_family" \
  "route_priority_table_version" \
  "selected_route_priority_source" \
  "preempted_reason"; do
  grep -F -q "$text" "$LEDGER" || {
    echo "[$TAG] missing ledger v1 implementation token: $text" >&2
    exit 1
  }
done

for text in \
  "output_contract=hako-fastpath-reachability-ledger-v1" \
  "route_priority_table_version=v0" \
  "candidate_N_preempted_reason"; do
  grep -F -q "$text" "$README" || {
    echo "[$TAG] README missing v1 contract token: $text" >&2
    exit 1
  }
done

python3 -m unittest tools.hako_check.tests.test_fastpath_reachability_ledger >/tmp/"$TAG".unittest.out

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

cat >"$tmpdir/preempted.json" <<'JSON'
{
  "functions": [
    {
      "name": "main",
      "metadata": {
        "exact_seed_backend_route": {
          "tag": "substring_concat_loop_ascii",
          "source_route": "string_kernel_plans.loop_payload",
          "proof": "string_kernel_plan_concat_triplet_loop_payload",
          "selected_value": 35
        },
        "string_dead_text_region_plans": [
          {
            "route_id": "string.dead_text_region.plan",
            "loop_header": 18
          }
        ]
      }
    }
  ]
}
JSON

python3 "$LEDGER" --mir-json "$tmpdir/preempted.json" --front synthetic_preempted >"$tmpdir/preempted.kv"

for expected in \
  "output_contract=hako-fastpath-reachability-ledger-v1" \
  "route_priority_table_version=v0" \
  "selected_route_priority=10" \
  "selected_route_priority_source=route_priority_table_v0" \
  "candidate_1_priority=30" \
  "candidate_1_preempted_reason=lower_priority_selected_route" \
  "winner_claim_allowed=0"; do
  grep -F -x -q "$expected" "$tmpdir/preempted.kv" || {
    echo "[$TAG] missing preempted output: $expected" >&2
    exit 1
  }
done

echo "[$TAG] ok"
