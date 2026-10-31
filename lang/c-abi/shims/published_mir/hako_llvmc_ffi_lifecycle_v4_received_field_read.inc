/* Received scalar reads borrow the same original result class and lifetime.
 * No NewBox, direct-null or generic tagged payload acquires this capability. */
static int lv4_received_field_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* op, const hako_lv4_state* state,
    uint32_t use) {
  uint32_t object = lv4_u(op, "object_id"), ordinal = lv4_u(op, "field_ordinal");
  yyjson_val* fields = yyjson_obj_get(lv4_layout(in, object), "fields");
  yyjson_val* field = yyjson_arr_get(fields, ordinal);
  uint32_t storage = lv4_u(field, "storage_kind");
  if (!field || lv4_slot(in, op) < 0 || (lv4_birth_reserved(in, op) && storage != HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC) ||
      (storage != HAKO_LLVMC_LIFECYCLE_STORAGE_I64 && storage != HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC)) return 0;
  hako_lv4_value* base = lv4_index_value(fn, lv4_u(op, "base"));
  if (!base) return 0;
  if (base->kind == LV4_SHARED_BORROW)
    return lv4_is(op, "kind", "field_get") && base->object_id_set && base->object_id == object &&
        lv4_shared_borrow_support(in, fn, base, state) != NULL;
  if (base->kind == LV4_BORROWED_OBJECT)
    return lv4_slot_result_readable(in, fn, base, state, use, object, 1);
  if (lv4_canonical_receiver(fn, base))
    return lv4_readable_object(in, fn, base->id, state, use, object, 1);
  if (storage == HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC && base->kind == LV4_HANDLE)
    return lv4_readable_object(in, fn, base->id, state, use, object, 1);
  if (base->kind == LV4_HANDLE) {
    hako_lv4_value* root = lv4_value_alias_root(fn, base);
    if (root && root->origin == LV4_OWNED_FIELD_BORROW)
      return root->object_id_set && root->object_id == object &&
          lv4_nullable_field_loan_readable(in, fn, base->id, state, use, object, 1);
    if (!root || !lv4_is(root->instruction, "op", "invoke_normal_result")) return 0;
    hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
    yyjson_val* call = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
    return lv4_is(call, "kind", "ordinary_call") &&
        (lv4_is(call, "result", "handle") || lv4_is(call, "result", "nullable_handle")) &&
        lv4_owned_value_object(in, fn, base, 1) && base->object_id_set && base->object_id == object &&
        lv4_readable_object(in, fn, base->id, state, use, object, 1);
  }
  if (base->kind == LV4_SLOT_BORROW)
    return base->object_id_set && base->object_id == object &&
        lv4_slot_borrow_readable(in, fn, base, state, use);
  if (base->kind == LV4_MIXED_OBJECT)
    return lv4_mixed_owned_class(in, fn, base, object) &&
        lv4_mixed_result_readable_depth(in, fn, base, state, use, fn->value_count + 1, object, 1);
  return 0;
}
