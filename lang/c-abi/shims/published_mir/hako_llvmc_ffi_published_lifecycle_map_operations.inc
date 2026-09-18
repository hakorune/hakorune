/* Checked Map operation shape validation. The V4 flow owns lifetime and
 * result-kind checks; this file only validates exact physical JSON fields. */
static int hako_physical_map_kind(const char* kind) {
  return kind && (!strcmp(kind, "map_new") || !strcmp(kind, "map_prepare_key") ||
      (!strcmp(kind, "map_install_indexed") || !strcmp(kind, "map_install_value")) ||
      !strcmp(kind, "map_install_text") || !strcmp(kind, "map_install_empty_array") ||
      !strcmp(kind, "map_checked_get") || !strcmp(kind, "map_array_index_map") ||
      !strcmp(kind, "map_get_text") || !strcmp(kind, "map_end_outcome") ||
      !strcmp(kind, "map_end"));
}

static int hako_physical_map_operation(yyjson_val* function, yyjson_val* layouts,
    yyjson_val* op, uint32_t block, uint32_t index) {
  static const char* const new_keys[] = {"kind", "site"};
  static const char* const key_keys[] = {"kind", "site", "utf8"};
  static const char* const install_keys[] = {"kind", "site", "map", "key", "object_id", "value"};
  static const char* const value_keys[] = {"kind", "site", "map", "key", "value_kind", "value"};
  static const char* const text_keys[] = {"kind", "site", "map", "key", "utf8"};
  static const char* const empty_array_keys[] = {"kind", "site", "map", "key"};
  static const char* const end_keys[] = {"kind", "site", "map"};
  static const char* const get_keys[] = {"kind", "site", "map", "utf8"};
  static const char* const array_index_keys[] = {"kind", "site", "map", "utf8", "index"};
  static const char* const outcome_keys[] = {"kind", "site", "outcome"};
  uint32_t value, object;
  if (!hako_physical_u64(yyjson_obj_get(op, "site"), NULL)) return 0;
  if (hako_physical_string(yyjson_obj_get(op, "kind"), "map_new"))
    return hako_physical_exact_keys(op, new_keys, 2);
  if (hako_physical_string(yyjson_obj_get(op, "kind"), "map_prepare_key"))
    return hako_physical_exact_keys(op, key_keys, 3) && yyjson_is_str(yyjson_obj_get(op, "utf8"));
  if (hako_physical_string(yyjson_obj_get(op, "kind"), "map_end_outcome"))
    return hako_physical_exact_keys(op, outcome_keys, 3) &&
        hako_physical_u32(yyjson_obj_get(op, "outcome"), &value) &&
        hako_physical_value_available_at(function, value, block, index);
  if (!hako_physical_u32(yyjson_obj_get(op, "map"), &value) ||
      !hako_physical_value_available_at(function, value, block, index)) return 0;
  if (hako_physical_string(yyjson_obj_get(op, "kind"), "map_end"))
    return hako_physical_exact_keys(op, end_keys, 3);
  if (hako_physical_string(yyjson_obj_get(op, "kind"), "map_checked_get") ||
      hako_physical_string(yyjson_obj_get(op, "kind"), "map_get_text"))
    return hako_physical_exact_keys(op, get_keys, 4) &&
        yyjson_is_str(yyjson_obj_get(op, "utf8"));
  if (hako_physical_string(yyjson_obj_get(op, "kind"), "map_array_index_map"))
    return hako_physical_exact_keys(op, array_index_keys, 5) &&
        yyjson_is_str(yyjson_obj_get(op, "utf8")) &&
        hako_physical_i64(yyjson_obj_get(op, "index"));
  if (hako_physical_string(yyjson_obj_get(op, "kind"), "map_install_value"))
    return hako_physical_exact_keys(op, value_keys, 6) &&
        hako_physical_u32(yyjson_obj_get(op, "value_kind"), &object) &&
        (object == NYRT_MAP_VALUE_I64 || object == NYRT_MAP_VALUE_BOOL ||
         object == NYRT_MAP_VALUE_BORROWED_HANDLE) &&
        hako_physical_u32(yyjson_obj_get(op, "key"), &value) &&
        hako_physical_value_available_at(function, value, block, index) &&
        hako_physical_u32(yyjson_obj_get(op, "value"), &value) &&
        hako_physical_value_available_at(function, value, block, index);
  if (hako_physical_string(yyjson_obj_get(op, "kind"), "map_install_text"))
    return hako_physical_exact_keys(op, text_keys, 5) &&
        hako_physical_u32(yyjson_obj_get(op, "key"), &value) &&
        hako_physical_value_available_at(function, value, block, index) &&
        yyjson_is_str(yyjson_obj_get(op, "utf8"));
  if (hako_physical_string(yyjson_obj_get(op, "kind"), "map_install_empty_array"))
    return hako_physical_exact_keys(op, empty_array_keys, 4) &&
        hako_physical_u32(yyjson_obj_get(op, "key"), &value) &&
        hako_physical_value_available_at(function, value, block, index);
  return hako_physical_exact_keys(op, install_keys, 6) &&
      hako_physical_u32(yyjson_obj_get(op, "key"), &value) &&
      hako_physical_value_available_at(function, value, block, index) &&
      hako_physical_u32(yyjson_obj_get(op, "value"), &value) &&
      hako_physical_value_available_at(function, value, block, index) &&
      hako_physical_u32(yyjson_obj_get(op, "object_id"), &object) &&
      hako_physical_layout_has_object(layouts, object);
}
