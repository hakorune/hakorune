#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-metadata-catalog"
DOC="$ROOT_DIR/docs/reference/mir/metadata-facts-ssot.md"
ROOT_EMIT="$ROOT_DIR/src/runner/mir_json_emit/root.rs"
FUNCTION_TYPES="$ROOT_DIR/src/mir/function/types.rs"
SEMANTIC_REFRESH="$ROOT_DIR/src/mir/semantic_refresh.rs"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" rg
guard_require_command "$TAG" realpath
guard_require_files "$TAG" "$DOC" "$ROOT_EMIT" "$FUNCTION_TYPES" "$SEMANTIC_REFRESH" "$INDEX"

require_doc_token() {
  local token="$1"
  if ! rg -Fq "$token" "$DOC"; then
    guard_fail "$TAG" "metadata SSOT missing token: $token"
  fi
}

require_source_token() {
  local token="$1"
  local file="$2"
  if ! rg -Fq "$token" "$file"; then
    guard_fail "$TAG" "$(realpath --relative-to="$ROOT_DIR" "$file") missing expected token: $token"
  fi
}

for class in \
  SourceAttrs \
  SemanticFacts \
  Contracts \
  LayoutPlans \
  PlacementPlans \
  LoweringRoutes \
  DiagnosticsMetadata \
  ExperimentalSeedRoutes; do
  require_doc_token "$class"
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
  require_doc_token "$field"
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
  require_doc_token "$state"
done

for suffix in \
  "*_decls" \
  "*_facts" \
  "*_contracts" \
  "*_plans" \
  "*_routes" \
  "*_seed_route" \
  "*_micro_seed_route"; do
  require_doc_token "$suffix"
done

for boundary in \
  "Stage0 metadata = transport" \
  "Stage1 metadata = meaning / facts / contracts / plans" \
  "CorePlan metadata = placement / lowering decision" \
  "Backend metadata = route consumption"; do
  require_doc_token "$boundary"
done

for split in \
  "RecordSpec Metadata" \
  "PackedResidence Metadata" \
  "AllocatorPackedStore Pilot" \
  "CorePlan Promotion Criteria" \
  "Plan and route are intentionally different"; do
  require_doc_token "$split"
done

for promotion_section in \
  "Current Promotion Matrix" \
  "Promote / Treat As Active Now" \
  "Promote Soon / Prepare A Dedicated Row" \
  "Keep As Metadata / Do Not Promote Directly" \
  "Promotion Task Queue"; do
  require_doc_token "$promotion_section"
done

active_promotion_tokens=(
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
  require_doc_token "$token"
done

future_promotion_tokens=(
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
  require_doc_token "$token"
done

metadata_only_tokens=(
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
  require_doc_token "$token"
done

for task in \
  "METADATA-PROMOTE-001" \
  "METADATA-PROMOTE-002" \
  "METADATA-PROMOTE-003" \
  "METADATA-PROMOTE-004" \
  "METADATA-PROMOTE-005" \
  "METADATA-PROMOTE-006"; do
  require_doc_token "$task"
done

require_doc_token "do not combine these with allocator behavior rows"

module_keys=(
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
  require_doc_token "\`$key\`"
  require_source_token "\"$key\"" "$ROOT_EMIT"
done

function_keys=(
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
  exact_numeric
)

for key in "${function_keys[@]}"; do
  require_doc_token "\`$key"
done

seed_keys=(
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
  require_doc_token "\`$key\`"
  require_source_token "$key" "$FUNCTION_TYPES"
done

require_source_token "refresh_module_record_layout_plans" "$SEMANTIC_REFRESH"
require_source_token "refresh_function_placement_effect_routes" "$SEMANTIC_REFRESH"
require_source_token "refresh_function_exact_seed_backend_route" "$SEMANTIC_REFRESH"

require_doc_token "MetadataContext"
require_doc_token "FunctionMetadata"
require_doc_token "ModuleMetadata"
require_doc_token "PluginMetadata"
require_doc_token "placement_effect_routes"
require_doc_token "family-specific rows"
require_doc_token "boxed fallback disabled"
require_doc_token "metadata-only"
require_doc_token "tools/checks/mir_metadata_catalog_guard.sh"

require_source_token "tools/checks/mir_metadata_catalog_guard.sh" "$INDEX"

echo "[$TAG] ok module_keys=${#module_keys[@]} seed_keys=${#seed_keys[@]}"
