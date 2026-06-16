#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="exact-seed-retire-inventory"
CARD="docs/development/current/main/phases/phase-296x/296x-985-EXACT-SEED-RETIRE-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-984-FASTPATH-REACHABILITY-LEDGER-V1-001.md"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/hako_check/exact_seed_retire_inventory.py"
TEST="tools/hako_check/tests/test_exact_seed_retire_inventory.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_seed_retire_inventory_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$TOOL" "$TEST"; do
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
  "output_contract=hako-exact-seed-retire-inventory-v0" \
  "source_evidence=296x-984" \
  "row_kind=inventory" \
  "replacement_candidate_required=1" \
  "replacement_reachable_required=1" \
  "retire_allowed=0" \
  "drive_by_retire_allowed=0" \
  "exact_seed_retired=0" \
  "backend_lowering_changed=0" \
  "route_priority_changed=0" \
  "forced_reachability_allowed=0" \
  "next_task=FASTPATH-CONSUMER-REACHABILITY-GATE-001" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "next_task=EXACT-SEED-RETIRE-INVENTORY-001" "$PREV_CARD" || {
  echo "[$TAG] previous card does not hand off to exact seed inventory" >&2
  exit 1
}

python3 -m unittest tools.hako_check.tests.test_exact_seed_retire_inventory >/tmp/"$TAG".unittest.out

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

cat >"$tmpdir/exact_seed_only.json" <<'JSON'
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
        }
      }
    }
  ]
}
JSON

python3 "$TOOL" --mir-json "$tmpdir/exact_seed_only.json" --front synthetic >"$tmpdir/exact_seed_only.kv"

for expected in \
  "output_contract=hako-exact-seed-retire-inventory-v0" \
  "exact_seed_present=1" \
  "replacement_candidate_exists=0" \
  "retire_allowed=0" \
  "retire_blocker=no_replacement_candidate" \
  "exact_seed_retired=0"; do
  grep -F -x -q "$expected" "$tmpdir/exact_seed_only.kv" || {
    echo "[$TAG] missing exact-seed-only output: $expected" >&2
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

python3 "$TOOL" --mir-json "$tmpdir/preempted.json" --front synthetic >"$tmpdir/preempted.kv"

for expected in \
  "replacement_family=string_dead_text_region" \
  "replacement_candidate_exists=1" \
  "replacement_reachable=0" \
  "preemption_detected=1" \
  "retire_allowed=0" \
  "retire_blocker=replacement_not_reachable" \
  "drive_by_retire_allowed=0"; do
  grep -F -x -q "$expected" "$tmpdir/preempted.kv" || {
    echo "[$TAG] missing preempted output: $expected" >&2
    exit 1
  }
done

echo "[$TAG] ok"
