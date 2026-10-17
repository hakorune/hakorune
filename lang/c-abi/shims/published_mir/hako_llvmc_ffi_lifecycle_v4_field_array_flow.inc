/* Field Array operations borrow the original residence, never an arbitrary
 * Integer payload. Reuse the object provider and lifetime consumers. */
static int lv4_field_array_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, uint32_t id, const hako_lv4_state* state, uint32_t use) {
  hako_lv4_value* array = lv4_value_alias_root(fn, lv4_index_value(fn, id));
  if (!array || array->kind != LV4_I64 || !array->definition_block ||
      !lv4_is(array->instruction, "op", "object_field_get") ||
      lv4_is(fn->function, "role", "birth_unit")) return 0;
  yyjson_val* read = array->instruction;
  uint32_t object = lv4_u(read, "object_id"), field = lv4_u(read, "field_ordinal");
  yyjson_val* slots = yyjson_obj_get(lv4_layout(in, object), "owned_residences");
  size_t found = 0;
  for (size_t i = 0; i < yyjson_arr_size(slots); i++)
    found += yyjson_get_uint(yyjson_arr_get(slots, i)) == field;
  hako_lv4_value* base = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(read, "base")));
  if (found != 1 || !base || !lv4_field_provider_writer(in, object, field, 0, 1) ||
      (!lv4_readable_object(in, fn, base->id, state, use, object, 1) &&
       !lv4_borrowed_object_formal(fn, base, object))) return 0;
  uint32_t born = lv4_u(fn->function, "entry");
  if (base->origin != -2 && !lv4_borrowed_object_formal(fn, base, object)) {
    hako_lv4_value* root = base;
    for (size_t depth = 0; root && root->origin == LV4_OWNED_FIELD_BORROW; depth++) {
      if (depth >= fn->value_count || !lv4_is(root->instruction, "op", "object_field_get")) return 0;
      root = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(root->instruction, "base")));
    }
    if (!root || root->origin < 0) return 0;
    size_t births = 0;
    for (size_t bi = 0; bi < fn->block_count; bi++) {
      yyjson_val* term = lv4_term(fn->blocks[bi].block);
      yyjson_val* op = yyjson_obj_get(term, "operation");
      if (!lv4_is(op, "kind", "birth_call")) continue;
      yyjson_val* call = yyjson_obj_get(op, "call");
      if (lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(call, "receiver"))) != root) continue;
      uint32_t target = lv4_u(call, "target");
      if (++births != 1 || target >= in->index_count || !root->object_id_set ||
          !in->index[target].receiver_object_set || in->index[target].receiver_object != root->object_id) return 0;
      born = lv4_u(term, "normal");
    }
    if (births != 1 || hako_physical_predecessor_count(fn->function, born) != 1) return 0;
  }
  return lv4_field_borrow_suffix(fn, lv4_u(array->definition_block, "id"), born, base, object, field) &&
      lv4_field_borrow_suffix(fn, use, born, base, object, field);
}
