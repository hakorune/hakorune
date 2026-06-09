#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mir-builder-same-module-helper-call-lowering-seam"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_154="docs/development/current/main/phases/phase-296x/296x-154-POST-PAGE-ACQUIRE-USIZE-SOURCE-MIR-REFRESH.md"
CARD_155="docs/development/current/main/phases/phase-296x/296x-155-MIR-BUILDER-SAME-MODULE-HELPER-CALL-LOWERING-SEAM.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_small_alloc_helper_copy_family_probe.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_builder_same_module_helper_call_lowering_seam_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

echo "[$TAG] checking same-module helper call lowering seam"

guard_require_files "$TAG" "$CARD_154" "$CARD_155" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT" "$APP"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_154" "row154 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_155" "row155 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=same-module-helper-call-lowering-seam-v0' "$CARD_155" "row155 must record output contract"
guard_expect_fixed_in_file "$TAG" 'helper_call_count=5' "$CARD_155" "row155 must record helper calls"
guard_expect_fixed_in_file "$TAG" 'helper_copy_count=14' "$CARD_155" "row155 must record helper copies"
guard_expect_fixed_in_file "$TAG" 'receiver_copy_count=7' "$CARD_155" "row155 must record receiver copies"
guard_expect_fixed_in_file "$TAG" 'arg_copy_count=1' "$CARD_155" "row155 must record arg copies"
guard_expect_fixed_in_file "$TAG" 'result_copy_count=6' "$CARD_155" "row155 must record result copies"
guard_expect_fixed_in_file "$TAG" 'dominant_copy_family=helper_result_local_ssa' "$CARD_155" "row155 must classify dominant copy family"
guard_expect_fixed_in_file "$TAG" 'dominant_callee_family=facade_result_helpers' "$CARD_155" "row155 must classify dominant callee family"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-155-MIR-BUILDER-SAME-MODULE-HELPER-CALL-LOWERING-SEAM"' "$CURRENT_STATE" "current state latest card must advance to row155"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "MIR-BUILDER-SAME-MODULE-HELPER-CALL-LOWERING-SEAM-296X-001"' "$CURRENT_STATE" "current state must select row155"
guard_expect_fixed_in_file "$TAG" '| 154 | `POST-PAGE-ACQUIRE-USIZE-SOURCE-MIR-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row154 must be landed"
guard_expect_fixed_in_file "$TAG" '| 155 | `MIR-BUILDER-SAME-MODULE-HELPER-CALL-LOWERING-SEAM-296X-001` | Current |' "$TASKBOARD" "taskboard row155 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_same_module_helper_call_lowering_seam.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/release/hakorune \
  --backend mir \
  --emit-mir-json "$mir_json" \
  "$APP" >/dev/null
python3 "$TOOL" --mir-json "$mir_json" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-helper-copy-family-probe-v0' "$report" "probe must emit output contract"
guard_expect_fixed_in_file "$TAG" 'selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' "$report" "probe must select owner"
guard_expect_fixed_in_file "$TAG" 'helper_call_count=5' "$report" "probe must record helper calls"
guard_expect_fixed_in_file "$TAG" 'helper_copy_count=14' "$report" "probe must record helper copies"
guard_expect_fixed_in_file "$TAG" 'receiver_copy_count=7' "$report" "probe must record receiver copies"
guard_expect_fixed_in_file "$TAG" 'arg_copy_count=1' "$report" "probe must record arg copies"
guard_expect_fixed_in_file "$TAG" 'result_copy_count=6' "$report" "probe must record result copies"
guard_expect_fixed_in_file "$TAG" 'local_ssa_copy_count=77' "$report" "probe must record local SSA copies"
guard_expect_fixed_in_file "$TAG" 'dominant_copy_family=helper_result_local_ssa' "$report" "probe must classify dominant copy family"
guard_expect_fixed_in_file "$TAG" 'dominant_callee_family=facade_result_helpers' "$report" "probe must classify dominant callee family"
guard_expect_fixed_in_file "$TAG" 'selected_next=same_module_helper_call_lowering_seam' "$report" "probe must select compiler seam"
guard_expect_fixed_in_file "$TAG" 'top_helper_0=recordSmallAllocSuccess' "$report" "probe must record top helper"
guard_expect_fixed_in_file "$TAG" 'top_helper_1=selectSinglePageFastPath' "$report" "probe must record top helper"
guard_expect_fixed_in_file "$TAG" 'top_helper_2=selectPage' "$report" "probe must record top helper"
guard_expect_fixed_in_file "$TAG" 'top_helper_3=reuse' "$report" "probe must record top helper"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "probe must end ok"

echo "[$TAG] ok"
