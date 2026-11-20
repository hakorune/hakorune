static int lv4_dynamic_operand_live(hako_lv4_function_index* fn, yyjson_val* operand,
    const hako_lv4_state* state) {
  if (lv4_is(operand, "kind", "null")) return 1;
  hako_lv4_value* value = lv4_index_value(fn, lv4_u(operand, "value"));
  if (!value) return 0;
  if (lv4_is(operand, "kind", "integer")) return value->kind == LV4_I64;
  if (lv4_is(operand, "kind", "bool")) return value->kind == LV4_BOOL;
  if (lv4_is(operand, "kind", "tagged_dynamic"))
    return value->kind == LV4_TAGGED && lv4_borrowed_tagged_formal(fn, value) &&
        !(hako_physical_finite_domain(lv4_borrowed_formal_parameter(fn, value)) & (1u << 4));
  return lv4_is(operand, "kind", "dynamic_carrier") &&
      value->kind == LV4_DYNAMIC_CARRIER && value->origin >= 0 &&
      state[value->origin] == LV4_DYNAMIC_CARRIER_LIVE;
}

static int lv4_dynamic_invoke_operands_live(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* op, const hako_lv4_state* state) {
  yyjson_val* receiver = yyjson_obj_get(op, "receiver");
  if (lv4_is(receiver, "kind", "borrowed_typed_object")) {
    if (!in->dispatch ||
        !lv4_borrowed_typed_live(in, fn, lv4_index_value(fn, lv4_u(receiver, "value")), state)) return 0;
  } else if (!lv4_dynamic_operand_live(fn, receiver, state)) return 0;

  yyjson_val* arguments = yyjson_obj_get(op, "arguments");
  for (size_t i = 0; i < yyjson_arr_size(arguments); i++)
    if (!lv4_dynamic_operand_live(fn, yyjson_arr_get(arguments, i), state)) return 0;
  return 1;
}

/* A strict program-level schema. Every runtime class has a unique layout ID,
 * including classes which deliberately have no provider row. */
static int lv4_dispatch_admit(const hako_lv4_input* in) {
  static const char* const keys[] = {"object_id", "selector", "arity", "parameters", "target"};
  yyjson_val* layouts = yyjson_obj_get(in->root, "layouts");
  if (in->profile != 1 || in->direct || in->native || !yyjson_is_arr(in->dispatch)) return 0;
  for (size_t i = 0; i < yyjson_arr_size(layouts); i++) {
    yyjson_val* layout = yyjson_arr_get(layouts, i);
    uint32_t type_id = lv4_u(layout, "runtime_type_id");
    yyjson_val* fields = yyjson_obj_get(layout, "fields");
    if (!type_id || yyjson_arr_size(yyjson_obj_get(layout, "owned_residences")) ||
        yyjson_arr_size(yyjson_obj_get(layout, "owned_object_residences"))) return 0;
    for (size_t j = 0; j < i; j++)
      if (type_id == lv4_u(yyjson_arr_get(layouts, j), "runtime_type_id")) return 0;
    for (size_t j = 0; j < yyjson_arr_size(fields); j++)
      if (lv4_u(yyjson_arr_get(fields, j), "storage_kind") != HAKO_LLVMC_LIFECYCLE_STORAGE_I64) return 0;
  }
  for (size_t i = 0; i < yyjson_arr_size(in->dispatch); i++) {
    yyjson_val* row = yyjson_arr_get(in->dispatch, i);
    yyjson_val* params = yyjson_obj_get(row, "parameters");
    yyjson_val* selector = yyjson_obj_get(row, "selector");
    uint32_t object, arity, target;
    if (!hako_physical_exact_keys(row, keys, 5) || !hako_physical_u32(yyjson_obj_get(row, "object_id"), &object) ||
        !hako_physical_u32(yyjson_obj_get(row, "arity"), &arity) || !hako_physical_u32(yyjson_obj_get(row, "target"), &target) ||
        !yyjson_is_arr(params) || yyjson_arr_size(params) != arity ||
        !yyjson_is_str(selector) || !yyjson_get_len(selector) ||
        target >= in->index_count || !lv4_layout(in, object)) return 0;
    hako_lv4_function_index* fn = &in->index[target];
    yyjson_val* actuals = yyjson_obj_get(fn->function, "params");
    if (!lv4_is(fn->function, "role", "ordinary_i64") ||
        !yyjson_is_uint(yyjson_obj_get(fn->function, "receiver")) ||
        lv4_u(fn->function, "receiver_object") != object ||
        yyjson_arr_size(actuals) != arity) return 0;
    for (size_t j = 0; j < arity; j++) {
      yyjson_val* kind = yyjson_arr_get(params, j);
      yyjson_val* param = yyjson_arr_get(actuals, j);
      if (!yyjson_is_uint(kind) || yyjson_get_uint(kind) > 1 ||
          lv4_is(param, "ownership", "take") || lv4_is(param, "ownership", "edit") ||
          !lv4_is(param, "representation", yyjson_get_uint(kind) ? "i64" : "borrowed_kind_payload_v1")) return 0;
    }
    yyjson_val* blocks = hako_physical_blocks(fn->function);
    for (size_t j = 0; j < yyjson_arr_size(blocks); j++) {
      yyjson_val* term = lv4_term(yyjson_arr_get(blocks, j));
      if (!lv4_is(term, "op", "return")) continue;
      hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
      if (!result || result->kind != LV4_I64) return 0;
    }
    for (size_t j = 0; j < i; j++) {
      yyjson_val* prior = yyjson_arr_get(in->dispatch, j);
      yyjson_val* prior_selector = yyjson_obj_get(prior, "selector");
      if (lv4_u(prior, "object_id") == object && lv4_u(prior, "arity") == arity &&
          yyjson_get_len(prior_selector) == yyjson_get_len(selector) &&
          !memcmp(yyjson_get_str(prior_selector), yyjson_get_str(selector), yyjson_get_len(selector))) return 0;
    }
  }
  return 1;
}
