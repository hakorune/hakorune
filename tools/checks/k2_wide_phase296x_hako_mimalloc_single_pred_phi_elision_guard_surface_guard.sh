#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-single-pred-phi-elision-guard-surface"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_117="docs/development/current/main/phases/phase-296x/296x-117-HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-GUARD-SURFACE.md"
CARD_118="docs/development/current/main/phases/phase-296x/296x-118-HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_single_pred_phi_elision_guard_surface.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_single_pred_phi_elision_guard_surface_guard.sh"

echo "[$TAG] checking single-pred PHI elision guard surface"

guard_require_files "$TAG" "$CARD_117" "$CARD_118" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_117" "row117 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_118" "row118 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-single-pred-phi-elision-guard-surface-v0' "$CARD_117" "row117 must record output contract"
guard_expect_fixed_in_file "$TAG" 'required_before_value=61' "$CARD_117" "row117 must record before metric"
guard_expect_fixed_in_file "$TAG" 'required_after_max=15' "$CARD_117" "row117 must record after bound"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-117-HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-GUARD-SURFACE"' "$CURRENT_STATE" "current state latest card must advance to row117"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION-296X-001"' "$CURRENT_STATE" "current state must select row118"
guard_expect_fixed_in_file "$TAG" '| 117 | `HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-GUARD-SURFACE-296X-001` | Landed |' "$TASKBOARD" "taskboard row117 must be landed"
guard_expect_fixed_in_file "$TAG" '| 118 | `HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION-296X-001` | Current |' "$TASKBOARD" "taskboard row118 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_single_pred_phi_surface.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-single-pred-phi-elision-guard-surface-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'guard_surface=single_pred_phi_elision' "$report" "tool must name guard surface"
guard_expect_fixed_in_file "$TAG" 'required_before_metric=single_incoming_phi_count' "$report" "tool must record before metric"
guard_expect_fixed_in_file "$TAG" 'required_after_metric=single_incoming_phi_count' "$report" "tool must record after metric"
guard_expect_fixed_in_file "$TAG" 'semantic_guard=current_state_pointer_guard' "$report" "tool must record semantic guard"
guard_expect_fixed_in_file "$TAG" 'perf_guard=object_lifecycle_exact_exe_measurement' "$report" "tool must record perf guard"
guard_expect_fixed_in_file "$TAG" 'next_action=implement_guarded_elision' "$report" "tool must select implementation"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
