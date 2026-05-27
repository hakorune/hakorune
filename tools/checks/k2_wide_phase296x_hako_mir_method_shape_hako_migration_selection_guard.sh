#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mir-method-shape-hako-migration-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_87="docs/development/current/main/phases/phase-296x/296x-87-HAKO-MIR-METHOD-SHAPE-HAKO-MIGRATION-SELECTION.md"
CARD_88="docs/development/current/main/phases/phase-296x/296x-88-HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mir_method_shape_hako_migration_selection.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mir_method_shape_hako_migration_selection_guard.sh"

echo "[$TAG] checking MIR method shape .hako migration selection"

guard_require_files "$TAG" "$CARD_87" "$CARD_88" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_87" "migration selection card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_88" "multi-method observation card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mir-method-shape-hako-migration-selection-v0' "$CARD_87" "card must record output contract"
guard_expect_fixed_in_file "$TAG" 'hako_migration_decision=parked' "$CARD_87" "card must park migration"
guard_expect_fixed_in_file "$TAG" 'python_contract_stable=0' "$CARD_87" "card must keep python contract unstable"

guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-87-HAKO-MIR-METHOD-SHAPE-HAKO-MIGRATION-SELECTION"' "$CURRENT_STATE" "current state latest card must advance to row 87"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION-296X-001"' "$CURRENT_STATE" "current state must select row 88"
guard_expect_fixed_in_file "$TAG" '| 87 | `HAKO-MIR-METHOD-SHAPE-HAKO-MIGRATION-SELECTION-296X-001` | Landed |' "$TASKBOARD" "taskboard row 87 must be landed"
guard_expect_fixed_in_file "$TAG" '| 88 | `HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION-296X-001` | Current |' "$TASKBOARD" "taskboard row 88 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_mir_hako_migration.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mir-method-shape-hako-migration-selection-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'python_contract_stable=0' "$report" "tool must keep python contract unstable"
guard_expect_fixed_in_file "$TAG" 'hako_migration_decision=parked' "$report" "tool must park migration"
guard_expect_fixed_in_file "$TAG" 'selected_scope=python_adapter_continues_multi_method_observation' "$report" "tool must select python continuation"
guard_expect_fixed_in_file "$TAG" 'next_row=HAKO-MIMALLOC-MULTI-METHOD-SOURCE-MIR-OBSERVATION-296X-001' "$report" "tool must select row 88"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
