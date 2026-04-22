static int hako_llvmc_userbox_known_receiver_method_route_shape_ok(
    const char* kind,
    const char* box_name,
    const char* method,
    const char* method_function,
    const char* proof,
    const char* consumer_capability,
    const char* publication_boundary,
    long long block_count,
    long long method_block_count,
    long long block,
    long long method_block,
    long long newbox_instruction_index,
    long long copy_instruction_index,
    long long call_instruction_index,
    long long box_value,
    long long copy_value,
    long long result_value) {
  if (!(kind && box_name && method && method_function && proof &&
        consumer_capability && publication_boundary &&
        hako_llvmc_str_eq(consumer_capability, "direct_userbox_known_receiver_method_local") &&
        hako_llvmc_str_eq(publication_boundary, "none") &&
        block_count == 1 &&
        method_block_count == 1 &&
        block >= 0 &&
        method_block >= 0 &&
        newbox_instruction_index >= 0 &&
        call_instruction_index >= 0 &&
        box_value > 0 &&
        result_value > 0)) {
    return 0;
  }

  if (hako_llvmc_str_eq(kind, "counter_step_local_i64")) {
    return hako_llvmc_str_eq(box_name, "Counter") &&
           hako_llvmc_str_eq(method, "step") &&
           hako_llvmc_str_eq(method_function, "Counter.step/1") &&
           hako_llvmc_str_eq(proof, "userbox_counter_step_local_i64_seed") &&
           newbox_instruction_index == 1 &&
           copy_instruction_index == 0 &&
           call_instruction_index == 3 &&
           copy_value == 0;
  }

  if (hako_llvmc_str_eq(kind, "counter_step_copy_local_i64")) {
    return hako_llvmc_str_eq(box_name, "Counter") &&
           hako_llvmc_str_eq(method, "step") &&
           hako_llvmc_str_eq(method_function, "Counter.step/1") &&
           hako_llvmc_str_eq(proof, "userbox_counter_step_local_i64_seed") &&
           newbox_instruction_index == 1 &&
           copy_instruction_index == 3 &&
           call_instruction_index == 4 &&
           copy_value > 0;
  }

  if (hako_llvmc_str_eq(kind, "point_sum_local_i64")) {
    return hako_llvmc_str_eq(box_name, "Point") &&
           hako_llvmc_str_eq(method, "sum") &&
           hako_llvmc_str_eq(method_function, "Point.sum/1") &&
           hako_llvmc_str_eq(proof, "userbox_point_sum_local_i64_seed") &&
           newbox_instruction_index == 2 &&
           copy_instruction_index == 0 &&
           call_instruction_index == 5 &&
           copy_value == 0;
  }

  if (hako_llvmc_str_eq(kind, "point_sum_copy_local_i64")) {
    return hako_llvmc_str_eq(box_name, "Point") &&
           hako_llvmc_str_eq(method, "sum") &&
           hako_llvmc_str_eq(method_function, "Point.sum/1") &&
           hako_llvmc_str_eq(proof, "userbox_point_sum_local_i64_seed") &&
           newbox_instruction_index == 2 &&
           copy_instruction_index == 5 &&
           call_instruction_index == 6 &&
           copy_value > 0;
  }

  return 0;
}

static int hako_llvmc_consume_userbox_known_receiver_method_seed_route(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn;
  yyjson_val* method_fn;
  yyjson_val* metadata;
  yyjson_val* route;
  hako_llvmc_entry_rune_selection selection;
  const char* kind = NULL;
  const char* box_name = NULL;
  const char* method = NULL;
  const char* method_function = NULL;
  const char* proof = NULL;
  const char* consumer_capability = NULL;
  const char* publication_boundary = NULL;
  long long block_count = 0;
  long long method_block_count = 0;
  long long block = -1;
  long long method_block = -1;
  long long newbox_instruction_index = -1;
  long long copy_instruction_index = 0;
  long long call_instruction_index = -1;
  long long box_value = 0;
  long long copy_value = 0;
  long long result_value = 0;
  long long base_i64 = 0;
  long long delta_i64 = 0;
  long long x_i64 = 0;
  long long y_i64 = 0;
  char extra[128];
  int rc = -1;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  fn = hako_llvmc_find_entry_function(fns);
  if (!(fn && yyjson_is_obj(fn))) {
    yyjson_doc_free(doc);
    return -1;
  }
  hako_llvmc_select_entry_runes(fn, &selection);

  metadata = yyjson_obj_get(fn, "metadata");
  route = metadata ? yyjson_obj_get(metadata, "userbox_known_receiver_method_seed_route") : NULL;
  if (!(route && yyjson_is_obj(route))) {
    yyjson_doc_free(doc);
    return -1;
  }

  kind = yyjson_get_str(yyjson_obj_get(route, "kind"));
  box_name = yyjson_get_str(yyjson_obj_get(route, "box"));
  method = yyjson_get_str(yyjson_obj_get(route, "method"));
  method_function = yyjson_get_str(yyjson_obj_get(route, "method_function"));
  proof = yyjson_get_str(yyjson_obj_get(route, "proof"));
  consumer_capability = yyjson_get_str(yyjson_obj_get(route, "consumer_capability"));
  publication_boundary = yyjson_get_str(yyjson_obj_get(route, "publication_boundary"));

  if (!hako_llvmc_userbox_route_get_i64(route, "block_count", &block_count) ||
      !hako_llvmc_userbox_route_get_i64(route, "method_block_count", &method_block_count) ||
      !hako_llvmc_userbox_route_get_i64(route, "block", &block) ||
      !hako_llvmc_userbox_route_get_i64(route, "method_block", &method_block) ||
      !hako_llvmc_userbox_route_get_i64(
          route, "newbox_instruction_index", &newbox_instruction_index) ||
      !hako_llvmc_userbox_route_get_optional_i64(
          route, "copy_instruction_index", &copy_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(
          route, "call_instruction_index", &call_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(route, "box_value", &box_value) ||
      !hako_llvmc_userbox_route_get_optional_i64(route, "copy_value", &copy_value) ||
      !hako_llvmc_userbox_route_get_i64(route, "result_value", &result_value) ||
      !hako_llvmc_userbox_known_receiver_method_route_shape_ok(
          kind,
          box_name,
          method,
          method_function,
          proof,
          consumer_capability,
          publication_boundary,
          block_count,
          method_block_count,
          block,
          method_block,
          newbox_instruction_index,
          copy_instruction_index,
          call_instruction_index,
          box_value,
          copy_value,
          result_value)) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          "userbox_known_receiver_method_seed",
          "skip",
          "metadata_contract_mismatch",
          "userbox_known_receiver_method_seed_route");
    }
    yyjson_doc_free(doc);
    return -1;
  }

  method_fn = hako_llvmc_find_named_function(fns, method_function);
  if (!method_fn) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (hako_llvmc_str_eq(kind, "counter_step_local_i64") ||
      hako_llvmc_str_eq(kind, "counter_step_copy_local_i64")) {
    if (!(hako_llvmc_userbox_route_get_i64(route, "base_i64", &base_i64) &&
          hako_llvmc_userbox_route_get_i64(route, "delta_i64", &delta_i64) &&
          hako_llvmc_userbox_counter_decl_ok(root) &&
          hako_llvmc_userbox_counter_inline_scalar_selection_ok(fn, method_fn) &&
          hako_llvmc_userbox_has_known_receiver_selection(fn, "Counter.step"))) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (hako_llvmc_route_trace_enabled()) {
      snprintf(extra, sizeof(extra), "%s mir_route_plan", kind);
      hako_llvmc_emit_route_trace(
          "userbox_known_receiver_method_seed",
          "emit",
          "exact_match",
          extra);
    }
    rc = hako_llvmc_emit_userbox_counter_step_local_i64_ir(
        obj_out, &selection, base_i64, delta_i64);
    yyjson_doc_free(doc);
    return rc;
  }

  if (hako_llvmc_str_eq(kind, "point_sum_local_i64") ||
      hako_llvmc_str_eq(kind, "point_sum_copy_local_i64")) {
    if (!(hako_llvmc_userbox_route_get_i64(route, "x_i64", &x_i64) &&
          hako_llvmc_userbox_route_get_i64(route, "y_i64", &y_i64) &&
          hako_llvmc_userbox_point_decl_ok(root) &&
          hako_llvmc_userbox_point_sum_local_i64_selection_ok(fn, method_fn) &&
          hako_llvmc_userbox_has_known_receiver_selection(fn, "Point.sum"))) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (hako_llvmc_route_trace_enabled()) {
      snprintf(extra, sizeof(extra), "%s mir_route_plan", kind);
      hako_llvmc_emit_route_trace(
          "userbox_known_receiver_method_seed",
          "emit",
          "exact_match",
          extra);
    }
    rc = hako_llvmc_emit_userbox_point_sum_local_i64_ir(
        obj_out, &selection, x_i64, y_i64);
    yyjson_doc_free(doc);
    return rc;
  }

  yyjson_doc_free(doc);
  return -1;
}
