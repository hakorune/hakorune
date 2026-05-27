#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-single-incoming-phi-copy-elision-owner-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_116="docs/development/current/main/phases/phase-296x/296x-116-HAKO-MIMALLOC-SINGLE-INCOMING-PHI-COPY-ELISION-OWNER-SELECTION.md"
CARD_117="docs/development/current/main/phases/phase-296x/296x-117-HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-GUARD-SURFACE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_single_incoming_phi_copy_elision_owner_selection.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_single_incoming_phi_copy_elision_owner_selection_guard.sh"

echo "[$TAG] checking single-incoming phi/copy elision owner selection"

guard_require_files "$TAG" "$CARD_116" "$CARD_117" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_116" "row116 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_117" "row117 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-single-incoming-phi-copy-elision-owner-selection-v0' "$CARD_116" "row116 must record output contract"
guard_expect_fixed_in_file "$TAG" 'selected_owner_file=src/mir/builder/emission/phi.rs' "$CARD_116" "row116 must select phi emission owner"
guard_expect_fixed_in_file "$TAG" 'supporting_copy_owner_file=src/mir/builder/ssa/local.rs' "$CARD_116" "row116 must record supporting copy owner"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-116-HAKO-MIMALLOC-SINGLE-INCOMING-PHI-COPY-ELISION-OWNER-SELECTION"' "$CURRENT_STATE" "current state latest card must advance to row116"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-GUARD-SURFACE-296X-001"' "$CURRENT_STATE" "current state must select row117"
guard_expect_fixed_in_file "$TAG" '| 116 | `HAKO-MIMALLOC-SINGLE-INCOMING-PHI-COPY-ELISION-OWNER-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row116 must be landed"
guard_expect_fixed_in_file "$TAG" '| 117 | `HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-GUARD-SURFACE-296X-001` | Current |' "$TASKBOARD" "taskboard row117 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_phi_copy_owner_selection.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-single-incoming-phi-copy-elision-owner-selection-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'selected_owner_file=src/mir/builder/emission/phi.rs' "$report" "tool must select owner file"
guard_expect_fixed_in_file "$TAG" 'selected_owner_module=crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry' "$report" "tool must select owner module"
guard_expect_fixed_in_file "$TAG" 'supporting_phi_helper=MirBuilder::insert_phi_single' "$report" "tool must record phi helper"
guard_expect_fixed_in_file "$TAG" 'supporting_copy_owner=crate::mir::builder::ssa::local::ensure' "$report" "tool must record copy owner"
guard_expect_fixed_in_file "$TAG" 'candidate_change_kind=mirbuilder_elision' "$report" "tool must classify change kind"
guard_expect_fixed_in_file "$TAG" 'next_action=probe_owner' "$report" "tool must select owner probe"
guard_expect_fixed_in_file "$TAG" 'next_diagnostic=single_pred_phi_elision_guard_surface' "$report" "tool must select guard surface"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
