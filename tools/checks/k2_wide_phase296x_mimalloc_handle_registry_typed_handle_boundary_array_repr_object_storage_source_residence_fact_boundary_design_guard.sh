#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-810-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-FACT-BOUNDARY-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-809-MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-RESIDENCE-INPUT-CONSUMER-CLOSEOUT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_handle_registry_typed_handle_boundary_array_repr_object_storage_source_residence_fact_boundary_design_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-array-residence-fact-boundary-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-array-residence-fact-boundary-design] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[mimalloc-array-residence-fact-boundary-design] card must be Active or Landed" >&2
  exit 1
}
grep -Eq '^Status: Landed$' "$PREV_CARD" || {
  echo "[mimalloc-array-residence-fact-boundary-design] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[mimalloc-array-residence-fact-boundary-design] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[mimalloc-array-residence-fact-boundary-design] missing line in $file: $expected" >&2
    exit 1
  fi
}

for expected in \
  "output_contract=hako-mimalloc-handle-registry-typed-handle-boundary-array-repr-object-storage-source-residence-fact-boundary-design-v0" \
  "decision=B_plus_C_lite" \
  "source_evidence=296x-809,296x-808,array-repr-proof-chain-guide" \
  "target_front=object_lifecycle_body" \
  "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1" \
  "target_symbol=nyash_array_length_h" \
  "closed_stage=ArrayReceiverResidenceInput" \
  "closed_stage_mode=fallback_only" \
  "closed_stage_backend_consumable=0" \
  "array_receiver_residence_input_backend_consumable=0" \
  "fallback_fact_producer_enabled=0" \
  "fallback_residence_fact_enabled=0" \
  "fallback_residence_fact_reserved=0" \
  "public_arraybox_fallback_fact_produced=0" \
  "public_arraybox_fallback_is_negative_evidence=1" \
  "direct_residence_fact_reserved=1" \
  "residence_fact_requires_direct_storage_proof=1" \
  "residence_fact_requires_backend_bypass_authorization=1" \
  "residence_fact_backend_consumable=1" \
  "backend_reads_input=0" \
  "backend_reads_fact=1" \
  "backend_direct_handle_bypass_enabled=0" \
  "mir_json_export_enabled=0" \
  "backend_consumption_enabled=0" \
  "mirbuilder_object_management_enabled=0" \
  "implementation_allowed=0" \
  "product_default_changed=0" \
  "source_hako_changed=0" \
  "compiler_lowering_changed=0" \
  "runtime_object_changed=0" \
  "next_task=MIMALLOC-HANDLE-REGISTRY-TYPED-HANDLE-BOUNDARY-ARRAY-REPR-OBJECT-STORAGE-SOURCE-DIRECT-RESIDENCE-PROOF-OWNER-SELECTION-001" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
done

for expected in \
  "do not implement ArrayReceiverResidenceFact from fallback-only evidence" \
  "do not create report-only Fact with backend-like name" \
  "do not create FallbackResidenceFact" \
  "do not let backend read ArrayReceiverResidenceInput" \
  "do not export fallback residence input to MIR JSON" \
  "do not treat PublicArrayBoxFallback as direct storage proof" \
  "do not move Box/Object management into MIRBuilder"; do
  grep -F -q "$expected" "$CARD" || {
    echo "[mimalloc-array-residence-fact-boundary-design] missing stop line: $expected" >&2
    exit 1
  }
done

echo "[mimalloc-array-residence-fact-boundary-design] ok"
