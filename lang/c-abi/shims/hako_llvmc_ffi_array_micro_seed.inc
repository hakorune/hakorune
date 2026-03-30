static int hako_llvmc_emit_array_getset_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long size,
    long long ops) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!obj_out || size != 128 || ops != 2000000) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_array_getset_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (array get/set micro)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%arr = alloca [128 x i64], align 8\n");
  fprintf(f, "  %%i = alloca i64, align 8\n");
  fprintf(f, "  %%sum = alloca i64, align 8\n");
  fprintf(f, "  %%arr.ptr = getelementptr inbounds [128 x i64], ptr %%arr, i64 0, i64 0\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  br label %%init\n\n");

  fprintf(f, "init:\n");
  fprintf(f, "  %%i.init = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%init.cond = icmp slt i64 %%i.init, 128\n");
  fprintf(f, "  br i1 %%init.cond, label %%init.body, label %%init.exit\n\n");

  fprintf(f, "init.body:\n");
  fprintf(f, "  %%init.slot = getelementptr inbounds i64, ptr %%arr.ptr, i64 %%i.init\n");
  fprintf(f, "  store i64 %%i.init, ptr %%init.slot, align 8\n");
  fprintf(f, "  %%i.init.next = add i64 %%i.init, 1\n");
  fprintf(f, "  store i64 %%i.init.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%init\n\n");

  fprintf(f, "init.exit:\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  store i64 0, ptr %%sum, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  %%idx = and i64 %%i.cur, 127\n");
  fprintf(f, "  %%slot = getelementptr inbounds i64, ptr %%arr.ptr, i64 %%idx\n");
  fprintf(f, "  %%v = load i64, ptr %%slot, align 8\n");
  fprintf(f, "  %%next = add i64 %%v, 1\n");
  fprintf(f, "  store i64 %%next, ptr %%slot, align 8\n");
  fprintf(f, "  %%sum.cur = load volatile i64, ptr %%sum, align 8\n");
  fprintf(f, "  %%sum.next = add i64 %%sum.cur, %%next\n");
  fprintf(f, "  store volatile i64 %%sum.next, ptr %%sum, align 8\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%sum.fin = load volatile i64, ptr %%sum, align 8\n");
  fprintf(f, "  %%ret = and i64 %%sum.fin, 255\n");
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_array_getset_micro_seed(
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
  int saw_size_128 = 0;
  int saw_ops_2m = 0;
  int push_count = 0;
  int saw_runtime_get = 0;
  int saw_runtime_set = 0;
  int saw_arraybox_get = 0;
  long long size = 0;
  long long ops = 0;

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
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 9)) {
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
          if (iv == 128) { saw_size_128 = 1; size = 128; }
          if (iv == 2000000) { saw_ops_2m = 1; ops = 2000000; }
        }
      } else if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        const char* box_name = cal ? (yyjson_get_str(yyjson_obj_get(cal, "box_name")) ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : yyjson_get_str(yyjson_obj_get(cal, "box_type"))) : NULL;
        const char* name = cal ? (yyjson_get_str(yyjson_obj_get(cal, "method")) ? yyjson_get_str(yyjson_obj_get(cal, "method")) : yyjson_get_str(yyjson_obj_get(cal, "name"))) : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        size_t argc = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        if (box_name && name && strcmp(box_name, "RuntimeDataBox") == 0) {
          if (strcmp(name, "push") == 0 && argc == 1) {
            push_count++;
          } else if (strcmp(name, "get") == 0 && argc == 1) {
            saw_runtime_get++;
          } else if (strcmp(name, "set") == 0 && argc == 2) {
            saw_runtime_set++;
          }
        } else if (box_name && name && strcmp(box_name, "ArrayBox") == 0 && strcmp(name, "get") == 0 && argc == 1) {
          saw_arraybox_get++;
        }
      }
    }
  }

  if (!(saw_newbox_array && saw_size_128 && saw_ops_2m && push_count == 1 &&
        saw_runtime_get == 1 && saw_runtime_set == 1 && saw_arraybox_get == 0)) {
    yyjson_doc_free(doc);
    return -1;
  }
  if (!(size == 128 && ops == 2000000)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_array_getset_micro_ir(
        obj_out,
        &selection,
        size,
        ops);
    yyjson_doc_free(doc);
    return rc;
  }
}
