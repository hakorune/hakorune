static int hako_llvmc_emit_userbox_counter_step_local_i64_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long base,
    long long delta) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!(obj_out && selection)) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_userbox_counter_step_local_i64_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox counter step local i64 seed)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%base = add i64 0, %lld\n", base);
  fprintf(f, "  %%delta = add i64 0, %lld\n", delta);
  fprintf(f, "  %%ret = add i64 %%base, %%delta\n");
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_userbox_counter_step_local_i64_seed(
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
  long long base_reg = 0;
  long long call_dst = 0;
  long long base_value = 0;
  long long delta_value = 0;
  long long field_reg = 0;
  long long add_reg = 0;
  int counter_decl_ok = 0;
  int counter_selection_ok = 0;
  int known_receiver_ok = 0;
  hako_llvmc_entry_rune_selection selection;
  size_t ii;

  if (!doc) return -1;
  hako_llvmc_emit_route_trace(
      "userbox_counter_step_chain_seed",
      "start",
      "Counter.step_chain/1",
      "");
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  fn = hako_llvmc_find_entry_function(fns);
  method_fn = hako_llvmc_find_named_function(fns, "Counter.step/1");
  if (!(fn && method_fn)) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);
  counter_decl_ok = hako_llvmc_userbox_counter_decl_ok(root);
  counter_selection_ok =
      hako_llvmc_userbox_counter_inline_scalar_selection_ok(fn, method_fn);
  known_receiver_ok =
      hako_llvmc_userbox_has_known_receiver_selection(fn, "Counter.step");
  if (!(counter_decl_ok && counter_selection_ok && known_receiver_ok)) {
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
    if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 5)) {
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
        if (!(kind && strcmp(kind, "i64") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        base_reg = hako_llvmc_userbox_read_int(ins, "dst");
        base_value = (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value"));
      } else if (strcmp(op, "newbox") == 0) {
        const char* box_type = yyjson_get_str(yyjson_obj_get(ins, "type"));
        if (!(box_type && strcmp(box_type, "Counter") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        box_reg = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "field_set") == 0) {
        const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
        yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
        const char* box_type =
            declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
        if (!(field && strcmp(field, "value") == 0 && box_type &&
              strcmp(box_type, "IntegerBox") == 0 &&
              hako_llvmc_userbox_read_int(ins, "box") == box_reg &&
              hako_llvmc_userbox_read_int(ins, "value") == base_reg)) {
          yyjson_doc_free(doc);
          return -1;
        }
      } else if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
        const char* bname = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : NULL;
        const char* mname = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
        if (!(ctype && strcmp(ctype, "Method") == 0 && bname &&
              strcmp(bname, "Counter") == 0 && mname && strcmp(mname, "step") == 0 &&
              hako_llvmc_userbox_read_int(cal, "receiver") == box_reg &&
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
        if (!(field && strcmp(field, "value") == 0 && box_type &&
              strcmp(box_type, "IntegerBox") == 0 &&
              hako_llvmc_userbox_read_int(ins, "box") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        field_reg = hako_llvmc_userbox_read_int(ins, "dst");
      } else if (strcmp(op, "const") == 0) {
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* type_val = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        const char* kind = yyjson_is_str(type_val)
                               ? yyjson_get_str(type_val)
                               : (type_val ? yyjson_get_str(yyjson_obj_get(type_val, "type"))
                                           : NULL);
        if (!(kind && strcmp(kind, "i64") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        delta_value = (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value"));
      } else if (strcmp(op, "binop") == 0) {
        const char* operation = yyjson_get_str(yyjson_obj_get(ins, "operation"));
        if (!(operation && strcmp(operation, "+") == 0 &&
              hako_llvmc_userbox_read_int(ins, "lhs") == field_reg)) {
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

  if (!(box_reg > 0 && base_reg > 0 && call_dst > 0 && field_reg > 0 && add_reg > 0)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_counter_step_local_i64_ir(
        obj_out, &selection, base_value, delta_value);
    yyjson_doc_free(doc);
    return rc;
  }
}
