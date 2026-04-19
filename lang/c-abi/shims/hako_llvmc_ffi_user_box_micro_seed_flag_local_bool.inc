static int hako_llvmc_emit_userbox_flag_local_bool_ir(
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
      "%s/hako_pure_userbox_flag_local_bool_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox flag local bool seed)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(
      f,
      "entry:\n"
      "  ret i64 1\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_userbox_flag_local_bool_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn;
  yyjson_val* blocks;
  yyjson_val* block;
  yyjson_val* insts;
  yyjson_val* ins;
  const char* op;
  const char* box_type;
  const char* field_name;
  hako_llvmc_entry_rune_selection selection;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  fn = hako_llvmc_find_entry_function(fns);
  if (!fn) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);
  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  if (!(hako_llvmc_userbox_flag_inline_scalar_selection_ok(fn) &&
        hako_llvmc_userbox_flag_decl_ok(root))) {
    yyjson_doc_free(doc);
    return -1;
  }

  block = yyjson_arr_get(blocks, 0);
  insts = block ? yyjson_obj_get(block, "instructions") : NULL;
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 5)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 0);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "const") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 1 &&
        (long long)yyjson_get_sint(yyjson_obj_get(yyjson_obj_get(ins, "value"), "value")) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 1);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  box_type = ins ? yyjson_get_str(yyjson_obj_get(ins, "type")) : NULL;
  if (!(op && strcmp(op, "newbox") == 0 && box_type && strcmp(box_type, "Flag") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 2)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 2);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_set") == 0 && field_name && strcmp(field_name, "enabled") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "BoolBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 2 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "value")) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 3);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_get") == 0 && field_name && strcmp(field_name, "enabled") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "BoolBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 2 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 3)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 4);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "ret") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "value")) == 3)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_flag_local_bool_ir(obj_out, &selection);
    yyjson_doc_free(doc);
    return rc;
  }
}
