#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-generic-nested-argument-single-eval-fixture"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_127="docs/development/current/main/phases/phase-296x/296x-127-GENERIC-NESTED-ARGUMENT-SINGLE-EVAL-FIXTURE.md"
CARD_128="docs/development/current/main/phases/phase-296x/296x-128-MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX.md"
SSOT="docs/development/current/main/design/nested-argument-single-evaluation-ssot.md"
APP="apps/mir-nested-argument-single-eval-proof/main.hako"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/generic_nested_argument_single_eval_fixture.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_generic_nested_argument_single_eval_fixture_guard.sh"

echo "[$TAG] checking generic nested argument single-eval fixture"

guard_require_files "$TAG" "$CARD_127" "$CARD_128" "$SSOT" "$APP" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_127" "row127 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_128" "row128 card must be current"
guard_expect_fixed_in_file "$TAG" 'return me.wrap(NestedArgumentSide.tick())' "$APP" "fixture must contain nested call argument"
guard_expect_fixed_in_file "$TAG" 'output_contract=generic-nested-argument-single-eval-fixture-v0' "$CARD_127" "row127 must record output contract"
guard_expect_fixed_in_file "$TAG" 'expected_nested_call_count=1' "$CARD_127" "row127 must record expected count"
guard_expect_fixed_in_file "$TAG" 'actual_nested_call_count=2' "$CARD_127" "row127 must record current bug"
guard_expect_fixed_in_file "$TAG" 'selected_next=mir_builder_nested_argument_single_eval_owner_fix' "$CARD_127" "row127 must select fix"
guard_expect_fixed_in_file "$TAG" 'Nested call arguments must be evaluated exactly once.' "$SSOT" "SSOT must define invariant"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-127-GENERIC-NESTED-ARGUMENT-SINGLE-EVAL-FIXTURE"' "$CURRENT_STATE" "current state latest card must advance to row127"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX-296X-001"' "$CURRENT_STATE" "current state must select row128"
guard_expect_fixed_in_file "$TAG" '| 127 | `GENERIC-NESTED-ARGUMENT-SINGLE-EVAL-FIXTURE-296X-001` | Landed |' "$TASKBOARD" "taskboard row127 must be landed"
guard_expect_fixed_in_file "$TAG" '| 128 | `MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX-296X-001` | Current |' "$TASKBOARD" "taskboard row128 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_generic_nested_arg_fixture.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/sample.mir.json" <<'JSON'
{
  "functions": [
    {
      "name": "NestedArgumentProbe.run/0",
      "blocks": [
        {
          "instructions": [
            {"op": "mir_call", "dst": 1, "mir_call": {"callee": {"name": "NestedArgumentSide.tick/0"}, "args": []}},
            {"op": "mir_call", "dst": 2, "mir_call": {"callee": {"name": "NestedArgumentSide.tick/0"}, "args": []}}
          ]
        }
      ]
    }
  ]
}
JSON

report="$tmp_dir/report.out"
python3 "$TOOL" --mir-json "$tmp_dir/sample.mir.json" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=generic-nested-argument-single-eval-fixture-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'nested_call_symbol=NestedArgumentSide.tick/0' "$report" "tool must name nested symbol"
guard_expect_fixed_in_file "$TAG" 'expected_nested_call_count=1' "$report" "tool must record expected count"
guard_expect_fixed_in_file "$TAG" 'actual_nested_call_count=2' "$report" "tool must count current bug"
guard_expect_fixed_in_file "$TAG" 'selected_next=mir_builder_nested_argument_single_eval_owner_fix' "$report" "tool must select fix"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
