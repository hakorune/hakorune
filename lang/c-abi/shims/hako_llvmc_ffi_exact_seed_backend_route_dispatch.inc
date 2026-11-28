/*
 * Exact-seed backend route dispatcher.
 *
 * The registry owns tag/source/proof acceptance. This dispatcher only maps an
 * already-accepted tag to the existing narrow seed consumer.
 */

static int hako_llvmc_dispatch_exact_seed_backend_route(
    const char* json_in,
    const char* obj_out,
    const char* exact_seed_tag,
    long long exact_seed_selected_value,
    const struct HakoLlvmcPhysicalOptions* options,
    char** err_out) {
  if (strcmp(exact_seed_tag, "substring_concat_loop_ascii") == 0) {
    if (hako_llvmc_consume_substring_concat_loop_ascii_route(
            json_in,
            obj_out,
            exact_seed_selected_value,
            options) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route substring_concat_loop_ascii failed");
  }
  return set_err_owned(err_out, "unsupported exact_seed_backend_route tag");
}
