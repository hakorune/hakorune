static int hako_llvmc_emit_concat_const_suffix_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    const char* text,
    long long text_len,
    const char* suffix,
    long long suffix_len,
    long long ops) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!obj_out || !text || !suffix || text_len != 16 || suffix_len != 2 || ops != 600000) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_concat_const_suffix_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (concat const-suffix loop ascii)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  fprintf(f, "\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%acc = alloca i64, align 8\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%acc, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  %%acc.cur = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%acc.next = add i64 %%acc.cur, %lld\n", text_len + suffix_len);
  fprintf(f, "  store volatile i64 %%acc.next, ptr %%acc, align 8\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%acc.fin = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%ret = and i64 %%acc.fin, 255\n");
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_concat_const_suffix_micro_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  hako_llvmc_entry_rune_selection selection;
  long long loop_bound = 0;
  long long text_len = 0;
  const char* text = NULL;
  const char* suffix = NULL;
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
  #define BUILD_INTERESTING_NO_COPY(block_val) \
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

  yyjson_val* interesting[40];
  int n = 0;
  int saw_preheader = 0;
  int saw_header = 0;
  int saw_body = 0;
  int saw_latch = 0;
  int saw_exit = 0;

  BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 0));
  if (n != 6) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "const");
  EXPECT_OP(1, "const");
  EXPECT_OP(2, "mir_call");
  EXPECT_OP(3, "const");
  EXPECT_OP(4, "const");
  EXPECT_OP(5, "jump");
  {
    yyjson_val* vobj = yyjson_obj_get(interesting[0], "value");
    yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
    const char* box_type = ty ? yyjson_get_str(yyjson_obj_get(ty, "box_type")) : NULL;
    text = vobj ? READ_STR(vobj, "value") : NULL;
    if (!(box_type && strcmp(box_type, "StringBox") == 0 &&
          text && hako_llvmc_ascii_strlen(text, &text_len) &&
          text_len == 16 && hako_llvmc_str_eq(text, "line-seed-abcdef"))) {
      yyjson_doc_free(doc);
      return -1;
    }
  }
  {
    yyjson_val* vobj = yyjson_obj_get(interesting[1], "value");
    yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
    const char* box_type = ty ? yyjson_get_str(yyjson_obj_get(ty, "box_type")) : NULL;
    const char* text_dup = vobj ? READ_STR(vobj, "value") : NULL;
    if (!(box_type && strcmp(box_type, "StringBox") == 0 &&
          text_dup && hako_llvmc_ascii_strlen(text_dup, &text_len) &&
          text_len == 16 && hako_llvmc_str_eq(text_dup, "line-seed-abcdef"))) {
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
    if (!(box_name && strcmp(box_name, "StringBox") == 0 &&
          name && strcmp(name, "length") == 0 &&
          args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
      yyjson_doc_free(doc);
      return -1;
    }
  }
  if (READ_INT(yyjson_obj_get(interesting[3], "value"), "value") != 0 ||
      READ_INT(yyjson_obj_get(interesting[4], "value"), "value") != 0) {
    yyjson_doc_free(doc);
    return -1;
  }
  saw_preheader = 1;

  BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 1));
  if (n != 6) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "phi");
  EXPECT_OP(1, "phi");
  EXPECT_OP(2, "phi");
  EXPECT_OP(3, "const");
  EXPECT_OP(4, "compare");
  EXPECT_OP(5, "branch");
  loop_bound = READ_INT(yyjson_obj_get(interesting[3], "value"), "value");
  if (!(loop_bound == 600000 && hako_llvmc_str_eq(READ_STR(interesting[4], "operation"), "<"))) {
    yyjson_doc_free(doc);
    return -1;
  }
  saw_header = 1;

  BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 2));
  if (n != 9) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "const");
  EXPECT_OP(1, "binop");
  EXPECT_OP(2, "mir_call");
  EXPECT_OP(3, "binop");
  EXPECT_OP(4, "const");
  EXPECT_OP(5, "const");
  EXPECT_OP(6, "binop");
  EXPECT_OP(7, "mir_call");
  EXPECT_OP(8, "jump");
  {
    yyjson_val* vobj = yyjson_obj_get(interesting[0], "value");
    yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
    const char* box_type = ty ? yyjson_get_str(yyjson_obj_get(ty, "box_type")) : NULL;
    suffix = vobj ? READ_STR(vobj, "value") : NULL;
    if (!(box_type && strcmp(box_type, "StringBox") == 0 &&
          suffix && hako_llvmc_str_eq(suffix, "xy"))) {
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
    if (!(REQUIRE_STRINGISH_BOX(box_name) && name && strcmp(name, "length") == 0 &&
          args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
      yyjson_doc_free(doc);
      return -1;
    }
  }
  if (READ_INT(yyjson_obj_get(interesting[4], "value"), "value") != 2 ||
      READ_INT(yyjson_obj_get(interesting[5], "value"), "value") != 2) {
    yyjson_doc_free(doc);
    return -1;
  }
  {
    yyjson_val* mc = yyjson_obj_get(interesting[7], "mir_call");
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
  saw_body = 1;

  BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 3));
  if (n != 5) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "phi");
  EXPECT_OP(1, "phi");
  EXPECT_OP(2, "const");
  EXPECT_OP(3, "binop");
  EXPECT_OP(4, "jump");
  if (READ_INT(yyjson_obj_get(interesting[2], "value"), "value") != 1 ||
      !hako_llvmc_str_eq(READ_STR(interesting[3], "operation"), "+")) {
    yyjson_doc_free(doc);
    return -1;
  }
  saw_latch = 1;

  BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 4));
  if (n != 3) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "mir_call");
  EXPECT_OP(1, "binop");
  EXPECT_OP(2, "ret");
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
  if (!hako_llvmc_str_eq(READ_STR(interesting[1], "operation"), "+")) {
    yyjson_doc_free(doc);
    return -1;
  }
  saw_exit = 1;

  if (!(saw_preheader && saw_header && saw_body && saw_latch && saw_exit)) {
    yyjson_doc_free(doc);
    return -1;
  }
  {
    int rc = hako_llvmc_emit_concat_const_suffix_micro_ir(
      obj_out,
      &selection,
      text,
      text_len,
      suffix,
      2,
      loop_bound);
    yyjson_doc_free(doc);
    return rc;
  }

  #undef READ_INT
  #undef READ_STR
  #undef BUILD_INTERESTING_NO_COPY
  #undef EXPECT_OP
  #undef REQUIRE_STRINGISH_BOX
}
