#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-696-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-695-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-GUARD-SURFACE-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-697-POST-CFG-STABLE-RECEIVER-REWRITE-MEASUREMENT-001.md"
POLICY_CARD="docs/development/current/main/phases/phase-296x/296x-690-CALL-OPERAND-RESIDUAL-POLICY-SELECTION-001.md"
INVENTORY_SOURCE="docs/development/current/main/phases/phase-296x/296x-683-POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-OWNER-REFRESH-REPEAT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_cfg_stable_receiver_rewrite_implementation_guard.sh"
DOM_TOOL="tools/allocator/hako_mimalloc_call_operand_dominance_required_forwarding_design.py"
INV_TOOL="tools/allocator/hako_mimalloc_call_operand_materialization_copy_chain_inventory.py"
APP="apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"

[[ -f "$CARD" ]] || { echo "[call-operand-cfg-stable-receiver-impl] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-cfg-stable-receiver-impl] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[call-operand-cfg-stable-receiver-impl] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$DOM_TOOL" ]] || { echo "[call-operand-cfg-stable-receiver-impl] missing tool: $DOM_TOOL" >&2; exit 1; }
[[ -f "$INV_TOOL" ]] || { echo "[call-operand-cfg-stable-receiver-impl] missing tool: $INV_TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[call-operand-cfg-stable-receiver-impl] row696 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[call-operand-cfg-stable-receiver-impl] row695 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[call-operand-cfg-stable-receiver-impl] row697 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[call-operand-cfg-stable-receiver-impl] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_call_operand_cfg_stable_receiver_impl.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
dom_report="$tmp_dir/dominance.out"
inventory_report="$tmp_dir/inventory.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  target/release/hakorune --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$DOM_TOOL" --mir-json "$mir_json" --policy-selection "$POLICY_CARD" --out "$dom_report"
python3 "$INV_TOOL" --mir-json "$mir_json" --source-evidence "$INVENTORY_SOURCE" --out "$inventory_report"

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[call-operand-cfg-stable-receiver-impl] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-call-operand-cfg-stable-receiver-rewrite-implementation-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-695"
  require_line_in_file "$file" "selected_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite"
  require_line_in_file "$file" "selected_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite"
  require_line_in_file "$file" "pre_selected_keeper_candidate_count=13"
  require_line_in_file "$file" "post_selected_keeper_candidate_count=0"
  require_line_in_file "$file" "post_call_operand_unique_copy_count=13"
  require_line_in_file "$file" "arg_forwarding_enabled=0"
  require_line_in_file "$file" "requires_cfg_stable_dominance_guard=1"
  require_line_in_file "$file" "dominance_source=final_mir_cfg_successors"
  require_line_in_file "$file" "receiver_only_rewrite=1"
  require_line_in_file "$file" "unknown_root_forwarding_enabled=0"
  require_line_in_file "$file" "helper_name_special_case=0"
  require_line_in_file "$file" "variable_map_semantics_changed=0"
  require_line_in_file "$file" "phi_lifecycle_changed=0"
  require_line_in_file "$file" "source_hako_changed=0"
  require_line_in_file "$file" "startup_lane_reopened=0"
  require_line_in_file "$file" "optimization_open=0"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "next_task=post_cfg_stable_receiver_rewrite_measurement"
  require_line_in_file "$file" "summary=ok"
done

require_line_in_file "$dom_report" "safe_receiver_candidate_count=0"
require_line_in_file "$dom_report" "safe_arg_candidate_count=1"
require_line_in_file "$dom_report" "selected_keeper_candidate_count=0"
require_line_in_file "$dom_report" "rejected_arg_forwarding_count=1"
require_line_in_file "$inventory_report" "call_operand_unique_copy_count=13"
require_line_in_file "$inventory_report" "dominance_required_candidate_count=1"

require_line_in_file "$NEXT_CARD" "Task: POST-CFG-STABLE-RECEIVER-REWRITE-MEASUREMENT-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-696"
require_line_in_file "$NEXT_CARD" "winner_claim=0"

echo "[call-operand-cfg-stable-receiver-impl] ok"
