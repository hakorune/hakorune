#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-746-MIR-CALL-COMPARE-OPERAND-FORWARDING-IMPLEMENTATION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-745-MIR-CALL-COMPARE-OPERAND-FORWARDING-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
POST_PROBE="tools/allocator/hako_mimalloc_mir_call_compare_operand_forwarding_post_probe.py"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mir_call_compare_operand_forwarding_implementation_guard.sh"

[[ -f "$CARD" ]] || { echo "[mir-call-compare-impl] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mir-call-compare-impl] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$POST_PROBE" ]] || { echo "[mir-call-compare-impl] missing post probe: $POST_PROBE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mir-call-compare-impl] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: (Active|Landed)$' "$PREV_CARD" || {
  echo "[mir-call-compare-impl] previous card must be Active or Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mir-call-compare-impl] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mir-call-compare-impl] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-mir-call-compare-operand-forwarding-implementation-v0" \
  "source_evidence=296x-745" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "implementation_file=src/mir/builder/ssa/local.rs" \
  "implemented_scope=dominance_guarded_mir_call_result_to_compare_operand" \
  "mirbuilder_object_management_enabled=0" \
  "arg_forwarding_enabled=0" \
  "field_get_forwarding_enabled=0" \
  "param_forwarding_enabled=0" \
  "helper_name_special_case=0" \
  "benchmark_name_branch_count=0" \
  "pre_compare_operand_forwarding_candidate_count=2" \
  "post_compare_operand_forwarding_candidate_count=0" \
  "post_mir_call_expression_copy_count=0" \
  "post_expression_materialization_copy_count=1" \
  "post_root_dominates_candidate_count=0" \
  "post_unsafe_candidate_count=0" \
  "target_met=1" \
  "winner_claim=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

echo "[mir-call-compare-impl] ok"
