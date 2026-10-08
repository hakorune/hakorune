/* One invocation's original owned Handle result correspondence.
 * Kind unions and release metadata are never object identity authorities. */
static int lv4_owned_return_object(const hako_lv4_input* in,
    hako_lv4_function_index* fn);

static int lv4_owned_value_object(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value) {
  if (!value || value->kind != LV4_HANDLE || value->origin < 0 ||
      (size_t)value->origin >= fn->block_count) return 0;
  /* Collapse only the exact Copy chain, with a finite inventory bound.
   * Call recursion is bounded separately by function visitation below. */
  hako_lv4_value* root = value;
  for (size_t steps = 0; lv4_is(root->instruction, "op", "copy"); steps++) {
    if (steps >= fn->value_count) return 0;
    root = lv4_index_value(fn, lv4_u(root->instruction, "src"));
    if (!root || root->kind != LV4_HANDLE || root->origin != value->origin) return 0;
  }
  if (!lv4_is(root->instruction, "op", "invoke_normal_result") ||
      !root->definition_block) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
  yyjson_val* term = producer ? lv4_term(producer->block) : NULL;
  yyjson_val* op = yyjson_obj_get(term, "operation");
  if (!producer || !lv4_is(term, "op", "invoke") ||
      root->origin != (int)producer->ordinal ||
      lv4_u(term, "normal") != lv4_u(root->definition_block, "id")) return 0;
  uint32_t object;
  if (lv4_is(op, "kind", "new_box")) {
    object = lv4_u(op, "object_id");
    if (!root->object_id_set || root->object_id != object || !lv4_layout(in, object))
      return 0;
  } else if (lv4_is(op, "kind", "ordinary_call") && lv4_is(op, "result", "handle")) {
    uint32_t target = lv4_u(yyjson_obj_get(op, "call"), "target");
    if (!target || target >= in->index_count ||
        !lv4_owned_return_object(in, &in->index[target])) return 0;
    object = in->index[target].return_object;
    if (root->object_id_set && root->object_id != object) return 0;
    root->object_id = object; root->object_id_set = 1;
  } else return 0;
  if (value->object_id_set && value->object_id != object) return 0;
  value->object_id = object; value->object_id_set = 1;
  return 1;
}

static int lv4_owned_return_object(const hako_lv4_input* in,
    hako_lv4_function_index* fn) {
  if (!lv4_is(fn->function, "role", "ordinary_handle")) return 0;
  if (fn->return_object_state == 2) return 1;
  if (fn->return_object_state == 1) return 0;
  fn->return_object_state = 1;
  int found = 0;
  uint32_t object = 0;
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* term = lv4_term(fn->blocks[bi].block);
    if (!lv4_is(term, "op", "return")) continue;
    hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
    if (!lv4_owned_value_object(in, fn, result) ||
        (found && object != result->object_id)) return 0;
    object = result->object_id; found = 1;
  }
  if (!found) return 0;
  fn->return_object = object; fn->return_object_state = 2;
  return 1;
}

static int lv4_index_owned_results(const hako_lv4_input* in) {
  /* Every callee index is complete before following any original Call edge. */
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    if (lv4_is(fn->function, "role", "ordinary_handle") &&
        !lv4_owned_return_object(in, fn)) return 0;
  }
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    for (size_t vi = 0; vi < fn->value_count; vi++) {
      hako_lv4_value* value = &fn->values[vi];
      if (value->kind != LV4_HANDLE || value->origin < 0) continue;
      hako_lv4_value* root = lv4_borrowed_copy_root(fn, value);
      if (!root || !lv4_is(root->instruction, "op", "invoke_normal_result")) continue;
      hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
      yyjson_val* op = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
      if (lv4_is(op, "kind", "ordinary_call") && lv4_is(op, "result", "handle") &&
          !lv4_owned_value_object(in, fn, value)) return 0;
    }
  }
  return 1;
}
