static int hako_llvmc_userbox_loop_micro_shape_ok(
    const char* kind,
    const char* box_name,
    const char* proof,
    const char* consumer_capability,
    const char* publication_boundary,
    long long block_count,
    long long newbox_block,
    long long newbox_instruction_index,
    long long box_value,
    long long ops,
    long long flip_at,
    long long field_get_count,
    long long field_set_count,
    long long compare_lt_count,
    long long compare_eq_count,
    long long binop_count) {
  if (!(kind && box_name && proof && consumer_capability && publication_boundary &&
        hako_llvmc_str_eq(consumer_capability, "direct_userbox_loop_micro") &&
        hako_llvmc_str_eq(publication_boundary, "none") &&
        block_count == 4 &&
        newbox_block >= 0 &&
        newbox_instruction_index >= 0 &&
        box_value > 0 &&
        ops == 2000000)) {
    return 0;
  }

  if (hako_llvmc_str_eq(kind, "point_add_micro")) {
    return hako_llvmc_str_eq(box_name, "Point") &&
           hako_llvmc_str_eq(proof, "userbox_point_add_loop_micro_seed") &&
           flip_at == 0 &&
           field_get_count == 6 &&
           field_set_count == 4 &&
           compare_lt_count == 1 &&
           compare_eq_count == 0 &&
           binop_count == 7;
  }

  if (hako_llvmc_str_eq(kind, "flag_toggle_micro")) {
    return hako_llvmc_str_eq(box_name, "Flag") &&
           hako_llvmc_str_eq(proof, "userbox_flag_toggle_loop_micro_seed") &&
           flip_at == 1000000 &&
           field_get_count == 2 &&
           field_set_count == 2 &&
           compare_lt_count == 2 &&
           compare_eq_count == 2 &&
           binop_count == 3;
  }

  return 0;
}

static int hako_llvmc_consume_userbox_loop_micro_route(
    const char* json_in,
    const char* obj_out) {
  struct hako_llvmc_userbox_route_context ctx;
  const char* kind = NULL;
  const char* box_name = NULL;
  const char* proof = NULL;
  const char* consumer_capability = NULL;
  const char* publication_boundary = NULL;
  long long block_count = 0;
  long long newbox_block = -1;
  long long newbox_instruction_index = -1;
  long long box_value = 0;
  long long ops = 0;
  long long flip_at = 0;
  long long field_get_count = 0;
  long long field_set_count = 0;
  long long compare_lt_count = 0;
  long long compare_eq_count = 0;
  long long binop_count = 0;
  char extra[128];
  int rc = -1;

  if (!hako_llvmc_userbox_route_context_open(
          json_in,
          "userbox_loop_micro_seed_route",
          &ctx)) {
    return -1;
  }

  kind = yyjson_get_str(yyjson_obj_get(ctx.route, "kind"));
  box_name = yyjson_get_str(yyjson_obj_get(ctx.route, "box"));
  proof = yyjson_get_str(yyjson_obj_get(ctx.route, "proof"));
  consumer_capability = yyjson_get_str(yyjson_obj_get(ctx.route, "consumer_capability"));
  publication_boundary = yyjson_get_str(yyjson_obj_get(ctx.route, "publication_boundary"));

  if (!hako_llvmc_userbox_route_get_i64(ctx.route, "block_count", &block_count) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "newbox_block", &newbox_block) ||
      !hako_llvmc_userbox_route_get_i64(
          ctx.route, "newbox_instruction_index", &newbox_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "box_value", &box_value) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "ops", &ops) ||
      !hako_llvmc_userbox_route_get_optional_i64(ctx.route, "flip_at", &flip_at) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "field_get_count", &field_get_count) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "field_set_count", &field_set_count) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "compare_lt_count", &compare_lt_count) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "compare_eq_count", &compare_eq_count) ||
      !hako_llvmc_userbox_route_get_i64(ctx.route, "binop_count", &binop_count) ||
      !hako_llvmc_userbox_loop_micro_shape_ok(
          kind,
          box_name,
          proof,
          consumer_capability,
          publication_boundary,
          block_count,
          newbox_block,
          newbox_instruction_index,
          box_value,
          ops,
          flip_at,
          field_get_count,
          field_set_count,
          compare_lt_count,
          compare_eq_count,
          binop_count)) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          "userbox_loop_micro",
          "skip",
          "metadata_contract_mismatch",
          "userbox_loop_micro_seed_route");
    }
    hako_llvmc_userbox_route_context_close(&ctx);
    return -1;
  }

  if (hako_llvmc_str_eq(kind, "point_add_micro")) {
    if (!(hako_llvmc_userbox_point_inline_scalar_selection_ok(ctx.fn) &&
          hako_llvmc_userbox_point_decl_ok(ctx.root))) {
      hako_llvmc_userbox_route_context_close(&ctx);
      return -1;
    }
    if (hako_llvmc_route_trace_enabled()) {
      snprintf(extra, sizeof(extra), "%s mir_route_plan", kind);
      hako_llvmc_emit_route_trace(
          "userbox_loop_micro",
          "emit",
          "exact_match",
          extra);
    }
    rc = hako_llvmc_emit_userbox_point_add_micro_ir(
        obj_out,
        &ctx.selection,
        ops);
    hako_llvmc_userbox_route_context_close(&ctx);
    return rc;
  }

  if (hako_llvmc_str_eq(kind, "flag_toggle_micro")) {
    if (!(hako_llvmc_userbox_flag_inline_scalar_selection_ok(ctx.fn) &&
          hako_llvmc_userbox_flag_decl_ok(ctx.root))) {
      hako_llvmc_userbox_route_context_close(&ctx);
      return -1;
    }
    if (hako_llvmc_route_trace_enabled()) {
      snprintf(extra, sizeof(extra), "%s mir_route_plan", kind);
      hako_llvmc_emit_route_trace(
          "userbox_loop_micro",
          "emit",
          "exact_match",
          extra);
    }
    rc = hako_llvmc_emit_userbox_flag_toggle_micro_ir(
        obj_out,
        &ctx.selection,
        ops,
        flip_at);
    hako_llvmc_userbox_route_context_close(&ctx);
    return rc;
  }

  hako_llvmc_userbox_route_context_close(&ctx);
  return -1;
}
