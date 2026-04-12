static yyjson_val* hako_llvmc_sum_placement_effect_routes(yyjson_val* fn) {
  yyjson_val* metadata;
  yyjson_val* routes;

  if (!(fn && yyjson_is_obj(fn))) {
    return NULL;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  routes = metadata ? yyjson_obj_get(metadata, "placement_effect_routes") : NULL;
  if (!(routes && yyjson_is_arr(routes))) {
    return NULL;
  }

  return routes;
}

static const char* hako_llvmc_sum_local_aggregate_manifest_row_for_surface(
    const char* surface) {
  if (!surface) {
    return NULL;
  }
  if (strcmp(surface, "variant_make") == 0) {
    return "variant_make.local_aggregate";
  }
  if (strcmp(surface, "variant_tag") == 0) {
    return "variant_tag.local_aggregate";
  }
  if (strcmp(surface, "variant_project") == 0) {
    return "variant_project.local_aggregate";
  }
  return NULL;
}

static int hako_llvmc_sum_route_matches(
    yyjson_val* row,
    const char* source_expected,
    const char* decision_expected,
    const char* subject_expected,
    long long value_expected,
    int require_value,
    const char* detail_expected,
    long long source_value_expected,
    int require_source_value) {
  yyjson_val* value_val;
  yyjson_val* source_value_val;
  const char* source;
  const char* decision;
  const char* subject;
  const char* detail;
  long long value;
  long long source_value;

  if (!(row && yyjson_is_obj(row) && source_expected && decision_expected && subject_expected &&
        detail_expected)) {
    return 0;
  }

  source = yyjson_get_str(yyjson_obj_get(row, "source"));
  decision = yyjson_get_str(yyjson_obj_get(row, "decision"));
  subject = yyjson_get_str(yyjson_obj_get(row, "subject"));
  detail = yyjson_get_str(yyjson_obj_get(row, "detail"));
  value_val = yyjson_obj_get(row, "value");
  if (!(source && decision && subject && detail)) {
    return 0;
  }
  if (strcmp(source, source_expected) != 0 ||
      strcmp(decision, decision_expected) != 0 ||
      strcmp(subject, subject_expected) != 0 ||
      strcmp(detail, detail_expected) != 0) {
    return 0;
  }

  if (require_value) {
    if (!(value_val && !yyjson_is_null(value_val))) {
      return 0;
    }
    value = (long long)yyjson_get_sint(value_val);
    if (value != value_expected) {
      return 0;
    }
  }

  if (!require_source_value) {
    return 1;
  }

  source_value_val = yyjson_obj_get(row, "source_value");
  if (!(source_value_val && !yyjson_is_null(source_value_val))) {
    return 0;
  }
  source_value = (long long)yyjson_get_sint(source_value_val);
  return source_value == source_value_expected;
}

static int hako_llvmc_sum_has_thin_internal_selection(
    yyjson_val* fn,
    const char* surface,
    const char* subject,
    const char* manifest_row_expected) {
  yyjson_val* metadata;
  yyjson_val* routes;
  yyjson_val* selections;
  size_t i;

  if (!(fn && yyjson_is_obj(fn) && surface && subject && manifest_row_expected)) {
    return 0;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  routes = hako_llvmc_sum_placement_effect_routes(fn);
  if (routes) {
    for (i = 0; i < yyjson_arr_size(routes); i++) {
      yyjson_val* row = yyjson_arr_get(routes, i);
      if (hako_llvmc_sum_route_matches(
              row,
              "thin_entry",
              "thin_internal_entry",
              subject,
              0,
              0,
              manifest_row_expected,
              0,
              0)) {
        return 1;
      }
    }
  }

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
  yyjson_val* routes;
  yyjson_val* facts;
  const char* manifest_row_expected;
  size_t i;

  if (!(fn && yyjson_is_obj(fn) && surface && subject)) {
    return 0;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  routes = hako_llvmc_sum_placement_effect_routes(fn);
  manifest_row_expected = hako_llvmc_sum_local_aggregate_manifest_row_for_surface(surface);
  if (routes && manifest_row_expected) {
    for (i = 0; i < yyjson_arr_size(routes); i++) {
      yyjson_val* row = yyjson_arr_get(routes, i);
      if (hako_llvmc_sum_route_matches(
              row,
              "sum_placement",
              "local_aggregate",
              subject,
              value,
              1,
              manifest_row_expected,
              0,
              0)) {
        return 1;
      }
    }
  }

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
  yyjson_val* routes;
  yyjson_val* selections;
  const char* manifest_row_expected;
  size_t i;

  if (!(fn && yyjson_is_obj(fn) && surface && subject)) {
    return 0;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  routes = hako_llvmc_sum_placement_effect_routes(fn);
  manifest_row_expected = hako_llvmc_sum_local_aggregate_manifest_row_for_surface(surface);
  if (routes && manifest_row_expected) {
    for (i = 0; i < yyjson_arr_size(routes); i++) {
      yyjson_val* row = yyjson_arr_get(routes, i);
      if (hako_llvmc_sum_route_matches(
              row,
              "sum_placement",
              "local_aggregate",
              subject,
              value,
              1,
              manifest_row_expected,
              source_sum,
              require_source_sum)) {
        return 1;
      }
    }
  }

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
  yyjson_val* routes;
  yyjson_val* layouts;
  char detail_expected[128];
  size_t i;

  if (!(fn && yyjson_is_obj(fn) && surface && subject && layout_expected)) {
    return 0;
  }

  metadata = yyjson_obj_get(fn, "metadata");
  routes = hako_llvmc_sum_placement_effect_routes(fn);
  if (routes) {
    int detail_len = snprintf(
        detail_expected, (int)sizeof(detail_expected), "sum_local_layout(%s)", layout_expected);
    if (detail_len > 0 && detail_len < (int)sizeof(detail_expected)) {
      for (i = 0; i < yyjson_arr_size(routes); i++) {
        yyjson_val* row = yyjson_arr_get(routes, i);
        if (hako_llvmc_sum_route_matches(
                row,
                "agg_local_scalarization",
                "local_aggregate",
                subject,
                value,
                1,
                detail_expected,
                0,
                0)) {
          return 1;
        }
      }
    }
  }

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
