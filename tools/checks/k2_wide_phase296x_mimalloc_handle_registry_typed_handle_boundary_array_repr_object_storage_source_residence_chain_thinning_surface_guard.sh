#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-803-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-SURFACE-001.md"
DESIGN_CARD="docs/development/current/main/phases/phase-296x/296x-802-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-CHAIN-THINNING-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_residence_chain_thinning_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-chain-thinning-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$DESIGN_CARD" ]] || { echo "[mimalloc-array-residence-chain-thinning-surface] missing design card: $DESIGN_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-surface] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$DESIGN_CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-surface] design card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-chain-thinning-surface] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-chain-thinning-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-chain-thinning-surface-v0" \
  "source_evidence=296x-802,296x-801,296x-800,array-repr-ssot,object-storage-plan-boundary-ssot" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "residence_proof_chain_surface_defined=1" \
  "developer_facing_entry=ArrayReceiverResidenceProofChain" \
  "facade_owner=RepresentationPlanner|ArrayReprSourcePlanner" \
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
  "facade_materialization_route=public_arraybox_fallback" \
  "backend_direct_handle_bypass_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "mirbuilder_object_management_enabled=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

grep -F -q "construct_input_source_from_representation_source" "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-surface] missing facade method" >&2
  exit 1
}
grep -F -q "do not implement the facade from this row" "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-surface] missing facade stop line" >&2
  exit 1
}
grep -F -q "do not collapse proof gates without preserving report fields" "$CARD" || {
  echo "[mimalloc-array-residence-chain-thinning-surface] missing proof gate stop line" >&2
  exit 1
}

echo "[mimalloc-array-residence-chain-thinning-surface] ok"
