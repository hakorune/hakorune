#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-743-MIR-CALL-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-742-MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002.md"
INDEX="docs/tools/check-scripts-index.md"
SELECTOR="tools/allocator/hako_mimalloc_mir_call_expression_copy_chain_policy_selection.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_call_expression_copy_chain_policy_selection_guard.sh"

[[ -f "$CARD" ]] || { echo "[mir-call-expression-policy] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mir-call-expression-policy] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$SELECTOR" ]] || { echo "[mir-call-expression-policy] missing selector: $SELECTOR" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mir-call-expression-policy] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: (Active|Landed)$' "$PREV_CARD" || {
  echo "[mir-call-expression-policy] previous card must be Active or Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mir-call-expression-policy] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mir-call-expression-policy] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-mir-call-expression-copy-chain-policy-selection-v0" \
  "input_contract=hako-mimalloc-expression-materialization-copy-origin-probe-v0" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "mir_call_origin_copy_count=2" \
  "expression_materialization_copy_count=3" \
  "mir_call_origin_ratio_bp=6666" \
  "mir_call_compare_sink_copy_count=2" \
  "mir_call_select_page_origin_copy_count=2" \
  "const_unused_copy_count=1" \
  "origin_copy_chain_len_0_count=2" \
  "origin_copy_chain_len_1_count=1" \
  "selected_chain_policy=mir_call_compare_operand_value_forwarding_candidate_probe" \
  "selected_chain_policy_confidence=medium" \
  "next_diagnostic=mir_call_compare_operand_forwarding_candidate_probe" \
  "optimization_open=0" \
  "winner_claim=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

echo "[mir-call-expression-policy] ok"
