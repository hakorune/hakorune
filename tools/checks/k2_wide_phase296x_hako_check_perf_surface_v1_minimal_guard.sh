#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-check-perf-surface-v1-minimal"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_83="docs/development/current/main/phases/phase-296x/296x-83-HAKO-CHECK-PERF-SURFACE-V1-MINIMAL.md"
CARD_84="docs/development/current/main/phases/phase-296x/296x-84-HAKO-MIMALLOC-KEEPER-BEFORE-AFTER-DIFF-ADAPTER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
README="tools/hako_check/README.md"
TOOL="tools/hako_check/perf_surface_inventory.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_check_perf_surface_v1_minimal_guard.sh"
QUEUE="lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako"

echo "[$TAG] checking hako_check perf-surface v1 minimal"

guard_require_files "$TAG" "$CARD_83" "$CARD_84" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$README" "$TOOL" "$SELF_SCRIPT" "$QUEUE"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_83" "v1 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_84" "diff card must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-perf-surface-v1' "$CARD_83" "card must record v1 contract"
guard_expect_fixed_in_file "$TAG" 'loop_field_get_count' "$README" "README must document loop field count"
guard_expect_fixed_in_file "$TAG" 'allocation_like_in_loop_count' "$TOOL" "tool must count allocation-like loops"

guard_expect_fixed_in_file "$TAG" 'latest_card = ' "$CURRENT_STATE" "current state must keep a latest-card pointer"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = ' "$CURRENT_STATE" "current state must keep a blocker pointer"
guard_expect_fixed_in_file "$TAG" '| 83 | `HAKO-CHECK-PERF-SURFACE-V1-MINIMAL-296X-001` | Landed |' "$TASKBOARD" "taskboard row 83 must be landed"
guard_expect_fixed_in_file "$TAG" '| 84 | `HAKO-MIMALLOC-KEEPER-BEFORE-AFTER-DIFF-ADAPTER-296X-001` | Landed |' "$TASKBOARD" "taskboard row 84 must be landed"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hako_check_v1.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --contract-version v1 --target "$QUEUE" --target-box HakoAllocObjectLifecyclePageQueue --methods selectPage --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-check-perf-surface-v1' "$report" "tool must emit v1 contract"
guard_expect_fixed_in_file "$TAG" 'target_method_0=selectPage' "$report" "tool must inspect selectPage"
guard_expect_fixed_in_file "$TAG" 'target_method=selectPage' "$report" "tool must select selectPage"
guard_expect_fixed_in_file "$TAG" 'loop_field_get_count=' "$report" "tool must emit loop field get count"
guard_expect_fixed_in_file "$TAG" 'loop_field_set_count=' "$report" "tool must emit loop field set count"
guard_expect_fixed_in_file "$TAG" 'loop_array_get_count=' "$report" "tool must emit loop array get count"
guard_expect_fixed_in_file "$TAG" 'loop_array_length_count=' "$report" "tool must emit loop array length count"
guard_expect_fixed_in_file "$TAG" 'allocation_like_in_loop_count=' "$report" "tool must emit allocation-like count"
guard_expect_fixed_in_file "$TAG" 'suggested_next_kind=' "$report" "tool must emit suggested kind"
guard_expect_fixed_in_file "$TAG" 'confidence=' "$report" "tool must emit confidence"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
