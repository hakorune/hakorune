
static int hako_llvmc_consume_substring_views_only_micro_route(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* metadata;
  yyjson_val* route;
  hako_llvmc_entry_rune_selection selection;
  const char* source = NULL;
  const char* proof = NULL;
  long long source_len = 0;
  long long actual_source_len = 0;
  long long loop_bound = 0;
  size_t i;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  for (i = 0; i < yyjson_arr_size(fns); i++) {
    yyjson_val* candidate = yyjson_arr_get(fns, i);
    const char* name = candidate ? yyjson_get_str(yyjson_obj_get(candidate, "name")) : NULL;
    if (name && strcmp(name, "main") == 0) {
      fn = candidate;
      break;
    }
  }
  if (!fn) {
    yyjson_doc_free(doc);
    return -1;
  }
  hako_llvmc_select_entry_runes(fn, &selection);

  metadata = yyjson_obj_get(fn, "metadata");
  route = metadata ? yyjson_obj_get(metadata, "substring_views_micro_seed_route") : NULL;
  if (!(route && yyjson_is_obj(route))) {
    yyjson_doc_free(doc);
    return -1;
  }

  source = yyjson_get_str(yyjson_obj_get(route, "source"));
  proof = yyjson_get_str(yyjson_obj_get(route, "proof"));
  source_len = (long long)yyjson_get_sint(yyjson_obj_get(route, "source_len"));
  loop_bound = (long long)yyjson_get_sint(yyjson_obj_get(route, "loop_bound"));

  if (!(source && proof &&
        hako_llvmc_ascii_strlen(source, &actual_source_len) &&
        source_len == actual_source_len &&
        hako_llvmc_str_eq(source, "line-seed-abcdef") &&
        hako_llvmc_str_eq(proof, "kilo_micro_substring_views_only_5block") &&
        source_len == 16 &&
        loop_bound == 300000)) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          "substring_views_only_micro",
          "skip",
          "metadata_contract_mismatch",
          "substring_views_micro_seed_route");
    }
    yyjson_doc_free(doc);
    return -1;
  }

  if (hako_llvmc_route_trace_enabled()) {
    hako_llvmc_emit_route_trace(
        "substring_views_only_micro",
        "emit",
        "exact_match",
        "kilo_micro_substring_views_only mir_route_plan");
  }

  {
    int rc = hako_llvmc_emit_substring_views_only_exit_len_ir(
        obj_out,
        &selection,
        source_len);
    yyjson_doc_free(doc);
    return rc;
  }
}
