#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

TAG="fastpath-unreachable-consumer-guard"
CARD="docs/development/current/main/phases/phase-296x/296x-982-FASTPATH-UNREACHABLE-CONSUMER-GUARD-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-981-FASTPATH-REACHABILITY-LEDGER-001.md"
INDEX="docs/tools/check-scripts-index.md"
README="tools/hako_check/README.md"
LEDGER="tools/hako_check/fastpath_reachability_ledger.py"
LEDGER_TEST="tools/hako_check/tests/test_fastpath_reachability_ledger.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_fastpath_unreachable_consumer_guard.sh"

for file in "$CARD" "$PREV_CARD" "$INDEX" "$README" "$LEDGER" "$LEDGER_TEST"; do
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
  "output_contract=hako-fastpath-unreachable-consumer-guard-v0" \
  "source_evidence=296x-981" \
  "row_kind=guard" \
  "unreachable_consumer_winner_claim_allowed=0" \
  "preempted_consumer_winner_claim_allowed=0" \
  "candidate_only_winner_claim_allowed=0" \
  "forced_reachability_allowed=0" \
  "backend_consumer_code_is_not_reachability=1" \
  "active_mir_metadata_candidate_required=1" \
  "selected_route_required_for_reachability=1" \
  "new_backend_consumer_requires_reachability_or_scaffold=1" \
  "scaffold_requires_winner_claim_allowed_0=1" \
  "scaffold_requires_followup_row=1" \
  "route_priority_changed=0" \
  "backend_lowering_changed=0" \
  "exact_seed_retired=0" \
  "summary=ok"; do
  require_card_line "$expected"
done

grep -F -q "The next row should prevent new backend consumers" "$PREV_CARD" || {
  echo "[$TAG] previous card does not hand off unreachable-consumer guard" >&2
  exit 1
}

for text in \
  "new backend consumer" \
  "reachable in the active front" \
  "intentionally_unreachable_scaffold=1" \
  "winner_claim_allowed=0" \
  "follow-up route selection/retire row named"; do
  grep -F -q "$text" "$CARD" || {
    echo "[$TAG] missing card invariant: $text" >&2
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

grep -F -x -q "new_consumer_exists=1" "$tmpdir/preempted.kv" || {
  echo "[$TAG] preempted fixture must expose new consumer" >&2
  exit 1
}
grep -F -x -q "new_consumer_reachable=0" "$tmpdir/preempted.kv" || {
  echo "[$TAG] preempted fixture must keep new consumer unreachable" >&2
  exit 1
}
grep -F -x -q "preemption_detected=1" "$tmpdir/preempted.kv" || {
  echo "[$TAG] preempted fixture must detect preemption" >&2
  exit 1
}
grep -F -x -q "winner_claim_allowed=0" "$tmpdir/preempted.kv" || {
  echo "[$TAG] preempted fixture must forbid winner claim" >&2
  exit 1
}

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

python3 "$LEDGER" --mir-json "$tmpdir/candidate_only.json" --front synthetic_candidate_only >"$tmpdir/candidate_only.kv"

grep -F -x -q "selected_route=none" "$tmpdir/candidate_only.kv" || {
  echo "[$TAG] candidate-only fixture must not select a route" >&2
  exit 1
}
grep -F -x -q "candidate_0_reachable=0" "$tmpdir/candidate_only.kv" || {
  echo "[$TAG] candidate-only fixture must keep candidate unreachable" >&2
  exit 1
}
grep -F -x -q "winner_claim_allowed=0" "$tmpdir/candidate_only.kv" || {
  echo "[$TAG] candidate-only fixture must forbid winner claim" >&2
  exit 1
}

echo "[$TAG] ok"
