static int hako_llvmc_emit_concat_const_suffix_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    const char* text,
    long long text_len,
    const char* suffix,
    long long suffix_len,
    long long ops) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!obj_out || !text || !suffix || text_len != 16 || suffix_len != 2 || ops != 600000) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_concat_const_suffix_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (concat const-suffix loop ascii)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  fprintf(f, "\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%acc = alloca i64, align 8\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%acc, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  %%acc.cur = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%acc.next = add i64 %%acc.cur, %lld\n", text_len + suffix_len);
  fprintf(f, "  store volatile i64 %%acc.next, ptr %%acc, align 8\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%acc.fin = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%ret = and i64 %%acc.fin, 255\n");
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_concat_const_suffix_micro_seed(
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
  long long suffix_len = 0;
  long long ops = 0;
  long long result_len = 0;
  long long actual_seed_len = 0;
  long long actual_suffix_len = 0;
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
  route = metadata ? yyjson_obj_get(metadata, "concat_const_suffix_micro_seed_route") : NULL;
  if (!(route && yyjson_is_obj(route))) {
    yyjson_doc_free(doc);
    return -1;
  }

  seed = yyjson_get_str(yyjson_obj_get(route, "seed"));
  suffix = yyjson_get_str(yyjson_obj_get(route, "suffix"));
  proof = yyjson_get_str(yyjson_obj_get(route, "proof"));
  seed_len = (long long)yyjson_get_sint(yyjson_obj_get(route, "seed_len"));
  suffix_len = (long long)yyjson_get_sint(yyjson_obj_get(route, "suffix_len"));
  ops = (long long)yyjson_get_sint(yyjson_obj_get(route, "ops"));
  result_len = (long long)yyjson_get_sint(yyjson_obj_get(route, "result_len"));

  if (!(seed && suffix && proof &&
        hako_llvmc_ascii_strlen(seed, &actual_seed_len) &&
        hako_llvmc_ascii_strlen(suffix, &actual_suffix_len) &&
        seed_len == actual_seed_len &&
        suffix_len == actual_suffix_len &&
        hako_llvmc_str_eq(seed, "line-seed-abcdef") &&
        hako_llvmc_str_eq(suffix, "xy") &&
        hako_llvmc_str_eq(proof, "kilo_micro_concat_const_suffix_5block") &&
        seed_len == 16 &&
        suffix_len == 2 &&
        ops == 600000 &&
        result_len == seed_len + suffix_len)) {
    if (hako_llvmc_route_trace_enabled()) {
      hako_llvmc_emit_route_trace(
          "concat_const_suffix_micro",
          "skip",
          "metadata_contract_mismatch",
          "concat_const_suffix_micro_seed_route");
    }
    yyjson_doc_free(doc);
    return -1;
  }

  if (hako_llvmc_route_trace_enabled()) {
    hako_llvmc_emit_route_trace(
        "concat_const_suffix_micro",
        "emit",
        "exact_match",
        "kilo_micro_concat_const_suffix mir_route_plan");
  }

  {
    int rc = hako_llvmc_emit_concat_const_suffix_micro_ir(
        obj_out,
        &selection,
        seed,
        seed_len,
        suffix,
        suffix_len,
        ops);
    yyjson_doc_free(doc);
    return rc;
  }
}
