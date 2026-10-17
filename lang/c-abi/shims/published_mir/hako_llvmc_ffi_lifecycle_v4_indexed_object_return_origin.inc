/* Exact source-issued Return correspondence in the existing result index.
 * No search can replace an omitted producer, Invoke or original Copy row. */
static yyjson_val* lv4_result_coordinate(hako_lv4_function_index* fn,
    yyjson_val* coordinate, uint32_t* block_id) {
  static const char* const keys[] = {"block", "index"};
  uint32_t block, index, observed;
  if (!hako_physical_exact_keys(coordinate, keys, 2) ||
      !hako_physical_u32(yyjson_obj_get(coordinate, "block"), &block) ||
      !hako_physical_u32(yyjson_obj_get(coordinate, "index"), &index)) return NULL;
  hako_lv4_block* selected = lv4_index_block(fn, block);
  if (!selected) return NULL;
  yyjson_val* instructions = yyjson_obj_get(selected->block, "instructions");
  size_t count = yyjson_arr_size(instructions);
  yyjson_val* row = index < count ? yyjson_arr_get(instructions, index)
      : index == count ? yyjson_obj_get(selected->block, "terminator") : NULL;
  if (!row || !hako_physical_u32(yyjson_obj_get(row, "index"), &observed) || observed != index)
    return NULL;
  if (block_id) *block_id = block;
  return yyjson_obj_get(row, "instruction");
}

/* One exact ordered chain owner for owned and non-owning Returns alike. */
static int lv4_result_copy_chain(hako_lv4_function_index* fn, yyjson_val* origin,
    uint32_t value, uint32_t result_value) {
  yyjson_val* copies = yyjson_obj_get(origin, "copies");
  if (!yyjson_is_arr(copies) || yyjson_arr_size(copies) > fn->value_count) return 0;
  for (size_t i = 0; i < yyjson_arr_size(copies); i++) {
    yyjson_val* copy = lv4_result_coordinate(fn, yyjson_arr_get(copies, i), NULL);
    uint32_t next;
    if (!copy || !lv4_is(copy, "op", "copy") || lv4_u(copy, "src") != value ||
        !hako_physical_u32(yyjson_obj_get(copy, "dst"), &next) || next == value) return 0;
    for (size_t prior = 0; prior < i; prior++)
      if (lv4_result_coordinate(fn, yyjson_arr_get(copies, prior), NULL) == copy) return 0;
    value = next;
  }
  return value == result_value;
}

static int lv4_acquired_return_origin(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* returned, int nullable) {
  static const char* const null_keys[] = {"kind", "producer", "copies"};
  static const char* const call_keys[] = {"kind", "producer", "invoke", "copies"};
  static const char* const field_keys[] = {"kind", "producer", "invoke", "copies", "field_supports"};
  yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
  int null = lv4_is(origin, "kind", "null");
  int fresh = lv4_is(origin, "kind", "fresh");
  int call = lv4_is(origin, "kind", "owned_call");
  if ((!null && !fresh && !call) || (null && !nullable) ||
      !hako_physical_exact_keys(origin, null ? null_keys :
          yyjson_obj_get(origin, "field_supports") ? field_keys : call_keys,
          null ? 3 : yyjson_obj_get(origin, "field_supports") ? 5 : 4)) return 0;
  uint32_t producer_block, value, result_value;
  yyjson_val* producer = lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), &producer_block);
  if (!producer || !hako_physical_u32(yyjson_obj_get(producer, "dst"), &value) ||
      !hako_physical_u32(yyjson_obj_get(returned, "value"), &result_value)) return 0;
  if (null) {
    if (!lv4_is(producer, "op", "const_null")) return 0;
  } else {
    uint32_t invoke_block;
    yyjson_val* invoke = lv4_result_coordinate(fn, yyjson_obj_get(origin, "invoke"), &invoke_block);
    if (!invoke) return 0;
    hako_lv4_block* block = lv4_index_block(fn, invoke_block);
    yyjson_val* operation = yyjson_obj_get(invoke, "operation");
    if (!invoke || !block || lv4_term(block->block) != invoke ||
        !lv4_is(invoke, "op", "invoke") || !lv4_is(producer, "op", "invoke_normal_result") ||
        lv4_u(producer, "invoke_block") != invoke_block || lv4_u(invoke, "normal") != producer_block)
      return 0;
    if (fresh) {
      if (!lv4_is(operation, "kind", "new_box")) return 0;
    } else if (!lv4_is(operation, "kind", "ordinary_call") ||
        (!lv4_is(operation, "result", "handle") &&
         !(nullable && lv4_is(operation, "result", "nullable_handle")))) return 0;
  }
  if (!lv4_result_copy_chain(fn, origin, value, result_value)) return 0;
  if (!null && !lv4_owned_value_object(in, fn, lv4_index_value(fn, result_value), nullable)) return 0;
  return 1;
}

static int lv4_owned_return_origin(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* returned) {
  return lv4_acquired_return_origin(in, fn, returned,
      lv4_is(fn->function, "role", "ordinary_nullable_handle"));
}
