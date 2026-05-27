#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-page-array-dynamic-weight-probe"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_146="docs/development/current/main/phases/phase-296x/296x-146-PAGE-ARRAY-DYNAMIC-WEIGHT-PROBE.md"
CARD_147="docs/development/current/main/phases/phase-296x/296x-147-PAGE-ARRAY-KEEPER-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/page_array_dynamic_weight_probe.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_page_array_dynamic_weight_probe_guard.sh"

echo "[$TAG] checking page-array dynamic weight probe"

guard_require_files "$TAG" "$CARD_146" "$CARD_147" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_146" "row146 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_147" "row147 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=page-array-dynamic-weight-probe-v0' "$CARD_146" "row146 must record output contract"
guard_expect_fixed_in_file "$TAG" 'total_array_weight=4194304' "$CARD_146" "row146 must record total page-array weight"
guard_expect_fixed_in_file "$TAG" 'dynamic_owner=allocator_page_array_surface' "$CARD_146" "row146 must select page-array owner"
guard_expect_fixed_in_file "$TAG" 'selected_next=page_array_keeper_selection' "$CARD_146" "row146 must select keeper selection"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-146-PAGE-ARRAY-DYNAMIC-WEIGHT-PROBE"' "$CURRENT_STATE" "current state latest card must advance to row146"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "PAGE-ARRAY-KEEPER-SELECTION-296X-001"' "$CURRENT_STATE" "current state must select row147"
guard_expect_fixed_in_file "$TAG" '| 146 | `PAGE-ARRAY-DYNAMIC-WEIGHT-PROBE-296X-001` | Landed |' "$TASKBOARD" "taskboard row146 must be landed"
guard_expect_fixed_in_file "$TAG" '| 147 | `PAGE-ARRAY-KEEPER-SELECTION-296X-001` | Current |' "$TASKBOARD" "taskboard row147 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_page_array_weight.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
"$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=page-array-dynamic-weight-probe-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'operation_repeat=8192' "$report" "tool must record operation repeat"
guard_expect_fixed_in_file "$TAG" 'alloc_count=524288' "$report" "tool must record alloc count"
guard_expect_fixed_in_file "$TAG" 'release_count=524288' "$report" "tool must record release count"
guard_expect_fixed_in_file "$TAG" 'reset_array_set_weight=1572864' "$report" "tool must record reset set weight"
guard_expect_fixed_in_file "$TAG" 'total_array_weight=4194304' "$report" "tool must record total weight"
guard_expect_fixed_in_file "$TAG" 'dynamic_owner=allocator_page_array_surface' "$report" "tool must select page-array owner"
guard_expect_fixed_in_file "$TAG" 'selected_next=page_array_keeper_selection' "$report" "tool must select keeper selection"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
