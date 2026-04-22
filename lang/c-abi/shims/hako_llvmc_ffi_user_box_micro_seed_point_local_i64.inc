static int hako_llvmc_emit_userbox_point_local_i64_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!(obj_out && selection)) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_userbox_point_local_i64_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox point local i64 seed)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%x = alloca i64, align 8\n");
  fprintf(f, "  %%y = alloca i64, align 8\n");
  fprintf(f, "  store i64 41, ptr %%x, align 8\n");
  fprintf(f, "  store i64 2, ptr %%y, align 8\n");
  fprintf(f, "  %%xv = load i64, ptr %%x, align 8\n");
  fprintf(f, "  %%yv = load i64, ptr %%y, align 8\n");
  fprintf(f, "  %%sum = add i64 %%xv, %%yv\n");
  fprintf(f, "  ret i64 %%sum\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}
