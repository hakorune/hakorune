#!/usr/bin/env bash
# Shared checks for the MIR metadata catalog guard.
#
# Callers must define:
# - TAG
# - DOC
# - ROOT_EMIT
# - FUNCTION_TYPES
# - SEMANTIC_REFRESH
# - RUNE_CONTRACTS
# - INLINE_REQUIRED
# - STRING_KERNEL_VERIFIER
# - EXACT_NUMERIC_CONTRACTS
# - EXACT_NUMERIC_BACKEND
# - EXACT_SEED_BACKEND
# - ARRAY_RECORD_BOUNDARY
# - ARRAY_RECORD_PACKED_PILOT
# - SOURCE_PACKED_AUTOUSE
# - SOURCE_PACKED_DIRECT
# - ARRAY_RECORD_BACKEND
# - LLVM_COMMON_SHIM
# - LLVM_SUM_LOCAL_SHIM
# - LLVM_STRING_CANDIDATE_SHIM
# - LLVM_PURE_COMPILE_SHIM
# - PACKED_BACKEND_GUARD
# - SOURCE_PACKED_AUTOUSE_GUARD
# - INDEX
# before invoking these helpers.

mir_metadata_catalog_fail() {
  echo "[$TAG] ERROR: $*" >&2
  exit 1
}

mir_metadata_catalog_require_doc_token() {
  local token="$1"
  if ! rg -Fq "$token" "$DOC"; then
    mir_metadata_catalog_fail "metadata SSOT missing token: $token"
  fi
}

mir_metadata_catalog_require_source_token() {
  local token="$1"
  local file="$2"
  if ! rg -Fq "$token" "$file"; then
    mir_metadata_catalog_fail "$(realpath --relative-to="$ROOT_DIR" "$file") missing expected token: $token"
  fi
}

mir_metadata_catalog_check_doc_taxonomy() {
  local class field state suffix boundary split promotion_section token

  for class in \
    SourceAttrs \
    SemanticFacts \
    Contracts \
    LayoutPlans \
    PlacementPlans \
    LoweringRoutes \
    DiagnosticsMetadata \
    ExperimentalSeedRoutes; do
    mir_metadata_catalog_require_doc_token "$class"
  done

  for field in \
    "owner:" \
    "producer:" \
    "consumer:" \
    "state:" \
    "backend_active:" \
    "fallback_allowed:" \
    "coreplan_promotion:" \
    "retire_condition:"; do
    mir_metadata_catalog_require_doc_token "$field"
  done

  for state in \
    transport_only \
    inspection_only \
    semantic_layout_truth \
    verifier_active \
    optimizer_active \
    backend_active \
    runtime_active \
    retired; do
    mir_metadata_catalog_require_doc_token "$state"
  done

  for suffix in \
    "*_decls" \
    "*_facts" \
    "*_contracts" \
    "*_plans" \
    "*_routes" \
    "*_seed_route" \
    "*_micro_seed_route"; do
    mir_metadata_catalog_require_doc_token "$suffix"
  done

  for boundary in \
    "Stage0 metadata = transport" \
    "Stage1 metadata = meaning / facts / contracts / plans" \
    "CorePlan metadata = placement / lowering decision" \
    "Backend metadata = route consumption"; do
    mir_metadata_catalog_require_doc_token "$boundary"
  done

  for split in \
    "RecordSpec Metadata" \
    "PackedResidence Metadata" \
    "AllocatorPackedStore Pilot" \
    "CorePlan Promotion Criteria" \
    "Plan and route are intentionally different"; do
    mir_metadata_catalog_require_doc_token "$split"
  done

  for promotion_section in \
    "Current Promotion Matrix" \
    "Promote / Treat As Active Now" \
    "Promote Soon / Prepare A Dedicated Row" \
    "Keep As Metadata / Do Not Promote Directly" \
    "Promotion Task Queue"; do
    mir_metadata_catalog_require_doc_token "$promotion_section"
  done
}

mir_metadata_catalog_check_promotion_vocab() {
  local token

  local active_promotion_tokens=(
    "lowering_plan"
    "typed_object_plans"
    "static_data_plans"
    "effect_plans"
    "inline_plans"
    "string_kernel_plans"
    "placement_effect_routes"
    "exact_numeric_runtime_check_contracts"
    "hako_alloc_*_packed_store_pilot_plans"
    "CorePlan layout/ABI truth"
    "backend_active"
    "verifier_active"
    "optimizer_active"
    "CorePlan placement/effect owner"
    "do not add new raw helper-name classifiers"
    "backends must not treat the row as an inline mandate"
    "Do not promote to CorePlan lowering until a real storage owner"
  )
  for token in "${active_promotion_tokens[@]}"; do
    mir_metadata_catalog_require_doc_token "$token"
  done

  local future_promotion_tokens=(
    "array_record_materialization_boundary_plans"
    "source_packed_array_direct_read_consumption_plans"
    "loop_range_facts"
    "array_text_*"
    "enum use rows derived from"
    "exact numeric binary/compare/shift route facts"
    "boxed_fallback=false"
    "proof-bearing route"
  )
  for token in "${future_promotion_tokens[@]}"; do
    mir_metadata_catalog_require_doc_token "$token"
  done

  local metadata_only_tokens=(
    "record_decls"
    "enum_decls"
    "user_box_decls"
    "record_layout_plans"
    "value_consumer_facts"
    "storage_classes"
    "string_corridor_*"
    "sum_placement_*"
    "thin_entry_*"
    "exact_seed_backend_route"
    "Declaration inventories are not lowering contracts"
    "Seed payloads are temporary exact-shape bridges"
  )
  for token in "${metadata_only_tokens[@]}"; do
    mir_metadata_catalog_require_doc_token "$token"
  done

  local task
  for task in \
    "METADATA-PROMOTE-001" \
    "METADATA-PROMOTE-002" \
    "METADATA-PROMOTE-003" \
    "METADATA-PROMOTE-004" \
    "METADATA-PROMOTE-005" \
    "METADATA-PROMOTE-006"; do
    mir_metadata_catalog_require_doc_token "$task"
  done

  mir_metadata_catalog_require_doc_token "do not combine these with allocator behavior rows"
}

mir_metadata_catalog_check_no_fallback_contracts() {
  local token

  local packed_no_fallback_tokens=(
    "PackedArray No-Fallback Contract"
    "storage requirements, not"
    "falling back to ordinary"
    "fail_fast_unmaterialized_record_value"
    "visible_record_materialization_enabled=false"
    "runtime_auto_use_enabled=false"
    "public_array_get_materialization_enabled=false"
    "hako_alloc_migration_enabled=false"
    "backend_lowering_enabled=false"
    "boxed_fallback_enabled=false"
    "backend_lowering_enabled=true"
    "proof-bearing direct-read route"
    "silent_fallback_allowed=false"
    "storage as an implicit fallback"
    "Do not enable packed backend lowering from source text alone"
  )
  for token in "${packed_no_fallback_tokens[@]}"; do
    mir_metadata_catalog_require_doc_token "$token"
  done
}

mir_metadata_catalog_check_function_contracts() {
  local token

  local active_function_contract_tokens=(
    "Active Function Contract Rows"
    "are the obligation source for live"
    "transport/provenance after refresh"
    "contract-active only for"
    "verified=true"
    "fallback=fail_fast"
    "StringKernelPlanVerifierOwner::LoweringDirectKernelEntry"
    "stable-view provenance"
    "exact_numeric_runtime_check_contracts"
    "enforce_exact_numeric_backend_supported"
    "names must not be backend-consumed"
  )
  for token in "${active_function_contract_tokens[@]}"; do
    mir_metadata_catalog_require_doc_token "$token"
  done
}

mir_metadata_catalog_check_seed_retirement() {
  local token

  local seed_retirement_tokens=(
    "Seed Route Retirement Ledger"
    "not CorePlan promotion candidates"
    "is a selector over already"
    "Generic replacement target"
    "array/string micro seed"
    "array get/set micro seed"
    "array RMW add1 leaf seed"
    "string const-suffix micro seed"
    "string substring views micro seed"
    "sum placement seed"
    "thin-entry / typed-object seed"
    "userbox loop micro seed"
    "known-receiver userbox method seed"
    "string_kernel_plans.loop_payload"
    "Do not add a new seed row without an owner family"
    "Do not promote seed payloads to CorePlan ownership"
    "app-specific block counts"
  )
  for token in "${seed_retirement_tokens[@]}"; do
    mir_metadata_catalog_require_doc_token "$token"
  done
}

mir_metadata_catalog_check_source_contracts() {
  mir_metadata_catalog_require_source_token "check_rune_contracts" "$RUNE_CONTRACTS"
  mir_metadata_catalog_require_source_token "RuneContractSet::from_effect_plans" "$RUNE_CONTRACTS"
  mir_metadata_catalog_require_source_token "check_required_inline_plans" "$INLINE_REQUIRED"
  mir_metadata_catalog_require_source_token "InlineRequest::Required" "$INLINE_REQUIRED"
  mir_metadata_catalog_require_source_token "check_string_kernel_plans" "$STRING_KERNEL_VERIFIER"
  mir_metadata_catalog_require_source_token "StringKernelPlanVerifierOwner::LoweringDirectKernelEntry" "$STRING_KERNEL_VERIFIER"
  mir_metadata_catalog_require_source_token "exact_numeric_runtime_check_contracts" "$EXACT_NUMERIC_CONTRACTS"
  mir_metadata_catalog_require_source_token "enforce_exact_numeric_runtime_checks_supported" "$EXACT_NUMERIC_CONTRACTS"
  mir_metadata_catalog_require_source_token "enforce_exact_numeric_backend_supported" "$EXACT_NUMERIC_BACKEND"
  mir_metadata_catalog_require_source_token "ExactSeedBackendRouteKind" "$EXACT_SEED_BACKEND"
  mir_metadata_catalog_require_source_token "source_route_field" "$EXACT_SEED_BACKEND"
  mir_metadata_catalog_require_source_token "selected_substring_concat_loop_plan" "$EXACT_SEED_BACKEND"
  mir_metadata_catalog_require_source_token "hako_llvmc_exact_seed_backend_route_supported" "$LLVM_PURE_COMPILE_SHIM"
  mir_metadata_catalog_require_source_token "unsupported exact_seed_backend_route metadata" "$LLVM_PURE_COMPILE_SHIM"
  mir_metadata_catalog_require_source_token "ARRAY_RECORD_MATERIALIZATION_ACTION_FAIL_FAST_UNMATERIALIZED_RECORD" "$ARRAY_RECORD_BOUNDARY"
  mir_metadata_catalog_require_source_token "visible_record_materialization_enabled: false" "$ARRAY_RECORD_BOUNDARY"
  mir_metadata_catalog_require_source_token "runtime_auto_use_enabled: false" "$ARRAY_RECORD_BOUNDARY"
  mir_metadata_catalog_require_source_token "public_array_get_materialization_enabled: false" "$ARRAY_RECORD_PACKED_PILOT"
  mir_metadata_catalog_require_source_token "hako_alloc_migration_enabled: false" "$ARRAY_RECORD_PACKED_PILOT"
  mir_metadata_catalog_require_source_token "backend_lowering_enabled: false" "$ARRAY_RECORD_PACKED_PILOT"
  mir_metadata_catalog_require_source_token "boxed_fallback_enabled: false" "$SOURCE_PACKED_AUTOUSE"
  mir_metadata_catalog_require_source_token "backend_lowering_enabled: false" "$SOURCE_PACKED_AUTOUSE"
  mir_metadata_catalog_require_source_token "boxed_fallback_enabled: false" "$SOURCE_PACKED_DIRECT"
  mir_metadata_catalog_require_source_token "backend_lowering_enabled: false" "$SOURCE_PACKED_DIRECT"
  mir_metadata_catalog_require_source_token "silent_fallback_allowed=false" "$ARRAY_RECORD_BACKEND"
  mir_metadata_catalog_require_source_token "filter(|plan| plan.backend_lowering_enabled)" "$ARRAY_RECORD_BACKEND"
  mir_metadata_catalog_require_source_token "silent_fallback_allowed=false" "$PACKED_BACKEND_GUARD"
  mir_metadata_catalog_require_source_token "boxed_fallback_enabled: false" "$SOURCE_PACKED_AUTOUSE_GUARD"
}

mir_metadata_catalog_check_keys() {
  local key

  local module_keys=(
    user_box_decls
    record_decls
    enum_decls
    typed_object_plans
    record_layout_plans
    array_record_storage_plans
    array_record_autouse_eligibility_plans
    array_record_materialization_boundary_plans
    array_record_packed_autouse_pilot_plans
    source_packed_array_autouse_pilot_plans
    source_packed_array_direct_read_consumption_plans
    hako_alloc_aligned_small_packed_store_pilot_plans
    hako_alloc_huge_page_packed_store_pilot_plans
    static_data_plans
  )
  for key in "${module_keys[@]}"; do
    mir_metadata_catalog_require_doc_token "\`$key\`"
    mir_metadata_catalog_require_source_token "\"$key\"" "$ROOT_EMIT"
  done

  local function_keys=(
    value_types
    value_consumer_facts
    loop_range_facts
    runes
    storage_classes
    string_corridor_facts
    string_corridor_relations
    string_corridor_candidates
    string_kernel_plans
    string_direct_set_window_routes
    thin_entry_candidates
    thin_entry_selections
    inline_plans
    effect_plans
    capability_plans
    generic_method_routes
    extern_call_routes
    global_call_routes
    user_box_method_routes
    map_lookup_fusion_routes
    lowering_plan
    sum_placement_facts
    sum_placement_selections
    sum_placement_layouts
    agg_local_scalarization_routes
    placement_effect_routes
    array_rmw_window_routes
    array_string_len_window_routes
    declared_param_decls
    declared_return_type_name
    exact_numeric_*
  )
  for key in "${function_keys[@]}"; do
    mir_metadata_catalog_require_doc_token "\`$key\`"
  done

  local seed_keys=(
    array_string_store_micro_seed_route
    array_getset_micro_seed_route
    array_rmw_add1_leaf_seed_route
    concat_const_suffix_micro_seed_route
    substring_views_micro_seed_route
    sum_variant_tag_seed_route
    sum_variant_project_seed_route
    userbox_local_scalar_seed_route
    userbox_loop_micro_seed_route
    userbox_known_receiver_method_seed_route
    exact_seed_backend_route
  )
  for key in "${seed_keys[@]}"; do
    mir_metadata_catalog_require_doc_token "\`$key\`"
    mir_metadata_catalog_require_source_token "$key" "$FUNCTION_TYPES"
  done

  mir_metadata_catalog_require_source_token "refresh_module_record_layout_plans" "$SEMANTIC_REFRESH"
  mir_metadata_catalog_require_source_token "refresh_function_placement_effect_routes" "$SEMANTIC_REFRESH"
  mir_metadata_catalog_require_source_token "refresh_function_exact_seed_backend_route" "$SEMANTIC_REFRESH"
}

mir_metadata_catalog_check_ledger_docs() {
  local token
  for token in \
    "MetadataContext" \
    "FunctionMetadata" \
    "ModuleMetadata" \
    "PluginMetadata" \
    "placement_effect_routes" \
    "family-specific rows" \
    "Placement Effect Consumer Fold-Up Plan" \
    "Family rows remain compatibility fallbacks" \
    "string corridor route windows" \
    "sum placement local aggregate" \
    "sum local aggregate layout" \
    "thin entry" \
    "string direct kernels" \
    "verifier-equivalent generic route shape" \
    "boxed fallback disabled" \
    "metadata-only" \
    "tools/checks/mir_metadata_catalog_guard.sh"; do
    mir_metadata_catalog_require_doc_token "$token"
  done

  mir_metadata_catalog_require_source_token "tools/checks/mir_metadata_catalog_guard.sh" "$INDEX"
  mir_metadata_catalog_require_source_token "hako_llvmc_placement_effect_routes" "$LLVM_COMMON_SHIM"
  mir_metadata_catalog_require_source_token "hako_llvmc_has_thin_entry_selection" "$LLVM_COMMON_SHIM"
  mir_metadata_catalog_require_source_token "hako_llvmc_sum_has_thin_internal_selection" "$LLVM_SUM_LOCAL_SHIM"
  mir_metadata_catalog_require_source_token "hako_llvmc_sum_has_local_aggregate_fact" "$LLVM_SUM_LOCAL_SHIM"
  mir_metadata_catalog_require_source_token "hako_llvmc_sum_has_local_aggregate_selection" "$LLVM_SUM_LOCAL_SHIM"
  mir_metadata_catalog_require_source_token "hako_llvmc_sum_has_layout" "$LLVM_SUM_LOCAL_SHIM"
  mir_metadata_catalog_require_source_token "hako_llvmc_string_corridor_read_route_window_from_placement_effect_routes" "$LLVM_STRING_CANDIDATE_SHIM"
}

mir_metadata_catalog_check_all() {
  mir_metadata_catalog_check_doc_taxonomy
  mir_metadata_catalog_check_promotion_vocab
  mir_metadata_catalog_check_no_fallback_contracts
  mir_metadata_catalog_check_function_contracts
  mir_metadata_catalog_check_seed_retirement
  mir_metadata_catalog_check_source_contracts
  mir_metadata_catalog_check_keys
  mir_metadata_catalog_check_ledger_docs
}
