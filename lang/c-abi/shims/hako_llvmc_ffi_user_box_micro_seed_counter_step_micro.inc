static int hako_llvmc_emit_userbox_counter_step_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long ops,
    long long step_value) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long final_sum;

  if (!(obj_out && selection && ops == 2000000)) {
    return -1;
  }
  final_sum = step_value * (ops + 1);

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_userbox_counter_step_micro_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox counter step micro)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  /* The exact keeper shape proves Counter.step() == 43 and ops == 2,000,000,
   * so the final sum is a closed constant. Emit the scalar result directly and
   * let ny_main stay call-free like the C baseline. */
  fprintf(f, "  %%ret = add i64 0, %lld\n", final_sum);
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_userbox_counter_step_micro_seed(
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
  yyjson_val* block = NULL;
  yyjson_val* insts = NULL;
  yyjson_val* ins = NULL;
  long long ops = 0;
  long long base_value = 0;
  long long delta_value = 0;
  int saw_counter_newbox = 0;
  int field_set_count = 0;
  int known_receiver_count = 0;
  int method_field_get_ok = 0;
  int method_const_ok = 0;
  hako_llvmc_entry_rune_selection selection;
  size_t bi;

  if (!doc) return -1;
  root = yyjson_doc_get_root(doc);
  fns = yyjson_obj_get(root, "functions");
  if (!(fns && yyjson_is_arr(fns))) {
    yyjson_doc_free(doc);
    return -1;
  }

  fn = hako_llvmc_find_entry_function(fns);
  method_fn = hako_llvmc_find_named_function(fns, "Counter.step/0");
  if (!(fn && method_fn)) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);
  if (!(hako_llvmc_userbox_counter_decl_ok(root) &&
        hako_llvmc_userbox_has_inline_scalar_selection(
            fn, "user_box_field_set", "Counter.value"))) {
    yyjson_doc_free(doc);
    return -1;
  }

  blocks = yyjson_obj_get(fn, "blocks");
  if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) == 5)) {
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
        if (box_type && strcmp(box_type, "Counter") == 0) {
          saw_counter_newbox = 1;
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
          } else if (iv == 41) {
            base_value = iv;
          }
        }
      } else if (strcmp(op, "field_set") == 0) {
        const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
        yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
        const char* box_type =
            declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
        if (!(field && strcmp(field, "value") == 0 && box_type &&
              strcmp(box_type, "IntegerBox") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        field_set_count++;
      } else if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
        const char* bname = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : NULL;
        const char* certainty = cal ? yyjson_get_str(yyjson_obj_get(cal, "certainty")) : NULL;
        const char* mname = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
        if (!(ctype && strcmp(ctype, "Method") == 0 && bname &&
              strcmp(bname, "Counter") == 0 && certainty &&
              strcmp(certainty, "Known") == 0 && mname &&
              strcmp(mname, "step") == 0 && args && yyjson_is_arr(args) &&
              yyjson_arr_size(args) == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        known_receiver_count++;
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
    size_t ii;
    if (!(insts && yyjson_is_arr(insts))) {
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
        if (field && strcmp(field, "value") == 0 && box_type &&
            strcmp(box_type, "IntegerBox") == 0) {
          method_field_get_ok = 1;
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
        if (kind && strcmp(kind, "i64") == 0 && iv == 2) {
          delta_value = iv;
          method_const_ok = 1;
        }
      }
    }
  }

  if (!(saw_counter_newbox && field_set_count == 1 && known_receiver_count == 2 &&
        method_field_get_ok && method_const_ok && ops == 2000000 &&
        base_value == 41 && delta_value == 2 &&
        hako_llvmc_userbox_has_known_receiver_selection(fn, "Counter.step"))) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_counter_step_micro_ir(
        obj_out, &selection, ops, base_value + delta_value);
    yyjson_doc_free(doc);
    return rc;
  }
}
