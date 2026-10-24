/* Rebase verified outgoing dependencies onto THIS invocation's original
 * actual Values. Class identity alone cannot lend an anchor's lifetime. */
static int lv4_field_dependencies_live_depth(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state,
    uint32_t use, size_t remaining);

static int lv4_mixed_result_readable_depth(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state,
    uint32_t use, size_t remaining, uint32_t object, int check_object);

static int lv4_field_anchor_live(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state,
    uint32_t use, size_t remaining, uint32_t object, int check_object) {
  if (!remaining) return 0;
  if (yyjson_obj_get(lv4_borrowed_result_operation(fn, value), "result_support"))
    return lv4_slot_result_readable(in, fn, value, state, use, object, check_object);
  value = lv4_borrowed_result_anchor(in, fn, value);
  if (!value) return 0;
  if (value->kind == LV4_MIXED_OBJECT)
    return lv4_mixed_result_readable_depth(in, fn, value, state, use,
        remaining - 1, object, check_object);
  if (value->kind == LV4_TAGGED)
    return check_object ? lv4_borrowed_object_formal(fn, value, object) :
        lv4_borrowed_tagged_formal(fn, value);
  if (value->kind != LV4_HANDLE) return 0;
  if (lv4_is(value->instruction, "op", "const_null")) return 1;
  if (check_object && ((!value->object_id_set && !lv4_owned_value_object(in, fn, value, 1)) ||
      value->object_id != object)) return 0;
  return lv4_object_base_readable_mode(in, fn, value->id, state, use, object, check_object, 1) &&
      lv4_field_dependencies_live_depth(in, fn, value, state, use, remaining);
}

static int lv4_field_dependencies_live_mode(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state,
    uint32_t use, size_t remaining, int released) {
  if (!remaining) return 0;
  value = lv4_value_alias_root(fn, value);
  if (!value) return 0;
  if (lv4_is(value->instruction, "op", "object_field_get")) {
    /* A field loan also borrows the SAME parent wrapper's dependencies. */
    return lv4_field_dependencies_live_depth(in, fn,
        lv4_index_value(fn, lv4_u(value->instruction, "base")), state, use, remaining - 1);
  }
  if (!lv4_is(value->instruction, "op", "invoke_normal_result")) return 1;
  hako_lv4_block* block = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  yyjson_val* invoke = block ? lv4_term(block->block) : NULL;
  yyjson_val* op = yyjson_obj_get(invoke, "operation");
  yyjson_val* call = yyjson_obj_get(op, "call");
  hako_lv4_function_index* owner = fn;
  int received = lv4_is(op, "kind", "ordinary_call") &&
      (lv4_is(op, "result", "handle") || lv4_is(op, "result", "nullable_handle"));
  if (received) {
    uint32_t target = lv4_u(call, "target");
    if (!target || target >= in->index_count) return 0;
    owner = &in->index[target];
  } else if (!lv4_is(op, "kind", "new_box")) return 1;
  for (size_t bi = 0; bi < owner->block_count; bi++) {
    yyjson_val* returned = lv4_term(owner->blocks[bi].block);
    if (!lv4_is(returned, "op", "return")) continue;
    yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
    if (!received && lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), NULL) != value->instruction) continue;
    yyjson_val* groups = yyjson_obj_get(origin, "field_supports");
    for (size_t i = 0; i < yyjson_arr_size(groups); i++) {
      yyjson_val* group = yyjson_arr_get(groups, i);
      yyjson_val* field = yyjson_obj_get(group, "field");
      uint32_t actual, born;
      if (received) {
        uint32_t ordinal = lv4_u(group, "formal_ordinal");
        yyjson_val* args = yyjson_obj_get(call, "args");
        if (ordinal >= yyjson_arr_size(args)) return 0;
        actual = lv4_u(yyjson_arr_get(args, ordinal), "value");
        born = lv4_u(invoke, "normal");
      } else {
        yyjson_val* birth = lv4_result_coordinate(fn, yyjson_obj_get(group, "construction"), NULL);
        if (!birth) return 0;
        born = lv4_u(birth, "normal");
        actual = lv4_u(group, "entry");
      }
      int interval = released ? lv4_field_dependency_discharged(fn, use, born, value,
          lv4_u(field, "object_id"), lv4_u(field, "field_ordinal"), lv4_u(field, "child_object_id")) :
          lv4_field_borrow_suffix(fn, use, born, value, lv4_u(field, "object_id"), lv4_u(field, "field_ordinal"));
      if (!interval || !lv4_field_anchor_live(in, fn, lv4_index_value(fn, actual), state, use, remaining - 1,
              lv4_u(field, "child_object_id"), 1)) return 0;
    }
  }
  return 1;
}

static int lv4_field_dependencies_live_depth(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state,
    uint32_t use, size_t remaining) {
  return lv4_field_dependencies_live_mode(in, fn, value, state, use, remaining, 0);
}

static int lv4_object_field_dependencies_released(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state, uint32_t use) {
  return lv4_field_dependencies_live_mode(in, fn, value, state, use, fn->value_count + 1, 1);
}

static int lv4_object_field_dependencies_live(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state, uint32_t use) {
  return lv4_field_dependencies_live_depth(in, fn, value, state, use, fn->value_count + 1);
}
