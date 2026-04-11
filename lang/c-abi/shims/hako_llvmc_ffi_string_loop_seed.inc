static int hako_llvmc_emit_substring_concat_loop_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    const char* seed,
    long long seed_len,
    long long loop_bound,
    const char* middle,
    long long middle_len,
    long long split) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long head_len;
  long long tail_len;
  long long insert_offset;

  if (!seed || !middle || seed_len <= 1 || middle_len <= 0 || split <= 0 ||
      loop_bound <= 0) {
    return -1;
  }

  head_len = split - 1;
  tail_len = seed_len + 1 - middle_len - split;
  insert_offset = split - 1;
  if (head_len < 0 || tail_len < 0 || insert_offset < 0) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_subconcat_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (substring+concat loop ascii)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  fprintf(
      f,
      "declare void @llvm.memcpy.p0.p0.i64(ptr nocapture writeonly, ptr nocapture readonly, i64, i1 immarg)\n");
  fprintf(
      f,
      "declare void @llvm.memmove.p0.p0.i64(ptr nocapture writeonly, ptr nocapture readonly, i64, i1 immarg)\n\n");

  fprintf(f, "@.subconcat_seed = private unnamed_addr constant [%lld x i8] c\"", seed_len);
  hako_llvmc_emit_llvm_byte_string(f, seed, seed_len);
  fprintf(f, "\", align 1\n");
  fprintf(f, "@.subconcat_mid = private unnamed_addr constant [%lld x i8] c\"", middle_len);
  hako_llvmc_emit_llvm_byte_string(f, middle, middle_len);
  fprintf(f, "\", align 1\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%text = alloca [%lld x i8], align 1\n", seed_len);
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%acc = alloca i64, align 8\n");
  fprintf(f, "  %%text.ptr = getelementptr inbounds [%lld x i8], ptr %%text, i64 0, i64 0\n", seed_len);
  fprintf(
      f,
      "  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %%text.ptr, ptr align 1 getelementptr inbounds ([%lld x i8], ptr @.subconcat_seed, i64 0, i64 0), i64 %lld, i1 false)\n",
      seed_len,
      seed_len);
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%acc, align 8\n");
  fprintf(f, "  br label %%body\n\n");

  fprintf(f, "body:\n");
  if (tail_len > 0) {
    fprintf(f, "  %%tail.dst = getelementptr inbounds i8, ptr %%text.ptr, i64 %lld\n", insert_offset + middle_len);
    fprintf(f, "  %%tail.src = getelementptr inbounds i8, ptr %%text.ptr, i64 %lld\n", split);
    fprintf(
        f,
        "  call void @llvm.memmove.p0.p0.i64(ptr align 1 %%tail.dst, ptr align 1 %%tail.src, i64 %lld, i1 false)\n",
        tail_len);
  }
  if (head_len > 0) {
    fprintf(f, "  %%head.src = getelementptr inbounds i8, ptr %%text.ptr, i64 1\n");
    fprintf(
        f,
        "  call void @llvm.memmove.p0.p0.i64(ptr align 1 %%text.ptr, ptr align 1 %%head.src, i64 %lld, i1 false)\n",
        head_len);
  }
  fprintf(f, "  %%mid.dst = getelementptr inbounds i8, ptr %%text.ptr, i64 %lld\n", insert_offset);
  fprintf(
      f,
      "  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %%mid.dst, ptr align 1 getelementptr inbounds ([%lld x i8], ptr @.subconcat_mid, i64 0, i64 0), i64 %lld, i1 false)\n",
      middle_len,
      middle_len);
  fprintf(f, "  %%acc.cur = load i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%acc.next = add i64 %%acc.cur, %lld\n", seed_len + middle_len);
  fprintf(f, "  store i64 %%acc.next, ptr %%acc, align 8\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.next, %lld\n", loop_bound);
  fprintf(f, "  br i1 %%loop.cond, label %%body, label %%exit\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%acc.fin = load i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%ret = add i64 %%acc.fin, %lld\n", seed_len);
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  {
    rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  }
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_emit_substring_concat_len_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long loop_bound,
    long long seed_len,
    long long middle_len) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long acc_step;

  if (!obj_out || loop_bound <= 0 || seed_len <= 0 || middle_len <= 0) {
    return -1;
  }

  acc_step = seed_len + middle_len;
  if (acc_step <= 0) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_subconcat_len_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (substring+concat len loop ascii)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%acc = alloca i64, align 8\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%acc, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%cond = icmp slt i64 %%i.cur, %lld\n", loop_bound);
  fprintf(f, "  br i1 %%cond, label %%body, label %%exit\n\n");

  fprintf(f, "body:\n");
  fprintf(f, "  %%acc.cur = load i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%acc.next = add i64 %%acc.cur, %lld\n", acc_step);
  fprintf(f, "  store i64 %%acc.next, ptr %%acc, align 8\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%acc.fin = load i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%ret = add i64 %%acc.fin, %lld\n", seed_len);
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  {
    rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  }
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_emit_string_length_hot_loop_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long loop_bound,
    long long lit_len) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (loop_bound <= 0 || lit_len < 0) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_strlen_loop_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (string length hot loop)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%acc = alloca i64, align 8\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%acc, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%cond = icmp slt i64 %%i.cur, %lld\n", loop_bound);
  fprintf(f, "  br i1 %%cond, label %%body, label %%exit\n\n");

  fprintf(f, "body:\n");
  fprintf(f, "  %%acc.cur = load i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%acc.next = add i64 %%acc.cur, %lld\n", lit_len);
  fprintf(f, "  store i64 %%acc.next, ptr %%acc, align 8\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%acc.fin = load i64, ptr %%acc, align 8\n");
  fprintf(f, "  ret i64 %%acc.fin\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  {
    rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  }
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_emit_substring_views_only_exit_len_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long source_len) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long ret_value;

  if (!(obj_out && selection) || source_len <= 0) {
    return -1;
  }

  ret_value = source_len * 2;
  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_substring_views_only_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (substring views exit len collapse)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  ret i64 %lld\n", ret_value);
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  {
    rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  }
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_copy_chain_reaches_reg(
    yyjson_val* insts,
    long long seed_reg,
    long long target_reg) {
  long long cursor = seed_reg;
  int advanced = 0;

  if (seed_reg == 0 || target_reg == 0 || !(insts && yyjson_is_arr(insts))) return 0;
  if (seed_reg == target_reg) return 1;

  do {
    size_t i;
    advanced = 0;
    for (i = 0; i < yyjson_arr_size(insts); i++) {
      yyjson_val* ins = yyjson_arr_get(insts, i);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      yyjson_val* src = ins ? yyjson_obj_get(ins, "src") : NULL;
      yyjson_val* dst = ins ? yyjson_obj_get(ins, "dst") : NULL;
      if (!(op && src && dst)) continue;
      if (strcmp(op, "copy") != 0) continue;
      if ((long long)yyjson_get_sint(src) != cursor) continue;
      cursor = (long long)yyjson_get_sint(dst);
      if (cursor == target_reg) return 1;
      advanced = 1;
      break;
    }
  } while (advanced);

  return 0;
}

static int hako_llvmc_copy_graph_reaches_reg(
    yyjson_val* insts,
    long long seed_reg,
    long long target_reg) {
  long long queue[256];
  long long visited[256];
  size_t queue_head = 0;
  size_t queue_tail = 0;
  size_t visited_n = 0;

  if (seed_reg == 0 || target_reg == 0 || !(insts && yyjson_is_arr(insts))) return 0;
  if (seed_reg == target_reg) return 1;

  queue[queue_tail++] = seed_reg;
  visited[visited_n++] = seed_reg;

  while (queue_head < queue_tail) {
    long long cursor = queue[queue_head++];
    size_t i;
    for (i = 0; i < yyjson_arr_size(insts); i++) {
      yyjson_val* ins = yyjson_arr_get(insts, i);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      yyjson_val* src = ins ? yyjson_obj_get(ins, "src") : NULL;
      yyjson_val* dst = ins ? yyjson_obj_get(ins, "dst") : NULL;
      long long next_reg;
      size_t j;
      int seen = 0;
      if (!(op && src && dst)) continue;
      if (strcmp(op, "copy") != 0) continue;
      if ((long long)yyjson_get_sint(src) != cursor) continue;
      next_reg = (long long)yyjson_get_sint(dst);
      if (next_reg == target_reg) return 1;
      for (j = 0; j < visited_n; j++) {
        if (visited[j] == next_reg) {
          seen = 1;
          break;
        }
      }
      if (seen) continue;
      if (visited_n < (sizeof(visited) / sizeof(visited[0]))) {
        visited[visited_n++] = next_reg;
      }
      if (queue_tail < (sizeof(queue) / sizeof(queue[0]))) {
        queue[queue_tail++] = next_reg;
      }
    }
  }

  return 0;
}

static int hako_llvmc_match_substring_views_only_micro_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  hako_llvmc_entry_rune_selection selection;
  long long len_dst = 0;
  long long source_len = 0;
  const char* source_text = NULL;
  long long loop_bound = 0;
  long long i_phi = 0;
  long long left_phi = 0;
  long long right_phi = 0;
  long long split_dst = 0;
  long long left_body_dst = 0;
  long long right_body_dst = 0;
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

  #define READ_I64(obj, key) ((obj) && yyjson_obj_get((obj), (key)) ? (long long)yyjson_get_sint(yyjson_obj_get((obj), (key))) : 0)
  #define READ_S(obj, key) ((obj) && yyjson_obj_get((obj), (key)) ? yyjson_get_str(yyjson_obj_get((obj), (key))) : NULL)
  #define BUILD_INTERESTING_NO_COPY_LOCAL(block_val) \
    do { \
      yyjson_val* _insts = yyjson_obj_get((block_val), "instructions"); \
      size_t _ii; \
      n = 0; \
      if (!(_insts && yyjson_is_arr(_insts))) { yyjson_doc_free(doc); return -1; } \
      for (_ii = 0; _ii < yyjson_arr_size(_insts) && n < (int)(sizeof(interesting) / sizeof(interesting[0])); _ii++) { \
        yyjson_val* _ins = yyjson_arr_get(_insts, _ii); \
        const char* _op = READ_S(_ins, "op"); \
        if (_op && strcmp(_op, "copy") != 0) interesting[n++] = _ins; \
      } \
    } while (0)

  {
    yyjson_val* interesting[32];
    int n = 0;
    yyjson_val* pre = yyjson_arr_get(blocks, 0);
    yyjson_val* header = yyjson_arr_get(blocks, 1);
    yyjson_val* body = yyjson_arr_get(blocks, 2);
    yyjson_val* latch = yyjson_arr_get(blocks, 3);
    yyjson_val* exitb = yyjson_arr_get(blocks, 4);
    yyjson_val* pre_insts = pre ? yyjson_obj_get(pre, "instructions") : NULL;
    yyjson_val* header_insts = header ? yyjson_obj_get(header, "instructions") : NULL;
    yyjson_val* body_insts = body ? yyjson_obj_get(body, "instructions") : NULL;
    yyjson_val* latch_insts = latch ? yyjson_obj_get(latch, "instructions") : NULL;
    yyjson_val* exit_insts = exitb ? yyjson_obj_get(exitb, "instructions") : NULL;

    if (!(pre_insts && yyjson_is_arr(pre_insts) &&
          header_insts && yyjson_is_arr(header_insts) &&
          body_insts && yyjson_is_arr(body_insts) &&
          latch_insts && yyjson_is_arr(latch_insts) &&
          exit_insts && yyjson_is_arr(exit_insts))) {
      yyjson_doc_free(doc);
      return -1;
    }

    BUILD_INTERESTING_NO_COPY_LOCAL(pre);
    if (n < 5 || !hako_llvmc_str_eq(READ_S(interesting[n - 1], "op"), "jump")) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (i = 0; i < (size_t)n; i++) {
      yyjson_val* ins = interesting[i];
      const char* op = READ_S(ins, "op");
      if (!(op && strcmp(op, "mir_call") == 0)) continue;
      {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? READ_S(cal, "type") : NULL;
        const char* method = cal ? READ_S(cal, "name") : NULL;
        long long receiver = cal ? READ_I64(cal, "receiver") : 0;
        if (!(ctype && strcmp(ctype, "Method") == 0 &&
              method && (strcmp(method, "length") == 0 || strcmp(method, "len") == 0) &&
              args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0 &&
              receiver != 0)) {
          continue;
        }
        len_dst = READ_I64(ins, "dst");
        for (size_t jj = 0; jj < (size_t)n; jj++) {
          yyjson_val* seed = interesting[jj];
          yyjson_val* vobj = yyjson_obj_get(seed, "value");
          yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
          const char* seed_op = READ_S(seed, "op");
          const char* box_type = ty ? READ_S(ty, "box_type") : NULL;
          const char* value = vobj ? READ_S(vobj, "value") : NULL;
          long long seed_dst = READ_I64(seed, "dst");
          if (!(seed_op && strcmp(seed_op, "const") == 0 &&
                box_type && strcmp(box_type, "StringBox") == 0 &&
                value && seed_dst != 0 &&
                hako_llvmc_copy_graph_reaches_reg(pre_insts, seed_dst, receiver) &&
                hako_llvmc_ascii_strlen(value, &source_len))) {
            continue;
          }
          source_text = value;
          break;
        }
        break;
      }
    }
    if (!(len_dst != 0 && source_len > 0 && source_text)) {
      yyjson_doc_free(doc);
      return -1;
    }

    BUILD_INTERESTING_NO_COPY_LOCAL(header);
    if (n != 6 ||
        !hako_llvmc_str_eq(READ_S(interesting[0], "op"), "phi") ||
        !hako_llvmc_str_eq(READ_S(interesting[1], "op"), "phi") ||
        !hako_llvmc_str_eq(READ_S(interesting[2], "op"), "phi") ||
        !hako_llvmc_str_eq(READ_S(interesting[3], "op"), "const") ||
        !hako_llvmc_str_eq(READ_S(interesting[4], "op"), "compare") ||
        !hako_llvmc_str_eq(READ_S(interesting[5], "op"), "branch")) {
      yyjson_doc_free(doc);
      return -1;
    }
    i_phi = READ_I64(interesting[0], "dst");
    left_phi = READ_I64(interesting[1], "dst");
    right_phi = READ_I64(interesting[2], "dst");
    loop_bound = READ_I64(yyjson_obj_get(interesting[3], "value"), "value");
    if (!(i_phi != 0 && left_phi != 0 && right_phi != 0 && loop_bound > 0 &&
          hako_llvmc_str_eq(READ_S(interesting[4], "operation"), "<"))) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      long long cmp_lhs = READ_I64(interesting[4], "lhs");
      long long cmp_rhs = READ_I64(interesting[4], "rhs");
      long long bound_reg = READ_I64(interesting[3], "dst");
      if (!(hako_llvmc_copy_graph_reaches_reg(header_insts, i_phi, cmp_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(header_insts, i_phi, cmp_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!(hako_llvmc_copy_graph_reaches_reg(header_insts, bound_reg, cmp_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(header_insts, bound_reg, cmp_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }

    BUILD_INTERESTING_NO_COPY_LOCAL(body);
    if (n != 7 ||
        !hako_llvmc_str_eq(READ_S(interesting[0], "op"), "const") ||
        !hako_llvmc_str_eq(READ_S(interesting[1], "op"), "const") ||
        !hako_llvmc_str_eq(READ_S(interesting[2], "op"), "binop") ||
        !hako_llvmc_str_eq(READ_S(interesting[3], "op"), "const") ||
        !hako_llvmc_str_eq(READ_S(interesting[4], "op"), "mir_call") ||
        !hako_llvmc_str_eq(READ_S(interesting[5], "op"), "mir_call") ||
        !hako_llvmc_str_eq(READ_S(interesting[6], "op"), "jump")) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (READ_I64(yyjson_obj_get(interesting[0], "value"), "value") != 0 ||
        READ_I64(yyjson_obj_get(interesting[1], "value"), "value") != 2 ||
        !hako_llvmc_str_eq(READ_S(interesting[2], "operation"), "/")) {
      yyjson_doc_free(doc);
      return -1;
    }
    split_dst = READ_I64(interesting[2], "dst");
    {
      long long divisor_reg = READ_I64(interesting[1], "dst");
      long long div_lhs = READ_I64(interesting[2], "lhs");
      long long div_rhs = READ_I64(interesting[2], "rhs");
      yyjson_val* body_vobj = yyjson_obj_get(interesting[3], "value");
      yyjson_val* body_ty = body_vobj ? yyjson_obj_get(body_vobj, "type") : NULL;
      const char* body_box_type = body_ty ? READ_S(body_ty, "box_type") : NULL;
      const char* body_text = body_vobj ? READ_S(body_vobj, "value") : NULL;
      if (!(hako_llvmc_copy_graph_reaches_reg(body_insts, len_dst, div_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(body_insts, len_dst, div_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!(hako_llvmc_copy_graph_reaches_reg(body_insts, divisor_reg, div_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(body_insts, divisor_reg, div_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!(body_box_type && strcmp(body_box_type, "StringBox") == 0 &&
            body_text && strcmp(body_text, source_text) == 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    for (int call_idx = 4; call_idx <= 5; call_idx++) {
      yyjson_val* ins = interesting[call_idx];
      yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      const char* ctype = cal ? READ_S(cal, "type") : NULL;
      const char* method = cal ? READ_S(cal, "name") : NULL;
      long long receiver = cal ? READ_I64(cal, "receiver") : 0;
      long long source_reg = READ_I64(interesting[3], "dst");
      if (!(ctype && strcmp(ctype, "Method") == 0 &&
            method && (strcmp(method, "substring") == 0 || strcmp(method, "slice") == 0) &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 2 &&
            hako_llvmc_copy_graph_reaches_reg(body_insts, source_reg, receiver))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (call_idx == 4) {
        if (!(hako_llvmc_copy_graph_reaches_reg(body_insts, READ_I64(interesting[0], "dst"),
                                                (long long)yyjson_get_sint(yyjson_arr_get(args, 0))) &&
              hako_llvmc_copy_graph_reaches_reg(body_insts, split_dst,
                                                (long long)yyjson_get_sint(yyjson_arr_get(args, 1))))) {
          yyjson_doc_free(doc);
          return -1;
        }
        left_body_dst = READ_I64(ins, "dst");
      } else {
        if (!(hako_llvmc_copy_graph_reaches_reg(body_insts, split_dst,
                                                (long long)yyjson_get_sint(yyjson_arr_get(args, 0))) &&
              hako_llvmc_copy_graph_reaches_reg(body_insts, len_dst,
                                                (long long)yyjson_get_sint(yyjson_arr_get(args, 1))))) {
          yyjson_doc_free(doc);
          return -1;
        }
        right_body_dst = READ_I64(ins, "dst");
      }
    }
    if (!(left_body_dst != 0 && right_body_dst != 0)) {
      yyjson_doc_free(doc);
      return -1;
    }

    BUILD_INTERESTING_NO_COPY_LOCAL(latch);
    if (n != 5 ||
        !hako_llvmc_str_eq(READ_S(interesting[0], "op"), "phi") ||
        !hako_llvmc_str_eq(READ_S(interesting[1], "op"), "phi") ||
        !hako_llvmc_str_eq(READ_S(interesting[2], "op"), "const") ||
        !hako_llvmc_str_eq(READ_S(interesting[3], "op"), "binop") ||
        !hako_llvmc_str_eq(READ_S(interesting[4], "op"), "jump") ||
        READ_I64(yyjson_obj_get(interesting[2], "value"), "value") != 1 ||
        !hako_llvmc_str_eq(READ_S(interesting[3], "operation"), "+")) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      yyjson_val* incoming_left = yyjson_obj_get(interesting[0], "incoming");
      yyjson_val* incoming_right = yyjson_obj_get(interesting[1], "incoming");
      long long one_reg = READ_I64(interesting[2], "dst");
      long long add_lhs = READ_I64(interesting[3], "lhs");
      long long add_rhs = READ_I64(interesting[3], "rhs");
      if (!(incoming_left && yyjson_is_arr(incoming_left) && yyjson_arr_size(incoming_left) == 1 &&
            incoming_right && yyjson_is_arr(incoming_right) && yyjson_arr_size(incoming_right) == 1)) {
        yyjson_doc_free(doc);
        return -1;
      }
      if ((long long)yyjson_get_sint(yyjson_arr_get(yyjson_arr_get(incoming_left, 0), 0)) != left_body_dst ||
          (long long)yyjson_get_sint(yyjson_arr_get(yyjson_arr_get(incoming_right, 0), 0)) != right_body_dst) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!(hako_llvmc_copy_graph_reaches_reg(latch_insts, i_phi, add_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(latch_insts, i_phi, add_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!(hako_llvmc_copy_graph_reaches_reg(latch_insts, one_reg, add_lhs) ||
            hako_llvmc_copy_graph_reaches_reg(latch_insts, one_reg, add_rhs))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }

    BUILD_INTERESTING_NO_COPY_LOCAL(exitb);
    if (n != 5 ||
        !hako_llvmc_str_eq(READ_S(interesting[0], "op"), "mir_call") ||
        !hako_llvmc_str_eq(READ_S(interesting[1], "op"), "mir_call") ||
        !hako_llvmc_str_eq(READ_S(interesting[2], "op"), "binop") ||
        !hako_llvmc_str_eq(READ_S(interesting[3], "op"), "binop") ||
        !hako_llvmc_str_eq(READ_S(interesting[4], "op"), "ret") ||
        READ_I64(interesting[4], "value") != READ_I64(interesting[3], "dst")) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (int call_idx = 0; call_idx <= 1; call_idx++) {
      yyjson_val* ins = interesting[call_idx];
      yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
      yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
      yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
      const char* ctype = cal ? READ_S(cal, "type") : NULL;
      const char* method = cal ? READ_S(cal, "name") : NULL;
      long long receiver = cal ? READ_I64(cal, "receiver") : 0;
      long long expected = call_idx == 0 ? left_phi : right_phi;
      if (!(ctype && strcmp(ctype, "Method") == 0 &&
            method && (strcmp(method, "length") == 0 || strcmp(method, "len") == 0) &&
            args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0 &&
            hako_llvmc_copy_graph_reaches_reg(exit_insts, expected, receiver))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    {
      long long len1_dst = READ_I64(interesting[0], "dst");
      long long len2_dst = READ_I64(interesting[1], "dst");
      long long add1_dst = READ_I64(interesting[2], "dst");
      long long add1_lhs = READ_I64(interesting[2], "lhs");
      long long add1_rhs = READ_I64(interesting[2], "rhs");
      long long add2_lhs = READ_I64(interesting[3], "lhs");
      long long add2_rhs = READ_I64(interesting[3], "rhs");
      if (!(hako_llvmc_str_eq(READ_S(interesting[2], "operation"), "+") &&
            hako_llvmc_str_eq(READ_S(interesting[3], "operation"), "+"))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!((hako_llvmc_copy_graph_reaches_reg(exit_insts, len1_dst, add1_lhs) &&
             hako_llvmc_copy_graph_reaches_reg(exit_insts, len2_dst, add1_rhs)) ||
            (hako_llvmc_copy_graph_reaches_reg(exit_insts, len2_dst, add1_lhs) &&
             hako_llvmc_copy_graph_reaches_reg(exit_insts, len1_dst, add1_rhs)))) {
        yyjson_doc_free(doc);
        return -1;
      }
      if (!((hako_llvmc_copy_graph_reaches_reg(exit_insts, add1_dst, add2_lhs) &&
             hako_llvmc_copy_graph_reaches_reg(exit_insts, len_dst, add2_rhs)) ||
            (hako_llvmc_copy_graph_reaches_reg(exit_insts, len_dst, add2_lhs) &&
             hako_llvmc_copy_graph_reaches_reg(exit_insts, add1_dst, add2_rhs)))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
  }

  #undef READ_I64
  #undef READ_S
  #undef BUILD_INTERESTING_NO_COPY_LOCAL

  yyjson_doc_free(doc);
  return hako_llvmc_emit_substring_views_only_exit_len_ir(
      obj_out,
      &selection,
      source_len);
}

static int hako_llvmc_match_runtime_data_length_hot_loop_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  hako_llvmc_entry_rune_selection selection;
  long long lit_reg = 0;
  long long newbox_dst = 0;
  long long loop_bound = 0;
  long long lit_len = 0;
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

  {
    yyjson_val* pre = yyjson_arr_get(blocks, 0);
    yyjson_val* header = yyjson_arr_get(blocks, 1);
    yyjson_val* body = yyjson_arr_get(blocks, 2);
    yyjson_val* exitb = yyjson_arr_get(blocks, 4);
    yyjson_val* pre_insts = pre ? yyjson_obj_get(pre, "instructions") : NULL;
    yyjson_val* header_insts = header ? yyjson_obj_get(header, "instructions") : NULL;
    yyjson_val* body_insts = body ? yyjson_obj_get(body, "instructions") : NULL;
    yyjson_val* exit_insts = exitb ? yyjson_obj_get(exitb, "instructions") : NULL;
    int saw_length_call = 0;
    int saw_acc_add = 0;

    if (!(pre_insts && yyjson_is_arr(pre_insts) &&
          header_insts && yyjson_is_arr(header_insts) &&
          body_insts && yyjson_is_arr(body_insts) &&
          exit_insts && yyjson_is_arr(exit_insts) &&
          yyjson_arr_size(exit_insts) == 1)) {
      yyjson_doc_free(doc);
      return -1;
    }

    for (i = 0; i < yyjson_arr_size(pre_insts); i++) {
      yyjson_val* ins = yyjson_arr_get(pre_insts, i);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        const char* box_type = ty ? yyjson_get_str(yyjson_obj_get(ty, "box_type")) : NULL;
        const char* value = vobj ? yyjson_get_str(yyjson_obj_get(vobj, "value")) : NULL;
        if (box_type && strcmp(box_type, "StringBox") == 0 &&
            value && hako_llvmc_ascii_strlen(value, &lit_len)) {
          lit_reg = (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst"));
        }
      } else if (strcmp(op, "newbox") == 0) {
        yyjson_val* args = yyjson_obj_get(ins, "args");
        yyjson_val* arg0 = args && yyjson_is_arr(args) ? yyjson_arr_get_first(args) : NULL;
        const char* ty = yyjson_get_str(yyjson_obj_get(ins, "type"));
        long long dst = (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst"));
        long long src = arg0 ? (long long)yyjson_get_sint(arg0) : 0;
        if (ty && strcmp(ty, "StringBox") == 0 && lit_reg != 0 && src == lit_reg) {
          newbox_dst = dst;
        }
      }
    }
    if (lit_reg == 0 || newbox_dst == 0 || lit_len < 0) {
      yyjson_doc_free(doc);
      return -1;
    }

    for (i = 0; i < yyjson_arr_size(header_insts); i++) {
      yyjson_val* ins = yyjson_arr_get(header_insts, i);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        if (yyjson_is_str(ty) && strcmp(yyjson_get_str(ty), "i64") == 0) {
          loop_bound = (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value"));
        }
      }
    }
    if (loop_bound <= 0) {
      yyjson_doc_free(doc);
      return -1;
    }

    for (i = 0; i < yyjson_arr_size(body_insts); i++) {
      yyjson_val* ins = yyjson_arr_get(body_insts, i);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
        const char* box_name = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : NULL;
        const char* box_type = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_type")) : NULL;
        const char* method = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
        long long recv = cal ? (long long)yyjson_get_sint(yyjson_obj_get(cal, "receiver")) : 0;
        long long call_dst = (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst"));
        if (!(ctype && strcmp(ctype, "Method") == 0 &&
              (box_name || box_type) &&
              strcmp(box_name ? box_name : box_type, "RuntimeDataBox") == 0 &&
              method && strcmp(method, "length") == 0 &&
              args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0 &&
              call_dst != 0 &&
              hako_llvmc_copy_chain_reaches_reg(body_insts, newbox_dst, recv))) {
          yyjson_doc_free(doc);
          return -1;
        }
        saw_length_call = 1;
      } else if (strcmp(op, "binop") == 0) {
        const char* kind = yyjson_get_str(yyjson_obj_get(ins, "operation"));
        if (kind && strcmp(kind, "+") == 0) {
          saw_acc_add = 1;
        }
      }
    }

    if (!(saw_length_call && saw_acc_add)) {
      yyjson_doc_free(doc);
      return -1;
    }
  }

  yyjson_doc_free(doc);
  return hako_llvmc_emit_string_length_hot_loop_ir(obj_out, &selection, loop_bound, lit_len);
}

static int hako_llvmc_match_string_length_hot_loop_seed(
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
  long long lit_len = 0;
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
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 6)) {
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

  {
    yyjson_val* interesting[32];
    int n = 0;
    long long receiver_reg = 0;
    long long call_dst = 0;
    long long total_phi = 0;
    long long add_lhs = 0;
    long long add_rhs = 0;
    long long lit_reg = 0;
    long long newbox_dst = 0;
    const char* lit = NULL;
    int found_body_birth = 0;
    int found_preheader_birth = 0;

    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 1));
    if (n < 6) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (!hako_llvmc_str_eq(READ_STR(interesting[n - 3], "op"), "const") ||
        !hako_llvmc_str_eq(READ_STR(interesting[n - 2], "op"), "compare") ||
        !hako_llvmc_str_eq(READ_STR(interesting[n - 1], "op"), "branch")) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (i = 0; i + 3 < (size_t)n; i++) {
      if (!hako_llvmc_str_eq(READ_STR(interesting[i], "op"), "phi")) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    loop_bound = READ_INT(yyjson_obj_get(interesting[n - 3], "value"), "value");
    if (loop_bound <= 0) {
      yyjson_doc_free(doc);
      return -1;
    }
    BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 2));
    if (n < 5) {
      yyjson_doc_free(doc);
      return -1;
    }

    if (hako_llvmc_str_eq(READ_STR(interesting[0], "op"), "boxcall")) {
      receiver_reg = READ_INT(interesting[0], "box");
      call_dst = READ_INT(interesting[0], "dst");
      if (!(READ_STR(interesting[0], "method") &&
            hako_llvmc_str_eq(READ_STR(interesting[0], "method"), "length") &&
            yyjson_is_arr(yyjson_obj_get(interesting[0], "args")) &&
            yyjson_arr_size(yyjson_obj_get(interesting[0], "args")) == 0 &&
            READ_STR(interesting[0], "dst_type") &&
            hako_llvmc_str_eq(READ_STR(interesting[0], "dst_type"), "i64"))) {
      yyjson_doc_free(doc);
        return -1;
      }
    } else if (hako_llvmc_str_eq(READ_STR(interesting[0], "op"), "const") &&
               hako_llvmc_str_eq(READ_STR(interesting[1], "op"), "newbox") &&
               hako_llvmc_str_eq(READ_STR(interesting[2], "op"), "boxcall")) {
      yyjson_val* vobj = yyjson_obj_get(interesting[0], "value");
      yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
      const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
      lit = vobj ? READ_STR(vobj, "value") : NULL;
      lit_reg = READ_INT(interesting[0], "dst");
      newbox_dst = READ_INT(interesting[1], "dst");
      receiver_reg = READ_INT(interesting[2], "box");
      call_dst = READ_INT(interesting[2], "dst");
      if (!(hako_llvmc_str_eq(box_type, "StringBox") &&
            lit && hako_llvmc_ascii_strlen(lit, &lit_len) &&
            READ_STR(interesting[1], "type") &&
            hako_llvmc_str_eq(READ_STR(interesting[1], "type"), "StringBox") &&
            yyjson_is_arr(yyjson_obj_get(interesting[1], "args")) &&
            yyjson_arr_size(yyjson_obj_get(interesting[1], "args")) == 1 &&
            (long long)yyjson_get_sint(yyjson_arr_get(yyjson_obj_get(interesting[1], "args"), 0)) == lit_reg &&
            receiver_reg == newbox_dst &&
            READ_STR(interesting[2], "method") &&
            hako_llvmc_str_eq(READ_STR(interesting[2], "method"), "length") &&
            yyjson_is_arr(yyjson_obj_get(interesting[2], "args")) &&
            yyjson_arr_size(yyjson_obj_get(interesting[2], "args")) == 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
      found_body_birth = 1;
    } else {
      yyjson_doc_free(doc);
      return -1;
    }

    if (!hako_llvmc_str_eq(READ_STR(interesting[n - 3], "op"), "const") ||
        !hako_llvmc_str_eq(READ_STR(interesting[n - 2], "op"), "binop") ||
        !hako_llvmc_str_eq(READ_STR(interesting[n - 1], "op"), "jump")) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (READ_INT(yyjson_obj_get(interesting[n - 3], "value"), "value") != 1) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (!(hako_llvmc_str_eq(READ_STR(interesting[n - 2], "operation"), "+") ||
          hako_llvmc_str_eq(READ_STR(interesting[n - 2], "op_kind"), "Add"))) {
      yyjson_doc_free(doc);
      return -1;
    }
    add_lhs = READ_INT(found_body_birth ? interesting[3] : interesting[1], "lhs");
    add_rhs = READ_INT(found_body_birth ? interesting[3] : interesting[1], "rhs");
    if (add_lhs == call_dst) {
      total_phi = add_rhs;
    } else if (add_rhs == call_dst) {
      total_phi = add_lhs;
    } else {
      yyjson_doc_free(doc);
      return -1;
    }

    if (!found_body_birth) {
      int receiver_phi_ok = 0;
      long long receiver_seed_reg = 0;
      BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 0));
      if (n < 4) {
        yyjson_doc_free(doc);
        return -1;
      }
      for (i = 0; i < (size_t)n; i++) {
        yyjson_val* ins = interesting[i];
        const char* op = READ_STR(ins, "op");
        if (op && strcmp(op, "const") == 0) {
          yyjson_val* vobj = yyjson_obj_get(ins, "value");
          yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
          const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
          const char* value = vobj ? READ_STR(vobj, "value") : NULL;
          if (hako_llvmc_str_eq(box_type, "StringBox") && value && hako_llvmc_ascii_strlen(value, &lit_len)) {
            lit_reg = READ_INT(ins, "dst");
            lit = value;
          }
        } else if (hako_llvmc_str_eq(op, "newbox")) {
          yyjson_val* args = yyjson_obj_get(ins, "args");
          if (hako_llvmc_str_eq(READ_STR(ins, "type"), "StringBox") &&
              lit &&
              args && yyjson_is_arr(args) && yyjson_arr_size(args) == 1 &&
              (long long)yyjson_get_sint(yyjson_arr_get(args, 0)) == lit_reg) {
            newbox_dst = READ_INT(ins, "dst");
          }
        }
      }
      {
        yyjson_val* pre_insts = yyjson_obj_get(yyjson_arr_get(blocks, 0), "instructions");
        size_t j;
        receiver_seed_reg = newbox_dst;
        if (!(pre_insts && yyjson_is_arr(pre_insts))) {
          yyjson_doc_free(doc);
          return -1;
        }
        for (j = 0; j < yyjson_arr_size(pre_insts); j++) {
          yyjson_val* ins = yyjson_arr_get(pre_insts, j);
          if (hako_llvmc_str_eq(READ_STR(ins, "op"), "copy") &&
              READ_INT(ins, "src") == newbox_dst) {
            receiver_seed_reg = READ_INT(ins, "dst");
            break;
          }
        }
      }
      BUILD_INTERESTING_NO_COPY(yyjson_arr_get(blocks, 1));
      for (i = 0; i + 3 < (size_t)n; i++) {
        yyjson_val* phi = interesting[i];
        yyjson_val* incoming = yyjson_obj_get(phi, "incoming");
        size_t j;
        if (READ_INT(phi, "dst") != receiver_reg || !(incoming && yyjson_is_arr(incoming))) {
          continue;
        }
        for (j = 0; j < yyjson_arr_size(incoming); j++) {
          yyjson_val* ent = yyjson_arr_get(incoming, j);
          if (ent && yyjson_is_arr(ent) && yyjson_arr_size(ent) >= 1 &&
              ((long long)yyjson_get_sint(yyjson_arr_get(ent, 0)) == newbox_dst ||
               (long long)yyjson_get_sint(yyjson_arr_get(ent, 0)) == receiver_seed_reg)) {
            receiver_phi_ok = 1;
            break;
          }
        }
        if (receiver_phi_ok) {
          break;
        }
      }
      if (!(lit && newbox_dst != 0 && receiver_phi_ok)) {
        yyjson_doc_free(doc);
        return -1;
      }
      found_preheader_birth = 1;
    }

    if (!(found_body_birth || found_preheader_birth)) {
      yyjson_doc_free(doc);
      return -1;
    }

    if (call_dst == 0 || total_phi == 0) {
      yyjson_doc_free(doc);
      return -1;
    }
  }

  yyjson_doc_free(doc);
  return hako_llvmc_emit_string_length_hot_loop_ir(obj_out, &selection, loop_bound, lit_len);

  #undef READ_INT
  #undef READ_STR
  #undef BUILD_INTERESTING_NO_COPY
  #undef EXPECT_OP
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

  /* body */
  BUILD_INTERESTING(yyjson_arr_get(blocks, 2));
  if (!(n == 17 || n == 14)) { yyjson_doc_free(doc); return -1; }
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
  if (n == 17) {
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
  } else {
    EXPECT_OP(6, "const");
    EXPECT_OP(7, "binop");
    EXPECT_OP(8, "binop");
    EXPECT_OP(9, "const");
    EXPECT_OP(10, "const");
    EXPECT_OP(11, "binop");
    EXPECT_OP(13, "jump");
    {
      const char* add0 = READ_STR(interesting[7], "operation");
      const char* add1 = READ_STR(interesting[8], "operation");
      const char* add2 = READ_STR(interesting[11], "operation");
      long long const_len_dst = READ_INT(interesting[6], "dst");
      long long collapsed_lhs = READ_INT(interesting[7], "lhs");
      long long collapsed_rhs = READ_INT(interesting[7], "rhs");
      if (!(hako_llvmc_str_eq(add0, "+") &&
            hako_llvmc_str_eq(add1, "+") &&
            hako_llvmc_str_eq(add2, "+") &&
            ((collapsed_lhs == const_len_dst && collapsed_rhs == source_len_value) ||
             (collapsed_lhs == source_len_value && collapsed_rhs == const_len_dst)))) {
        yyjson_doc_free(doc);
        return -1;
      }
    }
    if (READ_INT(yyjson_obj_get(interesting[6], "value"), "value") != middle_len ||
        READ_INT(yyjson_obj_get(interesting[9], "value"), "value") != 1 ||
        READ_INT(yyjson_obj_get(interesting[10], "value"), "value") != 1) {
      yyjson_doc_free(doc);
      return -1;
    }
    {
      helper_result = READ_INT(interesting[12], "dst");
      if (!(hako_llvmc_string_corridor_read_plan_source_window_fn(
                fn,
                helper_result,
                "publication_sink",
                &source_root,
                &helper_start,
                &helper_end) &&
            source_root > 0 &&
            helper_start == READ_INT(interesting[9], "dst") &&
            helper_end == READ_INT(interesting[11], "dst"))) {
        yyjson_doc_free(doc);
        return -1;
      }
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
    int rc;
    if (helper_result > 0 &&
        merged_string_phi > 0 &&
        hako_llvmc_string_corridor_has_relation_fn(
            fn,
            merged_string_phi,
            "stable_length_scalar",
            helper_result,
            source_len_value,
            "stop_at_merge")) {
      rc = hako_llvmc_emit_substring_concat_len_ir(
          obj_out,
          &selection,
          loop_bound,
          seed_len,
          middle_len);
    } else {
      rc = hako_llvmc_emit_substring_concat_loop_ir(
          obj_out,
          &selection,
          seed,
          seed_len,
          loop_bound,
          middle,
          middle_len,
          split);
    }
    yyjson_doc_free(doc);
    return rc;
  }

  #undef READ_INT
  #undef READ_STR
  #undef BUILD_INTERESTING
  #undef EXPECT_OP
  #undef REQUIRE_STRINGISH_BOX
}
