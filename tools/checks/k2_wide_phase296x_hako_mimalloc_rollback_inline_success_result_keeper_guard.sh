#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-rollback-inline-success-result-keeper"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_111="docs/development/current/main/phases/phase-296x/296x-111-HAKO-MIMALLOC-ROLLBACK-INLINE-SUCCESS-RESULT-KEEPER.md"
CARD_112="docs/development/current/main/phases/phase-296x/296x-112-HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-RESULT-MEASUREMENT.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_rollback_inline_success_result_keeper.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_rollback_inline_success_result_keeper_guard.sh"

echo "[$TAG] checking inline success result rollback"

guard_require_files "$TAG" "$CARD_111" "$CARD_112" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_111" "row111 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_112" "row112 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-rollback-inline-success-result-keeper-v0' "$CARD_111" "row111 must record output contract"
guard_expect_fixed_in_file "$TAG" 'inline_success_result_present=0' "$CARD_111" "row111 must remove inline success"
guard_expect_fixed_in_file "$TAG" 'small_alloc_direct_select_preserved=1' "$CARD_111" "row111 must preserve direct select"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-111-HAKO-MIMALLOC-ROLLBACK-INLINE-SUCCESS-RESULT-KEEPER"' "$CURRENT_STATE" "current state latest card must advance to row111"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-RESULT-MEASUREMENT-296X-001"' "$CURRENT_STATE" "current state must select row112"
guard_expect_fixed_in_file "$TAG" '| 111 | `HAKO-MIMALLOC-ROLLBACK-INLINE-SUCCESS-RESULT-KEEPER-296X-001` | Landed |' "$TASKBOARD" "taskboard row111 must be landed"
guard_expect_fixed_in_file "$TAG" '| 112 | `HAKO-MIMALLOC-POST-ROLLBACK-INLINE-SUCCESS-RESULT-MEASUREMENT-296X-001` | Current |' "$TASKBOARD" "taskboard row112 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_rollback_inline_success.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-rollback-inline-success-result-keeper-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'rolled_back_keeper=small_alloc_inline_success_result_fast_path' "$report" "tool must record rollback"
guard_expect_fixed_in_file "$TAG" 'inline_success_result_present=0' "$report" "tool must prove removal"
guard_expect_fixed_in_file "$TAG" 'small_alloc_direct_select_preserved=1' "$report" "tool must preserve direct select"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
