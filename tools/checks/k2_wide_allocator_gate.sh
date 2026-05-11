#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

run_step() {
  local label="$1"
  shift
  echo "[k2-wide-allocator-gate] >>> ${label}"
  "$@"
}

if [[ "${1:-}" == "--list" ]]; then
  cat <<'LIST'
[k2-wide-allocator-gate] steps:
  - tools/checks/k2_wide_mimalloc_raw_page_proof_guard.sh
  - tools/checks/k2_wide_profile_registry_docs_guard.sh
  - tools/checks/k2_wide_profile_expansion_to_facts_guard.sh
  - tools/checks/k2_wide_allocator_fast_path_exe_guard.sh
  - tools/checks/k2_wide_hako_mem_extern_pure_first_guard.sh
  - tools/checks/k2_wide_rawbuf_global_wrapper_exe_guard.sh
  - tools/checks/k2_wide_rawarray_slot_append_exe_guard.sh
  - tools/checks/k2_wide_rawarray_slot_len_exe_guard.sh
  - tools/checks/k2_wide_rawarray_slot_load_exe_guard.sh
  - tools/checks/k2_wide_rawarray_slot_store_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_raw_page_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_size_class_table_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_two_class_page_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_dynamic_bin_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_size_to_bin_inline_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_remote_free_i64_exe_guard.sh
  - tools/checks/k2_wide_atomic_memory_order_args_vocab_guard.sh
  - tools/checks/k2_wide_pointer_atomic_vocab_guard.sh
  - tools/checks/k2_wide_mimalloc_ptr_atomic_store_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_allocator_closeout_guard.sh
  - tools/checks/k2_wide_mimalloc_ptr_atomic_load_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_ptr_atomic_cas_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_ptr_remote_free_list_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_allocator_substrate_closeout_guard.sh
  - tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh
  - tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh
  - tools/checks/k2_wide_hako_alloc_local_page_policy_exe_guard.sh
  - tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh
  - tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh
  - tools/checks/k2_wide_hako_alloc_production_facade_stress_exe_guard.sh
  - tools/checks/k2_wide_production_allocator_port_closeout_guard.sh
  - tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh
  - tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh
  - tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh
  - tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh
  - tools/checks/k2_wide_allocator_hook_runtime_owner_guard.sh
  - tools/checks/k2_wide_allocator_hook_runtime_dry_run_code_guard.sh
  - tools/checks/k2_wide_allocator_hook_dry_run_manifest_callsite_guard.sh
  - tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh
  - tools/checks/k2_wide_allocator_hook_activation_proof_validator_guard.sh
  - tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh
  - tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh
  - tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh
  - tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh
  - tools/checks/k2_wide_allocator_provider_manifest_vocab_guard.sh
  - tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh
  - tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh
  - tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh
  - tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh
  - tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh
  - tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh
  - tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh
  - tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh
  - tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh
  - tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh
  - tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh
  - tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh
  - tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh
  - tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_safety_closeout_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_owner_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_decision_cli_surface_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_decision_closeout_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh
  - tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh
  - tools/checks/k2_wide_allocator_provider_diagnostic_inactive_actions_guard.sh
  - tools/checks/k2_wide_allocator_provider_registry_snapshot_cli_surface_guard.sh
  - tools/checks/k2_wide_allocator_provider_activation_diagnostic_closeout_guard.sh
  - tools/checks/k2_wide_allocator_provider_selection_decision_diagnostic_report_guard.sh
  - tools/checks/k2_wide_allocator_provider_selection_decision_cli_surface_guard.sh
  - tools/checks/k2_wide_allocator_provider_diagnostic_helper_cleanup_guard.sh
  - tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_diagnostic_report_guard.sh
LIST
  exit 0
fi

run_step "mimalloc raw-page proof guard" \
  bash tools/checks/k2_wide_mimalloc_raw_page_proof_guard.sh

run_step "Profile registry docs guard" \
  bash tools/checks/k2_wide_profile_registry_docs_guard.sh

run_step "Profile expansion to facts guard" \
  bash tools/checks/k2_wide_profile_expansion_to_facts_guard.sh

run_step "allocator fast-path EXE guard" \
  bash tools/checks/k2_wide_allocator_fast_path_exe_guard.sh

run_step "hako.mem extern pure-first guard" \
  bash tools/checks/k2_wide_hako_mem_extern_pure_first_guard.sh

run_step "RawBuf global wrapper EXE guard" \
  bash tools/checks/k2_wide_rawbuf_global_wrapper_exe_guard.sh

run_step "RawArray slot_append_any EXE guard" \
  bash tools/checks/k2_wide_rawarray_slot_append_exe_guard.sh

run_step "RawArray slot_len_i64 EXE guard" \
  bash tools/checks/k2_wide_rawarray_slot_len_exe_guard.sh

run_step "RawArray slot_load_i64 EXE guard" \
  bash tools/checks/k2_wide_rawarray_slot_load_exe_guard.sh

run_step "RawArray slot_store_i64 EXE guard" \
  bash tools/checks/k2_wide_rawarray_slot_store_exe_guard.sh

run_step "mimalloc raw-page EXE guard" \
  bash tools/checks/k2_wide_mimalloc_raw_page_exe_guard.sh

run_step "mimalloc size-class table EXE guard" \
  bash tools/checks/k2_wide_mimalloc_size_class_table_exe_guard.sh

run_step "mimalloc two-class page EXE guard" \
  bash tools/checks/k2_wide_mimalloc_two_class_page_exe_guard.sh

run_step "mimalloc dynamic bin EXE guard" \
  bash tools/checks/k2_wide_mimalloc_dynamic_bin_exe_guard.sh

run_step "mimalloc size_to_bin inline EXE guard" \
  bash tools/checks/k2_wide_mimalloc_size_to_bin_inline_exe_guard.sh

run_step "mimalloc OSVM page EXE guard" \
  bash tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh

run_step "mimalloc TLS cache-slot EXE guard" \
  bash tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh

run_step "mimalloc atomic CAS EXE guard" \
  bash tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh

run_step "mimalloc atomic load EXE guard" \
  bash tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh

run_step "mimalloc atomic store EXE guard" \
  bash tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh

run_step "mimalloc atomic fetch-add EXE guard" \
  bash tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh

run_step "mimalloc remote-free i64 EXE guard" \
  bash tools/checks/k2_wide_mimalloc_remote_free_i64_exe_guard.sh

run_step "atomic memory-order args vocab guard" \
  bash tools/checks/k2_wide_atomic_memory_order_args_vocab_guard.sh

run_step "pointer atomic vocab guard" \
  bash tools/checks/k2_wide_pointer_atomic_vocab_guard.sh

run_step "mimalloc ptr atomic store EXE guard" \
  bash tools/checks/k2_wide_mimalloc_ptr_atomic_store_exe_guard.sh

run_step "mimalloc TLS ptr remote-free EXE guard" \
  bash tools/checks/k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh

run_step "mimalloc remote-free policy EXE guard" \
  bash tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh

run_step "mimalloc allocator closeout guard" \
  bash tools/checks/k2_wide_mimalloc_allocator_closeout_guard.sh

run_step "mimalloc ptr atomic load EXE guard" \
  bash tools/checks/k2_wide_mimalloc_ptr_atomic_load_exe_guard.sh

run_step "mimalloc ptr atomic CAS EXE guard" \
  bash tools/checks/k2_wide_mimalloc_ptr_atomic_cas_exe_guard.sh

run_step "mimalloc ptr remote-free list EXE guard" \
  bash tools/checks/k2_wide_mimalloc_ptr_remote_free_list_exe_guard.sh

run_step "mimalloc remote-free list policy EXE guard" \
  bash tools/checks/k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh

run_step "mimalloc remote-free retry-loop EXE guard" \
  bash tools/checks/k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh

run_step "mimalloc allocator substrate closeout guard" \
  bash tools/checks/k2_wide_mimalloc_allocator_substrate_closeout_guard.sh

run_step "production allocator port entry plan guard" \
  bash tools/checks/k2_wide_production_allocator_port_entry_plan_guard.sh

run_step "hako_alloc production facade EXE guard" \
  bash tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh

run_step "hako_alloc local page policy EXE guard" \
  bash tools/checks/k2_wide_hako_alloc_local_page_policy_exe_guard.sh

run_step "hako_alloc remote-free policy EXE guard" \
  bash tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh

run_step "hako_alloc page-source policy EXE guard" \
  bash tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh

run_step "hako_alloc production facade stress EXE guard" \
  bash tools/checks/k2_wide_hako_alloc_production_facade_stress_exe_guard.sh

run_step "production allocator port closeout guard" \
  bash tools/checks/k2_wide_production_allocator_port_closeout_guard.sh

run_step "allocator replacement hook boundary guard" \
  bash tools/checks/k2_wide_allocator_replacement_hook_boundary_guard.sh

run_step "allocator HookPlan vocabulary guard" \
  bash tools/checks/k2_wide_allocator_hook_plan_vocab_guard.sh

run_step "allocator hook runtime dry-run guard" \
  bash tools/checks/k2_wide_allocator_hook_runtime_dry_run_guard.sh

run_step "allocator hook activation proof guard" \
  bash tools/checks/k2_wide_allocator_hook_activation_proof_guard.sh

run_step "allocator hook runtime owner guard" \
  bash tools/checks/k2_wide_allocator_hook_runtime_owner_guard.sh

run_step "allocator hook runtime dry-run code guard" \
  bash tools/checks/k2_wide_allocator_hook_runtime_dry_run_code_guard.sh

run_step "allocator hook dry-run manifest callsite guard" \
  bash tools/checks/k2_wide_allocator_hook_dry_run_manifest_callsite_guard.sh

run_step "allocator hook dry-run test surface guard" \
  bash tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh

run_step "allocator hook activation proof validator guard" \
  bash tools/checks/k2_wide_allocator_hook_activation_proof_validator_guard.sh

run_step "allocator hook dry-run CLI surface guard" \
  bash tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh

run_step "allocator hook activation preflight guard" \
  bash tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh

run_step "allocator hook activation preflight shape guard" \
  bash tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh

run_step "allocator provider boundary vocab guard" \
  bash tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh

run_step "allocator provider manifest vocab guard" \
  bash tools/checks/k2_wide_allocator_provider_manifest_vocab_guard.sh

run_step "allocator provider task breakdown guard" \
  bash tools/checks/k2_wide_allocator_provider_task_breakdown_guard.sh

run_step "allocator provider manifest parser guard" \
  bash tools/checks/k2_wide_allocator_provider_manifest_parser_guard.sh

run_step "allocator provider manifest CLI surface guard" \
  bash tools/checks/k2_wide_allocator_provider_manifest_cli_surface_guard.sh

run_step "allocator provider readiness preflight guard" \
  bash tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh

run_step "allocator provider combined dry-run guard" \
  bash tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh

run_step "allocator provider registry boundary guard" \
  bash tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh

run_step "allocator provider hako model proof guard" \
  bash tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh

run_step "allocator provider debug guarded proof guard" \
  bash tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh

run_step "allocator provider native system proof guard" \
  bash tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh

run_step "allocator provider native mimalloc proof guard" \
  bash tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh

run_step "allocator provider activation entry contract guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh

run_step "allocator provider registry snapshot guard" \
  bash tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh

run_step "allocator provider selection decision guard" \
  bash tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh

run_step "allocator provider proof bundle consumption guard" \
  bash tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh

run_step "allocator provider rollback preflight guard" \
  bash tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh

run_step "allocator provider activation safety gate guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh

run_step "allocator provider activation safety diagnostic owner guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_owner_guard.sh

run_step "allocator provider activation safety diagnostic report guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_safety_diagnostic_report_guard.sh

run_step "allocator provider activation safety CLI surface guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_safety_cli_surface_guard.sh

run_step "allocator provider activation safety closeout guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_safety_closeout_guard.sh

run_step "allocator provider activation decision surface proposal guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_decision_surface_proposal_guard.sh

run_step "allocator provider activation decision fixture contract guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_decision_fixture_contract_guard.sh

run_step "allocator provider activation decision diagnostic owner guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_owner_guard.sh

run_step "allocator provider activation decision diagnostic report guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_decision_diagnostic_report_guard.sh

run_step "allocator provider activation decision CLI surface guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_decision_cli_surface_guard.sh

run_step "allocator provider activation decision closeout guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_decision_closeout_guard.sh

run_step "allocator provider activation implementation entry contract guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_implementation_entry_contract_guard.sh

run_step "allocator provider registry snapshot diagnostic report guard" \
  bash tools/checks/k2_wide_allocator_provider_registry_snapshot_diagnostic_report_guard.sh

run_step "allocator provider diagnostic inactive actions guard" \
  bash tools/checks/k2_wide_allocator_provider_diagnostic_inactive_actions_guard.sh

run_step "allocator provider registry snapshot CLI surface guard" \
  bash tools/checks/k2_wide_allocator_provider_registry_snapshot_cli_surface_guard.sh

run_step "allocator provider activation diagnostic closeout guard" \
  bash tools/checks/k2_wide_allocator_provider_activation_diagnostic_closeout_guard.sh

run_step "allocator provider selection decision diagnostic report guard" \
  bash tools/checks/k2_wide_allocator_provider_selection_decision_diagnostic_report_guard.sh

run_step "allocator provider selection decision CLI surface guard" \
  bash tools/checks/k2_wide_allocator_provider_selection_decision_cli_surface_guard.sh

run_step "allocator provider diagnostic helper cleanup guard" \
  bash tools/checks/k2_wide_allocator_provider_diagnostic_helper_cleanup_guard.sh

run_step "allocator provider proof bundle consumption diagnostic report guard" \
  bash tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_diagnostic_report_guard.sh
