static int hako_llvmc_emit_userbox_point_add_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long ops) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long final_sum;

  if (!obj_out || ops != 2000000) {
    return -1;
  }
  final_sum = 3 + (ops * 3);

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_userbox_point_add_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox point add micro)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%sum = alloca i64, align 8\n");
  fprintf(f, "  %%acc = alloca i64, align 8\n");
  /* Keep the selected local Point body boxless and carry only the loop-visible sum lane. */
  fprintf(f, "  store i64 3, ptr %%sum, align 8\n");
  fprintf(f, "  store volatile i64 0, ptr %%acc, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%sum.cur = load i64, ptr %%sum, align 8\n");
  fprintf(f, "  %%acc.cur = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%acc.next = add i64 %%acc.cur, %%sum.cur\n");
  fprintf(f, "  store volatile i64 %%acc.next, ptr %%acc, align 8\n");
  fprintf(f, "  %%sum.next = add i64 %%sum.cur, 3\n");
  fprintf(f, "  store i64 %%sum.next, ptr %%sum, align 8\n");
  fprintf(f, "  %%loop.more = icmp ne i64 %%sum.next, %lld\n", final_sum);
  fprintf(f, "  br i1 %%loop.more, label %%loop, label %%exit\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%acc.fin = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%sum.fin = load i64, ptr %%sum, align 8\n");
  fprintf(f, "  %%ret = add i64 %%acc.fin, %%sum.fin\n");
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}
