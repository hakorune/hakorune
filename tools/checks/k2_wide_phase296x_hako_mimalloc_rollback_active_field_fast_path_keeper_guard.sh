#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-rollback-active-field-fast-path-keeper"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_103="docs/development/current/main/phases/phase-296x/296x-103-HAKO-MIMALLOC-ROLLBACK-ACTIVE-FIELD-FAST-PATH-KEEPER.md"
CARD_104="docs/development/current/main/phases/phase-296x/296x-104-HAKO-MIMALLOC-POST-ROLLBACK-ACTIVE-FIELD-FAST-PATH-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_rollback_active_field_fast_path_keeper.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_rollback_active_field_fast_path_keeper_guard.sh"

echo "[$TAG] checking active field fast path rollback"

guard_require_files "$TAG" "$CARD_103" "$CARD_104" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_103" "row103 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_104" "row104 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-rollback-active-field-fast-path-keeper-v0' "$CARD_103" "row103 must record output contract"
guard_expect_fixed_in_file "$TAG" 'active_field_fast_path_present=0' "$CARD_103" "row103 must remove active field fast path"
guard_expect_fixed_in_file "$TAG" 'first_page_cache_preserved=1' "$CARD_103" "row103 must preserve first-page cache"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-103-HAKO-MIMALLOC-ROLLBACK-ACTIVE-FIELD-FAST-PATH-KEEPER"' "$CURRENT_STATE" "current state latest card must advance to row103"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-POST-ROLLBACK-ACTIVE-FIELD-FAST-PATH-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row104"
guard_expect_fixed_in_file "$TAG" '| 103 | `HAKO-MIMALLOC-ROLLBACK-ACTIVE-FIELD-FAST-PATH-KEEPER-296X-001` | Landed |' "$TASKBOARD" "taskboard row103 must be landed"
guard_expect_fixed_in_file "$TAG" '| 104 | `HAKO-MIMALLOC-POST-ROLLBACK-ACTIVE-FIELD-FAST-PATH-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row104 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_rollback_active_field.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-rollback-active-field-fast-path-keeper-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'rolled_back_keeper=select_single_page_active_field_fast_path' "$report" "tool must record rollback"
guard_expect_fixed_in_file "$TAG" 'active_field_fast_path_present=0' "$report" "tool must prove removal"
guard_expect_fixed_in_file "$TAG" 'first_page_cache_preserved=1' "$report" "tool must preserve first-page cache"
guard_expect_fixed_in_file "$TAG" 'generic_lifecycle_fallback_preserved=1' "$report" "tool must preserve fallback"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
