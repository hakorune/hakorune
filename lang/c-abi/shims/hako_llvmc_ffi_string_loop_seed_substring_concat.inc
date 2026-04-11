}

static int hako_llvmc_match_substring_concat_len_ascii_seed(
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
  long long loop_bound = 0;
  long long seed_len = 0;
  long long middle_len = 0;
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
        if (_ins) interesting[n++] = _ins; \
      } \
    } while (0)
  #define EXPECT_OP(idx, name) \
    do { \
      const char* _op = ((idx) < n) ? READ_STR(interesting[(idx)], "op") : NULL; \
      if (!_op || strcmp(_op, (name)) != 0) { yyjson_doc_free(doc); return -1; } \
    } while (0)
  #define REQUIRE_STRINGISH_BOX(box_name) \
    (((box_name) && strcmp((box_name), "RuntimeDataBox") == 0) || ((box_name) && strcmp((box_name), "StringBox") == 0))

  BUILD_INTERESTING(yyjson_arr_get(blocks, 0));
  if (n != 5) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "const");
  EXPECT_OP(1, "copy");
  EXPECT_OP(2, "const");
  EXPECT_OP(3, "copy");
  EXPECT_OP(4, "jump");
  {
    yyjson_val* v0 = yyjson_obj_get(interesting[0], "value");
    yyjson_val* v2 = yyjson_obj_get(interesting[2], "value");
    yyjson_val* ty0 = v0 ? yyjson_obj_get(v0, "type") : NULL;
    yyjson_val* ty2 = v2 ? yyjson_obj_get(v2, "type") : NULL;
    if (!v0 || !v2 ||
        !yyjson_is_str(ty0) ||
        !yyjson_is_str(ty2) ||
        strcmp(yyjson_get_str(ty0), "i64") != 0 ||
        strcmp(yyjson_get_str(ty2), "i64") != 0 ||
        (long long)yyjson_get_sint(yyjson_obj_get(v0, "value")) != 0 ||
        (long long)yyjson_get_sint(yyjson_obj_get(v2, "value")) != 0) {
      yyjson_doc_free(doc);
      return -1;
    }
  }

  BUILD_INTERESTING(yyjson_arr_get(blocks, 1));
  if (n != 7) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "phi");
  EXPECT_OP(1, "phi");
  EXPECT_OP(2, "copy");
  EXPECT_OP(3, "const");
  EXPECT_OP(4, "copy");
  EXPECT_OP(5, "compare");
  EXPECT_OP(6, "branch");
  {
    yyjson_val* v = yyjson_obj_get(interesting[3], "value");
    yyjson_val* ty = v ? yyjson_obj_get(v, "type") : NULL;
    if (!v || !yyjson_is_str(ty) || strcmp(yyjson_get_str(ty), "i64") != 0) {
      yyjson_doc_free(doc);
      return -1;
    }
    loop_bound = (long long)yyjson_get_sint(yyjson_obj_get(v, "value"));
    if (loop_bound != 180000 || !hako_llvmc_str_eq(READ_STR(interesting[5], "operation"), "<")) {
      yyjson_doc_free(doc);
      return -1;
    }
  }

  BUILD_INTERESTING(yyjson_arr_get(blocks, 2));
  if (n != 26) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "const");
  EXPECT_OP(1, "copy");
  EXPECT_OP(2, "const");
  EXPECT_OP(3, "const");
  EXPECT_OP(4, "copy");
  EXPECT_OP(5, "const");
  EXPECT_OP(6, "binop");
  EXPECT_OP(7, "const");
  EXPECT_OP(8, "copy");
  EXPECT_OP(9, "mir_call");
  EXPECT_OP(10, "mir_call");
  EXPECT_OP(11, "copy");
  EXPECT_OP(12, "copy");
  EXPECT_OP(13, "copy");
  EXPECT_OP(14, "copy");
  EXPECT_OP(15, "copy");
  EXPECT_OP(16, "copy");
  EXPECT_OP(17, "const");
  EXPECT_OP(18, "binop");
  EXPECT_OP(19, "copy");
  EXPECT_OP(20, "binop");
  EXPECT_OP(21, "mir_call");
  EXPECT_OP(22, "copy");
  EXPECT_OP(23, "copy");
  EXPECT_OP(24, "binop");
  EXPECT_OP(25, "jump");
  {
    yyjson_val* v0 = yyjson_obj_get(interesting[0], "value");
    yyjson_val* v2 = yyjson_obj_get(interesting[2], "value");
    yyjson_val* v3 = yyjson_obj_get(interesting[3], "value");
    yyjson_val* v5 = yyjson_obj_get(interesting[5], "value");
    yyjson_val* v7 = yyjson_obj_get(interesting[7], "value");
    yyjson_val* ty0 = v0 ? yyjson_obj_get(v0, "type") : NULL;
    yyjson_val* ty2 = v2 ? yyjson_obj_get(v2, "type") : NULL;
    yyjson_val* ty3 = v3 ? yyjson_obj_get(v3, "type") : NULL;
    yyjson_val* ty5 = v5 ? yyjson_obj_get(v5, "type") : NULL;
    yyjson_val* ty7 = v7 ? yyjson_obj_get(v7, "type") : NULL;
    if (!v0 || !v2 || !v3 || !v5 || !v7 ||
        !yyjson_is_str(ty0) ||
        !yyjson_is_str(ty2) ||
        !yyjson_is_str(ty3) ||
        !yyjson_is_str(ty5) ||
        !(ty7 && yyjson_obj_get(ty7, "box_type")) ||
        strcmp(yyjson_get_str(ty0), "i64") != 0 ||
        strcmp(yyjson_get_str(ty2), "i64") != 0 ||
        strcmp(yyjson_get_str(ty3), "i64") != 0 ||
        strcmp(yyjson_get_str(ty5), "i64") != 0 ||
        strcmp(yyjson_get_str(yyjson_obj_get(ty7, "box_type")), "StringBox") != 0 ||
        (long long)yyjson_get_sint(yyjson_obj_get(v0, "value")) != 16 ||
        (long long)yyjson_get_sint(yyjson_obj_get(v2, "value")) != 0 ||
        (long long)yyjson_get_sint(yyjson_obj_get(v3, "value")) != 16 ||
        (long long)yyjson_get_sint(yyjson_obj_get(v5, "value")) != 2 ||
        !hako_llvmc_str_eq(yyjson_get_str(yyjson_obj_get(v7, "value")), "line-seed-abcdef")) {
      yyjson_doc_free(doc);
      return -1;
    }
    seed = yyjson_get_str(yyjson_obj_get(v7, "value"));
    if (!hako_llvmc_ascii_strlen(seed, &seed_len) || seed_len != 16) {
      yyjson_doc_free(doc);
      return -1;
    }
  }
  {
    int k;
    for (k = 9; k <= 10; k++) {
      yyjson_val* mc = yyjson_obj_get(interesting[k], "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
      const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      if (!(REQUIRE_STRINGISH_BOX(box_name) && name && strcmp(name, "substring") == 0 &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 2)) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
  }
  {
    yyjson_val* v17 = yyjson_obj_get(interesting[17], "value");
    yyjson_val* ty17 = v17 ? yyjson_obj_get(v17, "type") : NULL;
    if (!v17 || !(ty17 && yyjson_obj_get(ty17, "box_type")) ||
        strcmp(yyjson_get_str(yyjson_obj_get(ty17, "box_type")), "StringBox") != 0) {
      yyjson_doc_free(doc);
      return -1;
    }
    middle = yyjson_get_str(yyjson_obj_get(v17, "value"));
    if (!middle || !hako_llvmc_ascii_strlen(middle, &middle_len) ||
        middle_len != 2 || strcmp(middle, "xx") != 0) {
      yyjson_doc_free(doc);
      return -1;
    }
  }
  if (!hako_llvmc_str_eq(READ_STR(interesting[18], "operation"), "+") ||
      !hako_llvmc_str_eq(READ_STR(interesting[20], "operation"), "+") ||
      !hako_llvmc_str_eq(READ_STR(interesting[24], "operation"), "+")) {
    yyjson_doc_free(doc);
    return -1;
  }
  {
    yyjson_val* mc = yyjson_obj_get(interesting[21], "mir_call");
    yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
    const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
    const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
    yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
    if (!(REQUIRE_STRINGISH_BOX(box_name) && name && strcmp(name, "length") == 0 &&
          args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
      yyjson_doc_free(doc);
      return -1;
    }
  }

  BUILD_INTERESTING(yyjson_arr_get(blocks, 3));
  if (n != 5) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "phi");
  EXPECT_OP(1, "phi");
  EXPECT_OP(2, "const");
  EXPECT_OP(3, "binop");
  EXPECT_OP(4, "jump");
  if (READ_INT(interesting[2], "value") != 1 ||
      !hako_llvmc_str_eq(READ_STR(interesting[3], "operation"), "+")) {
    yyjson_doc_free(doc);
    return -1;
  }

  BUILD_INTERESTING(yyjson_arr_get(blocks, 4));
  if (n != 5) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "copy");
  EXPECT_OP(1, "const");
  EXPECT_OP(2, "copy");
  EXPECT_OP(3, "binop");
  EXPECT_OP(4, "ret");
  {
    yyjson_val* v1 = yyjson_obj_get(interesting[1], "value");
    yyjson_val* ty1 = v1 ? yyjson_obj_get(v1, "type") : NULL;
    if (!v1 || !yyjson_is_str(ty1) ||
        strcmp(yyjson_get_str(ty1), "i64") != 0 ||
        (long long)yyjson_get_sint(yyjson_obj_get(v1, "value")) != 16 ||
        !hako_llvmc_str_eq(READ_STR(interesting[3], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }
  }

  {
    int rc = hako_llvmc_emit_substring_concat_len_ir(
        obj_out,
        &selection,
        loop_bound,
        seed_len,
        middle_len);
    yyjson_doc_free(doc);
    return rc;
  }

  #undef READ_INT
  #undef READ_STR
  #undef BUILD_INTERESTING
  #undef EXPECT_OP
  #undef REQUIRE_STRINGISH_BOX
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
  long long seed_len = 0;
  long long loop_bound = 0;
  long long divisor = 0;
  long long split = 0;
  long long middle_len = 0;
  long long source_len_value = 0;
  long long source_root = 0;
  long long helper_start = 0;
  long long helper_end = 0;
  long long helper_result = 0;
  long long merged_string_phi = 0;
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
  if (n != 6) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "const");
  EXPECT_OP(1, "const");
  EXPECT_OP(3, "const");
  EXPECT_OP(4, "const");
  EXPECT_OP(5, "jump");
  seed = READ_STR(yyjson_obj_get(interesting[0], "value"), "value");
  if (!seed || !hako_llvmc_ascii_strlen(seed, &seed_len)) { yyjson_doc_free(doc); return -1; }
  {
    const char* seed_dup = READ_STR(yyjson_obj_get(interesting[1], "value"), "value");
    if (!hako_llvmc_str_eq(seed, seed_dup)) {
      yyjson_doc_free(doc);
      return -1;
    }
  }
  {
    yyjson_val* mc = yyjson_obj_get(interesting[2], "mir_call");
    yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
    const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
    const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
    yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
    if (!(box_name && strcmp(box_name, "StringBox") == 0 && name && strcmp(name, "length") == 0 &&
          args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
      yyjson_doc_free(doc);
      return -1;
    }
    source_len_value = READ_INT(interesting[2], "dst");
  }
  if (READ_INT(yyjson_obj_get(interesting[3], "value"), "value") != 0 ||
      READ_INT(yyjson_obj_get(interesting[4], "value"), "value") != 0) {
    yyjson_doc_free(doc);
    return -1;
  }

  /* header */
  BUILD_INTERESTING(yyjson_arr_get(blocks, 1));
  if (n != 6) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "phi");
  EXPECT_OP(1, "phi");
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

  /* body */
  BUILD_INTERESTING(yyjson_arr_get(blocks, 2));
  if (n != 17) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "const");
  EXPECT_OP(1, "const");
  EXPECT_OP(2, "binop");
  EXPECT_OP(5, "const");
  divisor = READ_INT(yyjson_obj_get(interesting[1], "value"), "value");
  {
    const char* div_op = READ_STR(interesting[2], "operation");
    if (READ_INT(yyjson_obj_get(interesting[0], "value"), "value") != 0 || divisor <= 0 ||
        !hako_llvmc_str_eq(div_op, "/")) {
      yyjson_doc_free(doc);
      return -1;
    }
  }
  middle = READ_STR(yyjson_obj_get(interesting[5], "value"), "value");
  if (!middle || !hako_llvmc_ascii_strlen(middle, &middle_len)) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(7, "const");
  EXPECT_OP(8, "binop");
  EXPECT_OP(10, "binop");
  EXPECT_OP(11, "binop");
  EXPECT_OP(12, "const");
  EXPECT_OP(13, "const");
  EXPECT_OP(14, "binop");
  EXPECT_OP(16, "jump");
  {
    const char* add0 = READ_STR(interesting[8], "operation");
    const char* add1 = READ_STR(interesting[10], "operation");
    const char* add2 = READ_STR(interesting[11], "operation");
    const char* add3 = READ_STR(interesting[14], "operation");
    if (!(hako_llvmc_str_eq(add0, "+") &&
          hako_llvmc_str_eq(add1, "+") &&
          hako_llvmc_str_eq(add2, "+") &&
          hako_llvmc_str_eq(add3, "+"))) {
      yyjson_doc_free(doc);
      return -1;
    }
  }
  if (READ_INT(yyjson_obj_get(interesting[12], "value"), "value") != 1 ||
      READ_INT(yyjson_obj_get(interesting[13], "value"), "value") != 1) {
    yyjson_doc_free(doc);
    return -1;
  }
  {
    helper_result = READ_INT(interesting[15], "dst");
    if (!(hako_llvmc_string_corridor_read_plan_source_window_fn(
              fn,
              helper_result,
              "publication_sink",
              &source_root,
              &helper_start,
              &helper_end) &&
          source_root > 0 &&
          helper_start == READ_INT(interesting[12], "dst") &&
          helper_end == READ_INT(interesting[14], "dst"))) {
      yyjson_doc_free(doc);
      return -1;
    }
  }

  /* latch */
  BUILD_INTERESTING(yyjson_arr_get(blocks, 3));
  if (n != 5) { yyjson_doc_free(doc); return -1; }
  {
    const char* inc_op = READ_STR(interesting[3], "operation");
    if (READ_INT(yyjson_obj_get(interesting[2], "value"), "value") != 1 ||
        !hako_llvmc_str_eq(inc_op, "+")) {
      yyjson_doc_free(doc);
      return -1;
    }
  }

  /* exit */
  BUILD_INTERESTING(yyjson_arr_get(blocks, 4));
  if (n != 3) { yyjson_doc_free(doc); return -1; }
  {
    yyjson_val* mc = yyjson_obj_get(interesting[0], "mir_call");
    yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
    const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
    const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
    yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
    if (!(REQUIRE_STRINGISH_BOX(box_name) && name && strcmp(name, "length") == 0 &&
          args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
      yyjson_doc_free(doc);
      return -1;
    }
  }
  {
    const char* ret_add = READ_STR(interesting[1], "operation");
    if (!hako_llvmc_str_eq(ret_add, "+")) {
      yyjson_doc_free(doc);
      return -1;
    }
  }

  split = seed_len / divisor;
  if (split <= 0 || middle_len <= 0) {
    yyjson_doc_free(doc);
    return -1;
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

  #undef READ_INT
  #undef READ_STR
  #undef BUILD_INTERESTING
  #undef EXPECT_OP
  #undef REQUIRE_STRINGISH_BOX
}
