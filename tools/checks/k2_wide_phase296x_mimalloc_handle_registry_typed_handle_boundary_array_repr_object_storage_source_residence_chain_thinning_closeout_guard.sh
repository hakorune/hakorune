#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-805-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-CLOSEOUT-001.md"
IMPL_CARD="docs/development/current/main/phases/phase-296x/296x-804-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-IMPLEMENTATION-001.md"
GUIDE="docs/development/current/main/phases/phase-296x/array-receiver-residence-proof-chain.md"
SOURCE="src/array_receiver_representation_source.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_residence_chain_thinning_closeout_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-chain-thinning-closeout] missing card: $CARD" >&2; exit 1; }
[[ -f "$IMPL_CARD" ]] || { echo "[mimalloc-array-residence-chain-thinning-closeout] missing implementation card: $IMPL_CARD" >&2; exit 1; }
[[ -f "$GUIDE" ]] || { echo "[mimalloc-array-residence-chain-thinning-closeout] missing guide: $GUIDE" >&2; exit 1; }
[[ -f "$SOURCE" ]] || { echo "[mimalloc-array-residence-chain-thinning-closeout] missing source: $SOURCE" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-closeout] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$IMPL_CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-closeout] implementation card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-chain-thinning-closeout] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-chain-thinning-closeout] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-chain-thinning-closeout-v0" \
  "source_evidence=296x-801,296x-802,296x-803,296x-804,array-repr-proof-chain-guide" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "thinning_detour_closed=1" \
  "developer_facing_entry=ArrayReceiverResidenceProofChain" \
  "developer_guide=docs/development/current/main/phases/phase-296x/array-receiver-residence-proof-chain.md" \
  "primary_entry_method=construct_input_source_from_representation_source" \
  "primary_flow=ArrayReceiverRepresentationSource->ArrayReceiverResidenceProofChain->ArrayReceiverResidenceInputSource->ArrayReceiverResidenceInput" \
  "constructor_handoff_primary_mental_model=0" \
  "constructor_handoff_compat_kept=1" \
  "constructor_handoff_report_gates_preserved=1" \
  "stage_reports_preserved=1" \
  "proof_gates_collapsed=0" \
  "facade_adds_direct_proof_power=0" \
  "fallback_source_is_not_direct_proof=1" \
  "public_arraybox_fallback_acceptance=1" \
  "direct_storage_source_acceptance=0" \
  "backend_direct_handle_bypass_enabled=0" \
  "mir_json_export_enabled=0" \
  "backend_consumption_enabled=0" \
  "mirbuilder_object_management_enabled=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-DESIGN-001" \
  "next_task_uses_entry=ArrayReceiverResidenceProofChain" \
  "next_task_target=ArrayReceiverResidenceInput" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "Use \`ArrayReceiverResidenceProofChain\` as the developer-facing entry." "$GUIDE" || {
  echo "[mimalloc-array-residence-chain-thinning-closeout] guide missing facade entry" >&2
  exit 1
}
grep -F -q "ArrayReceiverRepresentationSource" "$GUIDE" || {
  echo "[mimalloc-array-residence-chain-thinning-closeout] guide missing representation source" >&2
  exit 1
}
grep -F -q "ArrayReceiverResidenceInput" "$GUIDE" || {
  echo "[mimalloc-array-residence-chain-thinning-closeout] guide missing residence input target" >&2
  exit 1
}

for expected in \
  "pub struct ArrayReceiverResidenceProofChain" \
  "pub fn construct_input_source_from_representation_source" \
  "pub fn array_receiver_residence_proof_chain_report_fields" \
  "(\"developer_facing_entry\", \"ArrayReceiverResidenceProofChain\")" \
  "(\"facade_adds_direct_proof_power\", \"0\")" \
  "(\"backend_direct_handle_bypass_enabled\", \"0\")"; do
  grep -F -q "$expected" "$SOURCE" || {
    echo "[mimalloc-array-residence-chain-thinning-closeout] missing source token: $expected" >&2
    exit 1
  }
done

grep -F -q "do not implement ArrayReceiverResidenceInput from this row" "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-closeout] missing residence input stop line" >&2
  exit 1
}
grep -F -q "do not move Box/Object management into MIRBuilder" "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-closeout] missing MIRBuilder stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-chain-thinning-closeout] ok"
