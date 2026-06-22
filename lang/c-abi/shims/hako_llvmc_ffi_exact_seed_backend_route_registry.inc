/*
 * Exact-seed backend route registry.
 *
 * Keep the route tag / source_route / proof table in one place so the pure
 * compile shim does not re-list route tuples inline.
 */

struct HakoLlvmcExactSeedBackendRouteRow {
  const char* tag;
  const char* source_route;
  const char* proof;
  int requires_positive_selected_value;
};

static const struct HakoLlvmcExactSeedBackendRouteRow
    hako_llvmc_exact_seed_backend_route_rows[] = {
        {"array_string_store_micro", "array_string_store_micro_seed_route",
         "kilo_micro_array_string_store_8block", 0},
        {"array_getset_micro", "array_getset_micro_seed_route",
         "kilo_micro_array_getset_7block", 0},
        {"array_rmw_add1_leaf", "array_rmw_add1_leaf_seed_route",
         "kilo_leaf_array_rmw_add1_7block", 0},
        {"sum_variant_tag_local", "sum_variant_tag_seed_route",
         "sum_variant_tag_local_aggregate_seed", 0},
        {"sum_variant_project_local", "sum_variant_project_seed_route",
         "sum_variant_project_local_aggregate_seed", 0},
        {"userbox_point_local_scalar", "userbox_local_scalar_seed_route",
         "userbox_point_field_local_scalar_seed", 0},
        {"userbox_flag_pointf_local_scalar", "userbox_local_scalar_seed_route",
         "userbox_flag_field_local_scalar_seed", 0},
        {"userbox_flag_pointf_local_scalar", "userbox_local_scalar_seed_route",
         "userbox_pointf_field_local_scalar_seed", 0},
        {"userbox_loop_micro", "userbox_loop_micro_seed_route",
         "userbox_point_add_loop_micro_seed", 0},
        {"userbox_loop_micro", "userbox_loop_micro_seed_route",
         "userbox_flag_toggle_loop_micro_seed", 0},
        {"userbox_known_receiver_method_seed",
         "userbox_known_receiver_method_seed_route",
         "userbox_counter_step_local_i64_seed", 0},
        {"userbox_known_receiver_method_seed",
         "userbox_known_receiver_method_seed_route",
         "userbox_counter_step_chain_local_i64_seed", 0},
        {"userbox_known_receiver_method_seed",
         "userbox_known_receiver_method_seed_route",
         "userbox_counter_step_chain_micro_seed", 0},
        {"userbox_known_receiver_method_seed",
         "userbox_known_receiver_method_seed_route",
         "userbox_counter_step_micro_seed", 0},
        {"userbox_known_receiver_method_seed",
         "userbox_known_receiver_method_seed_route",
         "userbox_point_sum_local_i64_seed", 0},
        {"userbox_known_receiver_method_seed",
         "userbox_known_receiver_method_seed_route",
         "userbox_point_sum_micro_seed", 0},
        {"concat_const_suffix_micro", "concat_const_suffix_micro_seed_route",
         "kilo_micro_concat_const_suffix_5block", 0},
        {"substring_views_only_micro", "substring_views_micro_seed_route",
         "kilo_micro_substring_views_only_5block", 0},
        {"substring_concat_loop_ascii", "string_kernel_plans.loop_payload",
         "string_kernel_plan_concat_triplet_loop_payload", 1},
    };

static int hako_llvmc_exact_seed_backend_route_supported(
    const char* tag,
    const char* source_route,
    const char* proof,
    long long selected_value) {
  size_t i;
  if (!(tag && source_route && proof)) {
    return 0;
  }
  for (i = 0;
       i < sizeof(hako_llvmc_exact_seed_backend_route_rows) /
               sizeof(hako_llvmc_exact_seed_backend_route_rows[0]);
       i++) {
    const struct HakoLlvmcExactSeedBackendRouteRow* row =
        &hako_llvmc_exact_seed_backend_route_rows[i];
    if (strcmp(tag, row->tag) != 0) {
      continue;
    }
    if (strcmp(source_route, row->source_route) != 0) {
      continue;
    }
    if (strcmp(proof, row->proof) != 0) {
      continue;
    }
    if (row->requires_positive_selected_value) {
      if (selected_value > 0) {
        return 1;
      }
      continue;
    }
    if (selected_value == 0) {
      return 1;
    }
  }
  return 0;
}
