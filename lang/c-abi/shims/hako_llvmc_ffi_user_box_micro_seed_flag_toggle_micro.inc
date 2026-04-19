static int hako_llvmc_match_userbox_flag_toggle_micro_seed(
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
  long long flip_at = 0;
  int saw_flag_newbox = 0;
  int field_get_count = 0;
  int field_set_count = 0;
  int compare_lt_count = 0;
  int compare_eq_count = 0;
  int binop_count = 0;
  int flag_decl_ok = 0;
  int flag_selection_ok = 0;
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
  flag_selection_ok = hako_llvmc_userbox_flag_inline_scalar_selection_ok(fn);
  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 5)) {
    yyjson_doc_free(doc);
    return -1;
  }

  flag_decl_ok = hako_llvmc_userbox_flag_decl_ok(root);

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
        if (box_type && strcmp(box_type, "Flag") == 0) {
          saw_flag_newbox = 1;
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
        if (kind && strcmp(kind, "i64") == 0) {
          if (iv == 2000000) {
            ops = iv;
          } else if (iv == 1000000) {
            flip_at = iv;
          }
        }
      } else if (strcmp(op, "field_get") == 0 || strcmp(op, "field_set") == 0) {
        const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
        yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
        const char* box_type =
            declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
        if (!(field && box_type && strcmp(field, "enabled") == 0 &&
              strcmp(box_type, "BoolBox") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        if (strcmp(op, "field_get") == 0) {
          field_get_count++;
        } else {
          field_set_count++;
        }
      } else if (strcmp(op, "compare") == 0) {
        const char* operation = yyjson_get_str(yyjson_obj_get(ins, "operation"));
        if (!operation) {
          yyjson_doc_free(doc);
          return -1;
        }
        if (strcmp(operation, "<") == 0) {
          compare_lt_count++;
        } else if (strcmp(operation, "==") == 0) {
          compare_eq_count++;
        } else {
          yyjson_doc_free(doc);
          return -1;
        }
      } else if (strcmp(op, "binop") == 0) {
        binop_count++;
      }
    }
  }

  if (!(flag_decl_ok && flag_selection_ok && saw_flag_newbox && ops == 2000000 &&
        flip_at == 1000000 &&
        field_get_count == 2 && field_set_count == 2 && compare_lt_count == 2 &&
        compare_eq_count == 2 && binop_count == 3)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_flag_toggle_micro_ir(obj_out, &selection, ops, flip_at);
    yyjson_doc_free(doc);
    return rc;
  }
}
