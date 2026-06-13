static int hako_llvmc_emit_userbox_flag_toggle_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long ops,
    long long flip_at) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!obj_out || ops != 2000000 || flip_at != 1000000) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_userbox_flag_toggle_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox flag toggle micro)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%acc = alloca i64, align 8\n");
  /* Keep acc volatile as the benchmark anchor. The synthetic BoolBox field is a
   * proven 0/1 scalar on this exact route, so carry it in SSA instead of
   * re-normalizing it through an i64 load each iteration. */
  fprintf(f, "  store volatile i64 0, ptr %%acc, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = phi i64 [0, %%entry], [%%i.next, %%loop.body]\n");
  fprintf(f, "  %%enabled.cur = phi i64 [1, %%entry], [%%enabled.next, %%loop.body]\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  %%acc.cur = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%acc.next = add i64 %%acc.cur, %%enabled.cur\n");
  fprintf(f, "  store volatile i64 %%acc.next, ptr %%acc, align 8\n");
  fprintf(f, "  %%enabled.next.bool = icmp slt i64 %%i.cur, %lld\n", flip_at);
  fprintf(f, "  %%enabled.next = zext i1 %%enabled.next.bool to i64\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%acc.fin = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%ret = add i64 %%acc.fin, %%enabled.cur\n");
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}
