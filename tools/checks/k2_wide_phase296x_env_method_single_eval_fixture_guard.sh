#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-env-method-single-eval-fixture"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_131="docs/development/current/main/phases/phase-296x/296x-131-MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-FIXTURE.md"
CARD_132="docs/development/current/main/phases/phase-296x/296x-132-MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-OWNER-FIX.md"
APP="apps/mir-env-method-single-eval-proof/main.hako"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/env_method_single_eval_fixture.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_env_method_single_eval_fixture_guard.sh"

echo "[$TAG] checking env method single-eval fixture"

guard_require_files "$TAG" "$CARD_131" "$CARD_132" "$APP" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_131" "row131 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_132" "row132 card must be current"
guard_expect_fixed_in_file "$TAG" 'return env.unknown.missing(EnvArgSide.tick())' "$APP" "fixture must contain unsupported env fallback shape"
guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-env-method-single-eval-fixture-v0' "$CARD_131" "row131 must record output contract"
guard_expect_fixed_in_file "$TAG" 'expected_nested_call_count=1' "$CARD_131" "row131 must record expected count"
guard_expect_fixed_in_file "$TAG" 'actual_nested_call_count=2' "$CARD_131" "row131 must record current bug"
guard_expect_fixed_in_file "$TAG" 'selected_next=mir_builder_env_method_single_eval_owner_fix' "$CARD_131" "row131 must select owner fix"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-131-MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-FIXTURE"' "$CURRENT_STATE" "current state latest card must advance to row131"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-OWNER-FIX-296X-001"' "$CURRENT_STATE" "current state must select row132"
guard_expect_fixed_in_file "$TAG" '| 131 | `MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-FIXTURE-296X-001` | Landed |' "$TASKBOARD" "taskboard row131 must be landed"
guard_expect_fixed_in_file "$TAG" '| 132 | `MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-OWNER-FIX-296X-001` | Current |' "$TASKBOARD" "taskboard row132 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_env_method_fixture.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/sample.mir.json" <<'JSON'
{
  "functions": [
    {
      "name": "EnvMethodProbe.run/0",
      "blocks": [
        {
          "instructions": [
            {"op": "mir_call", "dst": 1, "mir_call": {"callee": {"name": "EnvArgSide.tick/0"}, "args": []}},
            {"op": "mir_call", "dst": 2, "mir_call": {"callee": {"name": "EnvArgSide.tick/0"}, "args": []}}
          ]
        }
      ]
    }
  ]
}
JSON

report="$tmp_dir/report.out"
python3 "$TOOL" --mir-json "$tmp_dir/sample.mir.json" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-env-method-single-eval-fixture-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'nested_call_symbol=EnvArgSide.tick/0' "$report" "tool must name nested symbol"
guard_expect_fixed_in_file "$TAG" 'expected_nested_call_count=1' "$report" "tool must record expected count"
guard_expect_fixed_in_file "$TAG" 'actual_nested_call_count=2' "$report" "tool must count current bug"
guard_expect_fixed_in_file "$TAG" 'selected_next=mir_builder_env_method_single_eval_owner_fix' "$report" "tool must select fix"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
