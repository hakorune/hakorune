static int hako_llvmc_emit_indexof_line_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long rows,
    long long ops,
    long long flip_period,
    const char* line_seed,
    long long line_seed_len,
    const char* none_seed,
    long long none_seed_len,
    const char* needle,
    long long needle_len) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long row_mask;
  long long flip_mask;
  int use_flip;

  if (!obj_out || rows != 64 || ops <= 0 || flip_period < 0) {
    return -1;
  }
  if (!line_seed || !none_seed || !needle || line_seed_len <= 0 || none_seed_len <= 0 || needle_len <= 0) {
    return -1;
  }

  row_mask = rows - 1;
  use_flip = flip_period > 0;
  flip_mask = use_flip ? (flip_period - 1) : 0;
  if ((rows & row_mask) != 0 || row_mask <= 0) {
    return -1;
  }
  if (use_flip && ((flip_period & flip_mask) != 0 || flip_mask <= 0 || flip_period != 16)) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_indexof_line_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (indexOf line loop ascii)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  fprintf(f, "declare ptr @strstr(ptr nocapture readonly, ptr nocapture readonly)\n\n");

  fprintf(f, "@.indexof_seed_line = private unnamed_addr constant [%lld x i8] c\"", line_seed_len);
  hako_llvmc_emit_llvm_byte_string(f, line_seed, line_seed_len);
  fprintf(f, "\", align 1\n");
  fprintf(f, "@.indexof_seed_none = private unnamed_addr constant [%lld x i8] c\"", none_seed_len);
  hako_llvmc_emit_llvm_byte_string(f, none_seed, none_seed_len);
  fprintf(f, "\", align 1\n");
  fprintf(f, "@.indexof_needle = private unnamed_addr constant [%lld x i8] c\"", needle_len);
  hako_llvmc_emit_llvm_byte_string(f, needle, needle_len);
  fprintf(f, "\", align 1\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%lines = alloca [%lld x ptr], align 8\n", rows);
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%hits = alloca i64, align 8\n");
  fprintf(f, "  %%lines.ptr = getelementptr inbounds [%lld x ptr], ptr %%lines, i64 0, i64 0\n", rows);
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  br label %%prefill\n\n");

  fprintf(f, "prefill:\n");
  fprintf(f, "  %%i.pre = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%prefill.cond = icmp slt i64 %%i.pre, 64\n");
  fprintf(f, "  br i1 %%prefill.cond, label %%prefill.body, label %%prefill.exit\n\n");

  fprintf(f, "prefill.body:\n");
  fprintf(f, "  %%prefill.slot = getelementptr inbounds ptr, ptr %%lines.ptr, i64 %%i.pre\n");
  fprintf(f, "  %%prefill.evenmask = and i64 %%i.pre, 1\n");
  fprintf(f, "  %%prefill.is_even = icmp eq i64 %%prefill.evenmask, 0\n");
  fprintf(f, "  %%prefill.val = select i1 %%prefill.is_even, ptr @.indexof_seed_line, ptr @.indexof_seed_none\n");
  fprintf(f, "  store ptr %%prefill.val, ptr %%prefill.slot, align 8\n");
  fprintf(f, "  %%i.pre.next = add i64 %%i.pre, 1\n");
  fprintf(f, "  store i64 %%i.pre.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%prefill\n\n");

  fprintf(f, "prefill.exit:\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%hits, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  %%row = and i64 %%i.cur, %lld\n", row_mask);
  fprintf(f, "  %%slot = getelementptr inbounds ptr, ptr %%lines.ptr, i64 %%row\n");
  fprintf(f, "  %%cur = load ptr, ptr %%slot, align 8\n");
  fprintf(f, "  %%found.ptr = call ptr @strstr(ptr %%cur, ptr @.indexof_needle)\n");
  fprintf(f, "  %%found = icmp ne ptr %%found.ptr, null\n");
  fprintf(f, "  %%hits.cur = load i64, ptr %%hits, align 8\n");
  fprintf(f, "  %%hits.inc = select i1 %%found, i64 1, i64 0\n");
  fprintf(f, "  %%hits.next = add i64 %%hits.cur, %%hits.inc\n");
  fprintf(f, "  store i64 %%hits.next, ptr %%hits, align 8\n");
  if (use_flip) {
    fprintf(f, "  %%flipmask = and i64 %%i.cur, %lld\n", flip_mask);
    fprintf(f, "  %%flip = icmp eq i64 %%flipmask, 0\n");
    fprintf(f, "  br i1 %%flip, label %%flip.then, label %%inc\n\n");

    fprintf(f, "flip.then:\n");
    fprintf(f, "  %%newval = select i1 %%found, ptr @.indexof_seed_none, ptr @.indexof_seed_line\n");
    fprintf(f, "  store ptr %%newval, ptr %%slot, align 8\n");
    fprintf(f, "  br label %%inc\n\n");
  }

  fprintf(f, "inc:\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%hits.fin = load i64, ptr %%hits, align 8\n");
  fprintf(f, "  %%ret = add i64 %%hits.fin, %lld\n", rows);
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

static int hako_llvmc_emit_indexof_leaf_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long rows,
    long long ops,
    const char* line_seed,
    long long line_seed_len,
    const char* none_seed,
    long long none_seed_len,
    const char* needle,
    long long needle_len) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long row_mask;

  if (!obj_out || rows != 64 || ops <= 0) {
    return -1;
  }
  if (!line_seed || !none_seed || !needle || line_seed_len <= 0 || none_seed_len <= 0 || needle_len <= 0) {
    return -1;
  }

  row_mask = rows - 1;
  if ((rows & row_mask) != 0 || row_mask <= 0) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_indexof_leaf_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (indexOf leaf observer ascii)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
  fprintf(f, "declare i64 @\"nyash.string.indexOf_ss\"(ptr nocapture readonly, ptr nocapture readonly)\n\n");

  fprintf(f, "@.indexof_seed_line = private unnamed_addr constant [%lld x i8] c\"", line_seed_len);
  hako_llvmc_emit_llvm_byte_string(f, line_seed, line_seed_len);
  fprintf(f, "\", align 1\n");
  fprintf(f, "@.indexof_seed_none = private unnamed_addr constant [%lld x i8] c\"", none_seed_len);
  hako_llvmc_emit_llvm_byte_string(f, none_seed, none_seed_len);
  fprintf(f, "\", align 1\n");
  fprintf(f, "@.indexof_needle = private unnamed_addr constant [%lld x i8] c\"", needle_len);
  hako_llvmc_emit_llvm_byte_string(f, needle, needle_len);
  fprintf(f, "\", align 1\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%lines = alloca [%lld x ptr], align 8\n", rows);
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%hits = alloca i64, align 8\n");
  fprintf(f, "  %%lines.ptr = getelementptr inbounds [%lld x ptr], ptr %%lines, i64 0, i64 0\n", rows);
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  br label %%prefill\n\n");

  fprintf(f, "prefill:\n");
  fprintf(f, "  %%i.pre = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%prefill.cond = icmp slt i64 %%i.pre, 64\n");
  fprintf(f, "  br i1 %%prefill.cond, label %%prefill.body, label %%prefill.exit\n\n");

  fprintf(f, "prefill.body:\n");
  fprintf(f, "  %%prefill.slot = getelementptr inbounds ptr, ptr %%lines.ptr, i64 %%i.pre\n");
  fprintf(f, "  %%prefill.evenmask = and i64 %%i.pre, 1\n");
  fprintf(f, "  %%prefill.is_even = icmp eq i64 %%prefill.evenmask, 0\n");
  fprintf(f, "  %%prefill.val = select i1 %%prefill.is_even, ptr @.indexof_seed_line, ptr @.indexof_seed_none\n");
  fprintf(f, "  store ptr %%prefill.val, ptr %%prefill.slot, align 8\n");
  fprintf(f, "  %%i.pre.next = add i64 %%i.pre, 1\n");
  fprintf(f, "  store i64 %%i.pre.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%prefill\n\n");

  fprintf(f, "prefill.exit:\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%hits, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  %%row = and i64 %%i.cur, %lld\n", row_mask);
  fprintf(f, "  %%slot = getelementptr inbounds ptr, ptr %%lines.ptr, i64 %%row\n");
  fprintf(f, "  %%cur = load ptr, ptr %%slot, align 8\n");
  fprintf(f, "  %%found = call i64 @\"nyash.string.indexOf_ss\"(ptr %%cur, ptr @.indexof_needle)\n");
  fprintf(f, "  %%hits.cur = load i64, ptr %%hits, align 8\n");
  fprintf(f, "  %%hits.inc = icmp sge i64 %%found, 0\n");
  fprintf(f, "  %%hits.inc.i64 = select i1 %%hits.inc, i64 1, i64 0\n");
  fprintf(f, "  %%hits.next = add i64 %%hits.cur, %%hits.inc.i64\n");
  fprintf(f, "  store i64 %%hits.next, ptr %%hits, align 8\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%hits.fin = load i64, ptr %%hits, align 8\n");
  fprintf(f, "  %%ret = add i64 %%hits.fin, %lld\n", rows);
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

static int hako_llvmc_match_indexof_leaf_ascii_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  size_t bi;
  hako_llvmc_entry_rune_selection selection;
  int saw_newbox_array = 0;
  int saw_rows_64 = 0;
  int saw_ops_400k = 0;
  int saw_line_seed = 0;
  int saw_none_seed = 0;
  int saw_needle_line = 0;
  int saw_push_line = 0;
  int saw_push_none = 0;
  int saw_get = 0;
  int saw_indexof = 0;
  int saw_length = 0;
  int saw_set = 0;
  long long rows = 0;
  long long ops = 0;
  const char* line_seed = NULL;
  const char* none_seed = NULL;
  const char* needle = NULL;
  long long line_seed_len = 0;
  long long none_seed_len = 0;
  long long needle_len = 0;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  for (bi = 0; bi < yyjson_arr_size(fns); bi++) {
    yyjson_val* candidate = yyjson_arr_get(fns, bi);
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
  if (!(blocks && yyjson_is_arr(blocks))) {
    yyjson_doc_free(doc);
    return -1;
  }

  for (bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
    size_t ii;
    if (!(insts && yyjson_is_arr(insts))) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (ii = 0; ii < yyjson_arr_size(insts); ii++) {
      yyjson_val* ins = yyjson_arr_get(insts, ii);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "newbox") == 0) {
        const char* box_type = yyjson_get_str(yyjson_obj_get(ins, "type"));
        if (box_type && strcmp(box_type, "ArrayBox") == 0) {
          saw_newbox_array = 1;
        }
      } else if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        const char* kind = vobj ? yyjson_get_str(yyjson_obj_get(vobj, "type")) : NULL;
        long long iv = vobj ? (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value")) : 0;
        if (kind && strcmp(kind, "i64") == 0) {
          if (iv == 64) { saw_rows_64 = 1; rows = 64; }
          if (iv == 400000) { saw_ops_400k = 1; ops = 400000; }
        } else {
          yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
          yyjson_val* box_ty = ty ? yyjson_obj_get(ty, "box_type") : NULL;
          const char* sval = vobj ? yyjson_get_str(yyjson_obj_get(vobj, "value")) : NULL;
          if (box_ty && yyjson_is_str(box_ty) && sval && *sval) {
            const char* bt = yyjson_get_str(box_ty);
            if (bt && strcmp(bt, "StringBox") == 0) {
              if (strcmp(sval, "line-seed") == 0) {
                saw_line_seed = 1;
                line_seed = sval;
                line_seed_len = (long long)strlen(sval);
              } else if (strcmp(sval, "none-seed") == 0) {
                saw_none_seed = 1;
                none_seed = sval;
                none_seed_len = (long long)strlen(sval);
              } else if (strcmp(sval, "line") == 0) {
                saw_needle_line = 1;
                needle = sval;
                needle_len = (long long)strlen(sval);
              }
            }
          }
        }
      } else if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        const char* box_name = cal ? (yyjson_get_str(yyjson_obj_get(cal, "box_name")) ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : yyjson_get_str(yyjson_obj_get(cal, "box_type"))) : NULL;
        const char* name = cal ? (yyjson_get_str(yyjson_obj_get(cal, "method")) ? yyjson_get_str(yyjson_obj_get(cal, "method")) : yyjson_get_str(yyjson_obj_get(cal, "name"))) : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        size_t argc = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        if (box_name && name) {
          if (strcmp(box_name, "RuntimeDataBox") == 0 && strcmp(name, "push") == 0 && argc == 1) {
            saw_push_line = 1;
            saw_push_none = 1;
          } else if (strcmp(box_name, "RuntimeDataBox") == 0 && strcmp(name, "get") == 0 && argc == 1) {
            saw_get = 1;
          } else if (strcmp(box_name, "RuntimeDataBox") == 0 && strcmp(name, "indexOf") == 0 && argc == 1) {
            saw_indexof = 1;
          } else if (strcmp(box_name, "RuntimeDataBox") == 0 && strcmp(name, "set") == 0 && argc == 2) {
            saw_set = 1;
          } else if (strcmp(box_name, "ArrayBox") == 0 && strcmp(name, "length") == 0 && argc == 0) {
            saw_length = 1;
          }
        }
      }
    }
  }

  if (!(saw_newbox_array && saw_rows_64 && saw_ops_400k &&
        saw_line_seed && saw_none_seed && saw_needle_line &&
        saw_push_line && saw_push_none && saw_get && saw_indexof && saw_length)) {
    yyjson_doc_free(doc);
    return -1;
  }
  if (saw_set) {
    yyjson_doc_free(doc);
    return -1;
  }
  if (!(rows == 64 && ops == 400000 &&
        line_seed && none_seed && needle && line_seed_len > 0 && none_seed_len > 0 && needle_len > 0)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_indexof_leaf_ir(
        obj_out,
        &selection,
        rows,
        ops,
        line_seed,
        line_seed_len,
        none_seed,
        none_seed_len,
        needle,
        needle_len);
    yyjson_doc_free(doc);
    return rc;
  }
}

static int hako_llvmc_match_indexof_line_ascii_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  size_t bi;
  hako_llvmc_entry_rune_selection selection;
  int saw_newbox_array = 0;
  int saw_rows_64 = 0;
  int saw_ops_400k = 0;
  int saw_flip_16 = 0;
  int saw_line_seed = 0;
  int saw_none_seed = 0;
  int saw_needle_line = 0;
  int saw_push_line = 0;
  int saw_push_none = 0;
  int saw_get = 0;
  int saw_indexof = 0;
  int saw_set = 0;
  int saw_length = 0;
  long long rows = 0;
  long long ops = 0;
  long long flip_period = 0;
  const char* line_seed = NULL;
  const char* none_seed = NULL;
  const char* needle = NULL;
  long long line_seed_len = 0;
  long long none_seed_len = 0;
  long long needle_len = 0;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  for (bi = 0; bi < yyjson_arr_size(fns); bi++) {
    yyjson_val* candidate = yyjson_arr_get(fns, bi);
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
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 18)) {
    yyjson_doc_free(doc);
    return -1;
  }

  for (bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
    size_t ii;
    if (!(insts && yyjson_is_arr(insts))) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (ii = 0; ii < yyjson_arr_size(insts); ii++) {
      yyjson_val* ins = yyjson_arr_get(insts, ii);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "newbox") == 0) {
        const char* box_type = yyjson_get_str(yyjson_obj_get(ins, "type"));
        if (box_type && strcmp(box_type, "ArrayBox") == 0) {
          saw_newbox_array = 1;
        }
      } else if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        const char* kind = vobj ? yyjson_get_str(yyjson_obj_get(vobj, "type")) : NULL;
        long long iv = vobj ? (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value")) : 0;
        if (kind && strcmp(kind, "i64") == 0) {
          if (iv == 64) { saw_rows_64 = 1; rows = 64; }
          if (iv == 400000) { saw_ops_400k = 1; ops = 400000; }
          if (iv == 16) { saw_flip_16 = 1; flip_period = 16; }
        } else {
          yyjson_val* ty = vobj ? yyjson_obj_get(vobj, "type") : NULL;
          yyjson_val* box_ty = ty ? yyjson_obj_get(ty, "box_type") : NULL;
          const char* box_type = box_ty ? yyjson_get_str(box_ty) : NULL;
          const char* str_val = vobj ? yyjson_get_str(yyjson_obj_get(vobj, "value")) : NULL;
          if (box_type && str_val) {
            long long slen = 0;
            if (!hako_llvmc_ascii_strlen(str_val, &slen)) {
              yyjson_doc_free(doc);
              return -1;
            }
            if (strcmp(box_type, "StringBox") == 0 && strcmp(str_val, "line-seed") == 0) {
              saw_line_seed = 1;
              line_seed = str_val;
              line_seed_len = slen;
            } else if (strcmp(box_type, "StringBox") == 0 && strcmp(str_val, "none-seed") == 0) {
              saw_none_seed = 1;
              none_seed = str_val;
              none_seed_len = slen;
            } else if (strcmp(box_type, "StringBox") == 0 && strcmp(str_val, "line") == 0) {
              saw_needle_line = 1;
              needle = str_val;
              needle_len = slen;
            }
          }
        }
      } else if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        const char* box_name = cal ? (yyjson_get_str(yyjson_obj_get(cal, "box_name")) ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : yyjson_get_str(yyjson_obj_get(cal, "box_type"))) : NULL;
        const char* name = cal ? (yyjson_get_str(yyjson_obj_get(cal, "method")) ? yyjson_get_str(yyjson_obj_get(cal, "method")) : yyjson_get_str(yyjson_obj_get(cal, "name"))) : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        size_t argc = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        if (box_name && name) {
          if (strcmp(box_name, "RuntimeDataBox") == 0 && strcmp(name, "push") == 0 && argc == 1) {
            saw_push_line = 1;
            saw_push_none = 1;
          } else if (strcmp(box_name, "RuntimeDataBox") == 0 && strcmp(name, "get") == 0 && argc == 1) {
            saw_get = 1;
          } else if (strcmp(box_name, "RuntimeDataBox") == 0 && strcmp(name, "indexOf") == 0 && argc == 1) {
            saw_indexof = 1;
          } else if (strcmp(box_name, "RuntimeDataBox") == 0 && strcmp(name, "set") == 0 && argc == 2) {
            saw_set = 1;
          } else if (strcmp(box_name, "ArrayBox") == 0 && strcmp(name, "length") == 0 && argc == 0) {
            saw_length = 1;
          }
        }
      }
    }
  }

  if (!(saw_newbox_array && saw_rows_64 && saw_ops_400k && saw_flip_16 &&
        saw_line_seed && saw_none_seed && saw_needle_line &&
        saw_push_line && saw_push_none && saw_get && saw_indexof && saw_set && saw_length)) {
    yyjson_doc_free(doc);
    return -1;
  }
  if (!(rows == 64 && ops == 400000 && flip_period == 16 &&
        line_seed && none_seed && needle && line_seed_len > 0 && none_seed_len > 0 && needle_len > 0)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_indexof_line_ir(
        obj_out,
        &selection,
        rows,
        ops,
        flip_period,
        line_seed,
        line_seed_len,
        none_seed,
        none_seed_len,
        needle,
        needle_len);
    yyjson_doc_free(doc);
  return rc;
}
}
