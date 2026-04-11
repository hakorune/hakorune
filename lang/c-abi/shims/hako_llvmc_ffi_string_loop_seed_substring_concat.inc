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
  yyjson_val* blocks;
  yyjson_val* interesting[40];
  int n;
  hako_llvmc_entry_rune_selection selection;
  long long source_len_value = 0;
  long long merged_string_phi = 0;
  long long seed_len = 0;
  long long loop_bound = 0;
  const char* seed = NULL;
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

  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 5)) {
    yyjson_doc_free(doc);
    return -1;
  }

  #define READ_INT(obj, key) ((obj) && yyjson_obj_get((obj), (key)) ? (long long)yyjson_get_sint(yyjson_obj_get((obj), (key))) : 0)
  #define READ_STR(obj, key) ((obj) && yyjson_obj_get((obj), (key)) ? yyjson_get_str(yyjson_obj_get((obj), (key))) : NULL)
  #define BUILD_INTERESTING(block_val) \
    do { \
      yyjson_val* _insts = yyjson_obj_get((block_val), "instructions"); \
      size_t _ii; \
      n = 0; \
      if (!(_insts && yyjson_is_arr(_insts))) { yyjson_doc_free(doc); return -1; } \
      for (_ii = 0; _ii < yyjson_arr_size(_insts) && n < (int)(sizeof(interesting) / sizeof(interesting[0])); _ii++) { \
        yyjson_val* _ins = yyjson_arr_get(_insts, _ii); \
        const char* _op = READ_STR(_ins, "op"); \
        if (_op && strcmp(_op, "copy") != 0) interesting[n++] = _ins; \
      } \
    } while (0)
  #define EXPECT_OP(idx, name) \
    do { \
      const char* _op = ((idx) < n) ? READ_STR(interesting[(idx)], "op") : NULL; \
      if (!_op || strcmp(_op, (name)) != 0) { yyjson_doc_free(doc); return -1; } \
    } while (0)
  #define REQUIRE_STRINGISH_BOX(box_name) \
    (((box_name) && strcmp((box_name), "RuntimeDataBox") == 0) || ((box_name) && strcmp((box_name), "StringBox") == 0))

  /* preheader */
  BUILD_INTERESTING(yyjson_arr_get(blocks, 0));
  if (n < 3) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "const");
  seed = READ_STR(yyjson_obj_get(interesting[0], "value"), "value");
  if (!seed || !hako_llvmc_ascii_strlen(seed, &seed_len)) { yyjson_doc_free(doc); return -1; }
  source_len_value = READ_INT(interesting[2], "dst");
  if (source_len_value <= 0) {
    yyjson_doc_free(doc);
    return -1;
  }

  /* header */
  BUILD_INTERESTING(yyjson_arr_get(blocks, 1));
  if (n < 5) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(2, "phi");
  merged_string_phi = READ_INT(interesting[2], "dst");
  loop_bound = READ_INT(yyjson_obj_get(interesting[3], "value"), "value");
  if (loop_bound <= 0) { yyjson_doc_free(doc); return -1; }
  {
    const char* cmp_op = READ_STR(interesting[4], "operation");
    if (!hako_llvmc_str_eq(cmp_op, "<")) { yyjson_doc_free(doc); return -1; }
  }

  /* metadata-first length-only route */
  {
    long long stable_base = 0;
    long long stable_witness = 0;
    long long plan_source_root = 0;
    long long plan_middle_value = 0;
    long long plan_middle_len = 0;
    const char* stable_window = NULL;

    if (merged_string_phi > 0 &&
        hako_llvmc_string_corridor_read_relation_details_fn(
            fn,
            merged_string_phi,
            "stable_length_scalar",
            &stable_base,
            &stable_witness,
            &stable_window) &&
        stable_base > 0 &&
        stable_witness == source_len_value &&
        hako_llvmc_str_eq(stable_window, "stop_at_merge") &&
        hako_llvmc_string_kernel_plan_read_concat_triplet_len_fn(
            fn,
            stable_base,
            &plan_source_root,
            &plan_middle_value,
            &plan_middle_len) &&
        plan_source_root == merged_string_phi &&
        plan_middle_value > 0 &&
        plan_middle_len > 0) {
      int rc = hako_llvmc_emit_substring_concat_len_ir(
          obj_out,
          &selection,
          loop_bound,
          seed_len,
          plan_middle_len);
      yyjson_doc_free(doc);
      return rc;
    }
  }

  {
    const char* plan_seed = NULL;
    const char* plan_middle = NULL;
    long long plan_source_root = 0;
    long long plan_seed_len = 0;
    long long plan_middle_len = 0;
    long long plan_loop_bound = 0;
    long long plan_split = 0;

    if (merged_string_phi > 0 &&
        hako_llvmc_string_kernel_plan_read_concat_triplet_loop_fn(
            fn,
            merged_string_phi,
            &plan_source_root,
            &plan_seed,
            &plan_seed_len,
            &plan_middle,
            &plan_middle_len,
            &plan_loop_bound,
            &plan_split) &&
        plan_source_root == merged_string_phi &&
        plan_seed && plan_middle &&
        plan_seed_len > 0 &&
        plan_middle_len > 0 &&
        plan_loop_bound > 0 &&
        plan_split > 0) {
      int rc = hako_llvmc_emit_substring_concat_loop_ir(
          obj_out,
          &selection,
          plan_seed,
          plan_seed_len,
          plan_loop_bound,
          plan_middle,
          plan_middle_len,
          plan_split);
      yyjson_doc_free(doc);
      return rc;
    }
  }

  yyjson_doc_free(doc);
  return -1;

  #undef READ_INT
  #undef READ_STR
  #undef BUILD_INTERESTING
  #undef EXPECT_OP
  #undef REQUIRE_STRINGISH_BOX
}
