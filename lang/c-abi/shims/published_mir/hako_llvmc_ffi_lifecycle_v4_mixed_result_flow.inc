/* A Mixed result owns one conditional obligation, never an ordinary LIVE
 * Home. Its Forward alternative borrows THIS original actual until use/End. */
static int lv4_mixed_result_readable_depth(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state,
    uint32_t use, size_t remaining, uint32_t object, int check_object) {
  if (!remaining) return 0;
  value = lv4_value_alias_root(fn, value);
  if (!value || value->kind != LV4_MIXED_OBJECT || value->origin < 0 ||
      (size_t)value->origin >= fn->block_count || state[value->origin] != LV4_CONDITIONAL_OBJECT ||
      !lv4_is(value->instruction, "op", "invoke_normal_result") || !value->definition_block) return 0;
  hako_lv4_block* block = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  yyjson_val* invoke = block ? lv4_term(block->block) : NULL;
  yyjson_val* op = yyjson_obj_get(invoke, "operation");
  yyjson_val* call = yyjson_obj_get(op, "call");
  uint32_t target, inner, declared;
  if (!block || value->origin != (int)block->ordinal || !lv4_is(invoke, "op", "invoke") ||
      lv4_u(invoke, "normal") != lv4_u(value->definition_block, "id") ||
      hako_physical_predecessor_count(fn->function, lv4_u(invoke, "normal")) != 1 ||
      !lv4_is(op, "kind", "ordinary_call") || !lv4_is(op, "result", "mixed_handle") ||
      !hako_physical_u32(yyjson_obj_get(call, "target"), &target) || !target || target >= in->index_count ||
      !hako_physical_u32(yyjson_obj_get(op, "result_formal_ordinal"), &inner)) return 0;
  yyjson_val* callee = in->index[target].function;
  yyjson_val* params = yyjson_obj_get(callee, "params");
  yyjson_val* args = yyjson_obj_get(call, "args");
  if (!lv4_is(callee, "role", "ordinary_mixed_handle") ||
      !hako_physical_u32(yyjson_obj_get(callee, "result_formal_ordinal"), &declared) || declared != inner ||
      inner >= yyjson_arr_size(params) || yyjson_arr_size(args) != yyjson_arr_size(params) ||
      !lv4_is(yyjson_arr_get(params, inner), "representation", "borrowed_kind_payload_v1")) return 0;
  if (check_object && (in->index[target].return_object_state != 2 ||
      in->index[target].return_object != object)) return 0;
  return lv4_field_anchor_live(in, fn, lv4_index_value(fn, lv4_u(yyjson_arr_get(args, inner), "value")),
      state, use, remaining - 1, object, check_object);
}

static int lv4_mixed_result_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state, uint32_t use) {
  return lv4_mixed_result_readable_depth(in, fn, value, state, use, fn->value_count + 1, 0, 0);
}

/* This class belongs only to the original callee's acquired alternatives. */
static int lv4_mixed_owned_class(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, uint32_t object) {
  value = lv4_value_alias_root(fn, value);
  if (!value || value->kind != LV4_MIXED_OBJECT ||
      !lv4_is(value->instruction, "op", "invoke_normal_result")) return 0;
  hako_lv4_block* block = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  yyjson_val* op = block ? yyjson_obj_get(lv4_term(block->block), "operation") : NULL;
  uint32_t target = lv4_u(yyjson_obj_get(op, "call"), "target");
  return target && target < in->index_count &&
      lv4_is(in->index[target].function, "role", "ordinary_mixed_handle") &&
      in->index[target].return_object_state == 2 && in->index[target].return_object == object;
}

/* Full body must match every marked residence in reverse declaration order,
 * then the outer release. A layout without this original plan grants no End. */
static int lv4_complete_end_body(const hako_lv4_input* in, yyjson_val* op) {
  static const char* const outer_keys[] = {"kind"};
  static const char* const array_keys[] = {"kind", "field_ordinal"};
  static const char* const object_keys[] = {"kind", "field_ordinal", "child_object_id"};
  yyjson_val* layout = lv4_layout(in, lv4_u(op, "object_id"));
  yyjson_val* arrays = yyjson_obj_get(layout, "owned_residences");
  yyjson_val* objects = yyjson_obj_get(layout, "owned_object_residences");
  yyjson_val* end = yyjson_obj_get(op, "end");
  size_t na = yyjson_arr_size(arrays), no = yyjson_arr_size(objects), at = 0;
  if (!layout || !yyjson_is_arr(end) || yyjson_arr_size(end) != na + no + 1) return 0;
  while (na || no) {
    yyjson_val* step = yyjson_arr_get(end, at++);
    int array = na && (!no || yyjson_get_uint(yyjson_arr_get(arrays, na - 1)) >
        lv4_u(yyjson_arr_get(objects, no - 1), "field_ordinal"));
    uint32_t ordinal;
    if (!hako_physical_u32(yyjson_obj_get(step, "field_ordinal"), &ordinal)) return 0;
    if (array) {
      if (!hako_physical_exact_keys(step, array_keys, 2) || !lv4_is(step, "kind", "array_field") ||
          ordinal != yyjson_get_uint(yyjson_arr_get(arrays, --na))) return 0;
    } else {
      yyjson_val* expected = yyjson_arr_get(objects, --no);
      uint32_t child;
      if (!hako_physical_exact_keys(step, object_keys, 3) || !lv4_is(step, "kind", "object_field") ||
          ordinal != lv4_u(expected, "field_ordinal") ||
          !hako_physical_u32(yyjson_obj_get(step, "child_object_id"), &child) ||
          child != lv4_u(expected, "child_object_id") || !lv4_layout(in, child)) return 0;
    }
  }
  yyjson_val* last = yyjson_arr_get(end, at);
  return hako_physical_exact_keys(last, outer_keys, 1) && lv4_is(last, "kind", "release_outer");
}

static int lv4_mixed_birth_actual(const hako_lv4_input* in,
    hako_lv4_function_index* fn, uint32_t target, size_t ordinal,
    hako_lv4_value* value, const hako_lv4_state* state, uint32_t use) {
  if (!target || target >= in->index_count ||
      !lv4_mixed_result_readable(in, fn, value, state, use)) return 0;
  hako_lv4_function_index* birth = &in->index[target];
  yyjson_val* param = yyjson_arr_get(yyjson_obj_get(birth->function, "params"), ordinal);
  hako_lv4_value* formal = lv4_index_value(birth, lv4_u(param, "value"));
  if (!lv4_is(birth->function, "role", "birth_unit") ||
      !lv4_is(param, "representation", "nullable_kind_payload_v1") ||
      !formal || formal->origin != -3 || formal->kind != LV4_HANDLE) return 0;
  size_t stores = 0;
  for (size_t bi = 0; bi < birth->block_count; bi++) {
    yyjson_val* store = lv4_birth_store(birth, bi);
    if (!lv4_is(store, "kind", "object_field_set") ||
        lv4_value_alias_root(birth, lv4_index_value(birth, lv4_u(store, "value"))) != formal) continue;
    if (++stores != 1 || !lv4_birth_base(birth, store) || !lv4_birth_owned(in, store, 1) ||
        !lv4_mixed_owned_class(in, fn, value, lv4_u(store, "child_object_id"))) return 0;
  }
  return stores == 1;
}
