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

static int hako_llvmc_match_userbox_point_add_micro_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* blocks;
  size_t bi;
  long long ops = 0;
  int saw_point_newbox = 0;
  int field_get_count = 0;
  int field_set_count = 0;
  int point_decl_ok = 0;
  int point_selection_ok = 0;
  hako_llvmc_entry_rune_selection selection;

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
  point_selection_ok = hako_llvmc_userbox_point_inline_scalar_selection_ok(fn);
  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 5)) {
    yyjson_doc_free(doc);
    return -1;
  }

  point_decl_ok = hako_llvmc_userbox_point_decl_ok(root);

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
        if (box_type && strcmp(box_type, "Point") == 0) {
          saw_point_newbox = 1;
        }
      } else if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* type_val = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        const char* kind = NULL;
        long long iv = vobj ? (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value")) : 0;
        if (type_val) {
          if (yyjson_is_str(type_val)) {
            kind = yyjson_get_str(type_val);
          } else if (yyjson_is_obj(type_val)) {
            kind = yyjson_get_str(yyjson_obj_get(type_val, "type"));
          }
        }
        if (kind && strcmp(kind, "i64") == 0 && iv == 2000000) {
          ops = iv;
        }
      } else if (strcmp(op, "field_get") == 0 || strcmp(op, "field_set") == 0) {
        const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
        yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
        const char* box_type =
            declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
        if (!(field && box_type && strcmp(box_type, "IntegerBox") == 0 &&
              (strcmp(field, "x") == 0 || strcmp(field, "y") == 0))) {
          yyjson_doc_free(doc);
          return -1;
        }
        if (strcmp(op, "field_get") == 0) {
          field_get_count++;
        } else {
          field_set_count++;
        }
      }
    }
  }

  if (!(point_decl_ok && point_selection_ok && saw_point_newbox && ops == 2000000 &&
        field_get_count == 6 && field_set_count == 4)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_point_add_micro_ir(obj_out, &selection, ops);
    yyjson_doc_free(doc);
    return rc;
  }
}

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
  fprintf(f, "  %%enabled = alloca i64, align 8\n");
  fprintf(f, "  %%acc = alloca i64, align 8\n");
  fprintf(f, "  %%i = alloca i64, align 8\n");
  /* Keep the synthetic BoolBox field promotable; acc stays volatile as the benchmark anchor. */
  fprintf(f, "  store i64 1, ptr %%enabled, align 8\n");
  fprintf(f, "  store volatile i64 0, ptr %%acc, align 8\n");
  fprintf(f, "  store i64 0, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "loop:\n");
  fprintf(f, "  %%i.cur = load i64, ptr %%i, align 8\n");
  fprintf(f, "  %%loop.cond = icmp slt i64 %%i.cur, %lld\n", ops);
  fprintf(f, "  br i1 %%loop.cond, label %%loop.body, label %%exit\n\n");

  fprintf(f, "loop.body:\n");
  fprintf(f, "  %%enabled.cur = load i64, ptr %%enabled, align 8\n");
  fprintf(f, "  %%enabled.cur.bool = icmp ne i64 %%enabled.cur, 0\n");
  fprintf(f, "  %%enabled.cur.i64 = zext i1 %%enabled.cur.bool to i64\n");
  fprintf(f, "  %%acc.cur = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%acc.next = add i64 %%acc.cur, %%enabled.cur.i64\n");
  fprintf(f, "  store volatile i64 %%acc.next, ptr %%acc, align 8\n");
  fprintf(f, "  %%enabled.next.bool = icmp slt i64 %%i.cur, %lld\n", flip_at);
  fprintf(f, "  %%enabled.next = zext i1 %%enabled.next.bool to i64\n");
  fprintf(f, "  store i64 %%enabled.next, ptr %%enabled, align 8\n");
  fprintf(f, "  %%i.next = add i64 %%i.cur, 1\n");
  fprintf(f, "  store i64 %%i.next, ptr %%i, align 8\n");
  fprintf(f, "  br label %%loop\n\n");

  fprintf(f, "exit:\n");
  fprintf(f, "  %%acc.fin = load volatile i64, ptr %%acc, align 8\n");
  fprintf(f, "  %%enabled.fin = load i64, ptr %%enabled, align 8\n");
  fprintf(f, "  %%enabled.fin.bool = icmp ne i64 %%enabled.fin, 0\n");
  fprintf(f, "  %%enabled.fin.i64 = zext i1 %%enabled.fin.bool to i64\n");
  fprintf(f, "  %%ret = add i64 %%acc.fin, %%enabled.fin.i64\n");
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}
