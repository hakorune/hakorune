#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase296x-hako-mimalloc-post-hako-reason-bind-source-mir-refresh"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD_125="docs/development/current/main/phases/phase-296x/296x-125-HAKO-MIMALLOC-POST-HAKO-REASON-BIND-SOURCE-MIR-REFRESH.md"
CARD_126="docs/development/current/main/phases/phase-296x/296x-126-HAKO-ALLOC-FACADE-REASON-DUPLICATE-EVAL-GUARD.md"
SSOT="docs/development/current/main/design/nested-argument-single-evaluation-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
TOOL="tools/allocator/hako_alloc_facade_reason_duplicate_inventory.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_hako_mimalloc_post_hako_reason_bind_source_mir_refresh_guard.sh"

echo "[$TAG] checking post .hako reason bind source/MIR refresh"

guard_require_files "$TAG" "$CARD_125" "$CARD_126" "$SSOT" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$TOOL" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$TOOL" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD_125" "row125 card must be landed"
guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD_126" "row126 card must be current"
guard_expect_fixed_in_file "$TAG" 'output_contract=hako-alloc-facade-reason-duplicate-inventory-v0' "$CARD_125" "row125 must record inventory contract"
guard_expect_fixed_in_file "$TAG" 'method_0=objectLifecycleSmallAlloc' "$CARD_125" "row125 must keep fixed method order"
guard_expect_fixed_in_file "$TAG" 'method_0_unused_duplicate_reason_call_count=0' "$CARD_125" "small alloc must remain fixed"
guard_expect_fixed_in_file "$TAG" 'failing_method_count=7' "$CARD_125" "row125 must record remaining failures"
guard_expect_fixed_in_file "$TAG" 'total_unused_duplicate_reason_call_count=20' "$CARD_125" "row125 must record duplicate total"
guard_expect_fixed_in_file "$TAG" 'selected_next=hako_alloc_facade_reason_duplicate_eval_guard' "$CARD_125" "row125 must select guard"
guard_expect_fixed_in_file "$TAG" 'Nested call arguments must be evaluated exactly once.' "$SSOT" "SSOT must define invariant"
guard_expect_fixed_in_file "$TAG" 'This SSOT does not authorize generic MIR CSE.' "$SSOT" "SSOT must reject generic CSE"
guard_expect_fixed_in_file "$TAG" 'latest_card = "296x-125-HAKO-MIMALLOC-POST-HAKO-REASON-BIND-SOURCE-MIR-REFRESH"' "$CURRENT_STATE" "current state latest card must advance to row125"
guard_expect_fixed_in_file "$TAG" 'current_blocker_token = "HAKO-ALLOC-FACADE-REASON-DUPLICATE-EVAL-GUARD-296X-001"' "$CURRENT_STATE" "current state must select row126"
guard_expect_fixed_in_file "$TAG" '| 125 | `HAKO-MIMALLOC-POST-HAKO-REASON-BIND-SOURCE-MIR-REFRESH-296X-001` | Landed |' "$TASKBOARD" "taskboard row125 must be landed"
guard_expect_fixed_in_file "$TAG" '| 126 | `HAKO-ALLOC-FACADE-REASON-DUPLICATE-EVAL-GUARD-296X-001` | Current |' "$TASKBOARD" "taskboard row126 must be current"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this guard"
guard_expect_fixed_in_file "$TAG" "$TOOL" "$INDEX" "check index must list this tool"

tmp_dir="$(mktemp -d /tmp/hakorune_phase296x_reason_duplicate_inventory.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
cat > "$tmp_dir/sample.mir.json" <<'JSON'
{
  "functions": [
    {
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
      "blocks": [{"instructions": [{"op": "mir_call", "dst": 1, "mir_call": {"callee": {"name": "HakoAllocObjectLifecycleFacadeReason.small_no_page/0"}, "args": []}}]}]
    },
    {
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleRecordAlignmentRequest/1",
      "blocks": [{"instructions": [
        {"op": "mir_call", "dst": 2, "mir_call": {"callee": {"name": "HakoAllocObjectLifecycleFacadeReason.alignment_unsupported/0"}, "args": []}},
        {"op": "mir_call", "dst": 3, "mir_call": {"callee": {"name": "HakoAllocObjectLifecycleFacadeReason.alignment_unsupported/0"}, "args": []}},
        {"op": "copy", "dst": 4, "src": 3}
      ]}]
    },
    {"name": "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAllocAligned/2", "blocks": [{"instructions": []}]},
    {"name": "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2", "blocks": [{"instructions": []}]},
    {"name": "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2", "blocks": [{"instructions": []}]},
    {"name": "HakoAllocObjectLifecycleFacade.objectLifecycleReallocGrowFromPage/4", "blocks": [{"instructions": []}]},
    {"name": "HakoAllocObjectLifecycleFacade.objectLifecycleReallocShrink/3", "blocks": [{"instructions": []}]},
    {"name": "HakoAllocObjectLifecycleFacade.objectLifecycleReallocGrow/3", "blocks": [{"instructions": []}]}
  ]
}
JSON
cat > "$tmp_dir/source.hako" <<'HAKO'
box HakoAllocObjectLifecycleFacade {
  objectLifecycleSmallAlloc(size) { return me.fail(HakoAllocObjectLifecycleFacadeReason.small_no_page()) }
  objectLifecycleRecordAlignmentRequest(alignment) { return me.fail(HakoAllocObjectLifecycleFacadeReason.alignment_unsupported()) }
  objectLifecycleSmallAllocAligned(size, alignment) { return 1 }
  objectLifecycleReleaseDirectCachedPage(page_id, block_id) { return 1 }
  objectLifecycleReleaseBlock(page_id, block_id) { return 1 }
  objectLifecycleReallocGrowFromPage(page, block_id, old_size, new_size) { return 1 }
  objectLifecycleReallocShrink(block_id, old_size, new_size) { return 1 }
  objectLifecycleReallocGrow(block_id, old_size, new_size) { return 1 }
}
HAKO

report="$tmp_dir/report.out"
python3 "$TOOL" --mir-json "$tmp_dir/sample.mir.json" --source "$tmp_dir/source.hako" --out "$report"

guard_expect_fixed_in_file "$TAG" 'output_contract=hako-alloc-facade-reason-duplicate-inventory-v0' "$report" "tool must emit output contract"
guard_expect_fixed_in_file "$TAG" 'method_0_source_reason_call_count=1' "$report" "tool must count fixed method"
guard_expect_fixed_in_file "$TAG" 'method_0_unused_duplicate_reason_call_count=0' "$report" "tool must not invent duplicates"
guard_expect_fixed_in_file "$TAG" 'method_1_unused_duplicate_reason_call_count=1' "$report" "tool must detect unused duplicate"
guard_expect_fixed_in_file "$TAG" 'failing_method_count=1' "$report" "tool must count failing methods"
guard_expect_fixed_in_file "$TAG" 'selected_next=hako_alloc_facade_reason_duplicate_eval_guard' "$report" "tool must select guard"
guard_expect_fixed_in_file "$TAG" 'winner_claim=0' "$report" "tool must keep winner closed"
guard_expect_fixed_in_file "$TAG" 'summary=ok' "$report" "tool must end ok"

echo "[$TAG] ok"
