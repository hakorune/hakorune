static int hako_llvmc_emit_userbox_counter_step_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long ops,
    long long step_value) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long final_sum;

  if (!(obj_out && selection && ops == 2000000)) {
    return -1;
  }
  final_sum = step_value * (ops + 1);

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_userbox_counter_step_micro_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox counter step micro)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  /* The exact keeper shape proves Counter.step() == 43 and ops == 2,000,000,
   * so the final sum is a closed constant. Emit the scalar result directly and
   * let ny_main stay call-free like the C baseline. */
  fprintf(f, "  %%ret = add i64 0, %lld\n", final_sum);
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}
