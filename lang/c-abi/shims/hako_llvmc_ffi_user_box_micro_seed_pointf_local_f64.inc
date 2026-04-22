static int hako_llvmc_emit_userbox_pointf_local_f64_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    double payload_value) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!(obj_out && selection)) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_userbox_pointf_local_f64_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox pointf local f64 seed)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_userbox_emit_entry_header_typed(f, selection, "double");
  fprintf(
      f,
      "entry:\n"
      "  ret double %.17g\n",
      payload_value);
  fprintf(f, "}\n");
  hako_llvmc_userbox_emit_entry_alias_if_needed_typed(f, selection, "double");

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}
