#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-683-POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-OWNER-REFRESH-REPEAT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-682-POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-MEASUREMENT-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-684-CALL-OPERAND-MATERIALIZATION-COPY-CHAIN-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_local_ssa_call_result_fallback_copy_policy_owner_refresh_repeat_guard.sh"
TOOL="tools/allocator/hako_mimalloc_post_local_ssa_call_result_fallback_copy_policy_owner_refresh_repeat.py"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$CARD" ]] || { echo "[post-local-ssa-owner-repeat] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[post-local-ssa-owner-repeat] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[post-local-ssa-owner-repeat] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[post-local-ssa-owner-repeat] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[post-local-ssa-owner-repeat] row683 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[post-local-ssa-owner-repeat] row682 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[post-local-ssa-owner-repeat] row684 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[post-local-ssa-owner-repeat] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_post_local_ssa_owner_repeat.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr="$tmp_dir/attr.out"
weight="$tmp_dir/weight.out"
position="$tmp_dir/position.out"
report="$tmp_dir/report.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 tools/allocator/mir_callsite_copy_attribution.py --mir-json "$mir_json" --out "$attr"
python3 tools/allocator/hako_mimalloc_local_ssa_dynamic_weight_probe.py \
  --attribution "$attr" \
  --method-invocation-count 524288 \
  --out "$weight"
python3 tools/allocator/mir_local_ssa_copy_position_probe.py --mir-json "$mir_json" --out "$position"
python3 "$TOOL" \
  --measurement "$PREV_CARD" \
  --attribution "$attr" \
  --dynamic-weight "$weight" \
  --position "$position" \
  --out "$report"

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[post-local-ssa-owner-repeat] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$report" "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-post-local-ssa-call-result-fallback-copy-policy-owner-refresh-repeat-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-682"
  require_line_in_file "$file" "copy_count=55"
  require_line_in_file "$file" "local_ssa_copy_materialization_copy_count=20"
  require_line_in_file "$file" "call_adjacent_copy_count=29"
  require_line_in_file "$file" "call_operand_route_carrier_copy_count=29"
  require_line_in_file "$file" "page_hotpath_helpers_attributed_copy_count=8"
  require_line_in_file "$file" "result_materialization_copy_count=7"
  require_line_in_file "$file" "dominant_copy_owner=local_ssa_copy_materialization"
  require_line_in_file "$file" "dominant_dynamic_owner=local_ssa_copy_materialization"
  require_line_in_file "$file" "dominant_position=call_adjacent"
  require_line_in_file "$file" "dominant_route_carrier_role=call_operand"
  require_line_in_file "$file" "selected_next_owner=call_operand_materialization_copy_chain_inventory"
  require_line_in_file "$file" "selected_owner_confidence=medium"
  require_line_in_file "$file" "next_task=call_operand_materialization_copy_chain_inventory"
  require_line_in_file "$file" "implementation_started=0"
  require_line_in_file "$file" "optimization_open=0"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "summary=ok"
done

require_line_in_file "$NEXT_CARD" "Task: CALL-OPERAND-MATERIALIZATION-COPY-CHAIN-INVENTORY-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-683"
require_line_in_file "$NEXT_CARD" "implementation_started=0"
require_line_in_file "$NEXT_CARD" "winner_claim=0"

echo "[post-local-ssa-owner-repeat] ok"
