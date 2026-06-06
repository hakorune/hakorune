#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hako_fastmem_producer_parity.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

BASELINE="$TMP_DIR/baseline.kv"
CANDIDATE="$TMP_DIR/candidate.kv"
BAD_CANDIDATE="$TMP_DIR/bad_candidate.kv"
READINESS_CANDIDATE="$TMP_DIR/readiness_candidate.kv"
BAD_READINESS_CANDIDATE="$TMP_DIR/bad_readiness_candidate.kv"
OUT="$TMP_DIR/out.kv"
BAD_OUT="$TMP_DIR/bad_out.kv"
READINESS_OUT="$TMP_DIR/readiness_out.kv"
BAD_READINESS_OUT="$TMP_DIR/bad_readiness_out.kv"

cat >"$BASELINE" <<'KV'
replacement_front_producer_taxonomy_v0=1
replacement_front_producer=python_template_c_bridge
replacement_front_backend_artifact=c
replacement_front_source_truth=hako_fastmem
replacement_front_python_template_c_semantic_ssot=0
replacement_front_python_template_c_retirement_required=1
replacement_front_mir_memop_enabled=0
replacement_front_mir_fastmem_region_enabled=0
replacement_front_mirbuilder_representation_only=1
replacement_front_mirbuilder_route_decision_count=0
replacement_front_producer_slice_selection_v0=1
replacement_front_next_producer_slice=layout_table_producer_pilot
replacement_front_selected_memop_family=layout_table
replacement_front_selected_memop_kinds=TableIndex,FieldLoad,FieldStore
replacement_front_deferred_memop_family=owner_runtime
replacement_front_deferred_memop_kinds=CurrentAllocOwnerId,OwnerEq
replacement_front_selection_behavior_change=0
replacement_front_selection_product_activation=0
replacement_front_selection_bridge_retirement_allowed=0
replacement_front_is_full_hako_algorithm=0
hako_mimalloc_algorithm_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
KV

cat >"$CANDIDATE" <<'KV'
replacement_front_producer_taxonomy_v0=1
replacement_front_producer=mir_to_llvm_lowering
replacement_front_backend_artifact=object
replacement_front_source_truth=hako_fastmem
replacement_front_python_template_c_semantic_ssot=0
replacement_front_python_template_c_retirement_required=1
replacement_front_mir_memop_enabled=1
replacement_front_mir_fastmem_region_enabled=1
replacement_front_mirbuilder_representation_only=1
replacement_front_mirbuilder_route_decision_count=0
replacement_front_producer_slice_selection_v0=1
replacement_front_next_producer_slice=layout_table_producer_pilot
replacement_front_selected_memop_family=layout_table
replacement_front_selected_memop_kinds=TableIndex,FieldLoad,FieldStore
replacement_front_deferred_memop_family=owner_runtime
replacement_front_deferred_memop_kinds=CurrentAllocOwnerId,OwnerEq
replacement_front_selection_behavior_change=0
replacement_front_selection_product_activation=0
replacement_front_selection_bridge_retirement_allowed=0
replacement_front_is_full_hako_algorithm=0
hako_mimalloc_algorithm_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
KV

bash "$ROOT/tools/hako_check.sh" fastmem-producer-parity \
  --baseline "$BASELINE" \
  --candidate "$CANDIDATE" \
  >"$OUT"

grep -q '^output_contract=hako-check-fastmem-producer-parity-v0$' "$OUT"
grep -q '^tool_surface=hako_check_fastmem_producer_parity$' "$OUT"
grep -q '^observation_only=1$' "$OUT"
grep -q '^benchmark_run_executed=0$' "$OUT"
grep -q '^baseline_replacement_front_producer=python_template_c_bridge$' "$OUT"
grep -q '^candidate_replacement_front_producer=mir_to_llvm_lowering$' "$OUT"
grep -q '^producer_neutral_report_schema=1$' "$OUT"
grep -q '^producer_neutral_parity_pass=1$' "$OUT"
grep -q '^fastmem_producer_readiness_v0=0$' "$OUT"
grep -q '^producer_neutral_mismatch_count=0$' "$OUT"
grep -q '^producer_neutral_missing_field_count=0$' "$OUT"
grep -q '^python_template_c_bridge_runtime_dependency_count=0$' "$OUT"
grep -q '^failure_count=0$' "$OUT"
grep -q '^summary=ok$' "$OUT"

cp "$CANDIDATE" "$BAD_CANDIDATE"
sed -i 's/^replacement_front_source_truth=.*/replacement_front_source_truth=hako_alloc.page_box/' "$BAD_CANDIDATE"

set +e
bash "$ROOT/tools/hako_check.sh" fastmem-producer-parity \
  --baseline "$BASELINE" \
  --candidate "$BAD_CANDIDATE" \
  >"$BAD_OUT"
BAD_RC=$?
set -e

test "$BAD_RC" -ne 0
grep -q '^producer_neutral_parity_pass=0$' "$BAD_OUT"
grep -q '^producer_neutral_mismatch_count=1$' "$BAD_OUT"
grep -q '^failure_0_reason=mismatch:replacement_front_source_truth$' "$BAD_OUT"
grep -q '^summary=failed$' "$BAD_OUT"

cp "$CANDIDATE" "$READINESS_CANDIDATE"
cat >>"$READINESS_CANDIDATE" <<'KV'
fastmem_producer_readiness_v0=1
fastmem_producer_readiness_scope=layout_table_owner_runtime
mir_fmem_008b_layout_table_producer_pilot=1
fastmem_owner_runtime_producer_pilot=1
fastmem_owner_runtime_current_owner_source=llvm_producer_intrinsic
fastmem_verified_mem_access_plan_count=3
memop_table_index_lowered_count=1
memop_field_load_lowered_count=1
memop_field_store_lowered_count=1
memop_current_alloc_owner_id_lowered_count=1
memop_owner_eq_lowered_count=1
memop_atomic_remote_head_lowered_count=0
tls_backing_transfer_enabled=0
allocator_owner_slot_reuse_enabled=0
fastmem_layout_ref_escape_count=0
fastmem_lowering_recomputed_layout_offset_count=0
fastmem_table_index_unchecked_count=0
fastmem_table_access_proof_incomplete_count=0
fastmem_table_overflow_proof_missing_count=0
fastmem_unknown_alignment_count=0
fastmem_atomic_field_plain_store_count=0
KV

bash "$ROOT/tools/hako_check.sh" fastmem-producer-parity \
  --baseline "$BASELINE" \
  --candidate "$READINESS_CANDIDATE" \
  >"$READINESS_OUT"

grep -q '^producer_neutral_parity_pass=1$' "$READINESS_OUT"
grep -q '^fastmem_producer_readiness_v0=1$' "$READINESS_OUT"
grep -q '^fastmem_producer_readiness_pass=1$' "$READINESS_OUT"
grep -q '^fastmem_producer_readiness_scope=layout_table_owner_runtime$' "$READINESS_OUT"
grep -q '^python_template_c_bridge_runtime_dependency_count=0$' "$READINESS_OUT"
grep -q '^failure_count=0$' "$READINESS_OUT"
grep -q '^summary=ok$' "$READINESS_OUT"

cp "$READINESS_CANDIDATE" "$BAD_READINESS_CANDIDATE"
sed -i 's/^memop_owner_eq_lowered_count=.*/memop_owner_eq_lowered_count=0/' "$BAD_READINESS_CANDIDATE"
sed -i 's/^tls_backing_transfer_enabled=.*/tls_backing_transfer_enabled=1/' "$BAD_READINESS_CANDIDATE"

set +e
bash "$ROOT/tools/hako_check.sh" fastmem-producer-parity \
  --baseline "$BASELINE" \
  --candidate "$BAD_READINESS_CANDIDATE" \
  >"$BAD_READINESS_OUT"
BAD_READINESS_RC=$?
set -e

test "$BAD_READINESS_RC" -ne 0
grep -q '^fastmem_producer_readiness_v0=1$' "$BAD_READINESS_OUT"
grep -q '^fastmem_producer_readiness_pass=0$' "$BAD_READINESS_OUT"
grep -q '^failure_0_reason=candidate_memop_owner_eq_lowered_count$' "$BAD_READINESS_OUT"
grep -q '^failure_1_reason=candidate_tls_backing_transfer_enabled$' "$BAD_READINESS_OUT"
grep -q '^summary=failed$' "$BAD_READINESS_OUT"

echo "[TEST/OK] fastmem_producer_parity"
