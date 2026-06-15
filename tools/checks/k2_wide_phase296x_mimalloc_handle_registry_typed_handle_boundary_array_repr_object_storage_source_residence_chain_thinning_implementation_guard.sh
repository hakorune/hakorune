#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-804-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-IMPLEMENTATION-001.md"
SURFACE_CARD="docs/development/current/main/phases/phase-296x/296x-803-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-SURFACE-001.md"
GUIDE="docs/development/current/main/phases/phase-296x/array-receiver-residence-proof-chain.md"
SOURCE="src/array_receiver_representation_source.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_residence_chain_thinning_implementation_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-chain-thinning-implementation] missing card: $CARD" >&2; exit 1; }
[[ -f "$SURFACE_CARD" ]] || { echo "[mimalloc-array-residence-chain-thinning-implementation] missing surface card: $SURFACE_CARD" >&2; exit 1; }
[[ -f "$GUIDE" ]] || { echo "[mimalloc-array-residence-chain-thinning-implementation] missing guide: $GUIDE" >&2; exit 1; }
[[ -f "$SOURCE" ]] || { echo "[mimalloc-array-residence-chain-thinning-implementation] missing source: $SOURCE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-implementation] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$SURFACE_CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-implementation] surface card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-chain-thinning-implementation] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-chain-thinning-implementation] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-chain-thinning-implementation-v0" \
  "source_evidence=296x-803,296x-802,296x-800,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "implementation_module=src/array_receiver_representation_source.rs" \
  "developer_facing_entry=ArrayReceiverResidenceProofChain" \
  "facade_input=ArrayReceiverRepresentationSource" \
  "facade_output=ArrayReceiverResidenceInputSource|none" \
  "facade_first_method=construct_input_source_from_representation_source" \
  "facade_keeps_constructor_handoff_compat=1" \
  "facade_hides_constructor_handoff_from_primary_docs=1" \
  "facade_preserves_stage_reports=1" \
  "facade_preserves_stop_lines=1" \
  "facade_adds_direct_proof_power=0" \
  "facade_exports_to_mir_json=0" \
  "facade_consumed_by_backend=0" \
  "facade_accepts_public_arraybox_fallback=1" \
  "facade_accepts_direct_storage_source=0" \
  "facade_output_direct_storage_proof=0" \
  "facade_output_backend_bypass_authorized=0" \
  "facade_report_fields_defined=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=1" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-CLOSEOUT-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "pub struct ArrayReceiverResidenceProofChain" \
  "pub fn construct_input_source_from_representation_source" \
  "pub fn array_receiver_residence_proof_chain_report_fields" \
  "ArrayReceiverResidenceSourceConstructor::construct" \
  "(\"developer_facing_entry\", \"ArrayReceiverResidenceProofChain\")" \
  "(\"facade_keeps_constructor_handoff_compat\", \"1\")" \
  "(\"facade_adds_direct_proof_power\", \"0\")" \
  "(\"facade_consumed_by_backend\", \"0\")" \
  "(\"backend_direct_handle_bypass_enabled\", \"0\")"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-residence-chain-thinning-implementation] missing source token: $expected" >&2
    exit 1
  }
done

grep -F -q "Use \`ArrayReceiverResidenceProofChain\` as the developer-facing entry." "$GUIDE" || {
  echo "[mimalloc-array-residence-chain-thinning-implementation] guide missing primary entry" >&2
  exit 1
}
grep -F -q "fallback_source_is_not_direct_proof=1" "$GUIDE" || {
  echo "[mimalloc-array-residence-chain-thinning-implementation] guide missing fallback invariant" >&2
  exit 1
}
grep -F -q "do not implement backend direct handle bypass from this row" "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-implementation] missing backend bypass stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-chain-thinning-implementation] ok"
