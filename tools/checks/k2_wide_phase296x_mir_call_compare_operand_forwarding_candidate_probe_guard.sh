#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-744-MIR-CALL-COMPARE-OPERAND-FORWARDING-CANDIDATE-PROBE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-743-MIR-CALL-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
PROBE="tools/allocator/hako_mimalloc_mir_call_compare_operand_forwarding_candidate_probe.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_call_compare_operand_forwarding_candidate_probe_guard.sh"

[[ -f "$CARD" ]] || { echo "[mir-call-compare-candidate] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mir-call-compare-candidate] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$PROBE" ]] || { echo "[mir-call-compare-candidate] missing probe: $PROBE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mir-call-compare-candidate] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: (Active|Landed)$' "$PREV_CARD" || {
  echo "[mir-call-compare-candidate] previous card must be Active or Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mir-call-compare-candidate] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mir-call-compare-candidate] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-mir-call-compare-operand-forwarding-candidate-probe-v0" \
  "input_contract=hako-mimalloc-mir-call-expression-copy-chain-policy-selection-v0" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "mir_call_expression_copy_count=2" \
  "compare_operand_forwarding_candidate_count=2" \
  "same_block_candidate_count=0" \
  "dominance_required_candidate_count=2" \
  "root_dominates_candidate_count=2" \
  "unsafe_candidate_count=0" \
  "dominant_candidate_sink=compare_eq" \
  "dominant_origin_detail=selectPage" \
  "selected_optimization_owner=dominance_guarded_mir_call_compare_operand_forwarding" \
  "selected_owner_confidence=medium" \
  "next_task=mir_call_compare_operand_forwarding_guard_surface" \
  "optimization_open=0" \
  "winner_claim=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "copy_chain_len_0_count=1" \
  "copy_chain_len_1_count=1" \
  "sink_compare_eq_copy_count=2" \
  "origin_detail_selectPage_copy_count=2" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

echo "[mir-call-compare-candidate] ok"
