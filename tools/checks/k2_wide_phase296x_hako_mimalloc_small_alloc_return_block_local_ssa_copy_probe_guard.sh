#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-small-alloc-return-block-local-ssa-copy-probe"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_120="docs/development/current/main/phases/phase-296x/296x-120-HAKO-MIMALLOC-SMALL-ALLOC-RETURN-BLOCK-LOCAL-SSA-COPY-PROBE.md"
CARD_121="docs/development/current/main/phases/phase-296x/296x-121-HAKO-MIMALLOC-SMALL-ALLOC-DUPLICATE-REASON-CALL-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_small_alloc_return_block_local_ssa_copy_probe.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_return_block_local_ssa_copy_probe_guard.sh"

echo "[$TAG] checking small-alloc return-block local SSA copy probe"

guard_require_files "$TAG" "$CARD_120" "$CARD_121" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_120" "row120 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_121" "row121 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-return-block-local-ssa-copy-probe-v0' "$CARD_120" "row120 must record output contract"
guard_expect_fixed_in_file "$TAG" 'receiver_copy_count=7' "$CARD_120" "row120 must record receiver copy count"
guard_expect_fixed_in_file "$TAG" 'arg_copy_count=9' "$CARD_120" "row120 must record arg copy count"
guard_expect_fixed_in_file "$TAG" 'duplicate_reason_call_count=5' "$CARD_120" "row120 must record duplicate reason count"
guard_expect_fixed_in_file "$TAG" 'next_action=reason_call_probe' "$CARD_120" "row120 must select reason-call probe"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-120-HAKO-MIMALLOC-SMALL-ALLOC-RETURN-BLOCK-LOCAL-SSA-COPY-PROBE"' "$CURRENT_STATE" "current state latest card must advance to row120"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SMALL-ALLOC-DUPLICATE-REASON-CALL-PROBE-296X-001"' "$CURRENT_STATE" "current state must select row121"
guard_expect_fixed_in_file "$TAG" '| 120 | `HAKO-MIMALLOC-SMALL-ALLOC-RETURN-BLOCK-LOCAL-SSA-COPY-PROBE-296X-001` | Landed |' "$TASKBOARD" "taskboard row120 must be landed"
guard_expect_fixed_in_file "$TAG" '| 121 | `HAKO-MIMALLOC-SMALL-ALLOC-DUPLICATE-REASON-CALL-PROBE-296X-001` | Current |' "$TASKBOARD" "taskboard row121 must be current"
guard_expect_fixed_in_file "$TAG" 'HAKO-MIMALLOC-SMALL-ALLOC-DUPLICATE-REASON-CALL-PROBE-296X-001:' "$TASKBOARD" "taskboard current blocker must select row121"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_return_block_local_ssa_probe.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/sample.mir.json" <<'JSON'
{
  "functions": [
    {
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
      "blocks": [
        {
          "id": 1,
          "instructions": [
            {"op": "copy", "dst": 20, "src": 10},
            {"op": "copy", "dst": 21, "src": 11},
            {"op": "copy", "dst": 22, "src": 12},
            {"op": "mir_call", "dst": 23, "mir_call": {"callee": {"name": "HakoAllocObjectLifecycleFacadeReason.small_no_page/0"}, "args": []}},
            {"op": "mir_call", "dst": 24, "mir_call": {"callee": {"name": "HakoAllocObjectLifecycleFacadeReason.small_no_page/0"}, "args": []}},
            {"op": "mir_call", "dst": 25, "mir_call": {"callee": {"name": "recordSmallAllocFailure", "receiver": 20}, "args": [21, 22]}},
            {"op": "ret", "value": 25}
          ]
        },
        {
          "id": 2,
          "instructions": [
            {"op": "copy", "dst": 30, "src": 13},
            {"op": "mir_call", "dst": 31, "mir_call": {"callee": {"name": "recordSmallAllocSuccess", "receiver": 30}, "args": []}},
            {"op": "ret", "value": 31}
          ]
        }
      ]
    }
  ]
}
JSON

report="$tmp_dir/report.out"
python3 "$TOOL" --mir-json "$tmp_dir/sample.mir.json" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-return-block-local-ssa-copy-probe-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'return_block_count=2' "$report" "tool must count return blocks"
guard_expect_fixed_in_file "$TAG" 'return_block_copy_count=4' "$report" "tool must count return-block copies"
guard_expect_fixed_in_file "$TAG" 'receiver_copy_count=2' "$report" "tool must count receiver copy uses"
guard_expect_fixed_in_file "$TAG" 'arg_copy_count=2' "$report" "tool must count arg copy uses"
guard_expect_fixed_in_file "$TAG" 'duplicate_reason_call_count=1' "$report" "tool must count duplicate reason calls"
guard_expect_fixed_in_file "$TAG" 'next_action=reason_call_probe' "$report" "tool must select reason-call probe"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
