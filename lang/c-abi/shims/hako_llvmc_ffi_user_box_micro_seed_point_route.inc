static int hako_llvmc_userbox_route_get_i64(
    yyjson_val* route,
    const char* field,
    long long* out) {
  yyjson_val* value = route ? yyjson_obj_get(route, field) : NULL;
  if (!(value && yyjson_is_num(value) && out)) {
    return 0;
  }
  *out = (long long)yyjson_get_sint(value);
  return 1;
}

static int hako_llvmc_userbox_route_get_optional_i64(
    yyjson_val* route,
    const char* field,
    long long* out) {
  yyjson_val* value = route ? yyjson_obj_get(route, field) : NULL;
  if (!out) {
    return 0;
  }
  *out = 0;
  if (!value || yyjson_is_null(value)) {
    return 1;
  }
  if (!yyjson_is_num(value)) {
    return 0;
  }
  *out = (long long)yyjson_get_sint(value);
  return 1;
}

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
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn;
  yyjson_val* metadata;
  yyjson_val* route;
  hako_llvmc_entry_rune_selection selection;
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
  route = metadata ? yyjson_obj_get(metadata, "userbox_local_scalar_seed_route") : NULL;
  if (!(route && yyjson_is_obj(route))) {
    yyjson_doc_free(doc);
    return -1;
  }

  kind = yyjson_get_str(yyjson_obj_get(route, "kind"));
  box_name = yyjson_get_str(yyjson_obj_get(route, "box"));
  x_field = yyjson_get_str(yyjson_obj_get(route, "x_field"));
  y_field = yyjson_get_str(yyjson_obj_get(route, "y_field"));
  proof = yyjson_get_str(yyjson_obj_get(route, "proof"));
  consumer_capability = yyjson_get_str(yyjson_obj_get(route, "consumer_capability"));
  publication_boundary = yyjson_get_str(yyjson_obj_get(route, "publication_boundary"));

  if (!hako_llvmc_userbox_route_get_i64(route, "block", &block) ||
      !hako_llvmc_userbox_route_get_i64(route, "newbox_instruction_index", &newbox_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(route, "set_x_instruction_index", &set_x_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(route, "set_y_instruction_index", &set_y_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(route, "get_x_instruction_index", &get_x_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(route, "get_y_instruction_index", &get_y_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(route, "point_value", &point_value) ||
      !hako_llvmc_userbox_route_get_optional_i64(route, "copy_value", &copy_value) ||
      !hako_llvmc_userbox_route_get_i64(route, "x_value", &x_value) ||
      !hako_llvmc_userbox_route_get_i64(route, "y_value", &y_value) ||
      !hako_llvmc_userbox_route_get_i64(route, "get_x_value", &get_x_value) ||
      !hako_llvmc_userbox_route_get_i64(route, "get_y_value", &get_y_value) ||
      !hako_llvmc_userbox_route_get_i64(route, "result_value", &result_value) ||
      !hako_llvmc_userbox_route_get_i64(route, "x_i64", &x_i64) ||
      !hako_llvmc_userbox_route_get_i64(route, "y_i64", &y_i64)) {
    yyjson_doc_free(doc);
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
        hako_llvmc_userbox_point_inline_scalar_selection_ok(fn) &&
        hako_llvmc_userbox_point_decl_ok(root) &&
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
    yyjson_doc_free(doc);
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

  rc = hako_llvmc_emit_userbox_point_local_i64_ir(obj_out, &selection);
  yyjson_doc_free(doc);
  return rc;
}
