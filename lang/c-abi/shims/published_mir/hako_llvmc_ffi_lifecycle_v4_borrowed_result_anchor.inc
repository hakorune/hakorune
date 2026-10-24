/* Dispatch the result support from its original Call, never from a type or
 * an attempted proof's success. Unknown/malformed slot support stays closed. */
static yyjson_val* lv4_borrowed_result_operation(hako_lv4_function_index* fn, hako_lv4_value* value) {
  value = lv4_value_alias_root(fn, value);
  if (!value || value->kind != LV4_BORROWED_OBJECT ||
      !lv4_is(value->instruction, "op", "invoke_normal_result")) return NULL;
  hako_lv4_block* block = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  return block ? yyjson_obj_get(lv4_term(block->block), "operation") : NULL;
}

/* A receiver slot result borrows its parent and unchanged slot after THIS
 * Call.Normal. Its Text key was checked at Call; it may already have ended. */
static int lv4_slot_result_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value,
    const hako_lv4_state* state, uint32_t use, uint32_t object, int check_object) {
  value = lv4_value_alias_root(fn, value);
  if (!value || value->kind != LV4_BORROWED_OBJECT ||
      !lv4_is(value->instruction, "op", "invoke_normal_result") || !value->definition_block) return 0;
  hako_lv4_block* block = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  yyjson_val* term = block ? lv4_term(block->block) : NULL;
  yyjson_val* op = yyjson_obj_get(term, "operation");
  yyjson_val* call = yyjson_obj_get(op, "call");
  uint32_t target;
  hako_physical_slot_support support;
  if (!block || value->origin != (int)block->ordinal || !lv4_is(term, "op", "invoke") ||
      !lv4_is(op, "kind", "ordinary_call") || !lv4_is(op, "result", "borrowed_nullable_handle") ||
      !hako_physical_result_slot(op, &support) ||
      !hako_physical_u32(yyjson_obj_get(call, "target"), &target) || !target || target >= in->index_count ||
      !hako_physical_result_slot_equal(op, in->index[target].function) ||
      in->index[target].return_object_state != 2 || in->index[target].return_object != support.payload_object ||
      (check_object && object != support.payload_object) ||
      lv4_u(term, "normal") != lv4_u(value->definition_block, "id") ||
      hako_physical_predecessor_count(fn->function, lv4_u(term, "normal")) != 1) return 0;
  hako_lv4_value* receiver = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(call, "receiver")));
  return receiver && receiver->kind == LV4_HANDLE &&
      lv4_readable_object(in, fn, receiver->id, state, use, support.receiver_object, 1) &&
      lv4_map_slot_interval(fn, use, lv4_u(term, "normal"), receiver, support.receiver_object, support.field);
}

/* Resolve the SAME fixed-formal actual through original borrowed Call edges.
 * No LIVE slot is created. Original Copy roots and the finite value inventory
 * bound this walk; an invalid relation never retries a different source. */
static hako_lv4_value* lv4_borrowed_result_anchor(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value) {
  for (size_t steps = 0; steps <= fn->value_count; steps++) {
    value = lv4_value_alias_root(fn, value);
    if (!value || value->kind != LV4_BORROWED_OBJECT) return value;
    if (!lv4_is(value->instruction, "op", "invoke_normal_result")) return NULL;
    hako_lv4_block* block = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
    yyjson_val* term = block ? lv4_term(block->block) : NULL;
    yyjson_val* op = yyjson_obj_get(term, "operation");
    yyjson_val* call = yyjson_obj_get(op, "call");
    uint32_t target, ordinal, declared;
    if (!block || value->origin != (int)block->ordinal ||
        !value->definition_block || lv4_u(term, "normal") != lv4_u(value->definition_block, "id") ||
        hako_physical_predecessor_count(fn->function, lv4_u(term, "normal")) != 1 ||
        !lv4_is(term, "op", "invoke") || !lv4_is(op, "kind", "ordinary_call") ||
        !lv4_is(op, "result", "borrowed_nullable_handle") ||
        !hako_physical_u32(yyjson_obj_get(call, "target"), &target) ||
        !target || target >= in->index_count ||
        !hako_physical_u32(yyjson_obj_get(op, "result_formal_ordinal"), &ordinal)) return NULL;
    yyjson_val* callee = in->index[target].function;
    yyjson_val* args = yyjson_obj_get(call, "args");
    yyjson_val* params = yyjson_obj_get(callee, "params");
    if (!lv4_is(callee, "role", "ordinary_borrowed_nullable_handle") ||
        !hako_physical_u32(yyjson_obj_get(callee, "result_formal_ordinal"), &declared) ||
        declared != ordinal || !yyjson_is_arr(args) ||
        yyjson_arr_size(args) != yyjson_arr_size(params) || ordinal >= yyjson_arr_size(params) ||
        !lv4_is(yyjson_arr_get(params, ordinal), "representation", "borrowed_kind_payload_v1")) return NULL;
    uint32_t actual;
    if (!hako_physical_u32(yyjson_obj_get(yyjson_arr_get(args, ordinal), "value"), &actual)) return NULL;
    value = lv4_index_value(fn, actual);
  }
  return NULL;
}

static int lv4_borrowed_result_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state, uint32_t use) {
  if (yyjson_obj_get(lv4_borrowed_result_operation(fn, value), "result_support"))
    return lv4_slot_result_readable(in, fn, value, state, use, 0, 0);
  hako_lv4_value* anchor = lv4_borrowed_result_anchor(in, fn, value);
  if (!anchor) return 0;
  if (anchor->kind == LV4_TAGGED) return lv4_borrowed_tagged_formal(fn, anchor);
  if (anchor->kind != LV4_HANDLE) return 0;
  if (lv4_is(anchor->instruction, "op", "const_null")) return 1;
  return lv4_nullable_field_loan_readable(in, fn, anchor->id, state, use, 0, 0);
}
