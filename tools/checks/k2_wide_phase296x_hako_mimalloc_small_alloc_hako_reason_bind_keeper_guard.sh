#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-small-alloc-hako-reason-bind-keeper"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_123="docs/development/current/main/phases/phase-296x/296x-123-HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-KEEPER.md"
CARD_124="docs/development/current/main/phases/phase-296x/296x-124-HAKO-MIMALLOC-POST-HAKO-REASON-BIND-MEASUREMENT.md"
SOURCE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_small_alloc_hako_reason_bind_keeper.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_hako_reason_bind_keeper_guard.sh"

echo "[$TAG] checking small-alloc .hako reason bind keeper"

guard_require_files "$TAG" "$CARD_123" "$CARD_124" "$SOURCE" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_123" "row123 card must be landed"
guard_expect_fixed_in_file "$TAG" 'local small_no_page_reason_selected = HakoAllocObjectLifecycleFacadeReason.small_no_page()' "$SOURCE" "source must bind selected-index reason"
guard_expect_fixed_in_file "$TAG" 'local small_no_page_reason_page = HakoAllocObjectLifecycleFacadeReason.small_no_page()' "$SOURCE" "source must bind null-page reason"
guard_expect_fixed_in_file "$TAG" 'local small_bad_selected_kind_reason = HakoAllocObjectLifecycleFacadeReason.small_bad_selected_kind()' "$SOURCE" "source must bind kind reason"
guard_expect_fixed_in_file "$TAG" 'local small_reuse_failed_reason = HakoAllocObjectLifecycleFacadeReason.small_reuse_failed()' "$SOURCE" "source must bind reuse reason"
guard_expect_fixed_in_file "$TAG" 'local small_acquire_failed_reason = HakoAllocObjectLifecycleFacadeReason.small_acquire_failed()' "$SOURCE" "source must bind acquire reason"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-hako-reason-bind-keeper-v0' "$CARD_123" "row123 must record output contract"
guard_expect_fixed_in_file "$TAG" 'duplicate_reason_call_count=0' "$CARD_123" "row123 must record duplicate removal"
guard_expect_fixed_in_file "$TAG" 'semantic_summary=ok' "$CARD_123" "row123 must record semantic proof"
guard_expect_fixed_in_file "$TAG" '296x-123 Landed the .hako reason-local bind keeper, removing duplicate small-alloc failure reason calls while preserving exact-EXE proof counters.' "$CURRENT_STATE" "current state landed tail must include row123"
guard_expect_fixed_in_file "$TAG" '| 123 | `HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-KEEPER-296X-001` | Landed |' "$TASKBOARD" "taskboard row123 must be landed"
guard_expect_fixed_in_file "$TAG" '| 124 | `HAKO-MIMALLOC-POST-HAKO-REASON-BIND-MEASUREMENT-296X-001` | Landed |' "$TASKBOARD" "taskboard row124 must exist"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hako_reason_bind_keeper.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/shape.out" <<'REPORT'
output_contract=hako-mimalloc-small-alloc-duplicate-reason-call-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
reason_call_count=5
duplicate_reason_call_count=0
summary=ok
REPORT
cat > "$tmp_dir/proof.out" <<'REPORT'
output_contract=hako-exe-memory-evidence-v0
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
release_known_page_fast_path_count=524288
host_replacement=0
hook_installed=0
global_allocator_installed=0
output_summary_ok=1
summary=ok
REPORT

report="$tmp_dir/report.out"
python3 "$TOOL" --shape-report "$tmp_dir/shape.out" --proof-report "$tmp_dir/proof.out" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-hako-reason-bind-keeper-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'reason_call_count=5' "$report" "tool must preserve reason count"
guard_expect_fixed_in_file "$TAG" 'duplicate_reason_call_count=0' "$report" "tool must preserve duplicate removal"
guard_expect_fixed_in_file "$TAG" 'semantic_summary=ok' "$report" "tool must record semantics"
guard_expect_fixed_in_file "$TAG" 'next_action=post_hako_reason_bind_measurement' "$report" "tool must select measurement"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
