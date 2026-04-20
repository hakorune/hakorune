static int hako_llvmc_emit_array_string_store_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    const char* seed,
    long long seed_len,
    long long size,
    long long ops) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long store_len;
  long long store_bytes;

  if (!obj_out || !seed || seed_len != 16 || size != 128 || ops != 800000) {
    return -1;
  }
  store_len = seed_len + 2;
  store_bytes = store_len + 1;

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_array_string_store_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (array string-store micro)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  fprintf(f, "declare void @llvm.memcpy.p0.p0.i64(ptr nocapture writeonly, ptr nocapture readonly, i64, i1 immarg)\n");
  fprintf(f, "\n");

  fprintf(f, "@.array_string_seed = private unnamed_addr constant [%lld x i8] c\"", seed_len);
  hako_llvmc_emit_llvm_byte_string(f, seed, seed_len);
  fprintf(f, "\", align 1\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%arr = alloca [128 x [32 x i8]], align 16\n");
  fprintf(f, "  %%text = alloca [32 x i8], align 16\n");
  fprintf(f, "  %%out = alloca [32 x i8], align 16\n");
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%sum = alloca i64, align 8\n");
  fprintf(f, "  %%arr.ptr = getelementptr inbounds [128 x [32 x i8]], ptr %%arr, i64 0, i64 0\n");
  fprintf(f, "  %%text.ptr = getelementptr inbounds [32 x i8], ptr %%text, i64 0, i64 0\n");
  fprintf(f, "  %%out.ptr = getelementptr inbounds [32 x i8], ptr %%out, i64 0, i64 0\n");
  fprintf(
      f,
      "  call void @llvm.memcpy.p0.p0.i64(ptr align 16 %%text.ptr, ptr align 1 getelementptr inbounds ([%lld x i8], ptr @.array_string_seed, i64 0, i64 0), i64 %lld, i1 false)\n",
      seed_len,
      seed_len);
  fprintf(f, "  %%text.term = getelementptr inbounds i8, ptr %%text.ptr, i64 16\n");
  fprintf(f, "  store i8 0, ptr %%text.term, align 1\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  br label %%init\n\n");

  fprintf(f, "init:\n");
  fprintf(f, "  %%i.init = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%init.cond = icmp slt i64 %%i.init, 128\n");
  fprintf(f, "  br i1 %%init.cond, label %%init.body, label %%init.exit\n\n");

  fprintf(f, "init.body:\n");
  fprintf(f, "  %%init.slot = getelementptr inbounds [32 x i8], ptr %%arr.ptr, i64 %%i.init\n");
  fprintf(
      f,
      "  call void @llvm.memcpy.p0.p0.i64(ptr align 16 %%init.slot, ptr align 16 %%text.ptr, i64 %lld, i1 false)\n",
      seed_len);
  fprintf(f, "  %%init.term = getelementptr inbounds i8, ptr %%init.slot, i64 16\n");
  fprintf(f, "  store i8 0, ptr %%init.term, align 1\n");
  fprintf(f, "  %%i.init.next = add i64 %%i.init, 1\n");
  fprintf(f, "  store i64 %%i.init.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%init\n\n");

  fprintf(f, "init.exit:\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%sum, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  %%idx = and i64 %%i.cur, 127\n");
  fprintf(f, "  %%slot = getelementptr inbounds [32 x i8], ptr %%arr.ptr, i64 %%idx\n");
  fprintf(
      f,
      "  call void @llvm.memcpy.p0.p0.i64(ptr align 16 %%out.ptr, ptr align 16 %%text.ptr, i64 %lld, i1 false)\n",
      seed_len);
  fprintf(f, "  %%out.16 = getelementptr inbounds i8, ptr %%out.ptr, i64 16\n");
  fprintf(f, "  store i8 120, ptr %%out.16, align 1\n");
  fprintf(f, "  %%out.17 = getelementptr inbounds i8, ptr %%out.ptr, i64 17\n");
  fprintf(f, "  store i8 121, ptr %%out.17, align 1\n");
  fprintf(f, "  %%out.18 = getelementptr inbounds i8, ptr %%out.ptr, i64 18\n");
  fprintf(f, "  store i8 0, ptr %%out.18, align 1\n");
  fprintf(f, "  call void @llvm.memcpy.p0.p0.i64(ptr align 16 %%slot, ptr align 16 %%out.ptr, i64 %lld, i1 false)\n", store_bytes);
  fprintf(f, "  %%sum.cur = load volatile i64, ptr %%sum, align 8\n");
  fprintf(f, "  %%sum.next = add i64 %%sum.cur, %lld\n", store_len);
  fprintf(f, "  store volatile i64 %%sum.next, ptr %%sum, align 8\n");
  fprintf(f, "  %%text.shift = getelementptr inbounds i8, ptr %%out.ptr, i64 2\n");
  fprintf(
      f,
      "  call void @llvm.memcpy.p0.p0.i64(ptr align 16 %%text.ptr, ptr align 1 %%text.shift, i64 %lld, i1 false)\n",
      seed_len);
  fprintf(f, "  %%text.16 = getelementptr inbounds i8, ptr %%text.ptr, i64 16\n");
  fprintf(f, "  store i8 0, ptr %%text.16, align 1\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%sum.fin = load volatile i64, ptr %%sum, align 8\n");
  fprintf(f, "  %%ret = add i64 %%sum.fin, 16\n");
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_array_string_store_micro_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  hako_llvmc_entry_rune_selection selection;
  size_t block_count = 0;
  long long size = 0;
  long long ops = 0;
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
  if (!(blocks && yyjson_is_arr(blocks))) {
    yyjson_doc_free(doc);
    return -1;
  }
  block_count = yyjson_arr_size(blocks);
  if (block_count != 8) {
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
    int push_count = 0;
    int set_count = 0;
    int length_count = 0;
    int substring_count = 0;
    int saw_seed = 0;
    int saw_suffix = 0;
    int saw_array_init = 0;
    int saw_main_loop = 0;

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 0));
    if (n != 4) { yyjson_doc_free(doc); return -1; }
    EXPECT_OP(0, "newbox");
    EXPECT_OP(1, "const");
    EXPECT_OP(2, "const");
    EXPECT_OP(3, "jump");
    {
      const char* array_type = READ_STR(interesting[0], "type");
      if (!(array_type && strcmp(array_type, "ArrayBox") == 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    {
      yyjson_val* vobj = yyjson_obj_get(interesting[1], "value");
      yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
      const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
      seed = vobj ? READ_STR(vobj, "value") : NULL;
      if (!(box_type && strcmp(box_type, "StringBox") == 0 &&
            seed && hako_llvmc_ascii_strlen(seed, &size) && size == 16 &&
            hako_llvmc_str_eq(seed, "line-seed-abcdef"))) {
        yyjson_doc_free(doc);
        return -1;
      }
      saw_seed = 1;
    }
    if (READ_INT(yyjson_obj_get(interesting[2], "value"), "value") != 0) {
      yyjson_doc_free(doc);
      return -1;
    }

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 1));
    if (n != 4) { yyjson_doc_free(doc); return -1; }
    EXPECT_OP(0, "phi");
    EXPECT_OP(1, "const");
    EXPECT_OP(2, "compare");
    EXPECT_OP(3, "branch");
    size = READ_INT(yyjson_obj_get(interesting[1], "value"), "value");
    if (!(size == 128 && hako_llvmc_str_eq(READ_STR(interesting[2], "operation"), "<"))) {
      yyjson_doc_free(doc);
      return -1;
    }

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 2));
    if (n != 5) { yyjson_doc_free(doc); return -1; }
    EXPECT_OP(0, "const");
    EXPECT_OP(1, "mir_call");
    EXPECT_OP(2, "const");
    EXPECT_OP(3, "binop");
    EXPECT_OP(4, "jump");
    {
      yyjson_val* mc = yyjson_obj_get(interesting[1], "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
      const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      const char* push_seed = READ_STR(yyjson_obj_get(interesting[0], "value"), "value");
      if (!(box_name && strcmp(box_name, "RuntimeDataBox") == 0 &&
            name && strcmp(name, "push") == 0 &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 1 &&
            push_seed && hako_llvmc_str_eq(push_seed, "line-seed-abcdef"))) {
        yyjson_doc_free(doc);
        return -1;
      }
      push_count++;
    }
    if (READ_INT(yyjson_obj_get(interesting[2], "value"), "value") != 1 ||
        !hako_llvmc_str_eq(READ_STR(interesting[3], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 3));
    for (i = 0; i < (size_t)n; i++) {
      if (hako_llvmc_str_eq(READ_STR(interesting[i], "op"), "release_strong")) {
        saw_array_init = 1;
        break;
      }
    }
    if (!saw_array_init) {
      yyjson_doc_free(doc);
      return -1;
    }

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 4));
    if (n != 6) { yyjson_doc_free(doc); return -1; }
    EXPECT_OP(0, "phi");
    EXPECT_OP(1, "phi");
    EXPECT_OP(2, "phi");
    EXPECT_OP(3, "const");
    EXPECT_OP(4, "compare");
    EXPECT_OP(5, "branch");
    if (READ_INT(yyjson_obj_get(interesting[3], "value"), "value") != 800000 ||
        !hako_llvmc_str_eq(READ_STR(interesting[4], "operation"), "<")) {
      yyjson_doc_free(doc);
      return -1;
    }
    saw_main_loop = 1;

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 5));
    if (n != 14) { yyjson_doc_free(doc); return -1; }
    EXPECT_OP(0, "const");
    EXPECT_OP(1, "binop");
    EXPECT_OP(2, "const");
    EXPECT_OP(3, "binop");
    EXPECT_OP(4, "mir_call");
    EXPECT_OP(5, "mir_call");
    EXPECT_OP(6, "binop");
    EXPECT_OP(7, "const");
    EXPECT_OP(8, "mir_call");
    EXPECT_OP(9, "const");
    EXPECT_OP(10, "const");
    EXPECT_OP(11, "binop");
    EXPECT_OP(12, "mir_call");
    EXPECT_OP(13, "jump");
    if (READ_INT(yyjson_obj_get(interesting[0], "value"), "value") != 128 ||
        !hako_llvmc_str_eq(READ_STR(interesting[1], "operation"), "%")) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      yyjson_val* vobj = yyjson_obj_get(interesting[2], "value");
      yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
      const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
      const char* suffix = vobj ? READ_STR(vobj, "value") : NULL;
      if (!(box_type && strcmp(box_type, "StringBox") == 0 &&
            suffix && hako_llvmc_str_eq(suffix, "xy"))) {
        yyjson_doc_free(doc);
        return -1;
      }
      saw_suffix = 1;
    }
    if (!hako_llvmc_str_eq(READ_STR(interesting[3], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      yyjson_val* mc = yyjson_obj_get(interesting[4], "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
      const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      if (!(REQUIRE_STRINGISH_BOX(box_name) && name && strcmp(name, "set") == 0 &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 2)) {
        yyjson_doc_free(doc);
        return -1;
      }
      set_count++;
    }
    {
      yyjson_val* mc = yyjson_obj_get(interesting[5], "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
      const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      if (!(REQUIRE_STRINGISH_BOX(box_name) && name && strcmp(name, "length") == 0 &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
      length_count++;
    }
    if (!hako_llvmc_str_eq(READ_STR(interesting[6], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      yyjson_val* vobj = yyjson_obj_get(interesting[7], "value");
      yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
      const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
      const char* seed_again = vobj ? READ_STR(vobj, "value") : NULL;
      if (!(box_type && strcmp(box_type, "StringBox") == 0 &&
            seed_again && hako_llvmc_str_eq(seed_again, "line-seed-abcdef"))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    {
      yyjson_val* mc = yyjson_obj_get(interesting[8], "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
      const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      if (!(REQUIRE_STRINGISH_BOX(box_name) && name && strcmp(name, "length") == 0 &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
      length_count++;
    }
    if (READ_INT(yyjson_obj_get(interesting[9], "value"), "value") != 2 ||
        READ_INT(yyjson_obj_get(interesting[10], "value"), "value") != 2 ||
        !hako_llvmc_str_eq(READ_STR(interesting[11], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      yyjson_val* mc = yyjson_obj_get(interesting[12], "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      const char* box_name = cal ? (READ_STR(cal, "box_name") ? READ_STR(cal, "box_name") : READ_STR(cal, "box_type")) : NULL;
      const char* name = cal ? (READ_STR(cal, "method") ? READ_STR(cal, "method") : READ_STR(cal, "name")) : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      if (!(REQUIRE_STRINGISH_BOX(box_name) && name && strcmp(name, "substring") == 0 &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 2)) {
        yyjson_doc_free(doc);
        return -1;
      }
      substring_count++;
    }

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 6));
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

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 7));
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
      length_count++;
    }
    if (!hako_llvmc_str_eq(READ_STR(interesting[1], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }

    if (!(saw_seed && saw_suffix && saw_array_init && saw_main_loop &&
          push_count == 1 && set_count == 1 && substring_count == 1 &&
          length_count == 3)) {
      if (hako_llvmc_route_trace_enabled()) {
        char extra[256];
        snprintf(
            extra,
            sizeof(extra),
            "seed=%d suffix=%d init=%d loop=%d push=%d set=%d substring=%d len=%d",
            saw_seed,
            saw_suffix,
            saw_array_init,
            saw_main_loop,
            push_count,
            set_count,
            substring_count,
            length_count);
        hako_llvmc_emit_route_trace(
            "array_string_store_micro",
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
            "array_string_store_micro",
            "emit",
            "exact_match",
            "kilo_micro_array_string_store");
      }
      int rc = hako_llvmc_emit_array_string_store_micro_ir(
          obj_out,
          &selection,
          seed,
          16,
          128,
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
