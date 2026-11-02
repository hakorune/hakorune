/* Mixed retains the source's Null/Fresh/owned-Call/borrowed-Call alternatives.
 * Exact producer coordinates and complete Return census corroborate the law;
 * a matching object class never proves the forwarded actual or its lifetime. */
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
      if (!lv4_borrowed_call_return(in, fn, returned, outer)) return 0;
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
