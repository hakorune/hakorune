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
  hako_lv4_value* anchor = lv4_borrowed_result_anchor(in, fn, value);
  if (!anchor) return 0;
  if (anchor->kind == LV4_TAGGED) return lv4_borrowed_tagged_formal(fn, anchor);
  if (anchor->kind != LV4_HANDLE) return 0;
  if (lv4_is(anchor->instruction, "op", "const_null")) return 1;
  return lv4_nullable_field_loan_readable(in, fn, anchor->id, state, use, 0, 0);
}
