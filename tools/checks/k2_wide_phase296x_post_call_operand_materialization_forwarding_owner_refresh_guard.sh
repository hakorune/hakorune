#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-689-POST-CALL-OPERAND-MATERIALIZATION-FORWARDING-OWNER-REFRESH-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-688-POST-CALL-OPERAND-MATERIALIZATION-FORWARDING-MEASUREMENT-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-690-CALL-OPERAND-RESIDUAL-POLICY-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_call_operand_materialization_forwarding_owner_refresh_guard.sh"
TOOL="tools/allocator/hako_mimalloc_post_call_operand_materialization_forwarding_owner_refresh.py"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$CARD" ]] || { echo "[post-call-operand-owner-refresh] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[post-call-operand-owner-refresh] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[post-call-operand-owner-refresh] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[post-call-operand-owner-refresh] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[post-call-operand-owner-refresh] row689 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[post-call-operand-owner-refresh] row688 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[post-call-operand-owner-refresh] row690 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[post-call-operand-owner-refresh] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_post_call_operand_owner_refresh.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
attr="$tmp_dir/attr.out"
position="$tmp_dir/position.out"
inventory="$tmp_dir/inventory.out"
report="$tmp_dir/report.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 tools/allocator/mir_callsite_copy_attribution.py --mir-json "$mir_json" --out "$attr"
python3 tools/allocator/mir_local_ssa_copy_position_probe.py --mir-json "$mir_json" --out "$position"
python3 tools/allocator/hako_mimalloc_call_operand_materialization_copy_chain_inventory.py \
  --mir-json "$mir_json" \
  --source-evidence "docs/development/current/main/phases/phase-296x/296x-683-POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-OWNER-REFRESH-REPEAT-001.md" \
  --out "$inventory"
python3 "$TOOL" \
  --measurement "$PREV_CARD" \
  --attribution "$attr" \
  --position "$position" \
  --inventory "$inventory" \
  --out "$report"

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[post-call-operand-owner-refresh] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$report" "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-post-call-operand-materialization-forwarding-owner-refresh-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-688"
  require_line_in_file "$file" "hako_body_elapsed_ns=375000000"
  require_line_in_file "$file" "c_body_elapsed_ns=3255360"
  require_line_in_file "$file" "body_elapsed_ratio=115.195"
  require_line_in_file "$file" "copy_count=54"
  require_line_in_file "$file" "local_ssa_copy_materialization_copy_count=20"
  require_line_in_file "$file" "call_operand_route_carrier_copy_count=27"
  require_line_in_file "$file" "call_adjacent_copy_count=27"
  require_line_in_file "$file" "call_operand_chain_count=24"
  require_line_in_file "$file" "arg_same_block_root_call_operand_chain_count=7"
  require_line_in_file "$file" "dominance_required_candidate_count=14"
  require_line_in_file "$file" "unknown_root_call_operand_chain_count=3"
  require_line_in_file "$file" "receiver_cross_block_root_call_operand_chain_count=13"
  require_line_in_file "$file" "dominant_copy_owner=local_ssa_copy_materialization"
  require_line_in_file "$file" "dominant_position=call_adjacent"
  require_line_in_file "$file" "dominant_route_carrier_role=call_operand"
  require_line_in_file "$file" "selected_next_owner=call_operand_residual_policy_selection"
  require_line_in_file "$file" "selected_owner_confidence=medium"
  require_line_in_file "$file" "next_task=call_operand_residual_policy_selection"
  require_line_in_file "$file" "implementation_started=0"
  require_line_in_file "$file" "optimization_open=0"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "summary=ok"
done

require_line_in_file "$NEXT_CARD" "Task: CALL-OPERAND-RESIDUAL-POLICY-SELECTION-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-689"
require_line_in_file "$NEXT_CARD" "implementation_started=0"
require_line_in_file "$NEXT_CARD" "winner_claim=0"

echo "[post-call-operand-owner-refresh] ok"
