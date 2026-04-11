static int hako_llvmc_emit_variant_tag_local_i64_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long tag_value) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!(obj_out && selection)) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_variant_tag_local_i64_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (sum local i64 tag seed)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  hako_llvmc_emit_entry_header(f, selection);
  fprintf(
      f,
      "entry:\n"
      "  ret i64 %lld\n",
      tag_value);
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_emit_variant_project_local_i64_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long payload_value) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!(obj_out && selection)) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_variant_project_local_i64_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (sum local i64 project seed)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  hako_llvmc_emit_entry_header(f, selection);
  fprintf(
      f,
      "entry:\n"
      "  ret i64 %lld\n",
      payload_value);
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static void hako_llvmc_emit_entry_header_typed(
    FILE* f,
    const hako_llvmc_entry_rune_selection* selection,
    const char* result_type) {
  const char* symbol = hako_llvmc_effective_entry_symbol(selection);
  const char* callconv_prefix = hako_llvmc_entry_callconv_prefix(selection);
  fprintf(f, "define %s%s @\"%s\"() {\n", callconv_prefix, result_type, symbol);
}

static void hako_llvmc_emit_entry_alias_if_needed_typed(
    FILE* f,
    const hako_llvmc_entry_rune_selection* selection,
    const char* result_type) {
  const char* symbol = hako_llvmc_effective_entry_symbol(selection);
  if (strcmp(symbol, hako_llvmc_entry_symbol_default()) == 0) {
    return;
  }
  fprintf(f, "\ndefine %s @ny_main() {\n", result_type);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%entry_alias = call %s @\"%s\"()\n", result_type, symbol);
  fprintf(f, "  ret %s %%entry_alias\n", result_type);
  fprintf(f, "}\n");
}

static int hako_llvmc_emit_variant_project_local_f64_ir(
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
      "%s/hako_pure_variant_project_local_f64_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (sum local f64 project seed)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  hako_llvmc_emit_entry_header_typed(f, selection, "double");
  fprintf(
      f,
      "entry:\n"
      "  ret double %.17g\n",
      payload_value);
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed_typed(f, selection, "double");

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_emit_variant_project_local_handle_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    const char* string_literal) {
  char llpath[1024];
  FILE* f;
  size_t slen;
  int rc;

  if (!(obj_out && selection && string_literal)) {
    return -1;
  }

  slen = strlen(string_literal);
  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_variant_project_local_handle_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (sum local handle project seed)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  fprintf(
      f,
      "@.str_proj_handle_%d = private unnamed_addr constant [%zu x i8] c\"%s\\00\"\n",
      (int)getpid(),
      slen + 1,
      string_literal);
  fprintf(f, "declare i64 @\"nyash.box.from_i8_string_const\"(ptr)\n\n");
  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(
      f,
      "  %%payload = call i64 @\"nyash.box.from_i8_string_const\"(ptr getelementptr inbounds ([%zu x i8], ptr @.str_proj_handle_%d, i64 0, i64 0))\n",
      slen + 1,
      (int)getpid());
  fprintf(f, "  ret i64 %%payload\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}
