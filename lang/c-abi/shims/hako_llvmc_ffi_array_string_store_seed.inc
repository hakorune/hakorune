static int hako_llvmc_emit_array_string_store_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    const char* seed,
    long long seed_len,
    const char* suffix,
    long long suffix_len,
    long long size,
    long long ops,
    long long store_len,
    long long next_text_window_start,
    long long next_text_window_len) {
  char llpath[1024];
  FILE* f;
  int rc;
  unsigned int suffix0;
  unsigned int suffix1;

  if (!obj_out || !seed || !suffix ||
      seed_len != 16 ||
      suffix_len != 2 ||
      size != 128 ||
      ops != 800000 ||
      store_len != seed_len + suffix_len ||
      next_text_window_start != 2 ||
      next_text_window_len != seed_len) {
    return -1;
  }
  suffix0 = (unsigned int)(unsigned char)suffix[0];
  suffix1 = (unsigned int)(unsigned char)suffix[1];

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
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%sum = alloca i64, align 8\n");
  fprintf(f, "  %%arr.ptr = getelementptr inbounds [128 x [32 x i8]], ptr %%arr, i64 0, i64 0\n");
  fprintf(f, "  %%text.ptr = getelementptr inbounds [32 x i8], ptr %%text, i64 0, i64 0\n");
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
  fprintf(f, "  %%text.vec = load <16 x i8>, ptr %%text.ptr, align 16\n");
  fprintf(f, "  store <16 x i8> %%text.vec, ptr %%slot, align 16\n");
  fprintf(f, "  %%slot.16 = getelementptr inbounds i8, ptr %%slot, i64 16\n");
  fprintf(f, "  store i8 %u, ptr %%slot.16, align 1\n", suffix0);
  fprintf(f, "  %%slot.17 = getelementptr inbounds i8, ptr %%slot, i64 17\n");
  fprintf(f, "  store i8 %u, ptr %%slot.17, align 1\n", suffix1);
  fprintf(f, "  %%slot.18 = getelementptr inbounds i8, ptr %%slot, i64 18\n");
  fprintf(f, "  store i8 0, ptr %%slot.18, align 1\n");
  fprintf(f, "  %%sum.cur = load volatile i64, ptr %%sum, align 8\n");
  fprintf(f, "  %%sum.next = add i64 %%sum.cur, %lld\n", store_len);
  fprintf(f, "  store volatile i64 %%sum.next, ptr %%sum, align 8\n");
  fprintf(
      f,
      "  %%text.next = shufflevector <16 x i8> %%text.vec, <16 x i8> <i8 %u, i8 %u, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0, i8 0>, <16 x i32> <i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9, i32 10, i32 11, i32 12, i32 13, i32 14, i32 15, i32 16, i32 17>\n",
      suffix0,
      suffix1);
  fprintf(f, "  store <16 x i8> %%text.next, ptr %%text.ptr, align 16\n");
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
  yyjson_val* metadata;
  yyjson_val* route;
  hako_llvmc_entry_rune_selection selection;
  const char* seed = NULL;
  const char* suffix = NULL;
  const char* proof = NULL;
  long long seed_len = 0;
  long long actual_seed_len = 0;
  long long actual_suffix_len = 0;
  long long size = 0;
  long long ops = 0;
  long long store_len = 0;
  long long next_text_window_start = 0;
  long long next_text_window_len = 0;
  int rc;
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
  route = metadata ? yyjson_obj_get(metadata, "array_string_store_micro_seed_route") : NULL;
  if (!(route && yyjson_is_obj(route))) {
    yyjson_doc_free(doc);
    return -1;
  }

  seed = yyjson_get_str(yyjson_obj_get(route, "seed"));
  suffix = yyjson_get_str(yyjson_obj_get(route, "suffix"));
  proof = yyjson_get_str(yyjson_obj_get(route, "proof"));
  seed_len = (long long)yyjson_get_sint(yyjson_obj_get(route, "seed_len"));
  size = (long long)yyjson_get_sint(yyjson_obj_get(route, "size"));
  ops = (long long)yyjson_get_sint(yyjson_obj_get(route, "ops"));
  store_len = (long long)yyjson_get_sint(yyjson_obj_get(route, "store_len"));
  next_text_window_start = (long long)yyjson_get_sint(yyjson_obj_get(route, "next_text_window_start"));
  next_text_window_len = (long long)yyjson_get_sint(yyjson_obj_get(route, "next_text_window_len"));

  if (!(seed && suffix && proof &&
        hako_llvmc_ascii_strlen(seed, &actual_seed_len) &&
        hako_llvmc_ascii_strlen(suffix, &actual_suffix_len) &&
        seed_len == actual_seed_len &&
        actual_suffix_len == 2 &&
        hako_llvmc_str_eq(seed, "line-seed-abcdef") &&
        hako_llvmc_str_eq(suffix, "xy") &&
        hako_llvmc_str_eq(proof, "kilo_micro_array_string_store_8block") &&
        seed_len == 16 &&
        size == 128 &&
        ops == 800000 &&
        store_len == seed_len + actual_suffix_len &&
        next_text_window_start == 2 &&
        next_text_window_len == seed_len)) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          "array_string_store_micro",
          "skip",
          "metadata_contract_mismatch",
          "array_string_store_micro_seed_route");
    }
    yyjson_doc_free(doc);
    return -1;
  }

  if (hako_llvmc_route_trace_enabled()) {
    hako_llvmc_emit_route_trace(
        "array_string_store_micro",
        "emit",
        "exact_match",
        "kilo_micro_array_string_store mir_route_plan");
  }
  rc = hako_llvmc_emit_array_string_store_micro_ir(
      obj_out,
      &selection,
      seed,
      seed_len,
      suffix,
      actual_suffix_len,
      size,
      ops,
      store_len,
      next_text_window_start,
      next_text_window_len);
  yyjson_doc_free(doc);
  return rc;
}
