#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-806-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-DESIGN-001.md"
CLOSEOUT_CARD="docs/development/current/main/phases/phase-296x/296x-805-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-CLOSEOUT-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-784-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-RESIDENCE-PRODUCER-INPUT-SURFACE-001.md"
GUIDE="docs/development/current/main/phases/phase-296x/array-receiver-residence-proof-chain.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_residence_input_consumer_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-input-consumer-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$CLOSEOUT_CARD" ]] || { echo "[mimalloc-array-residence-input-consumer-design] missing closeout card: $CLOSEOUT_CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-residence-input-consumer-design] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$GUIDE" ]] || { echo "[mimalloc-array-residence-input-consumer-design] missing guide: $GUIDE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-input-consumer-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$CLOSEOUT_CARD" || {
  echo "[mimalloc-array-residence-input-consumer-design] closeout card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-input-consumer-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-input-consumer-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-input-consumer-design-v0" \
  "source_evidence=296x-805,296x-804,296x-801,296x-784,296x-783,array-repr-proof-chain-guide" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "selected_consumer=ArrayReceiverResidenceInput" \
  "consumer_owner=RepresentationPlanner|ObjectStoragePlan|ArrayRepr" \
  "consumer_input=ArrayReceiverResidenceInputSource" \
  "consumer_input_entry=ArrayReceiverResidenceProofChain" \
  "consumer_output=ArrayReceiverResidenceInput|none" \
  "consumer_mode=fallback_only" \
  "consumer_accepts_public_arraybox_fallback=1" \
  "consumer_accepts_direct_storage_source=0" \
  "consumer_preserves_materialization_route=public_arraybox_fallback" \
  "residence_input_surface_reused=1" \
  "residence_input_surface_source=296x-784" \
  "residence_input_direct_array_access_plan_available=0" \
  "residence_input_object_storage_plan_available=0" \
  "residence_input_array_repr_available=1" \
  "residence_input_escape_facts_available=0" \
  "residence_input_candidate=public_arraybox_fallback" \
  "residence_input_direct_storage_proof=0" \
  "residence_input_backend_bypass_authorized=0" \
  "fallback_source_is_not_direct_proof=1" \
  "proof_chain_entry_required=1" \
  "constructor_handoff_primary_mental_model=0" \
  "constructor_handoff_compat_kept=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "mir_json_export_enabled=0" \
  "backend_consumption_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-SURFACE-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "ArrayReceiverResidenceProofChain" "$GUIDE" || {
  echo "[mimalloc-array-residence-input-consumer-design] guide missing proof chain entry" >&2
  exit 1
}
grep -F -q "do not implement ArrayReceiverResidenceInput from this row" "$CARD" || {
  echo "[mimalloc-array-residence-input-consumer-design] missing implementation stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-residence-input-consumer-design] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-input-consumer-design] ok"
