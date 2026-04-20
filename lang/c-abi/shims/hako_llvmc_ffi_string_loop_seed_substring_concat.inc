}

static int hako_llvmc_match_substring_concat_loop_ascii_seed(
    const char* json_in,
    const char* obj_out);

static int hako_llvmc_match_substring_concat_len_ascii_seed(
    const char* json_in,
    const char* obj_out) {
  /*
   * phase180x retirement cut:
   * the dedicated len-only benchmark matcher no longer reconstructs its own
   * block/op ladder. Reuse the main substring-concat matcher so the len route
   * comes from the same metadata-first `stable_length_scalar` + plan seam.
   */
  return hako_llvmc_match_substring_concat_loop_ascii_seed(json_in, obj_out);
}

static int hako_llvmc_match_substring_concat_loop_ascii_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  hako_llvmc_entry_rune_selection selection;
  long long plan_corridor_root = 0;
  long long seed_len = 0;
  long long loop_bound = 0;
  long long middle_len = 0;
  long long split = 0;
  const char* seed = NULL;
  const char* middle = NULL;
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

  if (!hako_llvmc_string_kernel_plan_find_concat_triplet_loop_fn(
          fn,
          &plan_corridor_root,
          NULL,
          &seed,
          &seed_len,
          &middle,
          &middle_len,
          &loop_bound,
          &split)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (hako_llvmc_string_corridor_has_stable_length_scalar_base_fn(
          fn,
          plan_corridor_root,
          "stop_at_merge")) {
    int rc = hako_llvmc_emit_substring_concat_len_ir(
        obj_out,
        &selection,
        loop_bound,
        seed_len,
        middle_len);
    yyjson_doc_free(doc);
    return rc;
  }

  {
    int rc = hako_llvmc_emit_substring_concat_loop_ir(
          obj_out,
          &selection,
          seed,
          seed_len,
          loop_bound,
          middle,
          middle_len,
          split);
    yyjson_doc_free(doc);
    return rc;
  }
}
