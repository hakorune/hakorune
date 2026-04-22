static int hako_llvmc_userbox_route_get_optional_f64(
    yyjson_val* route,
    const char* field,
    double* out,
    int* present_out) {
  yyjson_val* value = route ? yyjson_obj_get(route, field) : NULL;
  if (!(out && present_out)) {
    return 0;
  }
  *out = 0.0;
  *present_out = 0;
  if (!value || yyjson_is_null(value)) {
    return 1;
  }
  if (!yyjson_is_num(value)) {
    return 0;
  }
  *out = yyjson_get_real(value);
  *present_out = 1;
  return 1;
}

static int hako_llvmc_userbox_flag_pointf_route_shape_ok(
    const char* kind,
    long long block,
    long long newbox_instruction_index,
    long long set_field_instruction_index,
    long long get_field_instruction_index,
    long long box_value,
    long long copy_value,
    long long field_value,
    long long get_field_value,
    long long result_value,
    long long payload_i64,
    int payload_f64_present) {
  if (!(kind &&
        block == 0 &&
        newbox_instruction_index == 1 &&
        set_field_instruction_index == 2 &&
        box_value == 2 &&
        field_value == 1)) {
    return 0;
  }
  if (hako_llvmc_str_eq(kind, "flag_local_bool")) {
    return get_field_instruction_index == 3 &&
           copy_value == 0 &&
           get_field_value == 3 &&
           result_value == 3 &&
           payload_i64 == 1 &&
           payload_f64_present == 0;
  }
  if (hako_llvmc_str_eq(kind, "flag_copy_local_bool")) {
    return get_field_instruction_index == 4 &&
           copy_value == 3 &&
           get_field_value == 4 &&
           result_value == 4 &&
           payload_i64 == 1 &&
           payload_f64_present == 0;
  }
  if (hako_llvmc_str_eq(kind, "pointf_local_f64")) {
    return get_field_instruction_index == 3 &&
           copy_value == 0 &&
           get_field_value == 3 &&
           result_value == 3 &&
           payload_i64 == 0 &&
           payload_f64_present == 1;
  }
  if (hako_llvmc_str_eq(kind, "pointf_copy_local_f64")) {
    return get_field_instruction_index == 4 &&
           copy_value == 3 &&
           get_field_value == 4 &&
           result_value == 4 &&
           payload_i64 == 0 &&
           payload_f64_present == 1;
  }
  return 0;
}

static int hako_llvmc_consume_userbox_flag_pointf_local_scalar_route(
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
  const char* field = NULL;
  const char* proof = NULL;
  const char* consumer_capability = NULL;
  const char* publication_boundary = NULL;
  long long block = -1;
  long long newbox_instruction_index = -1;
  long long set_field_instruction_index = -1;
  long long get_field_instruction_index = -1;
  long long box_value = 0;
  long long copy_value = 0;
  long long field_value = 0;
  long long get_field_value = 0;
  long long result_value = 0;
  long long payload_i64 = 0;
  double payload_f64 = 0.0;
  int payload_f64_present = 0;
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
  route = metadata ? yyjson_obj_get(metadata, "userbox_local_scalar_seed_route") : NULL;
  if (!(route && yyjson_is_obj(route))) {
    yyjson_doc_free(doc);
    return -1;
  }

  kind = yyjson_get_str(yyjson_obj_get(route, "kind"));
  box_name = yyjson_get_str(yyjson_obj_get(route, "box"));
  field = yyjson_get_str(yyjson_obj_get(route, "field"));
  proof = yyjson_get_str(yyjson_obj_get(route, "proof"));
  consumer_capability = yyjson_get_str(yyjson_obj_get(route, "consumer_capability"));
  publication_boundary = yyjson_get_str(yyjson_obj_get(route, "publication_boundary"));

  if (!hako_llvmc_userbox_route_get_i64(route, "block", &block) ||
      !hako_llvmc_userbox_route_get_i64(route, "newbox_instruction_index", &newbox_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(route, "set_field_instruction_index", &set_field_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(route, "get_field_instruction_index", &get_field_instruction_index) ||
      !hako_llvmc_userbox_route_get_i64(route, "box_value", &box_value) ||
      !hako_llvmc_userbox_route_get_optional_i64(route, "copy_value", &copy_value) ||
      !hako_llvmc_userbox_route_get_i64(route, "field_value", &field_value) ||
      !hako_llvmc_userbox_route_get_i64(route, "get_field_value", &get_field_value) ||
      !hako_llvmc_userbox_route_get_i64(route, "result_value", &result_value) ||
      !hako_llvmc_userbox_route_get_optional_i64(route, "payload_i64", &payload_i64) ||
      !hako_llvmc_userbox_route_get_optional_f64(
          route,
          "payload_f64",
          &payload_f64,
          &payload_f64_present)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (!(kind && box_name && field && proof &&
        consumer_capability && publication_boundary &&
        hako_llvmc_userbox_flag_pointf_route_shape_ok(
            kind,
            block,
            newbox_instruction_index,
            set_field_instruction_index,
            get_field_instruction_index,
            box_value,
            copy_value,
            field_value,
            get_field_value,
            result_value,
            payload_i64,
            payload_f64_present) &&
        hako_llvmc_str_eq(publication_boundary, "none"))) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          "userbox_flag_pointf_local_scalar",
          "skip",
          "metadata_contract_mismatch",
          "userbox_local_scalar_seed_route");
    }
    yyjson_doc_free(doc);
    return -1;
  }

  if (hako_llvmc_str_eq(kind, "flag_local_bool") ||
      hako_llvmc_str_eq(kind, "flag_copy_local_bool")) {
    if (!(hako_llvmc_str_eq(box_name, "Flag") &&
          hako_llvmc_str_eq(field, "enabled") &&
          hako_llvmc_str_eq(proof, "userbox_flag_field_local_scalar_seed") &&
          hako_llvmc_str_eq(consumer_capability, "direct_userbox_flag_local_scalar") &&
          hako_llvmc_userbox_flag_inline_scalar_selection_ok(fn) &&
          hako_llvmc_userbox_flag_decl_ok(root))) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (hako_llvmc_route_trace_enabled()) {
      snprintf(extra, sizeof(extra), "%s mir_route_plan", kind);
      hako_llvmc_emit_route_trace(
          "userbox_flag_pointf_local_scalar",
          "emit",
          "exact_match",
          extra);
    }
    rc = hako_llvmc_emit_userbox_flag_local_bool_ir(obj_out, &selection);
    yyjson_doc_free(doc);
    return rc;
  }

  if (hako_llvmc_str_eq(kind, "pointf_local_f64") ||
      hako_llvmc_str_eq(kind, "pointf_copy_local_f64")) {
    if (!(hako_llvmc_str_eq(box_name, "PointF") &&
          hako_llvmc_str_eq(field, "x") &&
          hako_llvmc_str_eq(proof, "userbox_pointf_field_local_scalar_seed") &&
          hako_llvmc_str_eq(consumer_capability, "direct_userbox_pointf_local_scalar") &&
          hako_llvmc_userbox_pointf_inline_scalar_selection_ok(fn) &&
          hako_llvmc_userbox_pointf_decl_ok(root))) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (hako_llvmc_route_trace_enabled()) {
      snprintf(extra, sizeof(extra), "%s mir_route_plan", kind);
      hako_llvmc_emit_route_trace(
          "userbox_flag_pointf_local_scalar",
          "emit",
          "exact_match",
          extra);
    }
    rc = hako_llvmc_emit_userbox_pointf_local_f64_ir(obj_out, &selection, payload_f64);
    yyjson_doc_free(doc);
    return rc;
  }

  yyjson_doc_free(doc);
  return -1;
}
