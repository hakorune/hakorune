/* Mixed retains the source's Null/Fresh/owned-Call/borrowed-Call alternatives.
 * Exact producer coordinates and complete Return census corroborate the law;
 * a matching object class never proves the forwarded actual or its lifetime. */
static hako_lv4_value* lv4_borrowed_result_anchor(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value);

static int lv4_mixed_borrowed_return(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* returned, uint32_t outer) {
  static const char* const keys[] = {"kind", "producer", "invoke", "copies", "formal_ordinal"};
  yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
  uint32_t ordinal, producer_block, invoke_block, value, result;
  if (!hako_physical_exact_keys(origin, keys, 5) ||
      !lv4_is(origin, "kind", "borrowed_call") ||
      !hako_physical_u32(yyjson_obj_get(origin, "formal_ordinal"), &ordinal) || ordinal != outer ||
      !hako_physical_u32(yyjson_obj_get(returned, "value"), &result)) return 0;
  yyjson_val* producer = lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), &producer_block);
  yyjson_val* invoke = lv4_result_coordinate(fn, yyjson_obj_get(origin, "invoke"), &invoke_block);
  if (!producer || !invoke || !lv4_is(producer, "op", "invoke_normal_result") ||
      !lv4_is(invoke, "op", "invoke") || lv4_u(producer, "invoke_block") != invoke_block ||
      lv4_u(invoke, "normal") != producer_block ||
      !hako_physical_u32(yyjson_obj_get(producer, "dst"), &value)) return 0;
  hako_lv4_block* block = lv4_index_block(fn, invoke_block);
  if (!block || lv4_term(block->block) != invoke ||
      !lv4_result_copy_chain(fn, origin, value, result)) return 0;
  yyjson_val* param = yyjson_arr_get(yyjson_obj_get(fn->function, "params"), outer);
  hako_lv4_value* formal = lv4_index_value(fn, lv4_u(param, "value"));
  hako_lv4_value* anchor = lv4_borrowed_result_anchor(in, fn, lv4_index_value(fn, value));
  /* The helper independently checks INNER ordinal, exact Invoke/Normal,
     unique Normal predecessor, target and actual substitution. OUTER belongs
     to this function and must never be compared numerically with INNER. */
  return anchor && anchor == formal && anchor->kind == LV4_TAGGED;
}

static int lv4_mixed_return_origins(const hako_lv4_input* in,
    hako_lv4_function_index* fn) {
  uint32_t outer;
  yyjson_val* params = yyjson_obj_get(fn->function, "params");
  if (!hako_physical_u32(yyjson_obj_get(fn->function, "result_formal_ordinal"), &outer) ||
      outer >= yyjson_arr_size(params) ||
      !lv4_is(yyjson_arr_get(params, outer), "representation", "borrowed_kind_payload_v1")) return 0;
  size_t returns = 0;
  int owned = 0;
  uint32_t object = 0;
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* returned = lv4_term(fn->blocks[bi].block);
    if (!lv4_is(returned, "op", "return")) continue;
    returns++;
    yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
    /* Current Mixed source seals have no outgoing anchored field-End plan. */
    if (yyjson_obj_get(origin, "field_supports")) return 0;
    if (lv4_is(origin, "kind", "borrowed_call")) {
      if (!lv4_mixed_borrowed_return(in, fn, returned, outer)) return 0;
      continue;
    }
    if (!lv4_acquired_return_origin(in, fn, returned, 1)) return 0;
    if (lv4_is(origin, "kind", "null")) continue;
    hako_lv4_value* value = lv4_index_value(fn, lv4_u(returned, "value"));
    if (!value || !value->object_id_set || (owned && object != value->object_id)) return 0;
    object = value->object_id; owned = 1;
    /* An owned relay with dependencies requires a source-supported outgoing
       field plan, which the current Mixed seal deliberately does not issue. */
    if (lv4_is(origin, "kind", "owned_call")) {
      yyjson_val* invoke = lv4_result_coordinate(fn, yyjson_obj_get(origin, "invoke"), NULL);
      uint32_t target = lv4_u(yyjson_obj_get(yyjson_obj_get(invoke, "operation"), "call"), "target");
      if (!target || target >= in->index_count) return 0;
      hako_lv4_function_index* callee = &in->index[target];
      for (size_t ci = 0; ci < callee->block_count; ci++)
        if (yyjson_obj_get(yyjson_obj_get(lv4_term(callee->blocks[ci].block), "result_origin"), "field_supports")) return 0;
    }
  }
  if (!returns) return 0;
  /* This class describes only owned alternatives, never a Forward class. */
  if (owned) { fn->return_object = object; fn->return_object_state = 2; }
  return 1;
}
