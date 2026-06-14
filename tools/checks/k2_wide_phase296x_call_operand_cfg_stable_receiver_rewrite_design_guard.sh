#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-694-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-693-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-IMPLEMENTATION-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-695-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_cfg_stable_receiver_rewrite_design_guard.sh"
TOOL="tools/allocator/hako_mimalloc_call_operand_cfg_stable_receiver_rewrite_design.py"

[[ -f "$CARD" ]] || { echo "[call-operand-cfg-stable-receiver-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-cfg-stable-receiver-design] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[call-operand-cfg-stable-receiver-design] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[call-operand-cfg-stable-receiver-design] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[call-operand-cfg-stable-receiver-design] row694 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[call-operand-cfg-stable-receiver-design] row693 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[call-operand-cfg-stable-receiver-design] row695 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[call-operand-cfg-stable-receiver-design] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_call_operand_cfg_stable_receiver_design.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"

python3 "$TOOL" --implementation-rejection "$PREV_CARD" --out "$report"

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[call-operand-cfg-stable-receiver-design] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$report" "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-call-operand-cfg-stable-receiver-rewrite-design-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-693"
  require_line_in_file "$file" "selected_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite"
  require_line_in_file "$file" "selected_owner_reason=callsite_canonicalize_already_owns_cfg_stable_callsite_rewrites_in_late_call_and_inline"
  require_line_in_file "$file" "selected_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite"
  require_line_in_file "$file" "pre_selected_keeper_candidate_count=13"
  require_line_in_file "$file" "post_selected_keeper_candidate_count_target=0"
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
  require_line_in_file "$file" "implementation_started=0"
  require_line_in_file "$file" "optimization_open=0"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "next_task=call_operand_cfg_stable_receiver_rewrite_guard_surface"
  require_line_in_file "$file" "summary=ok"
done

require_line_in_file "$NEXT_CARD" "Task: CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-GUARD-SURFACE-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-694"
require_line_in_file "$NEXT_CARD" "selected_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite"
require_line_in_file "$NEXT_CARD" "implementation_started=0"
require_line_in_file "$NEXT_CARD" "winner_claim=0"

echo "[call-operand-cfg-stable-receiver-design] ok"
