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
