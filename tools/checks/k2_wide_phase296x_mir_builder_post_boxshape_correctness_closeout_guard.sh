#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mir-builder-post-boxshape-correctness-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_145="docs/development/current/main/phases/phase-296x/296x-145-MIR-BUILDER-POST-BOXSHAPE-CORRECTNESS-CLOSEOUT.md"
CARD_146="docs/development/current/main/phases/phase-296x/296x-146-PAGE-ARRAY-DYNAMIC-WEIGHT-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/post_boxshape_correctness_closeout.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_builder_post_boxshape_correctness_closeout_guard.sh"

echo "[$TAG] checking post-BoxShape correctness closeout"

guard_require_files "$TAG" "$CARD_145" "$CARD_146" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_145" "row145 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_146" "row146 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-post-boxshape-correctness-closeout-v0' "$CARD_145" "row145 must record output contract"
guard_expect_fixed_in_file "$TAG" 'helper_copy_post_boxshape_status=changed' "$CARD_145" "row145 must record changed helper-copy status"
guard_expect_fixed_in_file "$TAG" 'post_boxshape_next=page_array_dynamic_weight_probe' "$CARD_145" "row145 must select page array dynamic weight probe"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-145-MIR-BUILDER-POST-BOXSHAPE-CORRECTNESS-CLOSEOUT"' "$CURRENT_STATE" "current state latest card must advance to row145"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "PAGE-ARRAY-DYNAMIC-WEIGHT-PROBE-296X-001"' "$CURRENT_STATE" "current state must select row146"
guard_expect_fixed_in_file "$TAG" '| 145 | `MIR-BUILDER-POST-BOXSHAPE-CORRECTNESS-CLOSEOUT-296X-001` | Landed |' "$TASKBOARD" "taskboard row145 must be landed"
guard_expect_fixed_in_file "$TAG" '| 146 | `PAGE-ARRAY-DYNAMIC-WEIGHT-PROBE-296X-001` | Current |' "$TASKBOARD" "taskboard row146 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

cargo build --release --bin hakorune >/dev/null

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_post_boxshape_closeout.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
"$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-post-boxshape-correctness-closeout-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'build_ok=1' "$report" "tool must record build ok"
guard_expect_fixed_in_file "$TAG" 'single_eval_surface_ok=1' "$report" "tool must record single eval ok"
guard_expect_fixed_in_file "$TAG" 'small_alloc_helper_copy_probe_ok=1' "$report" "tool must record helper probe ok"
guard_expect_fixed_in_file "$TAG" 'helper_copy_count=28' "$report" "tool must record helper copy count"
guard_expect_fixed_in_file "$TAG" 'helper_copy_post_boxshape_status=changed' "$report" "tool must classify changed helper-copy status"
guard_expect_fixed_in_file "$TAG" 'post_boxshape_next=page_array_dynamic_weight_probe' "$report" "tool must select page array dynamic weight probe"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
