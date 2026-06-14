#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-673-POST-FIELD-GET-ALIAS-KEEPER-OWNER-REFRESH-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-672-CROSS-BLOCK-FIELD-GET-ALIAS-FORWARDING-KEEPER-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_field_get_alias_keeper_owner_refresh_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
ATTRIBUTION="tools/allocator/mir_callsite_copy_attribution.py"
WEIGHT="tools/allocator/hako_mimalloc_local_ssa_dynamic_weight_probe.py"
POSITION="tools/allocator/mir_local_ssa_copy_position_probe.py"
POST="tools/allocator/hako_mimalloc_field_get_alias_keeper_post_probe.py"
OWNER="tools/allocator/hako_mimalloc_post_field_get_alias_keeper_owner_refresh.py"

[[ -f "$CARD" ]] || { echo "[post-field-get-owner] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[post-field-get-owner] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$OWNER" ]] || { echo "[post-field-get-owner] missing owner refresh tool: $OWNER" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[post-field-get-owner] row673 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[post-field-get-owner] row672 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[post-field-get-owner] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_post_field_get_owner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr_report="$tmp_dir/attr.out"
weight_report="$tmp_dir/weight.out"
position_report="$tmp_dir/position.out"
post_report="$tmp_dir/post.out"
hako_report="$tmp_dir/hako.out"
c_report="$tmp_dir/c.out"
pair_report="$tmp_dir/pair.out"
taxonomy_report="$tmp_dir/taxonomy.out"
owner_report="$tmp_dir/owner.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null
python3 "$ATTRIBUTION" --mir-json "$mir_json" --out "$attr_report"
python3 "$WEIGHT" --attribution "$attr_report" --method-invocation-count 524288 --out "$weight_report"
python3 "$POSITION" --mir-json "$mir_json" --out "$position_report"
python3 "$POST" --mir-json "$mir_json" --out "$post_report"

bash tools/allocator/hako_exe_memory_runner.sh \
  --app "$APP" \
  --workload representative-object-lifecycle-small-block-v0 \
  --runtime-config empty \
  --operation-repeat 1 \
  --out "$hako_report" >/dev/null
bash tools/allocator/c_mimalloc_explicit_runner.sh \
  --out "$c_report" \
  --allow-ldconfig-discovery \
  --workload representative-object-lifecycle-small-block-v0 \
  --in-process-repeat 8192 \
  --operation-repeat 1 >/dev/null
python3 tools/allocator/hako_mimalloc_object_lifecycle_body_timing_pair_adapter.py \
  --hako-report "$hako_report" \
  --c-report "$c_report" \
  --out "$pair_report"
python3 tools/allocator/hako_mimalloc_object_lifecycle_body_timing_gap_taxonomy.py \
  --input "$pair_report" \
  --out "$taxonomy_report"
python3 "$OWNER" \
  --taxonomy "$taxonomy_report" \
  --attribution "$attr_report" \
  --dynamic-weight "$weight_report" \
  --position "$position_report" \
  --post "$post_report" \
  --out "$owner_report"

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$owner_report"; then
    echo "[post-field-get-owner] missing report line: $expected" >&2
    cat "$owner_report" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-post-field-get-alias-keeper-owner-refresh-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "gap_owner=compiler_lowering"
require_line "copy_count=69"
require_line "expression_materialization_copy_count=3"
require_line "dominant_copy_owner=result_materialization"
require_line "dominant_dynamic_owner=page_hotpath_helper_attribution"
require_line "dominant_position=call_adjacent"
require_line "dominant_route_carrier_role=call_operand"
require_line "page_hotpath_helpers_call_count=5"
require_line "page_hotpath_helpers_attributed_copy_count=22"
require_line "result_materialization_copy_count=21"
require_line "selected_next_owner=page_hotpath_helper_result_materialization_copy_chain"
require_line "selected_owner_confidence=medium"
require_line "next_task=page_hotpath_helper_result_materialization_inventory"
require_line "implementation_started=0"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "summary=ok"

echo "[post-field-get-owner] ok"
