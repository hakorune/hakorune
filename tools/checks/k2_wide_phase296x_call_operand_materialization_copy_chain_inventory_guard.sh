#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-684-CALL-OPERAND-MATERIALIZATION-COPY-CHAIN-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-683-POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-OWNER-REFRESH-REPEAT-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-685-CALL-OPERAND-MATERIALIZATION-FORWARDING-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_materialization_copy_chain_inventory_guard.sh"
TOOL="tools/allocator/hako_mimalloc_call_operand_materialization_copy_chain_inventory.py"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$CARD" ]] || { echo "[call-operand-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-inventory] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[call-operand-inventory] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[call-operand-inventory] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[call-operand-inventory] row684 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[call-operand-inventory] row683 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[call-operand-inventory] row685 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[call-operand-inventory] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_call_operand_inventory.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$TOOL" \
  --mir-json "$mir_json" \
  --source-evidence "$PREV_CARD" \
  --out "$report"

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[call-operand-inventory] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$report" "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-call-operand-materialization-copy-chain-inventory-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-683"
  require_line_in_file "$file" "copy_count=55"
  require_line_in_file "$file" "call_operand_route_carrier_copy_count=29"
  require_line_in_file "$file" "call_adjacent_copy_count=29"
  require_line_in_file "$file" "call_operand_chain_count=26"
  require_line_in_file "$file" "call_operand_unique_copy_count=29"
  require_line_in_file "$file" "same_block_call_operand_chain_count=26"
  require_line_in_file "$file" "cross_block_call_operand_chain_count=0"
  require_line_in_file "$file" "same_block_root_call_operand_chain_count=9"
  require_line_in_file "$file" "cross_block_root_call_operand_chain_count=14"
  require_line_in_file "$file" "unknown_root_call_operand_chain_count=3"
  require_line_in_file "$file" "receiver_operand_chain_count=17"
  require_line_in_file "$file" "arg_operand_chain_count=9"
  require_line_in_file "$file" "receiver_same_block_root_call_operand_chain_count=2"
  require_line_in_file "$file" "arg_same_block_root_call_operand_chain_count=7"
  require_line_in_file "$file" "receiver_cross_block_root_call_operand_chain_count=13"
  require_line_in_file "$file" "arg_cross_block_root_call_operand_chain_count=1"
  require_line_in_file "$file" "receiver_unknown_root_call_operand_chain_count=2"
  require_line_in_file "$file" "arg_unknown_root_call_operand_chain_count=1"
  require_line_in_file "$file" "safe_forwarding_candidate_count=9"
  require_line_in_file "$file" "dominance_required_candidate_count=14"
  require_line_in_file "$file" "selected_next_owner=call_operand_materialization_forwarding_design"
  require_line_in_file "$file" "selected_owner_confidence=medium"
  require_line_in_file "$file" "next_task=call_operand_materialization_forwarding_design"
  require_line_in_file "$file" "implementation_started=0"
  require_line_in_file "$file" "optimization_open=0"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "summary=ok"
done

require_line_in_file "$NEXT_CARD" "Task: CALL-OPERAND-MATERIALIZATION-FORWARDING-DESIGN-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-684"
require_line_in_file "$NEXT_CARD" "implementation_started=0"
require_line_in_file "$NEXT_CARD" "winner_claim=0"

echo "[call-operand-inventory] ok"
