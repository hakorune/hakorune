#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-small-alloc-hako-reason-bind-probe"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_122="docs/development/current/main/phases/phase-296x/296x-122-HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-PROBE.md"
CARD_123="docs/development/current/main/phases/phase-296x/296x-123-HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-KEEPER.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_small_alloc_hako_reason_bind_probe.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_hako_reason_bind_probe_guard.sh"

echo "[$TAG] checking small-alloc .hako reason bind probe"

guard_require_files "$TAG" "$CARD_122" "$CARD_123" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_122" "row122 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_123" "row123 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-hako-reason-bind-probe-v0' "$CARD_122" "row122 must record output contract"
guard_expect_fixed_in_file "$TAG" 'before_reason_call_count=10' "$CARD_122" "row122 must record before reason count"
guard_expect_fixed_in_file "$TAG" 'after_reason_call_count=5' "$CARD_122" "row122 must record after reason count"
guard_expect_fixed_in_file "$TAG" 'after_duplicate_reason_call_count=0' "$CARD_122" "row122 must record duplicate removal"
guard_expect_fixed_in_file "$TAG" 'next_action=apply_hako_reason_bind_keeper' "$CARD_122" "row122 must select keeper"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-122-HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-PROBE"' "$CURRENT_STATE" "current state latest card must advance to row122"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-KEEPER-296X-001"' "$CURRENT_STATE" "current state must select row123"
guard_expect_fixed_in_file "$TAG" '| 122 | `HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-PROBE-296X-001` | Landed |' "$TASKBOARD" "taskboard row122 must be landed"
guard_expect_fixed_in_file "$TAG" '| 123 | `HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-KEEPER-296X-001` | Current |' "$TASKBOARD" "taskboard row123 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_hako_reason_bind_probe.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/before.out" <<'REPORT'
output_contract=hako-mimalloc-small-alloc-duplicate-reason-call-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
reason_call_count=10
duplicate_reason_call_count=5
summary=ok
REPORT
cat > "$tmp_dir/after.out" <<'REPORT'
output_contract=hako-mimalloc-small-alloc-duplicate-reason-call-probe-v0
selected_owner=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
reason_call_count=5
duplicate_reason_call_count=0
summary=ok
REPORT

report="$tmp_dir/report.out"
python3 "$TOOL" --before-report "$tmp_dir/before.out" --after-report "$tmp_dir/after.out" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-hako-reason-bind-probe-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'before_reason_call_count=10' "$report" "tool must read before count"
guard_expect_fixed_in_file "$TAG" 'after_reason_call_count=5' "$report" "tool must read after count"
guard_expect_fixed_in_file "$TAG" 'after_duplicate_reason_call_count=0' "$report" "tool must read duplicate removal"
guard_expect_fixed_in_file "$TAG" 'reason_call_delta=-5' "$report" "tool must compute reason delta"
guard_expect_fixed_in_file "$TAG" 'next_action=apply_hako_reason_bind_keeper' "$report" "tool must select keeper"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
