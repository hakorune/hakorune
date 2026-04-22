static int hako_llvmc_sum_route_get_i64(
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

static int hako_llvmc_sum_route_get_optional_i64(
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

static int hako_llvmc_sum_variant_tag_route_kind_layout_ok(
    const char* kind,
    const char* layout,
    long long sum_value,
    long long tag_source_value,
    long long copy_value,
    long long payload_value) {
  if (!(kind && layout && sum_value > 0)) {
    return 0;
  }
  if (hako_llvmc_str_eq(kind, "variant_tag_local_i64")) {
    return hako_llvmc_str_eq(layout, "tag_i64_payload") &&
           tag_source_value == sum_value &&
           copy_value == 0 &&
           payload_value > 0;
  }
  if (hako_llvmc_str_eq(kind, "variant_tag_local_tag_only")) {
    return hako_llvmc_str_eq(layout, "tag_only") &&
           tag_source_value == sum_value &&
           copy_value == 0 &&
           payload_value == 0;
  }
  if (hako_llvmc_str_eq(kind, "variant_tag_local_f64")) {
    return hako_llvmc_str_eq(layout, "tag_f64_payload") &&
           tag_source_value == sum_value &&
           copy_value == 0 &&
           payload_value > 0;
  }
  if (hako_llvmc_str_eq(kind, "variant_tag_local_handle")) {
    return hako_llvmc_str_eq(layout, "tag_handle_payload") &&
           tag_source_value == sum_value &&
           copy_value == 0 &&
           payload_value > 0;
  }
  if (hako_llvmc_str_eq(kind, "variant_tag_copy_local_i64")) {
    return hako_llvmc_str_eq(layout, "tag_i64_payload") &&
           copy_value > 0 &&
           tag_source_value == copy_value &&
           payload_value > 0;
  }
  return 0;
}

static int hako_llvmc_consume_sum_variant_tag_seed_route(
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
  long long variant_tag = 0;
  long long make_block = -1;
  long long make_instruction_index = -1;
  long long tag_block = -1;
  long long tag_instruction_index = -1;
  long long sum_value = 0;
  long long tag_value = 0;
  long long tag_source_value = 0;
  long long copy_value = 0;
  long long payload_value = 0;
  char extra[160];
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
  route = metadata ? yyjson_obj_get(metadata, "sum_variant_tag_seed_route") : NULL;
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

  if (!hako_llvmc_sum_route_get_i64(route, "variant_tag", &variant_tag) ||
      !hako_llvmc_sum_route_get_i64(route, "make_block", &make_block) ||
      !hako_llvmc_sum_route_get_i64(route, "make_instruction_index", &make_instruction_index) ||
      !hako_llvmc_sum_route_get_i64(route, "tag_block", &tag_block) ||
      !hako_llvmc_sum_route_get_i64(route, "tag_instruction_index", &tag_instruction_index) ||
      !hako_llvmc_sum_route_get_i64(route, "sum_value", &sum_value) ||
      !hako_llvmc_sum_route_get_i64(route, "tag_value", &tag_value) ||
      !hako_llvmc_sum_route_get_i64(route, "tag_source_value", &tag_source_value) ||
      !hako_llvmc_sum_route_get_optional_i64(route, "copy_value", &copy_value) ||
      !hako_llvmc_sum_route_get_optional_i64(route, "payload_value", &payload_value)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (!(kind && proof && consumer_capability && publication_boundary &&
        enum_name && variant && subject && layout &&
        hako_llvmc_str_eq(proof, "sum_variant_tag_local_aggregate_seed") &&
        hako_llvmc_str_eq(consumer_capability, "direct_sum_variant_tag_local") &&
        hako_llvmc_str_eq(publication_boundary, "none") &&
        make_block == tag_block &&
        make_instruction_index >= 0 &&
        tag_instruction_index > make_instruction_index &&
        tag_value > 0 &&
        variant_tag >= 0 &&
        hako_llvmc_sum_variant_tag_route_kind_layout_ok(
            kind, layout, sum_value, tag_source_value, copy_value, payload_value))) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          "sum_variant_tag_local",
          "skip",
          "metadata_contract_mismatch",
          "sum_variant_tag_seed_route");
    }
    yyjson_doc_free(doc);
    return -1;
  }

  if (hako_llvmc_route_trace_enabled()) {
    snprintf(extra, sizeof(extra), "%s mir_route_plan", kind);
    hako_llvmc_emit_route_trace(
        "sum_variant_tag_local",
        "emit",
        "exact_match",
        extra);
  }
  rc = hako_llvmc_emit_variant_tag_local_i64_ir(
      obj_out,
      &selection,
      variant_tag);
  yyjson_doc_free(doc);
  return rc;
}
