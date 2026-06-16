#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="fastpath-consumer-reachability-gate"
CARD="docs/development/current/main/phases/phase-296x/296x-986-FASTPATH-CONSUMER-REACHABILITY-GATE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-985-EXACT-SEED-RETIRE-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
LEDGER="tools/hako_check/fastpath_reachability_ledger.py"
INVENTORY="tools/hako_check/exact_seed_retire_inventory.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_fastpath_consumer_reachability_gate_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$LEDGER" "$INVENTORY"; do
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
  "output_contract=hako-fastpath-consumer-reachability-gate-v0" \
  "source_evidence=296x-985" \
  "row_kind=process_guard" \
  "new_consumer_requires_reachable_or_scaffold=1" \
  "scaffold_requires_followup_row=1" \
  "scaffold_requires_winner_claim_allowed_0=1" \
  "backend_consumer_code_is_not_reachability=1" \
  "gate_reuses_reachability_ledger=1" \
  "gate_reuses_exact_seed_retire_inventory=1" \
  "exact_seed_retire_inventory_required_before_retire=1" \
  "forced_reachability_allowed=0" \
  "backend_lowering_changed=0" \
  "next_task=FASTPATH-CONSUMER-INVENTORY-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=FASTPATH-CONSUMER-REACHABILITY-GATE-001" "$PREV_CARD" || {
  echo "[$TAG] previous card does not hand off to consumer reachability gate" >&2
  exit 1
}

bash tools/checks/k2_wide_phase296x_fastpath_unreachable_consumer_guard.sh >/tmp/"$TAG".unreachable.out
bash tools/checks/k2_wide_phase296x_exact_seed_retire_inventory_guard.sh >/tmp/"$TAG".exact_seed.out

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

cat >"$tmpdir/candidate_only.json" <<'JSON'
{
  "functions": [
    {
      "name": "main",
      "metadata": {
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

python3 "$LEDGER" --mir-json "$tmpdir/candidate_only.json" --front synthetic >"$tmpdir/candidate_only.kv"

for expected in \
  "selected_route=none" \
  "new_consumer_exists=1" \
  "new_consumer_reachable=0" \
  "winner_claim_allowed=0"; do
  grep -F -x -q "$expected" "$tmpdir/candidate_only.kv" || {
    echo "[$TAG] candidate-only ledger violated gate: $expected" >&2
    exit 1
  }
done

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

python3 "$INVENTORY" --mir-json "$tmpdir/preempted.json" --front synthetic >"$tmpdir/preempted.kv"

for expected in \
  "replacement_candidate_exists=1" \
  "replacement_reachable=0" \
  "retire_allowed=0" \
  "retire_blocker=replacement_not_reachable"; do
  grep -F -x -q "$expected" "$tmpdir/preempted.kv" || {
    echo "[$TAG] preempted inventory violated gate: $expected" >&2
    exit 1
  }
done

echo "[$TAG] ok"
