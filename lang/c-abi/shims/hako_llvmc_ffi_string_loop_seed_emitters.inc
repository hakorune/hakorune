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
