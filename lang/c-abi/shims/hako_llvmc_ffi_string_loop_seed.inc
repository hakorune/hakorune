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

  if (!seed || !middle || seed_len <= 1 || middle_len <= 0 || split <= 0) {
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
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%cond = icmp slt i64 %%i.cur, %lld\n", loop_bound);
  fprintf(f, "  br i1 %%cond, label %%body, label %%exit\n\n");

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
    char cmd[2048];
    hako_llvmc_maybe_dump_ir_file(llpath);
    snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
    rc = system(cmd);
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
    char cmd[2048];
    hako_llvmc_maybe_dump_ir_file(llpath);
    snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
    rc = system(cmd);
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
    if (strcmp(READ_STR(interesting[n - 3], "op"), "const") != 0 ||
        strcmp(READ_STR(interesting[n - 2], "op"), "compare") != 0 ||
        strcmp(READ_STR(interesting[n - 1], "op"), "branch") != 0) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (i = 0; i + 3 < (size_t)n; i++) {
      if (strcmp(READ_STR(interesting[i], "op"), "phi") != 0) {
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

    if (strcmp(READ_STR(interesting[0], "op"), "boxcall") == 0) {
      receiver_reg = READ_INT(interesting[0], "box");
      call_dst = READ_INT(interesting[0], "dst");
      if (!(READ_STR(interesting[0], "method") &&
            strcmp(READ_STR(interesting[0], "method"), "length") == 0 &&
            yyjson_is_arr(yyjson_obj_get(interesting[0], "args")) &&
            yyjson_arr_size(yyjson_obj_get(interesting[0], "args")) == 0 &&
            READ_STR(interesting[0], "dst_type") &&
            strcmp(READ_STR(interesting[0], "dst_type"), "i64") == 0)) {
        yyjson_doc_free(doc);
        return -1;
      }
    } else if (strcmp(READ_STR(interesting[0], "op"), "const") == 0 &&
               strcmp(READ_STR(interesting[1], "op"), "newbox") == 0 &&
               strcmp(READ_STR(interesting[2], "op"), "boxcall") == 0) {
      yyjson_val* vobj = yyjson_obj_get(interesting[0], "value");
      yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
      const char* box_type = ty ? READ_STR(ty, "box_type") : NULL;
      lit = vobj ? READ_STR(vobj, "value") : NULL;
      lit_reg = READ_INT(interesting[0], "dst");
      newbox_dst = READ_INT(interesting[1], "dst");
      receiver_reg = READ_INT(interesting[2], "box");
      call_dst = READ_INT(interesting[2], "dst");
      if (!(box_type && strcmp(box_type, "StringBox") == 0 &&
            lit && hako_llvmc_ascii_strlen(lit, &lit_len) &&
            READ_STR(interesting[1], "type") &&
            strcmp(READ_STR(interesting[1], "type"), "StringBox") == 0 &&
            yyjson_is_arr(yyjson_obj_get(interesting[1], "args")) &&
            yyjson_arr_size(yyjson_obj_get(interesting[1], "args")) == 1 &&
            (long long)yyjson_get_sint(yyjson_arr_get(yyjson_obj_get(interesting[1], "args"), 0)) == lit_reg &&
            receiver_reg == newbox_dst &&
            READ_STR(interesting[2], "method") &&
            strcmp(READ_STR(interesting[2], "method"), "length") == 0 &&
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

    if (strcmp(READ_STR(interesting[n - 3], "op"), "const") != 0 ||
        strcmp(READ_STR(interesting[n - 2], "op"), "binop") != 0 ||
        strcmp(READ_STR(interesting[n - 1], "op"), "jump") != 0) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (READ_INT(yyjson_obj_get(interesting[n - 3], "value"), "value") != 1) {
      yyjson_doc_free(doc);
      return -1;
    }
    if (!(strcmp(READ_STR(interesting[n - 2], "operation"), "+") == 0 ||
          strcmp(READ_STR(interesting[n - 2], "op_kind"), "Add") == 0)) {
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
          if (box_type && strcmp(box_type, "StringBox") == 0 && value && hako_llvmc_ascii_strlen(value, &lit_len)) {
            lit_reg = READ_INT(ins, "dst");
            lit = value;
          }
        } else if (op && strcmp(op, "newbox") == 0) {
          yyjson_val* args = yyjson_obj_get(ins, "args");
          if (READ_STR(ins, "type") &&
              strcmp(READ_STR(ins, "type"), "StringBox") == 0 &&
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
          if (READ_STR(ins, "op") && strcmp(READ_STR(ins, "op"), "copy") == 0 &&
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
  EXPECT_OP(2, "mir_call");
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
  EXPECT_OP(3, "const");
  EXPECT_OP(4, "compare");
  EXPECT_OP(5, "branch");
  loop_bound = READ_INT(yyjson_obj_get(interesting[3], "value"), "value");
  if (loop_bound <= 0) { yyjson_doc_free(doc); return -1; }
  {
    const char* cmp_op = READ_STR(interesting[4], "operation");
    if (!hako_llvmc_str_eq(cmp_op, "<")) { yyjson_doc_free(doc); return -1; }
  }

  /* body */
  BUILD_INTERESTING(yyjson_arr_get(blocks, 2));
  if (n != 15) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "const");
  EXPECT_OP(1, "const");
  EXPECT_OP(2, "binop");
  EXPECT_OP(3, "mir_call");
  EXPECT_OP(4, "mir_call");
  EXPECT_OP(5, "const");
  EXPECT_OP(6, "binop");
  EXPECT_OP(7, "binop");
  EXPECT_OP(8, "mir_call");
  EXPECT_OP(9, "binop");
  EXPECT_OP(10, "const");
  EXPECT_OP(11, "const");
  EXPECT_OP(12, "binop");
  EXPECT_OP(13, "mir_call");
  EXPECT_OP(14, "jump");
  divisor = READ_INT(yyjson_obj_get(interesting[1], "value"), "value");
  {
    const char* div_op = READ_STR(interesting[2], "operation");
    if (READ_INT(yyjson_obj_get(interesting[0], "value"), "value") != 0 || divisor <= 0 ||
        !hako_llvmc_str_eq(div_op, "/")) {
      yyjson_doc_free(doc);
      return -1;
    }
  }
  {
    yyjson_val* mc = yyjson_obj_get(interesting[3], "mir_call");
    yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
    if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) == 2 &&
          READ_INT(interesting[0], "value") == 0 &&
          (long long)yyjson_get_sint(yyjson_arr_get(args, 0)) == READ_INT(interesting[0], "dst"))) {
      yyjson_doc_free(doc);
      return -1;
    }
  }
  {
    int k;
    for (k = 3; k <= 4; k++) {
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
  middle = READ_STR(yyjson_obj_get(interesting[5], "value"), "value");
  if (!middle || !hako_llvmc_ascii_strlen(middle, &middle_len)) { yyjson_doc_free(doc); return -1; }
  {
    const char* add0 = READ_STR(interesting[6], "operation");
    const char* add1 = READ_STR(interesting[7], "operation");
    const char* add2 = READ_STR(interesting[9], "operation");
    const char* add3 = READ_STR(interesting[12], "operation");
    if (!(hako_llvmc_str_eq(add0, "+") &&
          hako_llvmc_str_eq(add1, "+") &&
          hako_llvmc_str_eq(add2, "+") &&
          hako_llvmc_str_eq(add3, "+"))) {
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
  }
  if (READ_INT(yyjson_obj_get(interesting[10], "value"), "value") != 1 ||
      READ_INT(yyjson_obj_get(interesting[11], "value"), "value") != 1) {
    yyjson_doc_free(doc);
    return -1;
  }
  {
    yyjson_val* mc = yyjson_obj_get(interesting[13], "mir_call");
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

  /* latch */
  BUILD_INTERESTING(yyjson_arr_get(blocks, 3));
  if (n != 5) { yyjson_doc_free(doc); return -1; }
  EXPECT_OP(0, "phi");
  EXPECT_OP(1, "phi");
  EXPECT_OP(2, "const");
  EXPECT_OP(3, "binop");
  EXPECT_OP(4, "jump");
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

