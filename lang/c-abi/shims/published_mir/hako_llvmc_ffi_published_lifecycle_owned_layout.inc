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
        !hako_physical_layout_has_object(layouts, child) ||
        yyjson_get_uint(yyjson_obj_get(yyjson_arr_get(yyjson_obj_get(layout, "fields"), ordinal),
            "storage_kind")) != HAKO_LLVMC_LIFECYCLE_STORAGE_I64) return 0;
    for (size_t j = 0; j < yyjson_arr_size(arrays); j++)
      if (yyjson_get_uint(yyjson_arr_get(arrays, j)) == ordinal) return 0;
    previous = ordinal;
  }
  return 1;
}

/* A named SysV64 record, later independently compared with LLVM TargetData. */
static int hako_physical_direct_layout(yyjson_val* layout) {
  yyjson_val* plan = yyjson_obj_get(layout, "placement");
  static const char* const keys[] = {"kind", "representation", "target_layout", "descriptor_object_id",
      "size", "align", "parent_offset", "cursor_offset", "field_offsets", "end"};
  uint32_t object, alignment; uint64_t size;
  yyjson_val* offsets = yyjson_obj_get(plan, "field_offsets");
  yyjson_val* end = yyjson_obj_get(plan, "end");
  size_t count = yyjson_arr_size(yyjson_obj_get(layout, "fields"));
  if (!plan) return 1;
  if (!hako_physical_exact_keys(plan, keys, 10) ||
      !hako_physical_string(yyjson_obj_get(plan, "kind"), "direct_v0") ||
      !hako_physical_string(yyjson_obj_get(plan, "representation"), "unique_pointer_word_v0") ||
      !hako_physical_string(yyjson_obj_get(plan, "target_layout"), "x86_64_sysv64_v0") ||
      !hako_physical_u32(yyjson_obj_get(plan, "descriptor_object_id"), &object) ||
      object != yyjson_get_uint(yyjson_obj_get(layout, "object_id")) ||
      !hako_physical_u64(yyjson_obj_get(plan, "size"), &size) || count > (UINT64_MAX - 16) / 8 ||
      size != 16 + 8 * (uint64_t)count ||
      !hako_physical_u32(yyjson_obj_get(plan, "align"), &alignment) || alignment != 8 ||
      !yyjson_is_uint(yyjson_obj_get(plan, "parent_offset")) || yyjson_get_uint(yyjson_obj_get(plan, "parent_offset")) != 0 ||
      !yyjson_is_uint(yyjson_obj_get(plan, "cursor_offset")) || yyjson_get_uint(yyjson_obj_get(plan, "cursor_offset")) != 8 ||
      !yyjson_is_arr(offsets) || yyjson_arr_size(offsets) != count ||
      !yyjson_is_arr(end) || yyjson_arr_size(end) != 1 + yyjson_arr_size(yyjson_obj_get(layout, "owned_object_residences")) ||
      yyjson_arr_size(yyjson_obj_get(layout, "owned_residences"))) return 0;
  size_t children = yyjson_arr_size(yyjson_obj_get(layout, "owned_object_residences"));
  for (size_t i = 0; i < children; i++) {
    static const char* const child_keys[] = {"kind", "field_ordinal", "child_object_id"};
    yyjson_val* child = yyjson_arr_get(yyjson_obj_get(layout, "owned_object_residences"), children - i - 1);
    yyjson_val* step = yyjson_arr_get(end, i);
    if (!hako_physical_exact_keys(step, child_keys, 3) ||
        !hako_physical_string(yyjson_obj_get(step, "kind"), "object_field") ||
        !yyjson_equals(yyjson_obj_get(step, "field_ordinal"), yyjson_obj_get(child, "field_ordinal")) ||
        !yyjson_equals(yyjson_obj_get(step, "child_object_id"), yyjson_obj_get(child, "child_object_id"))) return 0;
  }
  if (!hako_physical_string(yyjson_obj_get(yyjson_arr_get(end, children), "kind"), "release_outer") ||
      yyjson_obj_size(yyjson_arr_get(end, children)) != 1) return 0;
  for (size_t i = 0; i < count; i++) {
    uint64_t offset;
    if (!hako_physical_u64(yyjson_arr_get(offsets, i), &offset) || offset != 16 + 8 * (uint64_t)i ||
        yyjson_get_uint(yyjson_obj_get(yyjson_arr_get(yyjson_obj_get(layout, "fields"), i), "storage_kind")) != HAKO_LLVMC_LIFECYCLE_STORAGE_I64) return 0;
  }
  return 1;
}

/* A direct pointer never crosses a runtime-handle owning edge. B admits
 * a DAG of declaration identities, not a runtime-depth special case. */
static int hako_physical_direct_closure(yyjson_val* layouts) {
  size_t direct = 0;
  for (size_t i = 0; i < yyjson_arr_size(layouts); i++)
    direct += yyjson_obj_get(yyjson_arr_get(layouts, i), "placement") != NULL;
  if (!direct) return 1; /* Keep the unselected profile allocation-free. */
  size_t count = yyjson_arr_size(layouts), completed = 0;
  unsigned char* done = calloc(count ? count : 1, 1);
  if (!done) return 0;
  for (size_t pass = 0; pass <= count; pass++) {
    size_t before = completed;
    for (size_t i = 0; i < count; i++) {
      if (done[i]) continue;
      yyjson_val* layout = yyjson_arr_get(layouts, i);
      int direct = yyjson_obj_get(layout, "placement") != NULL, ready = 1;
      yyjson_val* children = yyjson_obj_get(layout, "owned_object_residences");
      for (size_t ci = 0; ci < yyjson_arr_size(children); ci++) {
        uint32_t child = (uint32_t)yyjson_get_uint(yyjson_obj_get(yyjson_arr_get(children, ci), "child_object_id"));
        size_t j = 0;
        while (j < count && yyjson_get_uint(yyjson_obj_get(yyjson_arr_get(layouts, j), "object_id")) != child) j++;
        if (j == count || direct != (yyjson_obj_get(yyjson_arr_get(layouts, j), "placement") != NULL)) {
          free(done); return 0;
        }
        if (direct && !done[j]) ready = 0;
      }
      if (ready) { done[i] = 1; completed++; }
    }
    if (completed == count) { free(done); return 1; }
    if (completed == before) break;
  }
  free(done); return 0;
}

static int hako_physical_validate_layouts(yyjson_val* layouts) {
  static const char* const layout_keys[] = {"object_id", "runtime_type_id", "field_count", "fields", "owned_residences"};
  static const char* const owned_layout_keys[] = {"object_id", "runtime_type_id", "field_count", "fields", "owned_residences", "owned_object_residences"};
  static const char* const direct_keys[] = {"object_id", "runtime_type_id", "field_count", "fields", "owned_residences", "owned_object_residences", "placement"};
  static const char* const field_keys[] = {"declaration_ordinal", "runtime_slot", "storage_kind"};
  size_t i, j;
  if (!(layouts && yyjson_is_arr(layouts))) return 0;
  for (i = 0; i < yyjson_arr_size(layouts); i++) {
    yyjson_val* layout = yyjson_arr_get(layouts, i); yyjson_val* fields; yyjson_val* owned; uint32_t object, count;
    if (!(hako_physical_exact_keys(layout, layout_keys, 5) || hako_physical_exact_keys(layout, owned_layout_keys, 6) || hako_physical_exact_keys(layout, direct_keys, 7)) || !hako_physical_u32(yyjson_obj_get(layout, "object_id"), &object) || !hako_physical_u32(yyjson_obj_get(layout, "runtime_type_id"), NULL) || !hako_physical_u32(yyjson_obj_get(layout, "field_count"), &count) || !(fields = yyjson_obj_get(layout, "fields")) || !yyjson_is_arr(fields) || yyjson_arr_size(fields) != count || !(owned = yyjson_obj_get(layout, "owned_residences")) || !yyjson_is_arr(owned)) return 0;
    if (!hako_physical_direct_layout(layout)) return 0;
    for (j = 0; j < yyjson_arr_size(fields); j++) { uint32_t ordinal, storage; yyjson_val* field = yyjson_arr_get(fields, j);
      if (!hako_physical_exact_keys(field, field_keys, 3) || !hako_physical_u32(yyjson_obj_get(field, "declaration_ordinal"), &ordinal) || ordinal != j || !hako_physical_u32(yyjson_obj_get(field, "runtime_slot"), &storage) || storage != ordinal || !hako_physical_u32(yyjson_obj_get(field, "storage_kind"), &storage) || (storage != HAKO_LLVMC_LIFECYCLE_STORAGE_I64 && storage != HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC)) return 0; }
    { uint32_t previous = 0;
      for (j = 0; j < yyjson_arr_size(owned); j++) { uint32_t ordinal;
        /* Owned residences are declared `ArrayBox` field ordinals in
           declaration order: unique, ascending, resolving in this
           layout's own field rows — the release consumer walks them
           newest-first. */
        if (!hako_physical_u32(yyjson_arr_get(owned, j), &ordinal) || ordinal >= count || (j && ordinal <= previous) ||
            hako_physical_field_storage(layouts, object, ordinal) != HAKO_LLVMC_LIFECYCLE_STORAGE_I64) return 0;
        previous = ordinal; } }
    if (!hako_physical_owned_object_layout(layout, layouts, owned, count)) return 0;
    for (j = i + 1; j < yyjson_arr_size(layouts); j++) { uint32_t other; if (!hako_physical_u32(yyjson_obj_get(yyjson_arr_get(layouts, j), "object_id"), &other) || object == other) return 0; }
  }
  return hako_physical_direct_closure(layouts);
}
