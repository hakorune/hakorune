#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE_DIR="$ROOT/tools/hako_check/tests/fastmem_capability_inventory"
GOOD_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_good.XXXXXX")"
BAD_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad.XXXXXX")"
BAD_SAFE_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_safe.XXXXXX")"
BAD_SHAPE_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_shape.XXXXXX")"
BAD_BRIDGE_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_bridge.XXXXXX")"
BAD_SIZE_CLASS_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_size_class.XXXXXX")"
BAD_PAGE_LOCAL_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_page_local.XXXXXX")"
BAD_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_producer.XXXXXX")"
BAD_PRODUCER_SLICE_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_producer_slice.XXXXXX")"
BAD_LAYOUT_TABLE_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_layout_table.XXXXXX")"
BAD_TABLE_PROOF_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_table_proof.XXXXXX")"
LAYOUT_LOWERING_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_layout_lowering.XXXXXX")"
BAD_LAYOUT_LOWERING_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_layout_lowering.XXXXXX")"
OWNER_RUNTIME_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_owner_runtime.XXXXXX")"
BAD_OWNER_RUNTIME_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_owner_runtime.XXXXXX")"
ATOMIC_REMOTE_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_preflight.XXXXXX")"
BAD_ATOMIC_REMOTE_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_preflight.XXXXXX")"
ATOMIC_REMOTE_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_producer.XXXXXX")"
BAD_ATOMIC_REMOTE_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_producer.XXXXXX")"
ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_retry_preflight.XXXXXX")"
BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_retry_preflight.XXXXXX")"
ATOMIC_REMOTE_RETRY_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_retry_producer.XXXXXX")"
BAD_ATOMIC_REMOTE_RETRY_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_retry_producer.XXXXXX")"
ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_drain_preflight.XXXXXX")"
BAD_ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_drain_preflight.XXXXXX")"
ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_drain_exchange.XXXXXX")"
BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_drain_exchange.XXXXXX")"
ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_drain_exchange_producer.XXXXXX")"
BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_drain_exchange_producer.XXXXXX")"
ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_drain_to_local_selection.XXXXXX")"
BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_drain_to_local_selection.XXXXXX")"
ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_drain_to_local_producer.XXXXXX")"
BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_drain_to_local_producer.XXXXXX")"
ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_drain_local_list_mutation_preflight.XXXXXX")"
BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_drain_local_list_mutation_preflight.XXXXXX")"
ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_drain_local_list_mutation_proof.XXXXXX")"
BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_drain_local_list_mutation_proof.XXXXXX")"
ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_drain_local_list_mutation_vocabulary.XXXXXX")"
BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_drain_local_list_mutation_vocabulary.XXXXXX")"
ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_atomic_remote_drain_local_list_mutation_verifier.XXXXXX")"
BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_atomic_remote_drain_local_list_mutation_verifier.XXXXXX")"
trap 'rm -f "$GOOD_OUT" "$BAD_OUT" "$BAD_SAFE_OUT" "$BAD_SHAPE_OUT" "$BAD_BRIDGE_OUT" "$BAD_SIZE_CLASS_OUT" "$BAD_PAGE_LOCAL_OUT" "$BAD_PRODUCER_OUT" "$BAD_PRODUCER_SLICE_OUT" "$BAD_LAYOUT_TABLE_OUT" "$BAD_TABLE_PROOF_OUT" "$LAYOUT_LOWERING_OUT" "$BAD_LAYOUT_LOWERING_OUT" "$OWNER_RUNTIME_OUT" "$BAD_OWNER_RUNTIME_OUT" "$ATOMIC_REMOTE_PREFLIGHT_OUT" "$BAD_ATOMIC_REMOTE_PREFLIGHT_OUT" "$ATOMIC_REMOTE_PRODUCER_OUT" "$BAD_ATOMIC_REMOTE_PRODUCER_OUT" "$ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT" "$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT" "$ATOMIC_REMOTE_RETRY_PRODUCER_OUT" "$BAD_ATOMIC_REMOTE_RETRY_PRODUCER_OUT" "$ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT" "$BAD_ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT" "$ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT" "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT" "$ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT" "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT" "$ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT" "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT" "$ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT" "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT" "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT" "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT" "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT" "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT" "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT" "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT" "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT" "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT"' EXIT

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --report "$FIXTURE_DIR/report.kv" \
  --format kv \
  >"$GOOD_OUT"

grep -q '^output_contract=hako-check-fastmem-check-v0$' "$GOOD_OUT"
grep -q '^tool_surface=hako_check_fastmem_check$' "$GOOD_OUT"
grep -q '^failure_count=0$' "$GOOD_OUT"
grep -q '^summary=ok$' "$GOOD_OUT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/layout_table_lowering_coverage_inventory.kv" \
  --format kv \
  >"$LAYOUT_LOWERING_OUT"

grep -q '^failure_count=0$' "$LAYOUT_LOWERING_OUT"
grep -q '^summary=ok$' "$LAYOUT_LOWERING_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_layout_table_lowering_coverage_inventory.kv" \
  --format kv \
  >"$BAD_LAYOUT_LOWERING_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted missing layout/table lowered coverage" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_LAYOUT_LOWERING_OUT"
grep -q '^failure_0_reason=memop_field_store_lowered_count$' "$BAD_LAYOUT_LOWERING_OUT"
grep -q '^summary=failed$' "$BAD_LAYOUT_LOWERING_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_inventory.kv" \
  --format kv \
  >"$BAD_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad inventory" >&2
  exit 1
fi

grep -q '^output_contract=hako-check-fastmem-check-v0$' "$BAD_OUT"
grep -q '^failure_count=6$' "$BAD_OUT"
grep -q '^failure_0_reason=fastmem_escape_count$' "$BAD_OUT"
grep -q '^failure_1_reason=fastmem_contract_runtime_lookup_count$' "$BAD_OUT"
grep -q '^failure_2_reason=fastmem_memop_unclassified_count$' "$BAD_OUT"
grep -q '^failure_3_reason=fastmem_forbidden_call_count$' "$BAD_OUT"
grep -q '^failure_4_reason=type_abi_hot_path_lookup_count$' "$BAD_OUT"
grep -q '^failure_5_reason=provider_dispatch_hot_path$' "$BAD_OUT"
grep -q '^summary=failed$' "$BAD_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_safe_wrapper_inventory.kv" \
  --format kv \
  >"$BAD_SAFE_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad safe wrapper inventory" >&2
  exit 1
fi

grep -q '^failure_count=2$' "$BAD_SAFE_OUT"
grep -q '^failure_0_reason=safe_capability_wrapper_route$' "$BAD_SAFE_OUT"
grep -q '^failure_1_reason=safe_capability_wrapper_memop_equivalence$' "$BAD_SAFE_OUT"
grep -q '^summary=failed$' "$BAD_SAFE_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_shape_keeper_inventory.kv" \
  --format kv \
  >"$BAD_SHAPE_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad mimalloc shape keeper" >&2
  exit 1
fi

grep -q '^failure_count=3$' "$BAD_SHAPE_OUT"
grep -q '^failure_0_reason=mimalloc_shape_score$' "$BAD_SHAPE_OUT"
grep -q '^failure_1_reason=mimalloc_coverage_score$' "$BAD_SHAPE_OUT"
grep -q '^failure_2_reason=mimalloc_keeper_eligible$' "$BAD_SHAPE_OUT"
grep -q '^summary=failed$' "$BAD_SHAPE_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_product_bridge_inventory.kv" \
  --format kv \
  >"$BAD_BRIDGE_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad product-shaped bridge inventory" >&2
  exit 1
fi

grep -q '^failure_count=5$' "$BAD_BRIDGE_OUT"
grep -q '^failure_0_reason=replacement_front_product_shaped_bridge_activation_ready$' "$BAD_BRIDGE_OUT"
grep -q '^failure_1_reason=product_activation_ready$' "$BAD_BRIDGE_OUT"
grep -q '^failure_2_reason=replacement_front_product_shaped_bridge_missing_activation_row$' "$BAD_BRIDGE_OUT"
grep -q '^failure_3_reason=replacement_front_product_shaped_bridge_missing_product_gate_open$' "$BAD_BRIDGE_OUT"
grep -q '^failure_4_reason=replacement_front_product_shaped_bridge_block_reason$' "$BAD_BRIDGE_OUT"
grep -q '^summary=failed$' "$BAD_BRIDGE_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_size_class_bridge_inventory.kv" \
  --format kv \
  >"$BAD_SIZE_CLASS_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad SizeClassBox bridge inventory" >&2
  exit 1
fi

grep -q '^failure_count=4$' "$BAD_SIZE_CLASS_OUT"
grep -q '^failure_0_reason=replacement_front_size_class_bridge_source_truth$' "$BAD_SIZE_CLASS_OUT"
grep -q '^failure_1_reason=replacement_front_size_class_bridge_bound$' "$BAD_SIZE_CLASS_OUT"
grep -q '^failure_2_reason=replacement_front_size_class_bridge_missing$' "$BAD_SIZE_CLASS_OUT"
grep -q '^failure_3_reason=replacement_front_size_class_policy_mirror_matches_source$' "$BAD_SIZE_CLASS_OUT"
grep -q '^summary=failed$' "$BAD_SIZE_CLASS_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_page_local_bridge_inventory.kv" \
  --format kv \
  >"$BAD_PAGE_LOCAL_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad Page-local bridge inventory" >&2
  exit 1
fi

grep -q '^failure_count=4$' "$BAD_PAGE_LOCAL_OUT"
grep -q '^failure_0_reason=replacement_front_page_local_bridge_source_truth$' "$BAD_PAGE_LOCAL_OUT"
grep -q '^failure_1_reason=replacement_front_page_local_bridge_bound$' "$BAD_PAGE_LOCAL_OUT"
grep -q '^failure_2_reason=replacement_front_page_local_bridge_missing$' "$BAD_PAGE_LOCAL_OUT"
grep -q '^failure_3_reason=replacement_front_page_local_typed_meta_matches_source$' "$BAD_PAGE_LOCAL_OUT"
grep -q '^summary=failed$' "$BAD_PAGE_LOCAL_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_producer_taxonomy_inventory.kv" \
  --format kv \
  >"$BAD_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad producer taxonomy inventory" >&2
  exit 1
fi

grep -q '^failure_count=8$' "$BAD_PRODUCER_OUT"
grep -q '^failure_0_reason=replacement_front_python_template_c_semantic_ssot$' "$BAD_PRODUCER_OUT"
grep -q '^failure_1_reason=replacement_front_mirbuilder_representation_only$' "$BAD_PRODUCER_OUT"
grep -q '^failure_2_reason=replacement_front_mirbuilder_route_decision_count$' "$BAD_PRODUCER_OUT"
grep -q '^failure_3_reason=replacement_front_backend_artifact$' "$BAD_PRODUCER_OUT"
grep -q '^failure_4_reason=replacement_front_python_template_c_retirement_required$' "$BAD_PRODUCER_OUT"
grep -q '^failure_5_reason=replacement_front_mir_memop_enabled$' "$BAD_PRODUCER_OUT"
grep -q '^failure_6_reason=replacement_front_mir_fastmem_region_enabled$' "$BAD_PRODUCER_OUT"
grep -q '^failure_7_reason=replacement_front_producer_transition_state$' "$BAD_PRODUCER_OUT"
grep -q '^summary=failed$' "$BAD_PRODUCER_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_producer_slice_selection_inventory.kv" \
  --format kv \
  >"$BAD_PRODUCER_SLICE_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad producer-slice selection inventory" >&2
  exit 1
fi

grep -q '^failure_count=8$' "$BAD_PRODUCER_SLICE_OUT"
grep -q '^failure_0_reason=replacement_front_next_producer_slice$' "$BAD_PRODUCER_SLICE_OUT"
grep -q '^failure_1_reason=replacement_front_selected_memop_family$' "$BAD_PRODUCER_SLICE_OUT"
grep -q '^failure_2_reason=replacement_front_selected_memop_kinds$' "$BAD_PRODUCER_SLICE_OUT"
grep -q '^failure_3_reason=replacement_front_deferred_memop_family$' "$BAD_PRODUCER_SLICE_OUT"
grep -q '^failure_4_reason=replacement_front_deferred_memop_kinds$' "$BAD_PRODUCER_SLICE_OUT"
grep -q '^failure_5_reason=replacement_front_selection_behavior_change$' "$BAD_PRODUCER_SLICE_OUT"
grep -q '^failure_6_reason=replacement_front_selection_product_activation$' "$BAD_PRODUCER_SLICE_OUT"
grep -q '^failure_7_reason=replacement_front_selection_bridge_retirement_allowed$' "$BAD_PRODUCER_SLICE_OUT"
grep -q '^summary=failed$' "$BAD_PRODUCER_SLICE_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_layout_table_producer_pilot_inventory.kv" \
  --format kv \
  >"$BAD_LAYOUT_TABLE_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad layout/table producer pilot" >&2
  exit 1
fi

grep -q '^failure_count=3$' "$BAD_LAYOUT_TABLE_OUT"
grep -q '^failure_0_reason=memop_current_alloc_owner_id_lowered_count$' "$BAD_LAYOUT_TABLE_OUT"
grep -q '^failure_1_reason=fastmem_field_id_missing_count$' "$BAD_LAYOUT_TABLE_OUT"
grep -q '^failure_2_reason=fastmem_atomic_field_plain_store_count$' "$BAD_LAYOUT_TABLE_OUT"
grep -q '^summary=failed$' "$BAD_LAYOUT_TABLE_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_table_access_proof_inventory.kv" \
  --format kv \
  >"$BAD_TABLE_PROOF_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted incomplete table access proof" >&2
  exit 1
fi

grep -q '^failure_count=2$' "$BAD_TABLE_PROOF_OUT"
grep -q '^failure_0_reason=fastmem_table_access_proof_incomplete_count$' "$BAD_TABLE_PROOF_OUT"
grep -q '^failure_1_reason=fastmem_table_overflow_proof_missing_count$' "$BAD_TABLE_PROOF_OUT"
grep -q '^summary=failed$' "$BAD_TABLE_PROOF_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/owner_runtime_lowering_coverage_inventory.kv" \
  --format kv \
  >"$OWNER_RUNTIME_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected owner-runtime lowering coverage inventory" >&2
  cat "$OWNER_RUNTIME_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$OWNER_RUNTIME_OUT"
grep -q '^summary=ok$' "$OWNER_RUNTIME_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_owner_runtime_lowering_coverage_inventory.kv" \
  --format kv \
  >"$BAD_OWNER_RUNTIME_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad owner-runtime lowering coverage inventory" >&2
  exit 1
fi

grep -q '^failure_count=6$' "$BAD_OWNER_RUNTIME_OUT"
grep -q '^failure_0_reason=fastmem_owner_runtime_current_owner_source$' "$BAD_OWNER_RUNTIME_OUT"
grep -q '^failure_1_reason=tls_backing_transfer_enabled$' "$BAD_OWNER_RUNTIME_OUT"
grep -q '^failure_2_reason=allocator_owner_slot_reuse_enabled$' "$BAD_OWNER_RUNTIME_OUT"
grep -q '^failure_3_reason=memop_atomic_remote_head_lowered_count$' "$BAD_OWNER_RUNTIME_OUT"
grep -q '^failure_4_reason=memop_current_alloc_owner_id_lowered_count$' "$BAD_OWNER_RUNTIME_OUT"
grep -q '^failure_5_reason=memop_owner_eq_lowered_count$' "$BAD_OWNER_RUNTIME_OUT"
grep -q '^summary=failed$' "$BAD_OWNER_RUNTIME_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_cas_preflight_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead CAS preflight inventory" >&2
  cat "$ATOMIC_REMOTE_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_cas_preflight_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead CAS preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=5$' "$BAD_ATOMIC_REMOTE_PREFLIGHT_OUT"
grep -q '^failure_0_reason=atomic_remote_head_memory_order_policy$' "$BAD_ATOMIC_REMOTE_PREFLIGHT_OUT"
grep -q '^failure_1_reason=atomic_remote_head_cas_lowering_open$' "$BAD_ATOMIC_REMOTE_PREFLIGHT_OUT"
grep -q '^failure_2_reason=atomic_remote_head_push_lowerable_count$' "$BAD_ATOMIC_REMOTE_PREFLIGHT_OUT"
grep -q '^failure_3_reason=atomic_remote_head_remote_owner_missing_count$' "$BAD_ATOMIC_REMOTE_PREFLIGHT_OUT"
grep -q '^failure_4_reason=memop_atomic_remote_head_lowered_count$' "$BAD_ATOMIC_REMOTE_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_PREFLIGHT_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_cas_producer_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead CAS producer inventory" >&2
  cat "$ATOMIC_REMOTE_PRODUCER_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_PRODUCER_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_PRODUCER_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_cas_producer_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead CAS producer inventory" >&2
  exit 1
fi

grep -q '^failure_count=5$' "$BAD_ATOMIC_REMOTE_PRODUCER_OUT"
grep -q '^failure_0_reason=atomic_remote_head_memory_order_policy$' "$BAD_ATOMIC_REMOTE_PRODUCER_OUT"
grep -q '^failure_1_reason=atomic_remote_head_block_next_missing_count$' "$BAD_ATOMIC_REMOTE_PRODUCER_OUT"
grep -q '^failure_2_reason=atomic_remote_head_cas_lowering_open$' "$BAD_ATOMIC_REMOTE_PRODUCER_OUT"
grep -q '^failure_3_reason=atomic_remote_head_push_lowerable_count$' "$BAD_ATOMIC_REMOTE_PRODUCER_OUT"
grep -q '^failure_4_reason=memop_atomic_remote_head_lowered_count$' "$BAD_ATOMIC_REMOTE_PRODUCER_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_PRODUCER_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_retry_preflight_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead retry preflight inventory" >&2
  cat "$ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_retry_preflight_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead retry preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=8$' "$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"
grep -q '^failure_0_reason=replacement_front_next_producer_slice$' "$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"
grep -q '^failure_1_reason=replacement_front_deferred_memop_kinds$' "$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"
grep -q '^failure_2_reason=atomic_remote_head_retry_policy_open$' "$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"
grep -q '^failure_3_reason=atomic_remote_head_retry_lowered_count$' "$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"
grep -q '^failure_4_reason=atomic_remote_head_drain_open$' "$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"
grep -q '^failure_5_reason=remote_owner_branch_routing_open$' "$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"
grep -q '^failure_6_reason=atomic_remote_head_retry_policy_selected$' "$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"
grep -q '^failure_7_reason=atomic_remote_head_retry_attempt_limit$' "$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_RETRY_PREFLIGHT_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_retry_producer_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_RETRY_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead retry producer inventory" >&2
  cat "$ATOMIC_REMOTE_RETRY_PRODUCER_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_RETRY_PRODUCER_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_RETRY_PRODUCER_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_retry_producer_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_RETRY_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead retry producer inventory" >&2
  exit 1
fi

grep -q '^failure_count=5$' "$BAD_ATOMIC_REMOTE_RETRY_PRODUCER_OUT"
grep -q '^failure_0_reason=atomic_remote_head_drain_open$' "$BAD_ATOMIC_REMOTE_RETRY_PRODUCER_OUT"
grep -q '^failure_1_reason=remote_owner_branch_routing_open$' "$BAD_ATOMIC_REMOTE_RETRY_PRODUCER_OUT"
grep -q '^failure_2_reason=atomic_remote_head_retry_policy_open$' "$BAD_ATOMIC_REMOTE_RETRY_PRODUCER_OUT"
grep -q '^failure_3_reason=atomic_remote_head_retry_attempt_limit$' "$BAD_ATOMIC_REMOTE_RETRY_PRODUCER_OUT"
grep -q '^failure_4_reason=atomic_remote_head_retry_lowered_count$' "$BAD_ATOMIC_REMOTE_RETRY_PRODUCER_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_RETRY_PRODUCER_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_drain_preflight_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead drain preflight inventory" >&2
  cat "$ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_drain_preflight_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead drain preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=5$' "$BAD_ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT"
grep -q '^failure_0_reason=replacement_front_selected_memop_kinds$' "$BAD_ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT"
grep -q '^failure_1_reason=replacement_front_next_producer_slice$' "$BAD_ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT"
grep -q '^failure_2_reason=atomic_remote_head_drain_open$' "$BAD_ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT"
grep -q '^failure_3_reason=atomic_remote_head_drain_lowered_count$' "$BAD_ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT"
grep -q '^failure_4_reason=atomic_remote_head_drain_selected$' "$BAD_ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_DRAIN_PREFLIGHT_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_drain_exchange_selection_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead drain exchange selection inventory" >&2
  cat "$ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_drain_exchange_selection_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead drain exchange selection inventory" >&2
  exit 1
fi

grep -q '^failure_count=7$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"
grep -q '^failure_0_reason=replacement_front_selected_memop_kinds$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"
grep -q '^failure_1_reason=replacement_front_deferred_memop_kinds$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"
grep -q '^failure_2_reason=atomic_remote_head_drain_exchange_order$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"
grep -q '^failure_3_reason=atomic_remote_head_drain_result_kind$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"
grep -q '^failure_4_reason=atomic_remote_head_drain_lowered_count$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"
grep -q '^failure_5_reason=atomic_remote_head_drain_to_local_route_open$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"
grep -q '^failure_6_reason=atomic_remote_head_drain_exchange_selected$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_drain_exchange_producer_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead drain exchange producer inventory" >&2
  cat "$ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_drain_exchange_producer_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead drain exchange producer inventory" >&2
  exit 1
fi

grep -q '^failure_count=10$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^failure_0_reason=replacement_front_deferred_memop_kinds$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^failure_1_reason=atomic_remote_head_drain_exchange_order$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^failure_2_reason=atomic_remote_head_drain_result_kind$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^failure_3_reason=atomic_remote_head_memory_order_policy$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^failure_4_reason=atomic_remote_head_drain_to_local_route_open$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^failure_5_reason=atomic_remote_head_drain_exchange_selected$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^failure_6_reason=atomic_remote_head_drain_open$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^failure_7_reason=atomic_remote_head_drain_lowerable_count$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^failure_8_reason=atomic_remote_head_drain_lowered_count$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^failure_9_reason=memop_atomic_remote_head_drain_count$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_DRAIN_EXCHANGE_PRODUCER_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_drain_to_local_route_selection_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead drain-to-local selection inventory" >&2
  cat "$ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_drain_to_local_route_selection_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead drain-to-local selection inventory" >&2
  exit 1
fi

grep -q '^failure_count=6$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"
grep -q '^failure_0_reason=replacement_front_next_producer_slice$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"
grep -q '^failure_1_reason=replacement_front_deferred_memop_kinds$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"
grep -q '^failure_2_reason=atomic_remote_head_memory_order_policy$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"
grep -q '^failure_3_reason=atomic_remote_head_drain_to_local_route_open$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"
grep -q '^failure_4_reason=remote_owner_branch_routing_open$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"
grep -q '^failure_5_reason=atomic_remote_head_drain_to_local_route_selected$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_SELECTION_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_drain_to_local_route_producer_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead drain-to-local producer inventory" >&2
  cat "$ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_drain_to_local_route_producer_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead drain-to-local producer inventory" >&2
  exit 1
fi

grep -q '^failure_count=6$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"
grep -q '^failure_0_reason=replacement_front_next_producer_slice$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"
grep -q '^failure_1_reason=replacement_front_deferred_memop_kinds$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"
grep -q '^failure_2_reason=atomic_remote_head_memory_order_policy$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"
grep -q '^failure_3_reason=remote_owner_branch_routing_open$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"
grep -q '^failure_4_reason=atomic_remote_head_drain_to_local_route_open$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"
grep -q '^failure_5_reason=atomic_remote_head_drain_to_local_route_producer_pilot$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_DRAIN_TO_LOCAL_PRODUCER_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_drain_local_list_mutation_preflight_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead drain local-list mutation preflight inventory" >&2
  cat "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_drain_local_list_mutation_preflight_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead drain local-list mutation preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=6$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"
grep -q '^failure_0_reason=replacement_front_next_producer_slice$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"
grep -q '^failure_1_reason=replacement_front_deferred_memop_kinds$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"
grep -q '^failure_2_reason=atomic_remote_head_memory_order_policy$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"
grep -q '^failure_3_reason=atomic_remote_head_drain_local_list_mutation_open$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"
grep -q '^failure_4_reason=remote_owner_branch_routing_open$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"
grep -q '^failure_5_reason=atomic_remote_head_drain_local_list_mutation_selected$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PREFLIGHT_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_drain_local_list_mutation_proof_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead drain local-list mutation proof inventory" >&2
  cat "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_drain_local_list_mutation_proof_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead drain local-list mutation proof inventory" >&2
  exit 1
fi

grep -q '^failure_count=9$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"
grep -q '^failure_0_reason=replacement_front_next_producer_slice$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"
grep -q '^failure_1_reason=replacement_front_deferred_memop_kinds$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"
grep -q '^failure_2_reason=atomic_remote_head_memory_order_policy$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"
grep -q '^failure_3_reason=atomic_remote_head_drain_local_list_head_class$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"
grep -q '^failure_4_reason=atomic_remote_head_drain_local_list_publication_order$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"
grep -q '^failure_5_reason=atomic_remote_head_drain_local_list_mutation_open$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"
grep -q '^failure_6_reason=atomic_remote_head_drain_local_list_token_escape_count$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"
grep -q '^failure_7_reason=remote_owner_branch_routing_open$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"
grep -q '^failure_8_reason=atomic_remote_head_drain_local_list_head_class_resolved$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_PROOF_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_drain_local_list_mutation_vocabulary_preflight_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead drain local-list mutation vocabulary inventory" >&2
  cat "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_drain_local_list_mutation_vocabulary_preflight_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead drain local-list mutation vocabulary inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT"
grep -q '^failure_0_reason=atomic_remote_head_drain_local_list_mutation_lowerable_count$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VOCABULARY_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/atomic_remote_head_drain_local_list_mutation_verifier_preconditions_inventory.kv" \
  --format kv \
  >"$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected AtomicRemoteHead drain local-list mutation verifier inventory" >&2
  cat "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT"
grep -q '^summary=ok$' "$ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_atomic_remote_head_drain_local_list_mutation_verifier_preconditions_inventory.kv" \
  --format kv \
  >"$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad AtomicRemoteHead drain local-list mutation verifier inventory" >&2
  exit 1
fi

grep -q '^failure_count=4$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT"
grep -q '^failure_0_reason=drain_remote_list_to_local_plan_count$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT"
grep -q '^failure_1_reason=drain_remote_list_to_local_token_provenance_valid$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT"
grep -q '^failure_2_reason=drain_remote_list_to_local_page_operand_valid$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT"
grep -q '^failure_3_reason=drain_remote_list_to_local_head_class_resolved$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT"
grep -q '^summary=failed$' "$BAD_ATOMIC_REMOTE_DRAIN_LOCAL_LIST_MUTATION_VERIFIER_OUT"

REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_remote_owner_branch_routing_preflight.XXXXXX")"
BAD_REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_remote_owner_branch_routing_preflight.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/remote_owner_branch_routing_preflight_inventory.kv" \
  --format kv \
  >"$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected remote-owner branch routing preflight inventory" >&2
  cat "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_remote_owner_branch_routing_preflight_inventory.kv" \
  --format kv \
  >"$BAD_REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad remote-owner branch routing preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_OUT"
grep -q '^failure_0_reason=remote_owner_branch_routing_open$' \
  "$BAD_REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_REMOTE_OWNER_BRANCH_ROUTING_PREFLIGHT_OUT"

REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_remote_owner_branch_routing_lowering_preflight.XXXXXX")"
BAD_REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_remote_owner_branch_routing_lowering_preflight.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/remote_owner_branch_routing_lowering_preflight_inventory.kv" \
  --format kv \
  >"$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected remote-owner branch routing lowering preflight inventory" >&2
  cat "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_remote_owner_branch_routing_lowering_preflight_inventory.kv" \
  --format kv \
  >"$BAD_REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad remote-owner branch routing lowering preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_OUT"
grep -q '^failure_0_reason=remote_owner_branch_routing_open$' \
  "$BAD_REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_REMOTE_OWNER_BRANCH_ROUTING_LOWERING_PREFLIGHT_OUT"

REMOTE_OWNER_BRANCH_ROUTING_LOWERING_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_remote_owner_branch_routing_lowering.XXXXXX")"
BAD_REMOTE_OWNER_BRANCH_ROUTING_LOWERING_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_remote_owner_branch_routing_lowering.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/remote_owner_branch_routing_lowering_inventory.kv" \
  --format kv \
  >"$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected remote-owner branch routing lowering inventory" >&2
  cat "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_OUT"
grep -q '^summary=ok$' "$REMOTE_OWNER_BRANCH_ROUTING_LOWERING_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_remote_owner_branch_routing_lowering_inventory.kv" \
  --format kv \
  >"$BAD_REMOTE_OWNER_BRANCH_ROUTING_LOWERING_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad remote-owner branch routing lowering inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_REMOTE_OWNER_BRANCH_ROUTING_LOWERING_OUT"
grep -q '^failure_0_reason=remote_owner_branch_routing_lowered_count$' \
  "$BAD_REMOTE_OWNER_BRANCH_ROUTING_LOWERING_OUT"
grep -q '^summary=failed$' "$BAD_REMOTE_OWNER_BRANCH_ROUTING_LOWERING_OUT"

REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_remote_owner_branch_route_body_preflight.XXXXXX")"
BAD_REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_remote_owner_branch_route_body_preflight.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/remote_owner_branch_route_body_preflight_inventory.kv" \
  --format kv \
  >"$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected remote-owner branch route body preflight inventory" >&2
  cat "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_remote_owner_branch_route_body_preflight_inventory.kv" \
  --format kv \
  >"$BAD_REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad remote-owner branch route body preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_OUT"
grep -q '^failure_0_reason=remote_owner_branch_route_body_open$' \
  "$BAD_REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_REMOTE_OWNER_BRANCH_ROUTE_BODY_PREFLIGHT_OUT"

FASTMEM_BRANCH_CFG_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_branch_cfg_preflight.XXXXXX")"
BAD_FASTMEM_BRANCH_CFG_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_branch_cfg_preflight.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/fastmem_branch_cfg_preflight_inventory.kv" \
  --format kv \
  >"$FASTMEM_BRANCH_CFG_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected branch CFG preflight inventory" >&2
  cat "$FASTMEM_BRANCH_CFG_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$FASTMEM_BRANCH_CFG_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_fastmem_branch_cfg_preflight_inventory.kv" \
  --format kv \
  >"$BAD_FASTMEM_BRANCH_CFG_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad branch CFG preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_FASTMEM_BRANCH_CFG_PREFLIGHT_OUT"
grep -q '^failure_0_reason=fastmem_branch_cfg_open$' \
  "$BAD_FASTMEM_BRANCH_CFG_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_FASTMEM_BRANCH_CFG_PREFLIGHT_OUT"

FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_branch_cfg_lowering_preflight.XXXXXX")"
BAD_FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_branch_cfg_lowering_preflight.XXXXXX")"
FASTMEM_BRANCH_CFG_LOWERING_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_branch_cfg_lowering.XXXXXX")"
BAD_FASTMEM_BRANCH_CFG_LOWERING_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_branch_cfg_lowering.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/fastmem_branch_cfg_lowering_preflight_inventory.kv" \
  --format kv \
  >"$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected branch CFG lowering preflight inventory" >&2
  cat "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_fastmem_branch_cfg_lowering_preflight_inventory.kv" \
  --format kv \
  >"$BAD_FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad branch CFG lowering preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_OUT"
grep -q '^failure_0_reason=fastmem_branch_cfg_open$' \
  "$BAD_FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_FASTMEM_BRANCH_CFG_LOWERING_PREFLIGHT_OUT"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/fastmem_branch_cfg_lowering_inventory.kv" \
  --format kv \
  >"$FASTMEM_BRANCH_CFG_LOWERING_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected branch CFG lowering inventory" >&2
  cat "$FASTMEM_BRANCH_CFG_LOWERING_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$FASTMEM_BRANCH_CFG_LOWERING_OUT"
grep -q '^summary=ok$' "$FASTMEM_BRANCH_CFG_LOWERING_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_fastmem_branch_cfg_lowering_inventory.kv" \
  --format kv \
  >"$BAD_FASTMEM_BRANCH_CFG_LOWERING_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad branch CFG lowering inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_FASTMEM_BRANCH_CFG_LOWERING_OUT"
grep -q '^failure_0_reason=fastmem_branch_cfg_closed_guard$' \
  "$BAD_FASTMEM_BRANCH_CFG_LOWERING_OUT"
grep -q '^summary=failed$' "$BAD_FASTMEM_BRANCH_CFG_LOWERING_OUT"

SAME_REMOTE_FREE_BODY_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_same_remote_free_body_preflight.XXXXXX")"
BAD_SAME_REMOTE_FREE_BODY_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_same_remote_free_body_preflight.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/same_remote_free_body_preflight_inventory.kv" \
  --format kv \
  >"$SAME_REMOTE_FREE_BODY_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected same/remote free body preflight inventory" >&2
  cat "$SAME_REMOTE_FREE_BODY_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$SAME_REMOTE_FREE_BODY_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$SAME_REMOTE_FREE_BODY_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_same_remote_free_body_preflight_inventory.kv" \
  --format kv \
  >"$BAD_SAME_REMOTE_FREE_BODY_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad same/remote free body preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_SAME_REMOTE_FREE_BODY_PREFLIGHT_OUT"
grep -q '^failure_0_reason=same_remote_free_body_open$' \
  "$BAD_SAME_REMOTE_FREE_BODY_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_SAME_REMOTE_FREE_BODY_PREFLIGHT_OUT"

SAME_REMOTE_FREE_BODY_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_same_remote_free_body_producer.XXXXXX")"
BAD_SAME_REMOTE_FREE_BODY_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_same_remote_free_body_producer.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/same_remote_free_body_producer_inventory.kv" \
  --format kv \
  >"$SAME_REMOTE_FREE_BODY_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected same/remote free body producer inventory" >&2
  cat "$SAME_REMOTE_FREE_BODY_PRODUCER_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$SAME_REMOTE_FREE_BODY_PRODUCER_OUT"
grep -q '^summary=ok$' "$SAME_REMOTE_FREE_BODY_PRODUCER_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_same_remote_free_body_producer_inventory.kv" \
  --format kv \
  >"$BAD_SAME_REMOTE_FREE_BODY_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad same/remote free body producer inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_SAME_REMOTE_FREE_BODY_PRODUCER_OUT"
grep -q '^failure_0_reason=same_remote_free_body_lowered_count$' \
  "$BAD_SAME_REMOTE_FREE_BODY_PRODUCER_OUT"
grep -q '^summary=failed$' "$BAD_SAME_REMOTE_FREE_BODY_PRODUCER_OUT"

PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_page_local_free_route_cfg_preflight.XXXXXX")"
BAD_PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_page_local_free_route_cfg_preflight.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/page_local_free_route_cfg_preflight_inventory.kv" \
  --format kv \
  >"$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected page-local free route CFG preflight inventory" >&2
  cat "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_page_local_free_route_cfg_preflight_inventory.kv" \
  --format kv \
  >"$BAD_PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad page-local free route CFG preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_OUT"
grep -q '^failure_0_reason=page_local_free_route_cfg_lowering_enabled$' \
  "$BAD_PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_PAGE_LOCAL_FREE_ROUTE_CFG_PREFLIGHT_OUT"

PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_page_local_free_route_cfg_producer.XXXXXX")"
BAD_PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_page_local_free_route_cfg_producer.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/page_local_free_route_cfg_producer_inventory.kv" \
  --format kv \
  >"$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected page-local free route CFG producer inventory" >&2
  cat "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_OUT"
grep -q '^summary=ok$' "$PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_page_local_free_route_cfg_producer_inventory.kv" \
  --format kv \
  >"$BAD_PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad page-local free route CFG producer inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_OUT"
grep -q '^failure_0_reason=page_local_free_route_cfg_lowering_enabled$' \
  "$BAD_PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_OUT"
grep -q '^summary=failed$' "$BAD_PAGE_LOCAL_FREE_ROUTE_CFG_PRODUCER_OUT"

TLS_BACKING_TRANSFER_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_tls_backing_transfer_preflight.XXXXXX")"
BAD_TLS_BACKING_TRANSFER_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_tls_backing_transfer_preflight.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/tls_backing_transfer_preflight_inventory.kv" \
  --format kv \
  >"$TLS_BACKING_TRANSFER_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected TLS backing transfer preflight inventory" >&2
  cat "$TLS_BACKING_TRANSFER_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$TLS_BACKING_TRANSFER_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$TLS_BACKING_TRANSFER_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_tls_backing_transfer_preflight_inventory.kv" \
  --format kv \
  >"$BAD_TLS_BACKING_TRANSFER_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad TLS backing transfer preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_TLS_BACKING_TRANSFER_PREFLIGHT_OUT"
grep -q '^failure_0_reason=tls_backing_transfer_enabled$' \
  "$BAD_TLS_BACKING_TRANSFER_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_TLS_BACKING_TRANSFER_PREFLIGHT_OUT"

TLS_BACKING_TRANSFER_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_tls_backing_transfer_producer.XXXXXX")"
BAD_TLS_BACKING_TRANSFER_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_tls_backing_transfer_producer.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/tls_backing_transfer_producer_inventory.kv" \
  --format kv \
  >"$TLS_BACKING_TRANSFER_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected TLS backing transfer producer inventory" >&2
  cat "$TLS_BACKING_TRANSFER_PRODUCER_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$TLS_BACKING_TRANSFER_PRODUCER_OUT"
grep -q '^summary=ok$' "$TLS_BACKING_TRANSFER_PRODUCER_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_tls_backing_transfer_producer_inventory.kv" \
  --format kv \
  >"$BAD_TLS_BACKING_TRANSFER_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad TLS backing transfer producer inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_TLS_BACKING_TRANSFER_PRODUCER_OUT"
grep -q '^failure_0_reason=allocator_owner_slot_reuse_enabled$' \
  "$BAD_TLS_BACKING_TRANSFER_PRODUCER_OUT"
grep -q '^summary=failed$' "$BAD_TLS_BACKING_TRANSFER_PRODUCER_OUT"

OWNER_SLOT_REUSE_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_owner_slot_reuse_preflight.XXXXXX")"
BAD_OWNER_SLOT_REUSE_PREFLIGHT_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_owner_slot_reuse_preflight.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/owner_slot_reuse_preflight_inventory.kv" \
  --format kv \
  >"$OWNER_SLOT_REUSE_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected owner slot reuse preflight inventory" >&2
  cat "$OWNER_SLOT_REUSE_PREFLIGHT_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$OWNER_SLOT_REUSE_PREFLIGHT_OUT"
grep -q '^summary=ok$' "$OWNER_SLOT_REUSE_PREFLIGHT_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_owner_slot_reuse_preflight_inventory.kv" \
  --format kv \
  >"$BAD_OWNER_SLOT_REUSE_PREFLIGHT_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad owner slot reuse preflight inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_OWNER_SLOT_REUSE_PREFLIGHT_OUT"
grep -q '^failure_0_reason=allocator_owner_slot_reuse_enabled$' \
  "$BAD_OWNER_SLOT_REUSE_PREFLIGHT_OUT"
grep -q '^summary=failed$' "$BAD_OWNER_SLOT_REUSE_PREFLIGHT_OUT"

OWNER_SLOT_REUSE_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_owner_slot_reuse_producer.XXXXXX")"
BAD_OWNER_SLOT_REUSE_PRODUCER_OUT="$(mktemp "${TMPDIR:-/tmp}/hako_fastmem_check_bad_owner_slot_reuse_producer.XXXXXX")"

if ! bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/owner_slot_reuse_producer_inventory.kv" \
  --format kv \
  >"$OWNER_SLOT_REUSE_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check rejected owner slot reuse producer inventory" >&2
  cat "$OWNER_SLOT_REUSE_PRODUCER_OUT" >&2 || true
  exit 1
fi

grep -q '^failure_count=0$' "$OWNER_SLOT_REUSE_PRODUCER_OUT"
grep -q '^summary=ok$' "$OWNER_SLOT_REUSE_PRODUCER_OUT"

if bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --inventory "$FIXTURE_DIR/bad_owner_slot_reuse_producer_inventory.kv" \
  --format kv \
  >"$BAD_OWNER_SLOT_REUSE_PRODUCER_OUT"; then
  echo "[TEST/FAIL] fastmem-check accepted bad owner slot reuse producer inventory" >&2
  exit 1
fi

grep -q '^failure_count=1$' "$BAD_OWNER_SLOT_REUSE_PRODUCER_OUT"
grep -q '^failure_0_reason=allocator_owner_generation_bump_count$' \
  "$BAD_OWNER_SLOT_REUSE_PRODUCER_OUT"
grep -q '^summary=failed$' "$BAD_OWNER_SLOT_REUSE_PRODUCER_OUT"

echo "[TEST/OK] fastmem_check"
