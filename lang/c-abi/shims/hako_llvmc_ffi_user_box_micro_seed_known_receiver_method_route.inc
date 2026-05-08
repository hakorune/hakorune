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
        (hako_llvmc_str_eq(
             consumer_capability, "direct_userbox_known_receiver_method_local") ||
         hako_llvmc_str_eq(
             consumer_capability, "direct_userbox_known_receiver_method_micro")) &&
        hako_llvmc_str_eq(publication_boundary, "none") &&
        block_count >= 1 &&
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
           hako_llvmc_str_eq(
               consumer_capability, "direct_userbox_known_receiver_method_local") &&
           block_count == 1 &&
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
           hako_llvmc_str_eq(
               consumer_capability, "direct_userbox_known_receiver_method_local") &&
           block_count == 1 &&
           hako_llvmc_str_eq(method, "step") &&
           hako_llvmc_str_eq(method_function, "Counter.step/1") &&
           hako_llvmc_str_eq(proof, "userbox_counter_step_local_i64_seed") &&
           newbox_instruction_index == 1 &&
           copy_instruction_index == 3 &&
           call_instruction_index == 4 &&
           copy_value > 0;
  }

  if (hako_llvmc_str_eq(kind, "counter_step_chain_local_i64")) {
    return hako_llvmc_str_eq(box_name, "Counter") &&
           hako_llvmc_str_eq(
               consumer_capability, "direct_userbox_known_receiver_method_local") &&
           block_count == 1 &&
           hako_llvmc_str_eq(method, "step_chain") &&
           hako_llvmc_str_eq(method_function, "Counter.step_chain/1") &&
           hako_llvmc_str_eq(proof, "userbox_counter_step_chain_local_i64_seed") &&
           newbox_instruction_index == 1 &&
           copy_instruction_index == 3 &&
           call_instruction_index == 4 &&
           copy_value > 0;
  }

  if (hako_llvmc_str_eq(kind, "counter_step_chain_micro")) {
    return hako_llvmc_str_eq(box_name, "Counter") &&
           hako_llvmc_str_eq(
               consumer_capability, "direct_userbox_known_receiver_method_local") &&
           block_count >= 4 &&
           hako_llvmc_str_eq(method, "step_chain") &&
           hako_llvmc_str_eq(method_function, "Counter.step_chain/0") &&
           hako_llvmc_str_eq(proof, "userbox_counter_step_chain_micro_seed") &&
           copy_instruction_index == 0 &&
           copy_value == 0;
  }

  if (hako_llvmc_str_eq(kind, "counter_step_micro")) {
    return hako_llvmc_str_eq(box_name, "Counter") &&
           hako_llvmc_str_eq(
               consumer_capability, "direct_userbox_known_receiver_method_micro") &&
           block_count >= 4 &&
           hako_llvmc_str_eq(method, "step") &&
           hako_llvmc_str_eq(method_function, "Counter.step/0") &&
           hako_llvmc_str_eq(proof, "userbox_counter_step_micro_seed") &&
           copy_instruction_index == 0 &&
           copy_value == 0;
  }

  if (hako_llvmc_str_eq(kind, "point_sum_local_i64")) {
    return hako_llvmc_str_eq(box_name, "Point") &&
           hako_llvmc_str_eq(
               consumer_capability, "direct_userbox_known_receiver_method_local") &&
           block_count == 1 &&
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
           hako_llvmc_str_eq(
               consumer_capability, "direct_userbox_known_receiver_method_local") &&
           block_count == 1 &&
           hako_llvmc_str_eq(method, "sum") &&
           hako_llvmc_str_eq(method_function, "Point.sum/1") &&
           hako_llvmc_str_eq(proof, "userbox_point_sum_local_i64_seed") &&
           newbox_instruction_index == 2 &&
           copy_instruction_index == 5 &&
           call_instruction_index == 6 &&
           copy_value > 0;
  }

  if (hako_llvmc_str_eq(kind, "point_sum_micro")) {
    return hako_llvmc_str_eq(box_name, "Point") &&
           hako_llvmc_str_eq(
               consumer_capability, "direct_userbox_known_receiver_method_micro") &&
           block_count >= 4 &&
           hako_llvmc_str_eq(method, "sum") &&
           hako_llvmc_str_eq(method_function, "Point.sum/0") &&
           hako_llvmc_str_eq(proof, "userbox_point_sum_micro_seed") &&
           copy_instruction_index == 0 &&
           copy_value == 0;
  }

  return 0;
}

static int hako_llvmc_consume_userbox_known_receiver_method_seed_route(
    const char* json_in,
    const char* obj_out) {
  struct hako_llvmc_userbox_route_context ctx;
  yyjson_val* method_fn;
  yyjson_val* leaf_method_fn = NULL;
  const char* kind = NULL;
  const char* box_name = NULL;
  const char* method = NULL;
  const char* method_function = NULL;
  const char* leaf_method_function = NULL;
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
  long long ops = 0;
  long long step_i64 = 0;
  long long sum_i64 = 0;
  long long known_receiver_count = 0;
  long long field_set_count = 0;
  long long compare_lt_count = 0;
  long long branch_count = 0;
  long long jump_count = 0;
  long long ret_count = 0;
  long long add_count = 0;
  long long leaf_method_block_count = 0;
  long long leaf_method_block = -1;
  char extra[128];
  int rc = -1;

  if (!hako_llvmc_userbox_route_context_open(
          json_in,
          "userbox_known_receiver_method_seed_route",
          &ctx)) {
    return -1;
  }

  kind = yyjson_get_str(yyjson_obj_get(ctx.route, "kind"));
  box_name = yyjson_get_str(yyjson_obj_get(ctx.route, "box"));
  method = yyjson_get_str(yyjson_obj_get(ctx.route, "method"));
  method_function = yyjson_get_str(yyjson_obj_get(ctx.route, "method_function"));
  leaf_method_function = yyjson_get_str(yyjson_obj_get(ctx.route, "leaf_method_function"));
  proof = yyjson_get_str(yyjson_obj_get(ctx.route, "proof"));
  consumer_capability = yyjson_get_str(yyjson_obj_get(ctx.route, "consumer_capability"));
  publication_boundary = yyjson_get_str(yyjson_obj_get(ctx.route, "publication_boundary"));

  if (!hako_llvmc_userbox_route_get_i64(ctx.route, "block_count", &block_count) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "method_block_count", &method_block_count) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "block", &block) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "method_block", &method_block) ||
      !hako_llvmc_userbox_route_get_i64(
          ctx.route, "newbox_instruction_index", &newbox_instruction_index) ||
      !hako_llvmc_userbox_route_get_optional_i64(
          ctx.route, "copy_instruction_index", &copy_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(
          ctx.route, "call_instruction_index", &call_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "box_value", &box_value) ||
      !hako_llvmc_userbox_route_get_optional_i64(ctx.route, "copy_value", &copy_value) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "result_value", &result_value) ||
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
    hako_llvmc_userbox_route_context_close(&ctx);
    return -1;
  }

  method_fn = hako_llvmc_find_named_function(ctx.fns, method_function);
  if (!method_fn) {
    hako_llvmc_userbox_route_context_close(&ctx);
    return -1;
  }

  if (hako_llvmc_str_eq(kind, "counter_step_local_i64") ||
      hako_llvmc_str_eq(kind, "counter_step_copy_local_i64")) {
    if (!(hako_llvmc_userbox_route_get_i64(ctx.route, "base_i64", &base_i64) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "delta_i64", &delta_i64) &&
          hako_llvmc_userbox_counter_decl_ok(ctx.root) &&
          hako_llvmc_userbox_counter_inline_scalar_selection_ok(ctx.fn, method_fn) &&
          hako_llvmc_userbox_has_known_receiver_selection(ctx.fn, "Counter.step"))) {
      hako_llvmc_userbox_route_context_close(&ctx);
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
        obj_out, &ctx.selection, base_i64, delta_i64);
    hako_llvmc_userbox_route_context_close(&ctx);
    return rc;
  }

  if (hako_llvmc_str_eq(kind, "counter_step_chain_local_i64") ||
      hako_llvmc_str_eq(kind, "counter_step_chain_micro")) {
    if (!(leaf_method_function &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "base_i64", &base_i64) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "delta_i64", &delta_i64) &&
          hako_llvmc_userbox_route_get_i64(
              ctx.route, "known_receiver_count", &known_receiver_count) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "field_set_count", &field_set_count) &&
          hako_llvmc_userbox_route_get_i64(
              ctx.route, "leaf_method_block_count", &leaf_method_block_count) &&
          hako_llvmc_userbox_route_get_i64(
              ctx.route, "leaf_method_block", &leaf_method_block) &&
          base_i64 == 41 &&
          delta_i64 == 2 &&
          field_set_count == 1 &&
          leaf_method_block_count == 1 &&
          leaf_method_block >= 0 &&
          hako_llvmc_userbox_counter_decl_ok(ctx.root) &&
          hako_llvmc_userbox_has_inline_scalar_selection(
              ctx.fn, "user_box_field_set", "Counter.value") &&
          hako_llvmc_userbox_has_known_receiver_selection(ctx.fn, "Counter.step_chain"))) {
      hako_llvmc_userbox_route_context_close(&ctx);
      return -1;
    }

    leaf_method_fn = hako_llvmc_find_named_function(ctx.fns, leaf_method_function);
    if (!(leaf_method_fn &&
          hako_llvmc_userbox_has_inline_scalar_selection(
              leaf_method_fn, "user_box_field_get", "Counter.value"))) {
      hako_llvmc_userbox_route_context_close(&ctx);
      return -1;
    }

    if (hako_llvmc_str_eq(kind, "counter_step_chain_micro")) {
      if (!(hako_llvmc_userbox_route_get_i64(ctx.route, "ops", &ops) &&
            ops == 2000000 &&
            known_receiver_count == 2 &&
            hako_llvmc_userbox_has_known_receiver_selection(
                method_fn, "Counter.step"))) {
        hako_llvmc_userbox_route_context_close(&ctx);
        return -1;
      }
    } else if (known_receiver_count != 1) {
      hako_llvmc_userbox_route_context_close(&ctx);
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
        obj_out, &ctx.selection, base_i64, delta_i64);
    hako_llvmc_userbox_route_context_close(&ctx);
    return rc;
  }

  if (hako_llvmc_str_eq(kind, "counter_step_micro")) {
    if (!(hako_llvmc_userbox_route_get_i64(ctx.route, "base_i64", &base_i64) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "delta_i64", &delta_i64) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "ops", &ops) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "step_i64", &step_i64) &&
          hako_llvmc_userbox_route_get_i64(
              ctx.route, "known_receiver_count", &known_receiver_count) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "field_set_count", &field_set_count) &&
          base_i64 == 41 &&
          delta_i64 == 2 &&
          step_i64 == 43 &&
          ops == 2000000 &&
          known_receiver_count == 2 &&
          field_set_count == 1 &&
          hako_llvmc_userbox_counter_decl_ok(ctx.root) &&
          hako_llvmc_userbox_counter_inline_scalar_selection_ok(ctx.fn, method_fn) &&
          hako_llvmc_userbox_has_known_receiver_selection(ctx.fn, "Counter.step"))) {
      hako_llvmc_userbox_route_context_close(&ctx);
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
    rc = hako_llvmc_emit_userbox_counter_step_micro_ir(
        obj_out, &ctx.selection, ops, step_i64);
    hako_llvmc_userbox_route_context_close(&ctx);
    return rc;
  }

  if (hako_llvmc_str_eq(kind, "point_sum_local_i64") ||
      hako_llvmc_str_eq(kind, "point_sum_copy_local_i64")) {
    if (!(hako_llvmc_userbox_route_get_i64(ctx.route, "x_i64", &x_i64) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "y_i64", &y_i64) &&
          hako_llvmc_userbox_point_decl_ok(ctx.root) &&
          hako_llvmc_userbox_point_sum_local_i64_selection_ok(ctx.fn, method_fn) &&
          hako_llvmc_userbox_has_known_receiver_selection(ctx.fn, "Point.sum"))) {
      hako_llvmc_userbox_route_context_close(&ctx);
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
        obj_out, &ctx.selection, x_i64, y_i64);
    hako_llvmc_userbox_route_context_close(&ctx);
    return rc;
  }

  if (hako_llvmc_str_eq(kind, "point_sum_micro")) {
    if (!(hako_llvmc_userbox_route_get_i64(ctx.route, "x_i64", &x_i64) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "y_i64", &y_i64) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "ops", &ops) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "sum_i64", &sum_i64) &&
          hako_llvmc_userbox_route_get_i64(
              ctx.route, "known_receiver_count", &known_receiver_count) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "field_set_count", &field_set_count) &&
          hako_llvmc_userbox_route_get_i64(
              ctx.route, "compare_lt_count", &compare_lt_count) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "branch_count", &branch_count) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "jump_count", &jump_count) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "ret_count", &ret_count) &&
          hako_llvmc_userbox_route_get_i64(ctx.route, "add_count", &add_count) &&
          x_i64 == 1 &&
          y_i64 == 2 &&
          sum_i64 == 3 &&
          ops == 2000000 &&
          known_receiver_count == 2 &&
          field_set_count == 2 &&
          compare_lt_count == 1 &&
          branch_count == 1 &&
          jump_count >= 2 &&
          ret_count == 1 &&
          add_count >= 3 &&
          hako_llvmc_userbox_point_decl_ok(ctx.root) &&
          hako_llvmc_userbox_point_sum_local_i64_selection_ok(ctx.fn, method_fn) &&
          hako_llvmc_userbox_has_known_receiver_selection(ctx.fn, "Point.sum"))) {
      hako_llvmc_userbox_route_context_close(&ctx);
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
    rc = hako_llvmc_emit_userbox_point_sum_micro_ir(
        obj_out, &ctx.selection, ops, sum_i64);
    hako_llvmc_userbox_route_context_close(&ctx);
    return rc;
  }

  hako_llvmc_userbox_route_context_close(&ctx);
  return -1;
}
