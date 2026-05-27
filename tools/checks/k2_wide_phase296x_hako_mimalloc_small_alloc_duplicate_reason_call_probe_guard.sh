#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-small-alloc-duplicate-reason-call-probe"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_121="docs/development/current/main/phases/phase-296x/296x-121-HAKO-MIMALLOC-SMALL-ALLOC-DUPLICATE-REASON-CALL-PROBE.md"
CARD_122="docs/development/current/main/phases/phase-296x/296x-122-HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-PROBE.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_mimalloc_small_alloc_duplicate_reason_call_probe.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_small_alloc_duplicate_reason_call_probe_guard.sh"

echo "[$TAG] checking small-alloc duplicate reason call probe"

guard_require_files "$TAG" "$CARD_121" "$CARD_122" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_121" "row121 card must be landed"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-duplicate-reason-call-probe-v0' "$CARD_121" "row121 must record output contract"
guard_expect_fixed_in_file "$TAG" 'source_reason_call_count=5' "$CARD_121" "row121 must record source reason count"
guard_expect_fixed_in_file "$TAG" 'reason_call_count=10' "$CARD_121" "row121 must record MIR reason count"
guard_expect_fixed_in_file "$TAG" 'duplicate_unused_reason_call_count=5' "$CARD_121" "row121 must record unused duplicate count"
guard_expect_fixed_in_file "$TAG" 'next_action=hako_reason_bind_probe' "$CARD_121" "row121 must select .hako bind probe"
guard_expect_fixed_in_file "$TAG" '296x-121 Classified duplicate small-alloc failure reason calls as unused nested-call duplicates and selected a .hako reason-bind probe.' "$CURRENT_STATE" "current state landed tail must include row121"
guard_expect_fixed_in_file "$TAG" '| 121 | `HAKO-MIMALLOC-SMALL-ALLOC-DUPLICATE-REASON-CALL-PROBE-296X-001` | Landed |' "$TASKBOARD" "taskboard row121 must be landed"
guard_expect_fixed_in_file "$TAG" '| 122 | `HAKO-MIMALLOC-SMALL-ALLOC-HAKO-REASON-BIND-PROBE-296X-001` | Landed |' "$TASKBOARD" "taskboard row122 must exist"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_duplicate_reason_probe.XXXXXX)"
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
            {"op": "mir_call", "dst": 10, "mir_call": {"callee": {"name": "HakoAllocObjectLifecycleFacadeReason.small_no_page/0"}, "effects": ["IO"], "args": []}},
            {"op": "mir_call", "dst": 11, "mir_call": {"callee": {"name": "HakoAllocObjectLifecycleFacadeReason.small_no_page/0"}, "effects": ["IO"], "args": []}},
            {"op": "copy", "dst": 12, "src": 11},
            {"op": "mir_call", "dst": 13, "mir_call": {"callee": {"name": "recordSmallAllocFailure", "receiver": 99}, "args": [12]}},
            {"op": "ret", "value": 13}
          ]
        }
      ]
    }
  ]
}
JSON
cat > "$tmp_dir/source.hako" <<'HAKO'
box HakoAllocObjectLifecycleFacade {
    objectLifecycleSmallAlloc(size) {
        return me.recordSmallAllocFailure(HakoAllocObjectLifecycleFacadeReason.small_no_page())
    }
}
HAKO

report="$tmp_dir/report.out"
python3 "$TOOL" --mir-json "$tmp_dir/sample.mir.json" --source "$tmp_dir/source.hako" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-mimalloc-small-alloc-duplicate-reason-call-probe-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'source_reason_call_count=1' "$report" "tool must count source reason calls"
guard_expect_fixed_in_file "$TAG" 'reason_call_count=2' "$report" "tool must count MIR reason calls"
guard_expect_fixed_in_file "$TAG" 'reason_effect_io_count=2' "$report" "tool must count IO effects"
guard_expect_fixed_in_file "$TAG" 'duplicate_reason_call_count=1' "$report" "tool must count duplicate calls"
guard_expect_fixed_in_file "$TAG" 'duplicate_unused_reason_call_count=1' "$report" "tool must count unused duplicate calls"
guard_expect_fixed_in_file "$TAG" 'next_action=hako_reason_bind_probe' "$report" "tool must select .hako bind probe"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'replacement_active=0' "$report" "tool must keep replacement closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
