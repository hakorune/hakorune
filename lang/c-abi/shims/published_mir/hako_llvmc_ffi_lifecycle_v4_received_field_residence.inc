/* Borrow a received owning residence from every ORIGINAL callee Return.
 * Class/layout corroboration alone never proves a Provided store or lifetime. */
static uint32_t lv4_received_field_parent(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* root) {
  if (!lv4_owned_value_object(in, fn, root, 0) ||
      !lv4_is(root->instruction, "op", "invoke_normal_result")) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
  yyjson_val* term = producer ? lv4_term(producer->block) : NULL;
  yyjson_val* op = yyjson_obj_get(term, "operation");
  uint32_t target = lv4_u(yyjson_obj_get(op, "call"), "target");
  if (!lv4_is(op, "kind", "ordinary_call") || !lv4_is(op, "result", "handle") ||
      !root->definition_block || lv4_u(term, "normal") != lv4_u(root->definition_block, "id") ||
      hako_physical_predecessor_count(fn->function, lv4_u(term, "normal")) != 1 ||
      !target || target >= in->index_count ||
      !lv4_is(in->index[target].function, "role", "ordinary_handle")) return 0;
  return target;
}

static int lv4_received_field_fresh(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* returned, uint32_t returned_block,
    uint32_t object, uint32_t field, uint32_t child, int* nullable) {
  yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
  yyjson_val* producer = lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), NULL);
  hako_lv4_value* root = producer ? lv4_index_value(fn, lv4_u(producer, "dst")) : NULL;
  if (!root || !lv4_owned_value_object(in, fn, root, 0) || root->object_id != object) return 0;
  yyjson_val* birth = NULL;
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* term = lv4_term(fn->blocks[bi].block);
    yyjson_val* op = yyjson_obj_get(term, "operation");
    if (!lv4_is(op, "kind", "birth_call")) continue;
    yyjson_val* call = yyjson_obj_get(op, "call");
    if (lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(call, "receiver"))) != root) continue;
    if (birth) return 0;
    birth = term;
  }
  if (!birth || hako_physical_predecessor_count(fn->function, lv4_u(birth, "normal")) != 1 ||
      !lv4_field_borrow_suffix(fn, returned_block, lv4_u(birth, "normal"), root, object, field)) return 0;
  yyjson_val* call = yyjson_obj_get(yyjson_obj_get(birth, "operation"), "call");
  uint32_t target = lv4_u(call, "target");
  if (!target || target >= in->index_count) return 0;
  hako_lv4_function_index* constructor = &in->index[target];
  hako_lv4_function_index* writer = NULL;
  yyjson_val* store = lv4_unique_field_writer(in, object, field, &writer);
  if (!store || writer != constructor || !constructor->receiver_object_set ||
      constructor->receiver_object != object) return 0;
  if (lv4_field_borrow_writer(in, object, field, child)) { *nullable = 0; return 1; }
  yyjson_val* params = yyjson_obj_get(constructor->function, "params");
  yyjson_val* args = yyjson_obj_get(call, "args");
  if (!yyjson_is_arr(args) || yyjson_arr_size(params) != yyjson_arr_size(args)) return 0;
  size_t found = 0;
  for (size_t ordinal = 0; ordinal < yyjson_arr_size(params); ordinal++) {
    if (!lv4_field_dependency_store_tuple(constructor, object, field, child, (uint32_t)ordinal)) continue;
    /* The exact argument remains the constructor formal's payload. Full Birth
     * flow independently checks its source kind, conditional move and class. */
    if (!lv4_index_value(fn, lv4_u(yyjson_arr_get(args, ordinal), "value")) || ++found != 1) return 0;
  }
  *nullable = 1; /* Provided does not prove a nonzero child. */
  return found == 1;
}

static int lv4_received_field_residence(const hako_lv4_input* in,
    hako_lv4_function_index* fn, uint32_t object, uint32_t field, uint32_t child,
    size_t remaining, int* nullable) {
  if (!remaining || !lv4_is(fn->function, "role", "ordinary_handle") ||
      !lv4_owned_return_object(in, fn) || fn->return_object != object) return 0;
  size_t returns = 0;
  int maybe_null = 0;
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* returned = lv4_term(fn->blocks[bi].block);
    if (!lv4_is(returned, "op", "return")) continue;
    if (!lv4_owned_return_origin(in, fn, returned) ||
        !lv4_return_field_dependencies(in, fn, returned)) return 0;
    yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
    int alternative_nullable = 0;
    if (lv4_is(origin, "kind", "fresh")) {
      if (!lv4_received_field_fresh(in, fn, returned, fn->blocks[bi].id,
          object, field, child, &alternative_nullable)) return 0;
    } else if (lv4_is(origin, "kind", "owned_call")) {
      yyjson_val* producer = lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), NULL);
      hako_lv4_value* value = producer ? lv4_index_value(fn, lv4_u(producer, "dst")) : NULL;
      uint32_t target = value ? lv4_received_field_parent(in, fn, value) : 0;
      yyjson_val* invoke = lv4_result_coordinate(fn, yyjson_obj_get(origin, "invoke"), NULL);
      if (!target || !invoke ||
          !lv4_field_borrow_suffix(fn, fn->blocks[bi].id, lv4_u(invoke, "normal"), value, object, field) ||
          !lv4_received_field_residence(in, &in->index[target], object, field, child,
              remaining - 1, &alternative_nullable)) return 0;
    } else return 0;
    maybe_null |= alternative_nullable; returns++;
  }
  *nullable = maybe_null;
  return returns != 0;
}
