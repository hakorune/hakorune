static int hako_llvmc_match_userbox_point_sum_copy_local_i64_seed(
    const char* json_in,
    const char* obj_out) {
  yyjson_read_err rerr;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
  yyjson_val* root;
  yyjson_val* fns;
  yyjson_val* fn = NULL;
  yyjson_val* method_fn = NULL;
  yyjson_val* blocks;
  yyjson_val* method_blocks;
  long long box_reg = 0;
  long long alias_reg = 0;
  long long const0_reg = 0;
  long long const1_reg = 0;
  long long x_value = 0;
  long long y_value = 0;
  long long call_dst = 0;
  long long field_x_reg = 0;
  long long field_y_reg = 0;
  long long add_reg = 0;
  long long const0_value = 0;
  long long const1_value = 0;
  int saw_x_set = 0;
  int saw_y_set = 0;
  int saw_x_get = 0;
  int saw_y_get = 0;
  hako_llvmc_entry_rune_selection selection;
  size_t ii;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  fn = hako_llvmc_find_entry_function(fns);
  method_fn = hako_llvmc_find_named_function(fns, "Point.sum/1");
  if (!(fn && method_fn)) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);
  if (!(hako_llvmc_userbox_point_decl_ok(root) &&
        hako_llvmc_userbox_point_sum_local_i64_selection_ok(fn, method_fn) &&
        hako_llvmc_userbox_has_known_receiver_selection(fn, "Point.sum"))) {
    yyjson_doc_free(doc);
    return -1;
  }

  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    yyjson_val* block = yyjson_arr_get(blocks, 0);
    yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
    if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 8)) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (ii = 0; ii < yyjson_arr_size(insts); ii++) {
      yyjson_val* ins = yyjson_arr_get(insts, ii);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* type_val = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        const char* kind = yyjson_is_str(type_val)
                               ? yyjson_get_str(type_val)
                               : (type_val ? yyjson_get_str(yyjson_obj_get(type_val, "type"))
                                           : NULL);
        long long dst = hako_llvmc_userbox_read_int(ins, "dst");
        long long value = (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value"));
        if (!(kind && strcmp(kind, "i64") == 0 && dst > 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        if (const0_reg == 0) {
          const0_reg = dst;
          const0_value = value;
        } else if (const1_reg == 0) {
          const1_reg = dst;
          const1_value = value;
        } else {
          yyjson_doc_free(doc);
          return -1;
        }
      } else if (strcmp(op, "newbox") == 0) {
        const char* box_type = yyjson_get_str(yyjson_obj_get(ins, "type"));
        if (!(box_type && strcmp(box_type, "Point") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        box_reg = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "field_set") == 0) {
        const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
        yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
        const char* box_type =
            declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
        long long value_reg = hako_llvmc_userbox_read_int(ins, "value");
        if (!(field && box_type && strcmp(box_type, "IntegerBox") == 0 &&
              hako_llvmc_userbox_read_int(ins, "box") == box_reg)) {
          yyjson_doc_free(doc);
          return -1;
        }
        if (strcmp(field, "x") == 0) {
          if (value_reg == const0_reg) {
            x_value = const0_value;
          } else if (value_reg == const1_reg) {
            x_value = const1_value;
          } else {
            yyjson_doc_free(doc);
            return -1;
          }
          saw_x_set = 1;
        } else if (strcmp(field, "y") == 0) {
          if (value_reg == const0_reg) {
            y_value = const0_value;
          } else if (value_reg == const1_reg) {
            y_value = const1_value;
          } else {
            yyjson_doc_free(doc);
            return -1;
          }
          saw_y_set = 1;
        } else {
          yyjson_doc_free(doc);
          return -1;
        }
      } else if (strcmp(op, "copy") == 0) {
        if (!(hako_llvmc_userbox_read_int(ins, "src") == box_reg)) {
          yyjson_doc_free(doc);
          return -1;
        }
        alias_reg = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
        const char* bname = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : NULL;
        const char* mname = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
        if (!(ctype && strcmp(ctype, "Method") == 0 && bname &&
              strcmp(bname, "Point") == 0 && mname && strcmp(mname, "sum") == 0 &&
              hako_llvmc_userbox_read_int(cal, "receiver") == alias_reg &&
              args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        call_dst = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "ret") == 0) {
        if (hako_llvmc_userbox_read_int(ins, "value") != call_dst) {
          yyjson_doc_free(doc);
          return -1;
        }
      } else {
        yyjson_doc_free(doc);
        return -1;
      }
    }
  }

  method_blocks = yyjson_obj_get(method_fn, "blocks");
  if (!(method_blocks && yyjson_is_arr(method_blocks) &&
        yyjson_arr_size(method_blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  {
    yyjson_val* block = yyjson_arr_get(method_blocks, 0);
    yyjson_val* insts = block ? yyjson_obj_get(block, "instructions") : NULL;
    yyjson_val* params = yyjson_obj_get(method_fn, "params");
    if (!(params && yyjson_is_arr(params) && yyjson_arr_size(params) == 1 &&
          yyjson_get_sint(yyjson_arr_get(params, 0)) == 0 &&
          insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 4)) {
      yyjson_doc_free(doc);
      return -1;
    }
    for (ii = 0; ii < yyjson_arr_size(insts); ii++) {
      yyjson_val* ins = yyjson_arr_get(insts, ii);
      const char* op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
      if (!op) continue;
      if (strcmp(op, "field_get") == 0) {
        const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
        yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
        const char* box_type =
            declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
        if (!(field && box_type && strcmp(box_type, "IntegerBox") == 0 &&
              hako_llvmc_userbox_read_int(ins, "box") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        if (strcmp(field, "x") == 0) {
          field_x_reg = hako_llvmc_userbox_read_int(ins, "dst");
          saw_x_get = 1;
        } else if (strcmp(field, "y") == 0) {
          field_y_reg = hako_llvmc_userbox_read_int(ins, "dst");
          saw_y_get = 1;
        } else {
          yyjson_doc_free(doc);
          return -1;
        }
      } else if (strcmp(op, "binop") == 0) {
        const char* operation = yyjson_get_str(yyjson_obj_get(ins, "operation"));
        if (!(operation && strcmp(operation, "+") == 0 &&
              hako_llvmc_userbox_read_int(ins, "lhs") == field_x_reg &&
              hako_llvmc_userbox_read_int(ins, "rhs") == field_y_reg)) {
          yyjson_doc_free(doc);
          return -1;
        }
        add_reg = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "ret") == 0) {
        if (hako_llvmc_userbox_read_int(ins, "value") != add_reg) {
          yyjson_doc_free(doc);
          return -1;
        }
      } else {
        yyjson_doc_free(doc);
        return -1;
      }
    }
  }

  if (!(box_reg > 0 && alias_reg > 0 && const0_reg > 0 && const1_reg > 0 && call_dst > 0 &&
        saw_x_set && saw_y_set && saw_x_get && saw_y_get && add_reg > 0)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_point_sum_local_i64_ir(
        obj_out, &selection, x_value, y_value);
    yyjson_doc_free(doc);
    return rc;
  }
}
