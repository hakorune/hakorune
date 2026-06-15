#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-742-MIMALLOC-BODY-TIMING-FRESH-OWNER-SELECTION-002.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_body_timing_fresh_owner_selection_002_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-fresh-owner-002] missing card: $CARD" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-fresh-owner-002] card must be Active or Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-fresh-owner-002] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-fresh-owner-002] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-body-timing-fresh-owner-selection-002-v0" \
  "source_evidence=296x-731,296x-738,296x-741" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "hako_body_elapsed_ns=368000000" \
  "c_body_elapsed_ns=4056252" \
  "body_elapsed_ratio=90.724" \
  "gap_owner=compiler_lowering" \
  "gap_confidence=medium" \
  "selected_mir_body_owner=local_ssa_copy_materialization" \
  "selected_owner_confidence=high" \
  "dominant_dynamic_owner=local_ssa_copy_materialization" \
  "selected_copy_kind_policy=expression_materialization_copy_policy" \
  "dominant_expression_origin=mir_call" \
  "dominant_expression_sink=compare_eq" \
  "selected_origin_policy=mir_call_expression_value_copy_chain" \
  "fresh_narrow_owner=mir_call_selectPage_compare_eq_expression_copy_chain" \
  "fresh_narrow_owner_confidence=medium" \
  "implementation_allowed=0" \
  "selected_next=MIR-CALL-EXPRESSION-COPY-CHAIN-POLICY-SELECTION-001" \
  "winner_claim=0" \
  "product_default_changed=0" \
  "provider_active=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "mirbuilder_object_management_enabled=0" \
  "object_storage_plan_implementation_allowed=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

echo "[mimalloc-fresh-owner-002] ok"
