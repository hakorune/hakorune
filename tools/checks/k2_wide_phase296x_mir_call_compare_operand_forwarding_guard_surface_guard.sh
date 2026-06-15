#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-745-MIR-CALL-COMPARE-OPERAND-FORWARDING-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-744-MIR-CALL-COMPARE-OPERAND-FORWARDING-CANDIDATE-PROBE-001.md"
CURRENT="docs/development/current/main/CURRENT_STATE.toml"
TASK="CURRENT_TASK.md"
NOW="docs/development/current/main/10-Now.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_call_compare_operand_forwarding_guard_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[mir-call-compare-guard-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mir-call-compare-guard-surface] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mir-call-compare-guard-surface] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: (Active|Landed)$' "$PREV_CARD" || {
  echo "[mir-call-compare-guard-surface] previous card must be Active or Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mir-call-compare-guard-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mir-call-compare-guard-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-mir-call-compare-operand-forwarding-guard-surface-v0" \
  "source_evidence=296x-744" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "pre_mir_call_expression_copy_count=2" \
  "pre_compare_operand_forwarding_candidate_count=2" \
  "pre_same_block_candidate_count=0" \
  "pre_dominance_required_candidate_count=2" \
  "pre_root_dominates_candidate_count=2" \
  "pre_unsafe_candidate_count=0" \
  "selected_optimization_owner=dominance_guarded_mir_call_compare_operand_forwarding" \
  "selected_owner_confidence=medium" \
  "post_compare_operand_forwarding_candidate_count=0" \
  "post_mir_call_expression_copy_count_upper_bound=2" \
  "post_root_dominates_candidate_count=0" \
  "post_unsafe_candidate_count=0" \
  "allowed_scope=dominance_guarded_mir_call_result_to_compare_operand" \
  "arg_forwarding_enabled=0" \
  "field_get_forwarding_enabled=0" \
  "param_forwarding_enabled=0" \
  "helper_name_special_case=0" \
  "benchmark_name_branch_count=0" \
  "optimization_open=1" \
  "winner_claim=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q 'latest_card = "MIR-CALL-COMPARE-OPERAND-FORWARDING-GUARD-SURFACE-001"' "$CURRENT" || {
  echo "[mir-call-compare-guard-surface] CURRENT_STATE latest_card must point to 745" >&2
  exit 1
}
grep -F -q "296x-745-MIR-CALL-COMPARE-OPERAND-FORWARDING-GUARD-SURFACE-001.md" "$CURRENT" || {
  echo "[mir-call-compare-guard-surface] CURRENT_STATE missing 745 card path" >&2
  exit 1
}

read_next_block="$(sed -n '/^## Read Next$/,$p' "$TASK")"
grep -F -q "2. \`docs/development/current/main/phases/phase-296x/296x-745-MIR-CALL-COMPARE-OPERAND-FORWARDING-GUARD-SURFACE-001.md\`" <<<"$read_next_block" || {
  echo "[mir-call-compare-guard-surface] CURRENT_TASK Read Next must point to 745" >&2
  exit 1
}

now_read_next_block="$(sed -n '/^## Read Next$/,$p' "$NOW")"
grep -F -q "2. \`docs/development/current/main/phases/phase-296x/296x-745-MIR-CALL-COMPARE-OPERAND-FORWARDING-GUARD-SURFACE-001.md\`" <<<"$now_read_next_block" || {
  echo "[mir-call-compare-guard-surface] 10-Now Read Next must point to 745" >&2
  exit 1
}

echo "[mir-call-compare-guard-surface] ok"
