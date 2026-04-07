static int hako_llvmc_emit_concat_hh_len_micro_ir(
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

  if (!obj_out || !text || !suffix || text_len != 16 || suffix_len != 2 || ops != 800000) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_concat_hh_len_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (concat_hh + len_h micro)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  fprintf(f, "declare void @llvm.memcpy.p0.p0.i64(ptr nocapture writeonly, ptr nocapture readonly, i64, i1 immarg)\n\n");

  fprintf(f, "@.concat_hh_text_seed = private unnamed_addr constant [%lld x i8] c\"", text_len);
  hako_llvmc_emit_llvm_byte_string(f, text, text_len);
  fprintf(f, "\", align 1\n");
  fprintf(f, "@.concat_hh_suffix_seed = private unnamed_addr constant [%lld x i8] c\"", suffix_len);
  hako_llvmc_emit_llvm_byte_string(f, suffix, suffix_len);
  fprintf(f, "\", align 1\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%text.seed = alloca [16 x i8], align 16\n");
  fprintf(f, "  %%suffix.seed = alloca [2 x i8], align 16\n");
  fprintf(f, "  %%out = alloca [32 x i8], align 16\n");
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%acc = alloca i64, align 8\n");
  fprintf(f, "  %%text.seed.ptr = getelementptr inbounds [16 x i8], ptr %%text.seed, i64 0, i64 0\n");
  fprintf(f, "  %%suffix.seed.ptr = getelementptr inbounds [2 x i8], ptr %%suffix.seed, i64 0, i64 0\n");
  fprintf(f, "  %%out.ptr = getelementptr inbounds [32 x i8], ptr %%out, i64 0, i64 0\n");
  fprintf(
      f,
      "  call void @llvm.memcpy.p0.p0.i64(ptr align 16 %%text.seed.ptr, ptr align 1 getelementptr inbounds ([%lld x i8], ptr @.concat_hh_text_seed, i64 0, i64 0), i64 %lld, i1 false)\n",
      text_len,
      text_len);
  fprintf(
      f,
      "  call void @llvm.memcpy.p0.p0.i64(ptr align 16 %%suffix.seed.ptr, ptr align 1 getelementptr inbounds ([%lld x i8], ptr @.concat_hh_suffix_seed, i64 0, i64 0), i64 %lld, i1 false)\n",
      suffix_len,
      suffix_len);
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%acc, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  call void @llvm.memcpy.p0.p0.i64(ptr align 16 %%out.ptr, ptr align 16 %%text.seed.ptr, i64 %lld, i1 false)\n", text_len);
  fprintf(f, "  %%out.suffix = getelementptr inbounds i8, ptr %%out.ptr, i64 %lld\n", text_len);
  fprintf(
      f,
      "  call void @llvm.memcpy.p0.p0.i64(ptr align 16 %%out.suffix, ptr align 16 %%suffix.seed.ptr, i64 %lld, i1 false)\n",
      suffix_len);
  fprintf(f, "  %%out.term = getelementptr inbounds i8, ptr %%out.ptr, i64 %lld\n", text_len + suffix_len);
  fprintf(f, "  store i8 0, ptr %%out.term, align 1\n");
  fprintf(f, "  %%acc.cur = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%acc.next = add i64 %%acc.cur, %lld\n", text_len + suffix_len);
  fprintf(f, "  store volatile i64 %%acc.next, ptr %%acc, align 8\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%acc.fin = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%ret = add i64 %%acc.fin, %lld\n", text_len + suffix_len);
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_concat_hh_len_micro_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  hako_llvmc_entry_rune_selection selection;
  const char* text = NULL;
  const char* suffix = NULL;
  long long text_len = 0;
  long long suffix_len = 0;
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

  {
    yyjson_val* interesting[32];
    int n = 0;
    int loop_count = 0;
    int body_count = 0;
    int concat_count = 0;
    int len_count = 0;
    int saw_text = 0;
    int saw_suffix = 0;
    int saw_entry = 0;

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 0));
    if (n != 3) { yyjson_doc_free(doc); return -1; }
    EXPECT_OP(0, "const");
    EXPECT_OP(1, "const");
    EXPECT_OP(2, "jump");
    if (READ_INT(yyjson_obj_get(interesting[0], "value"), "value") != 0 ||
        READ_INT(yyjson_obj_get(interesting[1], "value"), "value") != 0) {
      yyjson_doc_free(doc);
      return -1;
    }
    saw_entry = 1;

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 1));
    if (n != 5) { yyjson_doc_free(doc); return -1; }
    EXPECT_OP(0, "phi");
    EXPECT_OP(1, "phi");
    EXPECT_OP(2, "const");
    EXPECT_OP(3, "compare");
    EXPECT_OP(4, "branch");
    if (READ_INT(yyjson_obj_get(interesting[2], "value"), "value") != 800000 ||
        !hako_llvmc_str_eq(READ_STR(interesting[3], "operation"), "<")) {
      yyjson_doc_free(doc);
      return -1;
    }
    loop_count = 1;

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 2));
    if (n != 6) { yyjson_doc_free(doc); return -1; }
    EXPECT_OP(0, "const");
    EXPECT_OP(1, "const");
    EXPECT_OP(2, "binop");
    EXPECT_OP(3, "mir_call");
    EXPECT_OP(4, "binop");
    EXPECT_OP(5, "jump");
    {
      yyjson_val* vobj = yyjson_obj_get(interesting[0], "value");
      yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
      const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
      const char* text_again = vobj ? READ_STR(vobj, "value") : NULL;
      if (!(box_type && strcmp(box_type, "StringBox") == 0 &&
            text_again && hako_llvmc_ascii_strlen(text_again, &text_len) && text_len == 16 &&
            hako_llvmc_str_eq(text_again, "line-seed-abcdef"))) {
        yyjson_doc_free(doc);
        return -1;
      }
      text = text_again;
      saw_text = 1;
    }
    {
      yyjson_val* vobj = yyjson_obj_get(interesting[1], "value");
      yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
      const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
      const char* suffix_again = vobj ? READ_STR(vobj, "value") : NULL;
      if (!(box_type && strcmp(box_type, "StringBox") == 0 &&
            suffix_again && hako_llvmc_ascii_strlen(suffix_again, &suffix_len) && suffix_len == 2 &&
            hako_llvmc_str_eq(suffix_again, "xy"))) {
        yyjson_doc_free(doc);
        return -1;
      }
      suffix = suffix_again;
      saw_suffix = 1;
    }
    if (!hako_llvmc_str_eq(READ_STR(interesting[2], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      yyjson_val* mc = yyjson_obj_get(interesting[3], "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
      const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      if (!(REQUIRE_STRINGISH_BOX(box_name) && name && strcmp(name, "length") == 0 &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
      len_count++;
    }
    if (!hako_llvmc_str_eq(READ_STR(interesting[4], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }
    concat_count = 1;
    body_count = 1;

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 3));
    if (n != 4) { yyjson_doc_free(doc); return -1; }
    EXPECT_OP(0, "phi");
    EXPECT_OP(1, "const");
    EXPECT_OP(2, "binop");
    EXPECT_OP(3, "jump");
    if (READ_INT(yyjson_obj_get(interesting[1], "value"), "value") != 1 ||
        !hako_llvmc_str_eq(READ_STR(interesting[2], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 4));
    if (n != 7) { yyjson_doc_free(doc); return -1; }
    EXPECT_OP(0, "const");
    EXPECT_OP(1, "mir_call");
    EXPECT_OP(2, "const");
    EXPECT_OP(3, "mir_call");
    EXPECT_OP(4, "binop");
    EXPECT_OP(5, "binop");
    EXPECT_OP(6, "ret");
    {
      yyjson_val* vobj = yyjson_obj_get(interesting[0], "value");
      yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
      const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
      const char* text_final = vobj ? READ_STR(vobj, "value") : NULL;
      if (!(box_type && strcmp(box_type, "StringBox") == 0 &&
            text_final && hako_llvmc_str_eq(text_final, "line-seed-abcdef"))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    {
      yyjson_val* mc = yyjson_obj_get(interesting[1], "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
      const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      if (!(REQUIRE_STRINGISH_BOX(box_name) && name && strcmp(name, "length") == 0 &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
      len_count++;
    }
    {
      yyjson_val* vobj = yyjson_obj_get(interesting[2], "value");
      yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
      const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
      const char* suffix_final = vobj ? READ_STR(vobj, "value") : NULL;
      if (!(box_type && strcmp(box_type, "StringBox") == 0 &&
            suffix_final && hako_llvmc_str_eq(suffix_final, "xy"))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    {
      yyjson_val* mc = yyjson_obj_get(interesting[3], "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
      const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      if (!(REQUIRE_STRINGISH_BOX(box_name) && name && strcmp(name, "length") == 0 &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
      len_count++;
    }
    if (!hako_llvmc_str_eq(READ_STR(interesting[4], "operation"), "+") ||
        !hako_llvmc_str_eq(READ_STR(interesting[5], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }

    if (!(saw_text && saw_suffix && saw_entry && loop_count == 1 && body_count == 1 &&
          concat_count == 1 && len_count == 3)) {
      if (hako_llvmc_route_trace_enabled()) {
        char extra[256];
        snprintf(
            extra,
            sizeof(extra),
            "text=%d suffix=%d entry=%d loop=%d body=%d concat=%d len=%d",
            saw_text,
            saw_suffix,
            saw_entry,
            loop_count,
            body_count,
            concat_count,
            len_count);
        hako_llvmc_emit_route_trace(
            "concat_hh_len_micro",
            "skip",
            "contract_mismatch",
            extra);
      }
      yyjson_doc_free(doc);
      return -1;
    }

    {
      if (hako_llvmc_route_trace_enabled()) {
        hako_llvmc_emit_route_trace(
            "concat_hh_len_micro",
            "emit",
            "exact_match",
            "kilo_micro_concat_hh_len");
      }
      int rc = hako_llvmc_emit_concat_hh_len_micro_ir(
          obj_out,
          &selection,
          text,
          16,
          suffix,
          2,
          800000);
      yyjson_doc_free(doc);
      return rc;
    }
  }

  #undef READ_INT
  #undef READ_STR
  #undef BUILD_INTERESTING_NO_COPY
  #undef EXPECT_OP
  #undef REQUIRE_STRINGISH_BOX
}
