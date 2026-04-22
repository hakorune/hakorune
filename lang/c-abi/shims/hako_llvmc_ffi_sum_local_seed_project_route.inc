static int hako_llvmc_sum_project_route_get_payload_i64(
    yyjson_val* route,
    long long* out) {
  yyjson_val* value = route ? yyjson_obj_get(route, "payload_i64") : NULL;
  if (!(value && yyjson_is_num(value) && out)) {
    return 0;
  }
  *out = (long long)yyjson_get_sint(value);
  return 1;
}

static int hako_llvmc_sum_project_route_get_payload_f64(
    yyjson_val* route,
    double* out) {
  yyjson_val* value = route ? yyjson_obj_get(route, "payload_f64") : NULL;
  if (!(value && yyjson_is_num(value) && out)) {
    return 0;
  }
  *out = yyjson_get_real(value);
  return 1;
}

static int hako_llvmc_sum_project_route_kind_layout_ok(
    const char* kind,
    const char* layout,
    const char* payload_kind,
    long long sum_value,
    long long project_source_value,
    long long copy_value,
    long long payload_value) {
  int is_copy = 0;
  if (!(kind && layout && payload_kind && sum_value > 0 && payload_value > 0)) {
    return 0;
  }
  if (hako_llvmc_str_eq(kind, "variant_project_local_i64")) {
    return hako_llvmc_str_eq(layout, "tag_i64_payload") &&
           hako_llvmc_str_eq(payload_kind, "i64") &&
           project_source_value == sum_value &&
           copy_value == 0;
  }
  if (hako_llvmc_str_eq(kind, "variant_project_local_f64")) {
    return hako_llvmc_str_eq(layout, "tag_f64_payload") &&
           hako_llvmc_str_eq(payload_kind, "f64") &&
           project_source_value == sum_value &&
           copy_value == 0;
  }
  if (hako_llvmc_str_eq(kind, "variant_project_local_handle")) {
    return hako_llvmc_str_eq(layout, "tag_handle_payload") &&
           hako_llvmc_str_eq(payload_kind, "string") &&
           project_source_value == sum_value &&
           copy_value == 0;
  }
  is_copy = hako_llvmc_str_eq(kind, "variant_project_copy_local_i64") ||
            hako_llvmc_str_eq(kind, "variant_project_copy_local_f64") ||
            hako_llvmc_str_eq(kind, "variant_project_copy_local_handle");
  if (!is_copy || copy_value <= 0 || project_source_value != copy_value) {
    return 0;
  }
  if (hako_llvmc_str_eq(kind, "variant_project_copy_local_i64")) {
    return hako_llvmc_str_eq(layout, "tag_i64_payload") &&
           hako_llvmc_str_eq(payload_kind, "i64");
  }
  if (hako_llvmc_str_eq(kind, "variant_project_copy_local_f64")) {
    return hako_llvmc_str_eq(layout, "tag_f64_payload") &&
           hako_llvmc_str_eq(payload_kind, "f64");
  }
  if (hako_llvmc_str_eq(kind, "variant_project_copy_local_handle")) {
    return hako_llvmc_str_eq(layout, "tag_handle_payload") &&
           hako_llvmc_str_eq(payload_kind, "string");
  }
  return 0;
}

static int hako_llvmc_consume_sum_variant_project_seed_route(
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
  const char* proof = NULL;
  const char* consumer_capability = NULL;
  const char* publication_boundary = NULL;
  const char* enum_name = NULL;
  const char* variant = NULL;
  const char* subject = NULL;
  const char* layout = NULL;
  const char* payload_kind = NULL;
  const char* payload_string = NULL;
  long long payload_i64 = 0;
  double payload_f64 = 0.0;
  long long variant_tag = 0;
  long long make_block = -1;
  long long make_instruction_index = -1;
  long long project_block = -1;
  long long project_instruction_index = -1;
  long long sum_value = 0;
  long long project_value = 0;
  long long project_source_value = 0;
  long long copy_value = 0;
  long long payload_value = 0;
  char extra[192];
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
  route = metadata ? yyjson_obj_get(metadata, "sum_variant_project_seed_route") : NULL;
  if (!(route && yyjson_is_obj(route))) {
    yyjson_doc_free(doc);
    return -1;
  }

  kind = yyjson_get_str(yyjson_obj_get(route, "kind"));
  proof = yyjson_get_str(yyjson_obj_get(route, "proof"));
  consumer_capability = yyjson_get_str(yyjson_obj_get(route, "consumer_capability"));
  publication_boundary = yyjson_get_str(yyjson_obj_get(route, "publication_boundary"));
  enum_name = yyjson_get_str(yyjson_obj_get(route, "enum"));
  variant = yyjson_get_str(yyjson_obj_get(route, "variant"));
  subject = yyjson_get_str(yyjson_obj_get(route, "subject"));
  layout = yyjson_get_str(yyjson_obj_get(route, "layout"));
  payload_kind = yyjson_get_str(yyjson_obj_get(route, "payload_literal_kind"));
  payload_string = yyjson_get_str(yyjson_obj_get(route, "payload_string"));

  if (!hako_llvmc_sum_route_get_i64(route, "variant_tag", &variant_tag) ||
      !hako_llvmc_sum_route_get_i64(route, "make_block", &make_block) ||
      !hako_llvmc_sum_route_get_i64(route, "make_instruction_index", &make_instruction_index) ||
      !hako_llvmc_sum_route_get_i64(route, "project_block", &project_block) ||
      !hako_llvmc_sum_route_get_i64(route, "project_instruction_index", &project_instruction_index) ||
      !hako_llvmc_sum_route_get_i64(route, "sum_value", &sum_value) ||
      !hako_llvmc_sum_route_get_i64(route, "project_value", &project_value) ||
      !hako_llvmc_sum_route_get_i64(route, "project_source_value", &project_source_value) ||
      !hako_llvmc_sum_route_get_optional_i64(route, "copy_value", &copy_value) ||
      !hako_llvmc_sum_route_get_i64(route, "payload_value", &payload_value)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (payload_kind && hako_llvmc_str_eq(payload_kind, "i64") &&
      !hako_llvmc_sum_project_route_get_payload_i64(route, &payload_i64)) {
    yyjson_doc_free(doc);
    return -1;
  }
  if (payload_kind && hako_llvmc_str_eq(payload_kind, "f64") &&
      !hako_llvmc_sum_project_route_get_payload_f64(route, &payload_f64)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (!(kind && proof && consumer_capability && publication_boundary &&
        enum_name && variant && subject && layout && payload_kind &&
        hako_llvmc_str_eq(proof, "sum_variant_project_local_aggregate_seed") &&
        hako_llvmc_str_eq(consumer_capability, "direct_sum_variant_project_local") &&
        hako_llvmc_str_eq(publication_boundary, "none") &&
        make_block == project_block &&
        make_instruction_index >= 0 &&
        project_instruction_index > make_instruction_index &&
        project_value > 0 &&
        variant_tag >= 0 &&
        hako_llvmc_sum_project_route_kind_layout_ok(
            kind, layout, payload_kind, sum_value, project_source_value, copy_value, payload_value))) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          "sum_variant_project_local",
          "skip",
          "metadata_contract_mismatch",
          "sum_variant_project_seed_route");
    }
    yyjson_doc_free(doc);
    return -1;
  }

  if (hako_llvmc_route_trace_enabled()) {
    snprintf(extra, sizeof(extra), "%s mir_route_plan", kind);
    hako_llvmc_emit_route_trace(
        "sum_variant_project_local",
        "emit",
        "exact_match",
        extra);
  }

  if (hako_llvmc_str_eq(payload_kind, "i64")) {
    rc = hako_llvmc_emit_variant_project_local_i64_ir(obj_out, &selection, payload_i64);
  } else if (hako_llvmc_str_eq(payload_kind, "f64")) {
    rc = hako_llvmc_emit_variant_project_local_f64_ir(obj_out, &selection, payload_f64);
  } else if (hako_llvmc_str_eq(payload_kind, "string") && payload_string) {
    rc = hako_llvmc_emit_variant_project_local_handle_ir(obj_out, &selection, payload_string);
  } else {
    rc = -1;
  }
  yyjson_doc_free(doc);
  return rc;
}
