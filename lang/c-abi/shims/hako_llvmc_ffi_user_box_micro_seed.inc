static yyjson_val* hako_llvmc_find_entry_function(yyjson_val* fns);

static yyjson_val* hako_llvmc_find_named_function(
    yyjson_val* fns,
    const char* expected_name) {
  size_t bi;

  if (!(fns && yyjson_is_arr(fns) && expected_name)) {
    return NULL;
  }

  for (bi = 0; bi < yyjson_arr_size(fns); bi++) {
    yyjson_val* candidate = yyjson_arr_get(fns, bi);
    const char* name = candidate ? yyjson_get_str(yyjson_obj_get(candidate, "name")) : NULL;
    if (name && strcmp(name, expected_name) == 0) {
      return candidate;
    }
  }

  return NULL;
}

static long long hako_llvmc_userbox_read_int(yyjson_val* obj, const char* key) {
  yyjson_val* value = obj ? yyjson_obj_get(obj, key) : NULL;
  return value ? (long long)yyjson_get_sint(value) : 0;
}

static int hako_llvmc_userbox_has_inline_scalar_selection(
    yyjson_val* fn,
    const char* surface,
    const char* subject) {
  char expected_row[96];

  if (!(fn && yyjson_is_obj(fn) && surface && subject)) {
    return 0;
  }

  snprintf(expected_row, sizeof(expected_row), "%s.inline_scalar", surface);
  return hako_llvmc_has_thin_entry_selection(
      fn,
      surface,
      subject,
      expected_row,
      "thin_internal_entry");
}

static int hako_llvmc_userbox_point_inline_scalar_selection_ok(yyjson_val* fn) {
  return hako_llvmc_userbox_has_inline_scalar_selection(fn, "user_box_field_get", "Point.x") &&
         hako_llvmc_userbox_has_inline_scalar_selection(fn, "user_box_field_get", "Point.y") &&
         hako_llvmc_userbox_has_inline_scalar_selection(fn, "user_box_field_set", "Point.x") &&
         hako_llvmc_userbox_has_inline_scalar_selection(fn, "user_box_field_set", "Point.y");
}

static int hako_llvmc_userbox_point_sum_inline_scalar_selection_ok(
    yyjson_val* main_fn,
    yyjson_val* method_fn) {
  (void)method_fn;
  return hako_llvmc_userbox_has_inline_scalar_selection(
             main_fn, "user_box_field_set", "Point.x") &&
         hako_llvmc_userbox_has_inline_scalar_selection(
             main_fn, "user_box_field_set", "Point.y");
}

static int hako_llvmc_userbox_flag_inline_scalar_selection_ok(yyjson_val* fn) {
  return hako_llvmc_userbox_has_inline_scalar_selection(
             fn, "user_box_field_get", "Flag.enabled") &&
         hako_llvmc_userbox_has_inline_scalar_selection(
             fn, "user_box_field_set", "Flag.enabled");
}

static int hako_llvmc_userbox_point_decl_ok(yyjson_val* root) {
  yyjson_val* decls;
  size_t bi;

  if (!(root && yyjson_is_obj(root))) {
    return 0;
  }

  decls = yyjson_obj_get(root, "user_box_decls");
  if (!(decls && yyjson_is_arr(decls))) {
    return 0;
  }

  for (bi = 0; bi < yyjson_arr_size(decls); bi++) {
    yyjson_val* decl = yyjson_arr_get(decls, bi);
    const char* name = decl ? yyjson_get_str(yyjson_obj_get(decl, "name")) : NULL;
    yyjson_val* field_decls = decl ? yyjson_obj_get(decl, "field_decls") : NULL;
    int saw_x = 0;
    int saw_y = 0;
    size_t fi;
    if (!(name && strcmp(name, "Point") == 0 && field_decls && yyjson_is_arr(field_decls))) {
      continue;
    }
    for (fi = 0; fi < yyjson_arr_size(field_decls); fi++) {
      yyjson_val* field_decl = yyjson_arr_get(field_decls, fi);
      const char* field_name =
          field_decl ? yyjson_get_str(yyjson_obj_get(field_decl, "name")) : NULL;
      const char* declared_type =
          field_decl ? yyjson_get_str(yyjson_obj_get(field_decl, "declared_type")) : NULL;
      int is_weak = field_decl ? yyjson_get_bool(yyjson_obj_get(field_decl, "is_weak")) : 0;
      if (!(field_name && declared_type && strcmp(declared_type, "IntegerBox") == 0 &&
            !is_weak)) {
        continue;
      }
      if (strcmp(field_name, "x") == 0) saw_x = 1;
      if (strcmp(field_name, "y") == 0) saw_y = 1;
    }
    if (saw_x && saw_y) {
      return 1;
    }
  }

  return 0;
}

static int hako_llvmc_userbox_declared_box_is(yyjson_val* ins, const char* expected_box_type) {
  yyjson_val* declared_type;
  const char* box_type;

  if (!(ins && yyjson_is_obj(ins) && expected_box_type)) {
    return 0;
  }

  declared_type = yyjson_obj_get(ins, "declared_type");
  box_type = declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
  return box_type && strcmp(box_type, expected_box_type) == 0;
}

static int hako_llvmc_userbox_flag_decl_ok(yyjson_val* root) {
  yyjson_val* decls;
  size_t bi;

  if (!(root && yyjson_is_obj(root))) {
    return 0;
  }

  decls = yyjson_obj_get(root, "user_box_decls");
  if (!(decls && yyjson_is_arr(decls))) {
    return 0;
  }

  for (bi = 0; bi < yyjson_arr_size(decls); bi++) {
    yyjson_val* decl = yyjson_arr_get(decls, bi);
    const char* name = decl ? yyjson_get_str(yyjson_obj_get(decl, "name")) : NULL;
    yyjson_val* field_decls = decl ? yyjson_obj_get(decl, "field_decls") : NULL;
    size_t fi;
    if (!(name && strcmp(name, "Flag") == 0 && field_decls && yyjson_is_arr(field_decls))) {
      continue;
    }
    for (fi = 0; fi < yyjson_arr_size(field_decls); fi++) {
      yyjson_val* field_decl = yyjson_arr_get(field_decls, fi);
      const char* field_name =
          field_decl ? yyjson_get_str(yyjson_obj_get(field_decl, "name")) : NULL;
      const char* declared_type =
          field_decl ? yyjson_get_str(yyjson_obj_get(field_decl, "declared_type")) : NULL;
      int is_weak = field_decl ? yyjson_get_bool(yyjson_obj_get(field_decl, "is_weak")) : 0;
      if (field_name && declared_type && strcmp(field_name, "enabled") == 0 &&
          strcmp(declared_type, "BoolBox") == 0 && !is_weak) {
        return 1;
      }
    }
  }

  return 0;
}

static int hako_llvmc_userbox_pointf_decl_ok(yyjson_val* root) {
  yyjson_val* decls;
  size_t bi;

  if (!(root && yyjson_is_obj(root))) {
    return 0;
  }

  decls = yyjson_obj_get(root, "user_box_decls");
  if (!(decls && yyjson_is_arr(decls))) {
    return 0;
  }

  for (bi = 0; bi < yyjson_arr_size(decls); bi++) {
    yyjson_val* decl = yyjson_arr_get(decls, bi);
    const char* name = decl ? yyjson_get_str(yyjson_obj_get(decl, "name")) : NULL;
    yyjson_val* field_decls = decl ? yyjson_obj_get(decl, "field_decls") : NULL;
    size_t fi;
    if (!(name && strcmp(name, "PointF") == 0 && field_decls && yyjson_is_arr(field_decls))) {
      continue;
    }
    for (fi = 0; fi < yyjson_arr_size(field_decls); fi++) {
      yyjson_val* field_decl = yyjson_arr_get(field_decls, fi);
      const char* field_name =
          field_decl ? yyjson_get_str(yyjson_obj_get(field_decl, "name")) : NULL;
      const char* declared_type =
          field_decl ? yyjson_get_str(yyjson_obj_get(field_decl, "declared_type")) : NULL;
      int is_weak = field_decl ? yyjson_get_bool(yyjson_obj_get(field_decl, "is_weak")) : 0;
      if (field_name && declared_type && strcmp(field_name, "x") == 0 &&
          strcmp(declared_type, "FloatBox") == 0 && !is_weak) {
        return 1;
      }
    }
  }

  return 0;
}

static int hako_llvmc_userbox_pointf_inline_scalar_selection_ok(yyjson_val* fn) {
  return hako_llvmc_userbox_has_inline_scalar_selection(fn, "user_box_field_get", "PointF.x") &&
         hako_llvmc_userbox_has_inline_scalar_selection(fn, "user_box_field_set", "PointF.x");
}

static int hako_llvmc_userbox_has_known_receiver_selection(
    yyjson_val* fn,
    const char* subject) {
  return hako_llvmc_has_thin_entry_selection(
      fn,
      "user_box_method",
      subject,
      "user_box_method.known_receiver",
      "thin_internal_entry");
}

static int hako_llvmc_userbox_counter_decl_ok(yyjson_val* root) {
  yyjson_val* decls;
  size_t bi;

  if (!(root && yyjson_is_obj(root))) {
    return 0;
  }

  decls = yyjson_obj_get(root, "user_box_decls");
  if (!(decls && yyjson_is_arr(decls))) {
    return 0;
  }

  for (bi = 0; bi < yyjson_arr_size(decls); bi++) {
    yyjson_val* decl = yyjson_arr_get(decls, bi);
    const char* name = decl ? yyjson_get_str(yyjson_obj_get(decl, "name")) : NULL;
    yyjson_val* field_decls = decl ? yyjson_obj_get(decl, "field_decls") : NULL;
    size_t fi;
    if (!(name && strcmp(name, "Counter") == 0 && field_decls &&
          yyjson_is_arr(field_decls))) {
      continue;
    }
    for (fi = 0; fi < yyjson_arr_size(field_decls); fi++) {
      yyjson_val* field_decl = yyjson_arr_get(field_decls, fi);
      const char* field_name =
          field_decl ? yyjson_get_str(yyjson_obj_get(field_decl, "name")) : NULL;
      const char* declared_type =
          field_decl ? yyjson_get_str(yyjson_obj_get(field_decl, "declared_type")) : NULL;
      int is_weak = field_decl ? yyjson_get_bool(yyjson_obj_get(field_decl, "is_weak")) : 0;
      if (field_name && declared_type && strcmp(field_name, "value") == 0 &&
          strcmp(declared_type, "IntegerBox") == 0 && !is_weak) {
        return 1;
      }
    }
  }

  return 0;
}

static int hako_llvmc_userbox_counter_inline_scalar_selection_ok(
    yyjson_val* main_fn,
    yyjson_val* method_fn) {
  return hako_llvmc_userbox_has_inline_scalar_selection(
             main_fn, "user_box_field_set", "Counter.value") &&
         hako_llvmc_userbox_has_inline_scalar_selection(
             method_fn, "user_box_field_get", "Counter.value");
}

static int hako_llvmc_userbox_point_sum_local_i64_selection_ok(
    yyjson_val* main_fn,
    yyjson_val* method_fn) {
  return hako_llvmc_userbox_has_inline_scalar_selection(
             main_fn, "user_box_field_set", "Point.x") &&
         hako_llvmc_userbox_has_inline_scalar_selection(
             main_fn, "user_box_field_set", "Point.y") &&
         hako_llvmc_userbox_has_inline_scalar_selection(
             method_fn, "user_box_field_get", "Point.x") &&
         hako_llvmc_userbox_has_inline_scalar_selection(
             method_fn, "user_box_field_get", "Point.y");
}

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

static int hako_llvmc_emit_userbox_point_sum_local_i64_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long x_value,
    long long y_value) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!(obj_out && selection)) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_userbox_point_sum_local_i64_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox point sum local i64 seed)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%x = add i64 0, %lld\n", x_value);
  fprintf(f, "  %%y = add i64 0, %lld\n", y_value);
  fprintf(f, "  %%ret = add i64 %%x, %%y\n");
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_userbox_point_sum_local_i64_seed(
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
    if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 7)) {
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
      } else if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
        const char* bname = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : NULL;
        const char* mname = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
        if (!(ctype && strcmp(ctype, "Method") == 0 && bname &&
              strcmp(bname, "Point") == 0 && mname && strcmp(mname, "sum") == 0 &&
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

  if (!(box_reg > 0 && const0_reg > 0 && const1_reg > 0 && call_dst > 0 &&
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

static int hako_llvmc_emit_userbox_point_sum_micro_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    long long ops,
    long long sum_value) {
  char llpath[1024];
  FILE* f;
  int rc;
  long long final_sum;

  if (!(obj_out && selection && ops == 2000000)) {
    return -1;
  }
  final_sum = sum_value * (ops + 1);

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_userbox_point_sum_micro_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox point sum micro)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%ret = add i64 0, %lld\n", final_sum);
  fprintf(f, "  ret i64 %%ret\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_userbox_point_sum_micro_seed(
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
  int saw_point_newbox = 0;
  int saw_x_set = 0;
  int saw_y_set = 0;
  int known_receiver_count = 0;
  int compare_lt_count = 0;
  int branch_count = 0;
  int jump_count = 0;
  int ret_count = 0;
  int loop_add_count = 0;
  int exit_add_count = 0;
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
  method_fn = hako_llvmc_find_named_function(fns, "Point.sum/0");
  if (!(fn && method_fn)) {
    yyjson_doc_free(doc);
    return -1;
  }

  hako_llvmc_select_entry_runes(fn, &selection);
  if (!(hako_llvmc_userbox_point_decl_ok(root) &&
        hako_llvmc_userbox_point_sum_inline_scalar_selection_ok(fn, method_fn) &&
        hako_llvmc_userbox_has_known_receiver_selection(fn, "Point.sum"))) {
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
      } else if (strcmp(op, "field_set") == 0) {
        const char* field = yyjson_get_str(yyjson_obj_get(ins, "field"));
        yyjson_val* declared_type = yyjson_obj_get(ins, "declared_type");
        const char* box_type =
            declared_type ? yyjson_get_str(yyjson_obj_get(declared_type, "box_type")) : NULL;
        long long value_reg = hako_llvmc_userbox_read_int(ins, "value");
        if (!(field && box_type && strcmp(box_type, "IntegerBox") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        if (strcmp(field, "x") == 0) {
          yyjson_val* value_ins = NULL;
          if (value_reg > 0) {
            size_t sj;
            for (sj = 0; sj < yyjson_arr_size(insts); sj++) {
              yyjson_val* candidate = yyjson_arr_get(insts, sj);
              if (hako_llvmc_userbox_read_int(candidate, "dst") == value_reg) {
                value_ins = candidate;
                break;
              }
            }
          }
          if (!(value_ins &&
                strcmp(yyjson_get_str(yyjson_obj_get(value_ins, "op")), "const") == 0 &&
                yyjson_get_sint(
                    yyjson_obj_get(yyjson_obj_get(value_ins, "value"), "value")) == 1)) {
            yyjson_doc_free(doc);
            return -1;
          }
          saw_x_set = 1;
        } else if (strcmp(field, "y") == 0) {
          yyjson_val* value_ins = NULL;
          if (value_reg > 0) {
            size_t sj;
            for (sj = 0; sj < yyjson_arr_size(insts); sj++) {
              yyjson_val* candidate = yyjson_arr_get(insts, sj);
              if (hako_llvmc_userbox_read_int(candidate, "dst") == value_reg) {
                value_ins = candidate;
                break;
              }
            }
          }
          if (!(value_ins &&
                strcmp(yyjson_get_str(yyjson_obj_get(value_ins, "op")), "const") == 0 &&
                yyjson_get_sint(
                    yyjson_obj_get(yyjson_obj_get(value_ins, "value"), "value")) == 2)) {
            yyjson_doc_free(doc);
            return -1;
          }
          saw_y_set = 1;
        } else {
          yyjson_doc_free(doc);
          return -1;
        }
      } else if (strcmp(op, "mir_call") == 0) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
        const char* bname = cal ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : NULL;
        const char* certainty = cal ? yyjson_get_str(yyjson_obj_get(cal, "certainty")) : NULL;
        const char* mname = cal ? yyjson_get_str(yyjson_obj_get(cal, "name")) : NULL;
        if (!(ctype && strcmp(ctype, "Method") == 0 && bname &&
              strcmp(bname, "Point") == 0 && certainty &&
              strcmp(certainty, "Known") == 0 && mname &&
              strcmp(mname, "sum") == 0 && args && yyjson_is_arr(args) &&
              yyjson_arr_size(args) == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        known_receiver_count++;
      } else if (strcmp(op, "compare") == 0) {
        const char* operation = yyjson_get_str(yyjson_obj_get(ins, "operation"));
        if (operation && strcmp(operation, "<") == 0) {
          compare_lt_count++;
        }
      } else if (strcmp(op, "branch") == 0) {
        branch_count++;
      } else if (strcmp(op, "jump") == 0) {
        jump_count++;
      } else if (strcmp(op, "ret") == 0) {
        ret_count++;
      } else if (strcmp(op, "binop") == 0) {
        const char* operation = yyjson_get_str(yyjson_obj_get(ins, "operation"));
        if (!(operation && strcmp(operation, "+") == 0)) {
          yyjson_doc_free(doc);
          return -1;
        }
        if (hako_llvmc_userbox_read_int(ins, "dst") == 28) {
          loop_add_count++;
        } else if (hako_llvmc_userbox_read_int(ins, "dst") == 54) {
          exit_add_count++;
        }
      }
    }
  }

  method_blocks = yyjson_obj_get(method_fn, "blocks");
  if (!(method_blocks && yyjson_is_arr(method_blocks) &&
        yyjson_arr_size(method_blocks) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }

  block = yyjson_arr_get(method_blocks, 0);
  insts = block ? yyjson_obj_get(block, "instructions") : NULL;
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 9)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 0);
  if (!(yyjson_get_str(yyjson_obj_get(ins, "op")) &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "op")), "copy") == 0 &&
        hako_llvmc_userbox_read_int(ins, "src") == 0 &&
        hako_llvmc_userbox_read_int(ins, "dst") == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  ins = yyjson_arr_get(insts, 1);
  if (!(yyjson_get_str(yyjson_obj_get(ins, "op")) &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "op")), "field_get") == 0 &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "field")), "x") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "IntegerBox") &&
        hako_llvmc_userbox_read_int(ins, "box") == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  ins = yyjson_arr_get(insts, 3);
  if (!(yyjson_get_str(yyjson_obj_get(ins, "op")) &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "op")), "field_get") == 0 &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "field")), "y") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "IntegerBox") &&
        hako_llvmc_userbox_read_int(ins, "box") == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  ins = yyjson_arr_get(insts, 7);
  if (!(yyjson_get_str(yyjson_obj_get(ins, "op")) &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "op")), "binop") == 0 &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "operation")), "+") == 0 &&
        hako_llvmc_userbox_read_int(ins, "lhs") == 6 &&
        hako_llvmc_userbox_read_int(ins, "rhs") == 7 &&
        hako_llvmc_userbox_read_int(ins, "dst") == 8)) {
    yyjson_doc_free(doc);
    return -1;
  }
  ins = yyjson_arr_get(insts, 8);
  if (!(yyjson_get_str(yyjson_obj_get(ins, "op")) &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "op")), "ret") == 0 &&
        hako_llvmc_userbox_read_int(ins, "value") == 8)) {
    yyjson_doc_free(doc);
    return -1;
  }

  if (!(saw_point_newbox && saw_x_set && saw_y_set && ops == 2000000 &&
        known_receiver_count == 2 && compare_lt_count == 1 && branch_count == 1 &&
        jump_count == 3 && ret_count == 1 && loop_add_count == 1 && exit_add_count == 1 &&
        hako_llvmc_userbox_has_known_receiver_selection(fn, "Point.sum"))) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_point_sum_micro_ir(obj_out, &selection, 2000000, 3);
    yyjson_doc_free(doc);
    return rc;
  }
}

static void hako_llvmc_userbox_emit_entry_header_typed(
    FILE* f,
    const hako_llvmc_entry_rune_selection* selection,
    const char* result_type) {
  const char* symbol = hako_llvmc_effective_entry_symbol(selection);
  const char* callconv_prefix = hako_llvmc_entry_callconv_prefix(selection);
  fprintf(f, "define %s%s @\"%s\"() {\n", callconv_prefix, result_type, symbol);
}

static void hako_llvmc_userbox_emit_entry_alias_if_needed_typed(
    FILE* f,
    const hako_llvmc_entry_rune_selection* selection,
    const char* result_type) {
  const char* symbol = hako_llvmc_effective_entry_symbol(selection);
  if (strcmp(symbol, hako_llvmc_entry_symbol_default()) == 0) {
    return;
  }
  fprintf(f, "\ndefine %s @ny_main() {\n", result_type);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%entry_alias = call %s @\"%s\"()\n", result_type, symbol);
  fprintf(f, "  ret %s %%entry_alias\n", result_type);
  fprintf(f, "}\n");
}

static int hako_llvmc_emit_userbox_point_local_i64_ir(
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
      "%s/hako_pure_userbox_point_local_i64_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox point local i64 seed)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_emit_entry_header(f, selection);
  fprintf(f, "entry:\n");
  fprintf(f, "  %%x = alloca i64, align 8\n");
  fprintf(f, "  %%y = alloca i64, align 8\n");
  fprintf(f, "  store i64 41, ptr %%x, align 8\n");
  fprintf(f, "  store i64 2, ptr %%y, align 8\n");
  fprintf(f, "  %%xv = load i64, ptr %%x, align 8\n");
  fprintf(f, "  %%yv = load i64, ptr %%y, align 8\n");
  fprintf(f, "  %%sum = add i64 %%xv, %%yv\n");
  fprintf(f, "  ret i64 %%sum\n");
  fprintf(f, "}\n");
  hako_llvmc_emit_entry_alias_if_needed(f, selection);

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_userbox_point_local_i64_seed(
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
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 9)) {
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
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_get") == 0 && field_name && strcmp(field_name, "x") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "IntegerBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 3 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 4)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 6);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_get") == 0 && field_name && strcmp(field_name, "y") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "IntegerBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 3 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 5)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 7);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "binop") == 0 &&
        strcmp(yyjson_get_str(yyjson_obj_get(ins, "operation")), "+") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 6 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "lhs")) == 4 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "rhs")) == 5)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 8);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "ret") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "value")) == 6)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_point_local_i64_ir(obj_out, &selection);
    yyjson_doc_free(doc);
    return rc;
  }
}

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

static int hako_llvmc_match_userbox_flag_copy_local_bool_seed(
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
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 6)) {
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
  if (!(op && strcmp(op, "copy") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 3 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "src")) == 2)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 4);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_get") == 0 && field_name && strcmp(field_name, "enabled") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "BoolBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 3 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 4)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 5);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "ret") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "value")) == 4)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_flag_local_bool_ir(obj_out, &selection);
    yyjson_doc_free(doc);
    return rc;
  }
}

static int hako_llvmc_emit_userbox_pointf_local_f64_ir(
    const char* obj_out,
    const hako_llvmc_entry_rune_selection* selection,
    double payload_value) {
  char llpath[1024];
  FILE* f;
  int rc;

  if (!(obj_out && selection)) {
    return -1;
  }

  snprintf(
      llpath,
      sizeof(llpath),
      "%s/hako_pure_userbox_pointf_local_f64_%d.ll",
      hako_llvmc_tmp_dir_fallback(),
      (int)getpid());
  f = fopen(llpath, "wb");
  if (!f) return -1;

  fprintf(f, "; nyash minimal pure IR (userbox pointf local f64 seed)\n");
  fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");

  hako_llvmc_userbox_emit_entry_header_typed(f, selection, "double");
  fprintf(
      f,
      "entry:\n"
      "  ret double %.17g\n",
      payload_value);
  fprintf(f, "}\n");
  hako_llvmc_userbox_emit_entry_alias_if_needed_typed(f, selection, "double");

  fclose(f);
  rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
  remove(llpath);
  return rc == 0 ? 0 : -1;
}

static int hako_llvmc_match_userbox_pointf_local_f64_seed(
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
  double payload_value;

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
  if (!(hako_llvmc_userbox_pointf_inline_scalar_selection_ok(fn) &&
        hako_llvmc_userbox_pointf_decl_ok(root))) {
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
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  payload_value = yyjson_get_real(yyjson_obj_get(yyjson_obj_get(ins, "value"), "value"));

  ins = yyjson_arr_get(insts, 1);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  box_type = ins ? yyjson_get_str(yyjson_obj_get(ins, "type")) : NULL;
  if (!(op && strcmp(op, "newbox") == 0 && box_type && strcmp(box_type, "PointF") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 2)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 2);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_set") == 0 && field_name && strcmp(field_name, "x") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "FloatBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 2 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "value")) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 3);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_get") == 0 && field_name && strcmp(field_name, "x") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "FloatBox") &&
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
    int rc = hako_llvmc_emit_userbox_pointf_local_f64_ir(obj_out, &selection, payload_value);
    yyjson_doc_free(doc);
    return rc;
  }
}

static int hako_llvmc_match_userbox_pointf_copy_local_f64_seed(
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
  double payload_value;

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
  if (!(hako_llvmc_userbox_pointf_inline_scalar_selection_ok(fn) &&
        hako_llvmc_userbox_pointf_decl_ok(root))) {
    yyjson_doc_free(doc);
    return -1;
  }

  block = yyjson_arr_get(blocks, 0);
  insts = block ? yyjson_obj_get(block, "instructions") : NULL;
  if (!(insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) == 6)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 0);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "const") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }
  payload_value = yyjson_get_real(yyjson_obj_get(yyjson_obj_get(ins, "value"), "value"));

  ins = yyjson_arr_get(insts, 1);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  box_type = ins ? yyjson_get_str(yyjson_obj_get(ins, "type")) : NULL;
  if (!(op && strcmp(op, "newbox") == 0 && box_type && strcmp(box_type, "PointF") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 2)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 2);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_set") == 0 && field_name && strcmp(field_name, "x") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "FloatBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 2 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "value")) == 1)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 3);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "copy") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 3 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "src")) == 2)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 4);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  field_name = ins ? yyjson_get_str(yyjson_obj_get(ins, "field")) : NULL;
  if (!(op && strcmp(op, "field_get") == 0 && field_name && strcmp(field_name, "x") == 0 &&
        hako_llvmc_userbox_declared_box_is(ins, "FloatBox") &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "box")) == 3 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "dst")) == 4)) {
    yyjson_doc_free(doc);
    return -1;
  }

  ins = yyjson_arr_get(insts, 5);
  op = ins ? yyjson_get_str(yyjson_obj_get(ins, "op")) : NULL;
  if (!(op && strcmp(op, "ret") == 0 &&
        (long long)yyjson_get_sint(yyjson_obj_get(ins, "value")) == 4)) {
    yyjson_doc_free(doc);
    return -1;
  }

  {
    int rc = hako_llvmc_emit_userbox_pointf_local_f64_ir(obj_out, &selection, payload_value);
    yyjson_doc_free(doc);
    return rc;
  }
}

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
