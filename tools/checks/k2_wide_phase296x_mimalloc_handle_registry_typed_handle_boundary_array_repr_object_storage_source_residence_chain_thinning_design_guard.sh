#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-802-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-801-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-CONSTRUCTOR-HANDOFF-INPUT-SOURCE-CONSUMER-INVENTORY-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_residence_chain_thinning_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-chain-thinning-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-array-residence-chain-thinning-design] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-design] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-chain-thinning-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-chain-thinning-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-chain-thinning-design-v0" \
  "source_evidence=296x-801,296x-800,296x-786,296x-783,296x-780,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "selected_design=array_receiver_residence_proof_chain_facade" \
  "selected_design_confidence=medium" \
  "developer_facing_entry=ArrayReceiverResidenceProofChain" \
  "implementation_facade_owner=RepresentationPlanner|ArrayReprSourcePlanner" \
  "facade_first_method=construct_input_source_from_representation_source" \
  "facade_preserves_existing_stage_reports=1" \
  "facade_collapses_public_mental_model=1" \
  "facade_removes_proof_gates=0" \
  "facade_turns_fallback_into_direct_proof=0" \
  "stage_0=ArrayReceiverRepresentationSource" \
  "stage_1=ArrayReceiverConstructorHandoff" \
  "stage_1_role=compat_internal_handoff_not_primary_mental_model" \
  "stage_2=ArrayReceiverResidenceInputSource" \
  "stage_3=ArrayReceiverResidenceInput" \
  "stage_4=ArrayReceiverResidenceFact" \
  "constructor_handoff_public_primary_entry=0" \
  "constructor_handoff_keep_compat_vocabulary=1" \
  "constructor_handoff_can_be_private_later=1" \
  "construct_from_source_entry_allowed_next_row=1" \
  "direct_proof_path_allowed_next_row=0" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "reject: delete ConstructorHandoff immediately" "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-design] missing delete rejection" >&2
  exit 1
}
grep -F -q "reject: continue adding public nouns without a facade" "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-design] missing public noun rejection" >&2
  exit 1
}
grep -F -q "do not implement the facade from this row" "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-design] missing facade stop line" >&2
  exit 1
}
grep -F -q "do not collapse proof gates without preserving report fields" "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-design] missing proof gate stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-chain-thinning-design] ok"
