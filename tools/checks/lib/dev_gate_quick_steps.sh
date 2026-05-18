dev_gate_script_step "current-state pointer guard" tools/checks/current_state_pointer_guard.sh
dev_gate_script_step "MIR metadata catalog guard" tools/checks/mir_metadata_catalog_guard.sh
dev_gate_script_step "Stage0 shape inventory guard" tools/checks/stage0_shape_inventory_guard.sh
dev_gate_script_step "Stage1 emit-program runtime-helper guard" tools/checks/stage1_emit_program_json_runtime_helper_guard.sh
dev_gate_script_step "Program(JSON) dev surface guard" tools/checks/program_json_dev_surface_guard.sh
dev_gate_script_step "Program(JSON v0) compat caller guard" tools/checks/program_json_v0_compat_caller_guard.sh
dev_gate_script_step "Program(JSON)->MIR bridge caller guard" tools/checks/program_json_mir_bridge_caller_guard.sh
dev_gate_script_step "Stage-B Program(JSON) capture caller guard" tools/checks/stageb_program_json_capture_caller_guard.sh
dev_gate_script_step "Stage1 Program(JSON) compat caller guard" tools/checks/stage1_program_json_compat_caller_guard.sh
dev_gate_script_step "phase29ch route probe surface guard" tools/checks/phase29ch_route_probe_surface_guard.sh
dev_gate_script_step "phase29ck pre-perf probe surface guard" tools/checks/phase29ck_preperf_probe_surface_guard.sh
dev_gate_script_step "phase29ck small-entry probe surface guard" tools/checks/phase29ck_small_entry_probe_surface_guard.sh
dev_gate_script_step "phase29ci verify probe surface guard" tools/checks/phase29ci_verify_probe_surface_guard.sh
dev_gate_script_step "phase216/217 normalization canary surface guard" tools/checks/phase216217_normalization_canary_surface_guard.sh
dev_gate_script_step "legacy dev utility surface guard" tools/checks/legacy_dev_utility_surface_guard.sh
dev_gate_script_step "lang include surface guard" tools/checks/lang_include_surface_guard.sh
dev_gate_script_step "tools/dev surface inventory guard" tools/checks/tools_dev_surface_inventory_guard.sh
dev_gate_script_step "hakorune emit MIR direct-caller guard" tools/checks/hakorune_emit_mir_direct_caller_guard.sh
dev_gate_script_step "MIR builder layer dependency guard" tools/checks/mir_builder_layer_dependency_guard.sh
dev_gate_script_step "LoopPatternContext zero guard" tools/checks/loop_pattern_context_zero_guard.sh
dev_gate_script_step ".inc codegen thin-shim guard" tools/checks/inc_codegen_thin_shim_guard.sh
dev_gate_script_step "generic-method Set policy mirror guard" tools/checks/generic_method_set_policy_mirror_guard.sh
dev_gate_script_step "CoreMethodContract manifest guard" tools/checks/core_method_contract_manifest_guard.sh
dev_gate_script_step "CoreMethodContract .inc no-growth guard" tools/checks/core_method_contract_inc_no_growth_guard.sh
dev_gate_script_step "MIR root facade guard" tools/checks/mir_root_facade_guard.sh
dev_gate_script_step "MIR root import hygiene guard" tools/checks/mir_root_import_hygiene_guard.sh
dev_gate_script_step "MIR builder CallTarget owner guard" tools/checks/mir_builder_calltarget_owner_guard.sh
dev_gate_script_step "MapLookup fusion reader boundary guard" tools/checks/map_lookup_fusion_reader_boundary_guard.sh
dev_gate_script_step "route detector legacy-surface guard" tools/checks/route_detector_legacy_surface_guard.sh
dev_gate_script_step "route no-fallback guard" tools/checks/route_no_fallback_guard.sh
dev_gate_cmd_step "cargo check" "cargo check --bin hakorune" cargo check --bin hakorune
dev_gate_cmd_step "ABI/decl codegen drift guard" "python3 tools/abi_manifest_codegen.py --check && python3 tools/backend_runtime_decl_manifest_codegen.py --check && python3 tools/backend_static_data_manifest_codegen.py --check" bash -lc 'python3 tools/abi_manifest_codegen.py --check && python3 tools/backend_runtime_decl_manifest_codegen.py --check && python3 tools/backend_static_data_manifest_codegen.py --check'
dev_gate_cmd_step "llvm_py unittest (strlen_fast)" "PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py" env PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_strlen_fast.py
dev_gate_script_step "K2-core RawArray acceptance guard" tools/checks/k2_core_rawarray_acceptance_guard.sh
dev_gate_script_step "K2-wide RawBuf first-row guard" tools/checks/k2_wide_rawbuf_first_row_guard.sh
dev_gate_script_step "K2-wide RawMap first-slice guard" tools/checks/k2_wide_rawmap_first_slice_guard.sh
dev_gate_script_step "K2-wide RawMap clear guard" tools/checks/k2_wide_rawmap_clear_guard.sh
dev_gate_script_step "K2-wide RawMap delete guard" tools/checks/k2_wide_rawmap_delete_guard.sh
dev_gate_script_step "K2-wide Atomic first-row guard" tools/checks/k2_wide_atomic_first_row_guard.sh
dev_gate_script_step "K2-wide TLS first-row guard" tools/checks/k2_wide_tls_first_row_guard.sh
dev_gate_script_step "K2-wide GC first-row guard" tools/checks/k2_wide_gc_first_row_guard.sh
dev_gate_script_step "K2-wide OSVM first-row guard" tools/checks/k2_wide_osvm_first_row_guard.sh
dev_gate_script_step "K2-wide Intrin first-row guard" tools/checks/k2_wide_intrin_first_row_guard.sh
dev_gate_script_step "K2-wide export attrs consistency guard" tools/checks/k2_wide_export_attrs_consistency_guard.sh
dev_gate_script_step "K2-wide static data first-row guard" tools/checks/k2_wide_static_data_first_row_guard.sh
dev_gate_script_step "K2-wide static const table decl guard" tools/checks/k2_wide_static_const_table_decl_guard.sh
dev_gate_script_step "K2-wide static const table load guard" tools/checks/k2_wide_static_const_table_load_guard.sh
dev_gate_script_step "K2-wide static const table eval guard" tools/checks/k2_wide_static_const_table_eval_guard.sh
dev_gate_script_step "K2-wide InlinePlan preserve guard" tools/checks/k2_wide_inline_plan_preserve_guard.sh
dev_gate_script_step "K2-wide InlinePlan soft leaf guard" tools/checks/k2_wide_inline_plan_soft_leaf_guard.sh
dev_gate_script_step "K2-wide Inline required vocab guard" tools/checks/k2_wide_inline_required_vocab_guard.sh
dev_gate_script_step "K2-wide rune Contract repeat guard" tools/checks/k2_wide_rune_contract_repeat_guard.sh
dev_gate_script_step "K2-wide Inline required verify guard" tools/checks/k2_wide_inline_required_verify_guard.sh
dev_gate_script_step "K2-wide Effect/Capability plan guard" tools/checks/k2_wide_effect_capability_plan_guard.sh
dev_gate_script_step "K2-wide return proof vocabulary guard" tools/checks/k2_wide_return_proof_vocab_guard.sh
dev_gate_script_step "K2-wide runtime-decl return proof row guard" tools/checks/k2_wide_runtime_decl_return_proof_row_guard.sh
dev_gate_script_step "K2-wide native ptr declare type guard" tools/checks/k2_wide_native_ptr_decl_type_guard.sh
dev_gate_script_step "K2-wide hako_mem runtime-decl guard" tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh
dev_gate_script_step "K2-wide mimalloc substrate route inventory guard" tools/checks/k2_wide_mimalloc_substrate_route_inventory_guard.sh
dev_gate_script_step "allocator provider inactive sentinel guard" tools/checks/allocator_provider_inactive_sentinel_guard.sh
dev_gate_script_step "K2-wide hako_alloc handle policy guard" tools/checks/k2_wide_hako_alloc_handle_policy_guard.sh
dev_gate_script_step "K2-wide hako_alloc GC trigger policy guard" tools/checks/k2_wide_hako_alloc_gc_trigger_policy_guard.sh
