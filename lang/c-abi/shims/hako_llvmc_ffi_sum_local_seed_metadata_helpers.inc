static int hako_llvmc_sum_has_thin_internal_selection(
    yyjson_val* fn,
    const char* surface,
    const char* subject,
    const char* manifest_row_expected) {
  yyjson_val* metadata;
  yyjson_val* selections;
  size_t i;

  if (!(fn && yyjson_is_obj(fn) && surface && subject && manifest_row_expected)) {
    return 0;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  selections = metadata ? yyjson_obj_get(metadata, "thin_entry_selections") : NULL;
  if (!(selections && yyjson_is_arr(selections))) {
    return 0;
  }

  for (i = 0; i < yyjson_arr_size(selections); i++) {
    yyjson_val* row = yyjson_arr_get(selections, i);
    const char* row_surface = row ? yyjson_get_str(yyjson_obj_get(row, "surface")) : NULL;
    const char* row_subject = row ? yyjson_get_str(yyjson_obj_get(row, "subject")) : NULL;
    const char* manifest_row = row ? yyjson_get_str(yyjson_obj_get(row, "manifest_row")) : NULL;
    const char* selected_entry =
        row ? yyjson_get_str(yyjson_obj_get(row, "selected_entry")) : NULL;
    if (!(row_surface && row_subject && manifest_row && selected_entry)) {
      continue;
    }
    if (strcmp(row_surface, surface) == 0 &&
        strcmp(row_subject, subject) == 0 &&
        strcmp(manifest_row, manifest_row_expected) == 0 &&
        strcmp(selected_entry, "thin_internal_entry") == 0) {
      return 1;
    }
  }

  return 0;
}

static int hako_llvmc_sum_has_local_aggregate_fact(
    yyjson_val* fn,
    const char* surface,
    const char* subject,
    long long value) {
  yyjson_val* metadata;
  yyjson_val* facts;
  size_t i;

  if (!(fn && yyjson_is_obj(fn) && surface && subject)) {
    return 0;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  facts = metadata ? yyjson_obj_get(metadata, "sum_placement_facts") : NULL;
  if (!(facts && yyjson_is_arr(facts))) {
    return 0;
  }

  for (i = 0; i < yyjson_arr_size(facts); i++) {
    yyjson_val* row = yyjson_arr_get(facts, i);
    yyjson_val* barriers = row ? yyjson_obj_get(row, "barriers") : NULL;
    const char* row_surface = row ? yyjson_get_str(yyjson_obj_get(row, "surface")) : NULL;
    const char* row_subject = row ? yyjson_get_str(yyjson_obj_get(row, "subject")) : NULL;
    const char* state = row ? yyjson_get_str(yyjson_obj_get(row, "state")) : NULL;
    long long row_value = row ? (long long)yyjson_get_sint(yyjson_obj_get(row, "value")) : 0;
    if (!(row_surface && row_subject && state && barriers && yyjson_is_arr(barriers))) {
      continue;
    }
    if (strcmp(row_surface, surface) == 0 &&
        strcmp(row_subject, subject) == 0 &&
        strcmp(state, "local_agg_candidate") == 0 &&
        row_value == value &&
        yyjson_arr_size(barriers) == 0) {
      return 1;
    }
  }

  return 0;
}

static int hako_llvmc_sum_has_local_aggregate_selection(
    yyjson_val* fn,
    const char* surface,
    const char* subject,
    long long value,
    long long source_sum,
    int require_source_sum) {
  yyjson_val* metadata;
  yyjson_val* selections;
  size_t i;

  if (!(fn && yyjson_is_obj(fn) && surface && subject)) {
    return 0;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  selections = metadata ? yyjson_obj_get(metadata, "sum_placement_selections") : NULL;
  if (!(selections && yyjson_is_arr(selections))) {
    return 0;
  }

  for (i = 0; i < yyjson_arr_size(selections); i++) {
    yyjson_val* row = yyjson_arr_get(selections, i);
    yyjson_val* source_sum_val = row ? yyjson_obj_get(row, "source_sum") : NULL;
    const char* row_surface = row ? yyjson_get_str(yyjson_obj_get(row, "surface")) : NULL;
    const char* row_subject = row ? yyjson_get_str(yyjson_obj_get(row, "subject")) : NULL;
    const char* selected_path =
        row ? yyjson_get_str(yyjson_obj_get(row, "selected_path")) : NULL;
    long long row_value = row ? (long long)yyjson_get_sint(yyjson_obj_get(row, "value")) : 0;
    long long row_source_sum =
        source_sum_val && !yyjson_is_null(source_sum_val)
            ? (long long)yyjson_get_sint(source_sum_val)
            : 0;
    int row_has_source_sum = source_sum_val && !yyjson_is_null(source_sum_val);
    if (!(row_surface && row_subject && selected_path)) {
      continue;
    }
    if (strcmp(row_surface, surface) != 0 ||
        strcmp(row_subject, subject) != 0 ||
        strcmp(selected_path, "local_aggregate") != 0 ||
        row_value != value) {
      continue;
    }
    if (require_source_sum && (!row_has_source_sum || row_source_sum != source_sum)) {
      continue;
    }
    return 1;
  }

  return 0;
}

static int hako_llvmc_sum_has_layout(
    yyjson_val* fn,
    const char* surface,
    const char* subject,
    long long value,
    const char* layout_expected) {
  yyjson_val* metadata;
  yyjson_val* layouts;
  size_t i;

  if (!(fn && yyjson_is_obj(fn) && surface && subject && layout_expected)) {
    return 0;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  layouts = metadata ? yyjson_obj_get(metadata, "sum_placement_layouts") : NULL;
  if (!(layouts && yyjson_is_arr(layouts))) {
    return 0;
  }

  for (i = 0; i < yyjson_arr_size(layouts); i++) {
    yyjson_val* row = yyjson_arr_get(layouts, i);
    const char* row_surface = row ? yyjson_get_str(yyjson_obj_get(row, "surface")) : NULL;
    const char* row_subject = row ? yyjson_get_str(yyjson_obj_get(row, "subject")) : NULL;
    const char* layout = row ? yyjson_get_str(yyjson_obj_get(row, "layout")) : NULL;
    long long row_value = row ? (long long)yyjson_get_sint(yyjson_obj_get(row, "value")) : 0;
    if (!(row_surface && row_subject && layout)) {
      continue;
    }
    if (strcmp(row_surface, surface) == 0 &&
        strcmp(row_subject, subject) == 0 &&
        strcmp(layout, layout_expected) == 0 &&
        row_value == value) {
      return 1;
    }
  }

  return 0;
}
