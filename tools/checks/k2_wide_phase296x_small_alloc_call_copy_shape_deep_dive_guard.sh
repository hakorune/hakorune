#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-small-alloc-call-copy-shape-deep-dive"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_141="docs/development/current/main/phases/phase-296x/296x-141-SMALL-ALLOC-CALL-COPY-SHAPE-DEEP-DIVE.md"
CARD_142="docs/development/current/main/phases/phase-296x/296x-142-MIR-BUILDER-MEMBER-CALL-ROUTE-CLASSIFICATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_small_alloc_helper_copy_family_probe.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_small_alloc_call_copy_shape_deep_dive_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

echo "[$TAG] checking small-alloc call/copy shape deep dive"

guard_require_files "$TAG" "$CARD_141" "$CARD_142" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT" "$APP"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_141" "row141 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_142" "row142 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=small-alloc-call-copy-shape-deep-dive-v0' "$CARD_141" "row141 must record output contract"
guard_expect_fixed_in_file "$TAG" 'selected_next=same_module_helper_call_lowering_seam' "$CARD_141" "row141 must select helper lowering seam"
guard_expect_fixed_in_file "$TAG" 'source-level facade wrapper inline trial' "$CARD_141" "row141 must record inline non-keeper"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-141-SMALL-ALLOC-CALL-COPY-SHAPE-DEEP-DIVE"' "$CURRENT_STATE" "current state latest card must advance to row141"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIR-BUILDER-MEMBER-CALL-ROUTE-CLASSIFICATION-296X-001"' "$CURRENT_STATE" "current state must select row142"
guard_expect_fixed_in_file "$TAG" '| 141 | `SMALL-ALLOC-CALL-COPY-SHAPE-DEEP-DIVE-296X-001` | Landed |' "$TASKBOARD" "taskboard row141 must be landed"
guard_expect_fixed_in_file "$TAG" '| 142 | `MIR-BUILDER-MEMBER-CALL-ROUTE-CLASSIFICATION-296X-001` | Current |' "$TASKBOARD" "taskboard row142 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_small_alloc_copy_probe.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"
target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
"$TOOL" --mir-json "$mir_json" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-helper-copy-family-probe-v0' "$report" "probe must emit output contract"
guard_expect_fixed_in_file "$TAG" 'helper_call_count=16' "$report" "probe must record helper calls"
guard_expect_fixed_in_file "$TAG" 'helper_copy_count=62' "$report" "probe must record helper copies"
guard_expect_fixed_in_file "$TAG" 'receiver_copy_count=38' "$report" "probe must record receiver copies"
guard_expect_fixed_in_file "$TAG" 'arg_copy_count=15' "$report" "probe must record arg copies"
guard_expect_fixed_in_file "$TAG" 'result_copy_count=9' "$report" "probe must record result copies"
guard_expect_fixed_in_file "$TAG" 'local_ssa_copy_count=44' "$report" "probe must record local SSA copies"
guard_expect_fixed_in_file "$TAG" 'dominant_callee_family=facade_result_helpers' "$report" "probe must classify dominant callee family"
guard_expect_fixed_in_file "$TAG" 'selected_next=same_module_helper_call_lowering_seam' "$report" "probe must select compiler seam"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "probe must end ok"

echo "[$TAG] ok"
