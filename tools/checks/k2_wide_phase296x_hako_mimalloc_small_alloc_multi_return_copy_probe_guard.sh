#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-small-alloc-multi-return-copy-probe"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_119="docs/development/current/main/phases/phase-296x/296x-119-HAKO-MIMALLOC-SMALL-ALLOC-MULTI-RETURN-COPY-PROBE.md"
CARD_120="docs/development/current/main/phases/phase-296x/296x-120-HAKO-MIMALLOC-SMALL-ALLOC-RETURN-BLOCK-LOCAL-SSA-COPY-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_small_alloc_multi_return_copy_probe.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_multi_return_copy_probe_guard.sh"

echo "[$TAG] checking small-alloc multi-return copy probe"

guard_require_files "$TAG" "$CARD_119" "$CARD_120" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_119" "row119 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_120" "row120 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-multi-return-copy-probe-v0' "$CARD_119" "row119 must record output contract"
guard_expect_fixed_in_file "$TAG" 'copy_to_return_value_count=0' "$CARD_119" "row119 must reject return-value copy ownership"
guard_expect_fixed_in_file "$TAG" 'next_action=local_ssa_copy_probe' "$CARD_119" "row119 must select local SSA probe"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-119-HAKO-MIMALLOC-SMALL-ALLOC-MULTI-RETURN-COPY-PROBE"' "$CURRENT_STATE" "current state latest card must advance to row119"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-MIMALLOC-SMALL-ALLOC-RETURN-BLOCK-LOCAL-SSA-COPY-PROBE-296X-001"' "$CURRENT_STATE" "current state must select row120"
guard_expect_fixed_in_file "$TAG" '| 119 | `HAKO-MIMALLOC-SMALL-ALLOC-MULTI-RETURN-COPY-PROBE-296X-001` | Landed |' "$TASKBOARD" "taskboard row119 must be landed"
guard_expect_fixed_in_file "$TAG" '| 120 | `HAKO-MIMALLOC-SMALL-ALLOC-RETURN-BLOCK-LOCAL-SSA-COPY-PROBE-296X-001` | Current |' "$TASKBOARD" "taskboard row120 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_multi_return_copy_probe.XXXXXX)"
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
            {"op": "phi", "dst": 10, "incoming": [[1, 0], [2, 2]]},
            {"op": "copy", "dst": 20, "src": 10},
            {"op": "copy", "dst": 21, "src": 20},
            {"op": "mir_call", "dst": 22, "mir_call": {"callee": {"name": "recordSmallAllocFailure"}}},
            {"op": "ret", "value": 22}
          ]
        },
        {
          "id": 2,
          "instructions": [
            {"op": "copy", "dst": 30, "src": 10},
            {"op": "mir_call", "dst": 31, "mir_call": {"callee": {"name": "recordSmallAllocSuccess"}}},
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

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-multi-return-copy-probe-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'return_count=2' "$report" "tool must count return blocks"
guard_expect_fixed_in_file "$TAG" 'copy_count=3' "$report" "tool must count copies"
guard_expect_fixed_in_file "$TAG" 'copy_from_phi_count=2' "$report" "tool must count copy from phi"
guard_expect_fixed_in_file "$TAG" 'return_block_copy_count=3' "$report" "tool must count return-block copies"
guard_expect_fixed_in_file "$TAG" 'copy_to_return_value_count=0' "$report" "tool must reject return-value copy"
guard_expect_fixed_in_file "$TAG" 'next_action=local_ssa_copy_probe' "$report" "tool must select local SSA probe"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
