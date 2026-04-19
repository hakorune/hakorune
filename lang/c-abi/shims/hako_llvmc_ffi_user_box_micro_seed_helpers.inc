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

static int hako_llvmc_userbox_reg_matches_aliases(
    long long reg,
    const long long* aliases,
    size_t alias_n) {
  size_t bi;

  if (!(reg > 0 && aliases)) {
    return 0;
  }

  for (bi = 0; bi < alias_n; bi++) {
    if (aliases[bi] == reg) {
      return 1;
    }
  }

  return 0;
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
