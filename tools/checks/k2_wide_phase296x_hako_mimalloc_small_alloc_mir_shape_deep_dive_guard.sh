#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-small-alloc-mir-shape-deep-dive"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_114="docs/development/current/main/phases/phase-296x/296x-114-HAKO-MIMALLOC-SMALL-ALLOC-MIR-SHAPE-DEEP-DIVE.md"
CARD_115="docs/development/current/main/phases/phase-296x/296x-115-HAKO-MIMALLOC-SMALL-ALLOC-PHI-COPY-LOWERING-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_small_alloc_mir_shape_deep_dive.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_mir_shape_deep_dive_guard.sh"

echo "[$TAG] checking small-alloc MIR shape deep dive"

guard_require_files "$TAG" "$CARD_114" "$CARD_115" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_114" "row114 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_115" "row115 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-mir-shape-deep-dive-v0' "$CARD_114" "row114 must record output contract"
guard_expect_fixed_in_file "$TAG" 'dominant_shape_owner=phi_copy' "$CARD_114" "row114 must classify dominant shape"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=small_alloc_phi_copy_lowering_probe' "$CARD_114" "row114 must select lowering probe"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-114-HAKO-MIMALLOC-SMALL-ALLOC-MIR-SHAPE-DEEP-DIVE"' "$CURRENT_STATE" "current state latest card must advance to row114"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SMALL-ALLOC-PHI-COPY-LOWERING-PROBE-296X-001"' "$CURRENT_STATE" "current state must select row115"
guard_expect_fixed_in_file "$TAG" '| 114 | `HAKO-MIMALLOC-SMALL-ALLOC-MIR-SHAPE-DEEP-DIVE-296X-001` | Landed |' "$TASKBOARD" "taskboard row114 must be landed"
guard_expect_fixed_in_file "$TAG" '| 115 | `HAKO-MIMALLOC-SMALL-ALLOC-PHI-COPY-LOWERING-PROBE-296X-001` | Current |' "$TASKBOARD" "taskboard row115 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_small_alloc_mir_deep_dive.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/sample.mir.json" <<'JSON'
{
  "functions": [
    {
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
      "blocks": [
        {
          "instructions": [
            {"op": "mir_call", "mir_call": {"callee": {"name": "recordSmallAllocFailure"}}},
            {"op": "field_get", "field": "alloc_result"},
            {"op": "field_set", "field": "alloc_result"},
            {"op": "phi"},
            {"op": "phi"},
            {"op": "copy"},
            {"op": "copy"},
            {"op": "copy"},
            {"op": "branch"},
            {"op": "ret"}
          ]
        }
      ]
    }
  ]
}
JSON

report="$tmp_dir/report.out"
python3 "$TOOL" --mir-json "$tmp_dir/sample.mir.json" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-mir-shape-deep-dive-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' "$report" "tool must select owner"
guard_expect_fixed_in_file "$TAG" 'mir_instruction_count=10' "$report" "tool must count instructions"
guard_expect_fixed_in_file "$TAG" 'mir_call_count=1' "$report" "tool must count calls"
guard_expect_fixed_in_file "$TAG" 'mir_field_access_count=2' "$report" "tool must count field access"
guard_expect_fixed_in_file "$TAG" 'mir_phi_count=2' "$report" "tool must count phi"
guard_expect_fixed_in_file "$TAG" 'mir_copy_count=3' "$report" "tool must count copy"
guard_expect_fixed_in_file "$TAG" 'dominant_shape_owner=phi_copy' "$report" "tool must classify phi/copy dominance"
guard_expect_fixed_in_file "$TAG" 'next_action=mir_lowering_probe' "$report" "tool must select lowering probe"
guard_expect_fixed_in_file "$TAG" 'top_callee_0=recordSmallAllocFailure' "$report" "tool must report callee"
guard_expect_fixed_in_file "$TAG" 'top_field_0=alloc_result' "$report" "tool must report field"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
