#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-mir-builder-single-eval-surface-sweep"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_134="docs/development/current/main/phases/phase-296x/296x-134-MIR-BUILDER-SINGLE-EVAL-SURFACE-SWEEP.md"
CARD_135="docs/development/current/main/phases/phase-296x/296x-135-STATIC-SCALAR-METHOD-FACT-SELECTION.md"
APP="apps/mir-single-eval-surface-sweep/main.hako"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/mir_builder_single_eval_surface_sweep.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_builder_single_eval_surface_sweep_guard.sh"

echo "[$TAG] checking MIR builder single-eval surface sweep"

guard_require_files "$TAG" "$CARD_134" "$CARD_135" "$APP" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_134" "row134 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_135" "row135 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-single-eval-surface-sweep-v0' "$CARD_134" "row134 must record output contract"
guard_expect_fixed_in_file "$TAG" 'surface_count=6' "$CARD_134" "row134 must record surface count"
guard_expect_fixed_in_file "$TAG" 'failing_surface_count=0' "$CARD_134" "row134 must record no failures"
guard_expect_fixed_in_file "$TAG" 'selected_next=static_scalar_method_fact_selection' "$CARD_134" "row134 must select static scalar fact selection"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-134-MIR-BUILDER-SINGLE-EVAL-SURFACE-SWEEP"' "$CURRENT_STATE" "current state latest card must advance to row134"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "STATIC-SCALAR-METHOD-FACT-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select row135"
guard_expect_fixed_in_file "$TAG" '| 134 | `MIR-BUILDER-SINGLE-EVAL-SURFACE-SWEEP-296X-001` | Landed |' "$TASKBOARD" "taskboard row134 must be landed"
guard_expect_fixed_in_file "$TAG" '| 135 | `STATIC-SCALAR-METHOD-FACT-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row135 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_single_eval_surface_sweep.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
target/release/hakorune --backend mir --emit-mir-json "$tmp_dir/sweep.mir.json" "$APP" >/dev/null
python3 "$TOOL" --mir-json "$tmp_dir/sweep.mir.json" --out "$tmp_dir/report.out"

guard_expect_fixed_in_file "$TAG" 'output_contract=mir-builder-single-eval-surface-sweep-v0' "$tmp_dir/report.out" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'surface_count=6' "$tmp_dir/report.out" "tool must record surface count"
guard_expect_fixed_in_file "$TAG" 'symbol_count=8' "$tmp_dir/report.out" "tool must record symbol count"
guard_expect_fixed_in_file "$TAG" 'failing_surface_count=0' "$tmp_dir/report.out" "tool must record no failures"
guard_expect_fixed_in_file "$TAG" 'selected_next=static_scalar_method_fact_selection' "$tmp_dir/report.out" "tool must select next row"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$tmp_dir/report.out" "tool must end ok"

echo "[$TAG] ok"
