/* Exact Return/Fault lease completion for the existing indexed flow owner.
 * This child borrows the caller's checked state; it issues no identity or role. */
static int lv4_indexed_return(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* term, hako_lv4_state* state, int faulted) {
  size_t count = fn->block_count;
  int birth = lv4_is(fn->function, "role", "birth_unit");
  int ordinary_bool = lv4_is(fn->function, "role", "ordinary_bool");
  int ordinary_map = lv4_is(fn->function, "role", "ordinary_map");
  int ordinary_text = lv4_is(fn->function, "role", "ordinary_text");
  int ordinary_handle = lv4_is(fn->function, "role", "ordinary_handle");
  int ordinary_nullable = lv4_is(fn->function, "role", "ordinary_nullable_handle");
  if (ordinary_text && lv4_is(term, "op", "return")) {
    hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
    /* The source return transfers the SAME installed residence after all
       Normal cleanup. Pending replacements and Fault never publish a pair. */
    if (faulted || !result || result->kind != LV4_OWNED_STRING || result->origin < 0 ||
        (size_t)result->origin >= count || lv4_string_pending(state, count) ||
        state[result->origin] <= LV4_STRING_HOME_BASE || state[result->origin] >= LV4_STRING_LIMIT)
      return 0;
    state[result->origin] = 0;
  }
  if (ordinary_map && lv4_is(term, "op", "return")) {
    hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
    /* A live LV4_MAP lease transfers into caller-owned storage. A spent
       lease or any other value is not a valid ordinary_map result. */
    if (faulted || !result || result->kind != LV4_MAP || result->origin < 0 ||
        state[result->origin] != LV4_LIVE) return 0;
    state[result->origin] = 0;
    for (size_t i = 0; i < count; i++)
      if (state[i] == lv4_map_transfer(result->origin)) state[i] = 0;
  }
  if (ordinary_handle && lv4_is(term, "op", "return")) {
    hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
    /* Identity is independently checked; only an owned LIVE lease transfers.
     * Null and borrowed handles never satisfy this nonnullable result role. */
    if (faulted || !result || result->kind != LV4_HANDLE || !result->object_id_set ||
        fn->return_object_state != 2 || result->object_id != fn->return_object ||
        result->origin < 0 || (size_t)result->origin >= count ||
        state[result->origin] != LV4_LIVE) return 0;
    state[result->origin] = 0;
  }
  if (ordinary_nullable && lv4_is(term, "op", "return")) {
    hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
    /* A live handle result transfers the lease to the caller; the Void
       sentinel (or any borrowed value) owns nothing and hands over
       cleanly. A spent or non-handle value is not a valid result. */
    if (faulted || !result || result->kind != LV4_HANDLE ||
        result->origin == LV4_OWNED_FIELD_BORROW) return 0;
    if (result->origin >= 0) {
      if (state[result->origin] != LV4_LIVE) return 0;
      state[result->origin] = 0;
    }
  }
  uint32_t return_block = 0;
  for (size_t bi = 0; bi < count; bi++)
    if (lv4_term(fn->blocks[bi].block) == term) return_block = fn->blocks[bi].id;
  if (lv4_is(fn->function, "role", "ordinary_mixed_handle") && lv4_is(term, "op", "return")) {
    hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
    yyjson_val* origin = yyjson_obj_get(term, "result_origin");
    if (faulted || !result) return 0;
    if (lv4_is(origin, "kind", "fresh") || lv4_is(origin, "kind", "owned_call")) {
      if (result->kind != LV4_HANDLE || result->origin < 0 ||
          (size_t)result->origin >= count || state[result->origin] != LV4_LIVE ||
          !lv4_object_field_dependencies_live(in, fn, result, state, return_block)) return 0;
      state[result->origin] = 0;
    } else if (lv4_is(origin, "kind", "borrowed_call")) {
      /* Forward has no owned state; still require its actual anchor now. */
      if (result->kind != LV4_BORROWED_OBJECT ||
          !lv4_borrowed_result_readable(in, fn, result, state, return_block)) return 0;
    } else if (!lv4_is(origin, "kind", "null")) return 0;
  }
  if (birth && lv4_is(term, "op", "return") &&
      (faulted || !lv4_birth_finish(in, fn, state))) return 0;
  if (birth && faulted && lv4_is(term, "op", "return_fault") &&
      !lv4_birth_drop_provided(in, fn, state)) return 0;
  for (size_t i = 0; i < count; i++) if (state[i]) return 0;
  if (lv4_is(term, "op", "return_fault")) return faulted;
  if (lv4_is(fn->function, "role", "ordinary_unit")) {
    yyjson_val* value = yyjson_obj_get(term, "value");
    if (yyjson_is_null(value)) return !faulted;
    hako_lv4_value* unit = lv4_index_value(fn, lv4_u(term, "value"));
    return !faulted && unit && unit->kind == LV4_UNIT;
  }
  if (lv4_is(fn->function, "role", "ordinary_borrowed_nullable_handle")) {
    /* The complete original Return law was checked by the result index. No
       owned state was consumed above, and all other Homes must already end. */
    return !faulted && lv4_borrowed_return_origin(fn, term);
  }
  if (lv4_is(fn->function, "role", "ordinary_mixed_handle")) return !faulted;
  hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
  /* Only the ordinary I64 result may read a borrowed tagged formal/Copy.
     Runtime tag1 is checked by the Return emitter before any payload store. */
  int tagged_i64 = lv4_is(fn->function, "role", "ordinary_i64") &&
      lv4_borrowed_tagged_formal(fn, result);
  return !faulted && result && (tagged_i64 ||
      result->kind == (birth ? LV4_UNIT : ordinary_bool ? LV4_BOOL : ordinary_map ? LV4_MAP :
          ordinary_text ? LV4_OWNED_STRING :
          (ordinary_handle || ordinary_nullable) ? LV4_HANDLE : LV4_I64));
}
