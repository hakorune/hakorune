/* Normal-committed Birth field markers reuse store block origin slots. */
enum { LV4_BIRTH_FIELD = 258 }; /* Map tags are 2 + origin, origin < 256. */
_Static_assert(LV4_BIRTH_FIELD > LV4_TRANSFER_BASE + 255, "Birth/Map marker domains overlap");

static int lv4_birth_owned(const hako_lv4_input* in, yyjson_val* op, int object) {
  yyjson_val* layout = lv4_layout(in, lv4_u(op, "object_id"));
  yyjson_val* rows = yyjson_obj_get(layout,
      object ? "owned_object_residences" : "owned_residences");
  for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
    yyjson_val* row = yyjson_arr_get(rows, i);
    if (object ? (lv4_u(row, "field_ordinal") == lv4_u(op, "field_ordinal") &&
        lv4_u(row, "child_object_id") == lv4_u(op, "child_object_id")) :
        yyjson_get_uint(row) == lv4_u(op, "field_ordinal")) return 1;
  }
  return 0;
}

/* "Reserved" names any metadata-declared owned slot — array or object —
 * in any caller role; Birth commit/discharge is a separate gate. */
static int lv4_birth_reserved(const hako_lv4_input* in, yyjson_val* op) {
  yyjson_val* layout = lv4_layout(in, lv4_u(op, "object_id"));
  for (int object = 0; object <= 1; object++) {
    yyjson_val* rows = yyjson_obj_get(layout,
        object ? "owned_object_residences" : "owned_residences");
    for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
      yyjson_val* row = yyjson_arr_get(rows, i);
      if ((object ? lv4_u(row, "field_ordinal") : yyjson_get_uint(row)) ==
          lv4_u(op, "field_ordinal")) return 1;
    }
  }
  return 0;
}

static int lv4_birth_array_value(hako_lv4_function_index* fn, hako_lv4_value* value) {
  return value && value->origin >= 0 && (size_t)value->origin < fn->block_count &&
      lv4_is(yyjson_obj_get(lv4_term(yyjson_arr_get(hako_physical_blocks(fn->function),
          value->origin)), "operation"), "kind", "array_new");
}

static int lv4_birth_base(hako_lv4_function_index* fn, yyjson_val* op) {
  hako_lv4_value* base = lv4_index_value(fn, lv4_u(op, "base"));
  hako_lv4_value* root = lv4_borrowed_copy_root(fn, base);
  return base && root && base->kind == LV4_HANDLE && base->origin == -2 &&
      fn->receiver_object_set && root->id == lv4_u(fn->function, "receiver") &&
      root->object_id == fn->receiver_object &&
      lv4_u(op, "object_id") == fn->receiver_object;
}

static yyjson_val* lv4_birth_store(hako_lv4_function_index* fn, size_t origin) {
  return yyjson_obj_get(lv4_term(yyjson_arr_get(
      hako_physical_blocks(fn->function), origin)), "operation");
}

static int lv4_birth_same_slot(yyjson_val* a, yyjson_val* b, int object) {
  return lv4_u(a, "object_id") == lv4_u(b, "object_id") &&
      lv4_u(a, "field_ordinal") == lv4_u(b, "field_ordinal") &&
      (!object || lv4_u(a, "child_object_id") == lv4_u(b, "child_object_id"));
}

static int lv4_birth_commit(const hako_lv4_input* in, hako_lv4_function_index* fn,
    yyjson_val* op, uint16_t* normal, size_t origin, int object) {
  if (!lv4_birth_base(fn, op) || !lv4_birth_owned(in, op, object) || normal[origin]) return 0;
  for (size_t i = 0; i < fn->block_count; i++)
    if (normal[i] == LV4_BIRTH_FIELD &&
        lv4_birth_same_slot(lv4_birth_store(fn, i), op, 0)) return 0;
  normal[origin] = LV4_BIRTH_FIELD;
  return 1;
}

static int lv4_birth_discharge(const hako_lv4_input* in, hako_lv4_function_index* fn,
    yyjson_val* op, uint16_t* state, uint16_t* normal, int faulted, int object) {
  if (!faulted || !lv4_birth_base(fn, op) || !lv4_birth_owned(in, op, object)) return 0;
  size_t found = fn->block_count;
  for (size_t i = 0; i < fn->block_count; i++) {
    yyjson_val* store = lv4_birth_store(fn, i);
    if (state[i] != LV4_BIRTH_FIELD ||
        !lv4_is(store, "kind", object ? "object_field_set" : "field_set") ||
        !lv4_birth_base(fn, store) || !lv4_birth_same_slot(store, op, object)) continue;
    if (found != fn->block_count) return 0;
    found = i;
  }
  if (found == fn->block_count) return 0;
  normal[found] = state[found] = 0;
  return 1;
}

static int lv4_birth_finish(const hako_lv4_input* in, hako_lv4_function_index* fn,
    uint16_t* state) {
  yyjson_val* layout = lv4_layout(in, fn->receiver_object);
  size_t expected = 0, actual = 0;
  for (int object = 0; object <= 1; object++) {
    yyjson_val* rows = yyjson_obj_get(layout,
        object ? "owned_object_residences" : "owned_residences");
    for (size_t j = 0; j < yyjson_arr_size(rows); j++) {
      yyjson_val* row = yyjson_arr_get(rows, j);
      uint32_t ordinal = object ? lv4_u(row, "field_ordinal") : yyjson_get_uint(row);
      size_t matched = 0;
      for (size_t i = 0; i < fn->block_count; i++) {
        yyjson_val* store = lv4_birth_store(fn, i);
        if (state[i] == LV4_BIRTH_FIELD &&
            lv4_is(store, "kind", object ? "object_field_set" : "field_set") &&
            lv4_birth_base(fn, store) && lv4_birth_owned(in, store, object) &&
            lv4_u(store, "field_ordinal") == ordinal) matched++;
      }
      if (matched != 1) return 0;
      expected++;
    }
  }
  for (size_t i = 0; i < fn->block_count; i++)
    if (state[i] == LV4_BIRTH_FIELD) { actual++; state[i] = 0; }
  return actual == expected;
}
