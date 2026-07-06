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
    char** err_out) {
  if (strcmp(exact_seed_tag, "array_string_store_micro") == 0) {
    if (hako_llvmc_consume_array_string_store_micro_route(json_in, obj_out) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route array_string_store_micro failed");
  }
  if (strcmp(exact_seed_tag, "array_getset_micro") == 0) {
    if (hako_llvmc_consume_array_getset_micro_route(json_in, obj_out) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route array_getset_micro failed");
  }
  if (strcmp(exact_seed_tag, "array_rmw_add1_leaf") == 0) {
    if (hako_llvmc_consume_array_rmw_add1_leaf_route(json_in, obj_out) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route array_rmw_add1_leaf failed");
  }
  if (strcmp(exact_seed_tag, "sum_variant_tag_local") == 0) {
    if (hako_llvmc_consume_sum_variant_tag_seed_route(json_in, obj_out) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route sum_variant_tag_local failed");
  }
  if (strcmp(exact_seed_tag, "sum_variant_project_local") == 0) {
    if (hako_llvmc_consume_sum_variant_project_seed_route(json_in, obj_out) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route sum_variant_project_local failed");
  }
  if (strcmp(exact_seed_tag, "userbox_point_local_scalar") == 0) {
    if (hako_llvmc_consume_userbox_point_local_scalar_route(json_in, obj_out) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route userbox_point_local_scalar failed");
  }
  if (strcmp(exact_seed_tag, "userbox_flag_pointf_local_scalar") == 0) {
    if (hako_llvmc_consume_userbox_flag_pointf_local_scalar_route(json_in, obj_out) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route userbox_flag_pointf_local_scalar failed");
  }
  if (strcmp(exact_seed_tag, "userbox_loop_micro") == 0) {
    if (hako_llvmc_consume_userbox_loop_micro_route(json_in, obj_out) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route userbox_loop_micro failed");
  }
  if (strcmp(exact_seed_tag, "userbox_known_receiver_method_seed") == 0) {
    if (hako_llvmc_consume_userbox_known_receiver_method_seed_route(json_in, obj_out) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route userbox_known_receiver_method_seed failed");
  }
  if (strcmp(exact_seed_tag, "concat_const_suffix_micro") == 0) {
    if (hako_llvmc_consume_concat_const_suffix_micro_route(json_in, obj_out) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route concat_const_suffix_micro failed");
  }
  if (strcmp(exact_seed_tag, "substring_views_only_micro") == 0) {
    if (hako_llvmc_consume_substring_views_only_micro_route(json_in, obj_out) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route substring_views_only_micro failed");
  }
  if (strcmp(exact_seed_tag, "substring_concat_loop_ascii") == 0) {
    if (hako_llvmc_consume_substring_concat_loop_ascii_route(
            json_in,
            obj_out,
            exact_seed_selected_value) == 0) {
      return 0;
    }
    return set_err_owned(err_out, "exact_seed_backend_route substring_concat_loop_ascii failed");
  }
  return set_err_owned(err_out, "unsupported exact_seed_backend_route tag");
}
