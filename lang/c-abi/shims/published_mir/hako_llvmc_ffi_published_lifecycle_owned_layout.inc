/* Optional source-issued object residence tuples; absence grants no ownership. */
static int hako_physical_owned_object_layout(yyjson_val* layout, yyjson_val* layouts,
    yyjson_val* arrays, uint32_t count) {
  static const char* const keys[] = {"field_ordinal", "child_object_id"};
  yyjson_val* rows = yyjson_obj_get(layout, "owned_object_residences");
  uint32_t previous = 0;
  if (!rows) return 1;
  if (!yyjson_is_arr(rows)) return 0;
  for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
    yyjson_val* row = yyjson_arr_get(rows, i);
    uint32_t ordinal, child;
    if (!hako_physical_exact_keys(row, keys, 2) ||
        !hako_physical_u32(yyjson_obj_get(row, "field_ordinal"), &ordinal) ||
        !hako_physical_u32(yyjson_obj_get(row, "child_object_id"), &child) ||
        ordinal >= count || (i && ordinal <= previous) ||
        !hako_physical_layout_has_object(layouts, child)) return 0;
    for (size_t j = 0; j < yyjson_arr_size(arrays); j++)
      if (yyjson_get_uint(yyjson_arr_get(arrays, j)) == ordinal) return 0;
    previous = ordinal;
  }
  return 1;
}
