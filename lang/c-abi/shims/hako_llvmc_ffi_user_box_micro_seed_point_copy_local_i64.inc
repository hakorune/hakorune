static int hako_llvmc_match_userbox_point_copy_local_i64_seed(
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
  if (!(hako_llvmc_userbox_point_inline_scalar_selection_ok(fn) &&
        hako_llvmc_userbox_point_decl_ok(root))) {
    yyjson_doc_free(doc);
    return -1;
  }

  block = yyjson_arr_get(blocks, 0);
  insts = block ? yyjson_obj_get(block, "instructions") : NULL;
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 10)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 0);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "const") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 1 &&
        (long long)yyjson_get_sint(yyjson_obj_get(yyjson_obj_get(ins, "value"), "value")) == 41)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 1);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "const") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 2 &&
        (long long)yyjson_get_sint(yyjson_obj_get(yyjson_obj_get(ins, "value"), "value")) == 2)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 2);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  box_type = ins ? yyjson_get_str(yyjson_obj_get(ins, "type")) : NULL;
  if (!(op && strcmp(op, "newbox") == 0 && box_type && strcmp(box_type, "Point") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 3)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 3);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_set") == 0 && field_name && strcmp(field_name, "x") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "IntegerBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 3 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "value")) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 4);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_set") == 0 && field_name && strcmp(field_name, "y") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "IntegerBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 3 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "value")) == 2)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 5);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "copy") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 6 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "src")) == 3)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 6);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_get") == 0 && field_name && strcmp(field_name, "x") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "IntegerBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 6 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 7)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 7);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_get") == 0 && field_name && strcmp(field_name, "y") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "IntegerBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 6 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 8)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 8);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "binop") == 0 &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "operation")), "+") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 9 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "lhs")) == 7 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "rhs")) == 8)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 9);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "ret") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "value")) == 9)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_point_local_i64_ir(obj_out, &selection);
    yyjson_doc_free(doc);
    return rc;
  }
}
