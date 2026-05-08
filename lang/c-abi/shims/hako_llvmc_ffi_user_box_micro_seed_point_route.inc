static int hako_llvmc_userbox_point_route_shape_ok(
    const char* kind,
    long long block,
    long long newbox_instruction_index,
    long long set_x_instruction_index,
    long long set_y_instruction_index,
    long long get_x_instruction_index,
    long long get_y_instruction_index,
    long long point_value,
    long long copy_value,
    long long x_value,
    long long y_value,
    long long get_x_value,
    long long get_y_value,
    long long result_value,
    long long x_i64,
    long long y_i64) {
  if (!(kind &&
        block == 0 &&
        newbox_instruction_index == 2 &&
        set_x_instruction_index == 3 &&
        set_y_instruction_index == 4 &&
        point_value == 3 &&
        x_value == 1 &&
        y_value == 2 &&
        x_i64 == 41 &&
        y_i64 == 2)) {
    return 0;
  }
  if (hako_llvmc_str_eq(kind, "point_local_i64")) {
    return get_x_instruction_index == 5 &&
           get_y_instruction_index == 6 &&
           copy_value == 0 &&
           get_x_value == 4 &&
           get_y_value == 5 &&
           result_value == 6;
  }
  if (hako_llvmc_str_eq(kind, "point_copy_local_i64")) {
    return get_x_instruction_index == 6 &&
           get_y_instruction_index == 7 &&
           copy_value == 6 &&
           get_x_value == 7 &&
           get_y_value == 8 &&
           result_value == 9;
  }
  return 0;
}

static int hako_llvmc_consume_userbox_point_local_scalar_route(
    const char* json_in,
    const char* obj_out) {
  struct hako_llvmc_userbox_route_context ctx;
  const char* kind = NULL;
  const char* box_name = NULL;
  const char* x_field = NULL;
  const char* y_field = NULL;
  const char* proof = NULL;
  const char* consumer_capability = NULL;
  const char* publication_boundary = NULL;
  long long block = -1;
  long long newbox_instruction_index = -1;
  long long set_x_instruction_index = -1;
  long long set_y_instruction_index = -1;
  long long get_x_instruction_index = -1;
  long long get_y_instruction_index = -1;
  long long point_value = 0;
  long long copy_value = 0;
  long long x_value = 0;
  long long y_value = 0;
  long long get_x_value = 0;
  long long get_y_value = 0;
  long long result_value = 0;
  long long x_i64 = 0;
  long long y_i64 = 0;
  char extra[128];
  int rc;

  if (!hako_llvmc_userbox_route_context_open(
          json_in,
          "userbox_local_scalar_seed_route",
          &ctx)) {
    return -1;
  }

  kind = yyjson_get_str(yyjson_obj_get(ctx.route, "kind"));
  box_name = yyjson_get_str(yyjson_obj_get(ctx.route, "box"));
  x_field = yyjson_get_str(yyjson_obj_get(ctx.route, "x_field"));
  y_field = yyjson_get_str(yyjson_obj_get(ctx.route, "y_field"));
  proof = yyjson_get_str(yyjson_obj_get(ctx.route, "proof"));
  consumer_capability = yyjson_get_str(yyjson_obj_get(ctx.route, "consumer_capability"));
  publication_boundary = yyjson_get_str(yyjson_obj_get(ctx.route, "publication_boundary"));

  if (!hako_llvmc_userbox_route_get_i64(ctx.route, "block", &block) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "newbox_instruction_index", &newbox_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "set_x_instruction_index", &set_x_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "set_y_instruction_index", &set_y_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "get_x_instruction_index", &get_x_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "get_y_instruction_index", &get_y_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "point_value", &point_value) ||
      !hako_llvmc_userbox_route_get_optional_i64(ctx.route, "copy_value", &copy_value) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "x_value", &x_value) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "y_value", &y_value) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "get_x_value", &get_x_value) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "get_y_value", &get_y_value) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "result_value", &result_value) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "x_i64", &x_i64) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "y_i64", &y_i64)) {
    hako_llvmc_userbox_route_context_close(&ctx);
    return -1;
  }

  if (!(kind && box_name && x_field && y_field && proof &&
        consumer_capability && publication_boundary &&
        hako_llvmc_str_eq(box_name, "Point") &&
        hako_llvmc_str_eq(x_field, "x") &&
        hako_llvmc_str_eq(y_field, "y") &&
        hako_llvmc_str_eq(proof, "userbox_point_field_local_scalar_seed") &&
        hako_llvmc_str_eq(consumer_capability, "direct_userbox_point_local_scalar") &&
        hako_llvmc_str_eq(publication_boundary, "none") &&
        hako_llvmc_userbox_point_inline_scalar_selection_ok(ctx.fn) &&
        hako_llvmc_userbox_point_decl_ok(ctx.root) &&
        hako_llvmc_userbox_point_route_shape_ok(
            kind,
            block,
            newbox_instruction_index,
            set_x_instruction_index,
            set_y_instruction_index,
            get_x_instruction_index,
            get_y_instruction_index,
            point_value,
            copy_value,
            x_value,
            y_value,
            get_x_value,
            get_y_value,
            result_value,
            x_i64,
            y_i64))) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          "userbox_point_local_scalar",
          "skip",
          "metadata_contract_mismatch",
          "userbox_local_scalar_seed_route");
    }
    hako_llvmc_userbox_route_context_close(&ctx);
    return -1;
  }

  if (hako_llvmc_route_trace_enabled()) {
    snprintf(extra, sizeof(extra), "%s mir_route_plan", kind);
    hako_llvmc_emit_route_trace(
        "userbox_point_local_scalar",
        "emit",
        "exact_match",
        extra);
  }

  rc = hako_llvmc_emit_userbox_point_local_i64_ir(obj_out, &ctx.selection);
  hako_llvmc_userbox_route_context_close(&ctx);
  return rc;
}
