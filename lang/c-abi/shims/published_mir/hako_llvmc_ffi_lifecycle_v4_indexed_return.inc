/* Exact Return/Fault lease completion for the existing indexed flow owner.
 * This child borrows the caller's checked state; it issues no identity or role. */
static int lv4_indexed_return(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* term, uint16_t* state, int faulted) {
  size_t count = fn->block_count;
  int birth = lv4_is(fn->function, "role", "birth_unit");
  int ordinary_map = lv4_is(fn->function, "role", "ordinary_map");
  int ordinary_nullable = lv4_is(fn->function, "role", "ordinary_nullable_handle");
  if (ordinary_map && lv4_is(term, "op", "return")) {
    hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
    /* A live LV4_MAP lease transfers into caller-owned storage. A spent
       lease or any other value is not a valid ordinary_map result. */
    if (faulted || !result || result->kind != LV4_MAP || result->origin < 0 ||
        state[result->origin] != LV4_LIVE) return 0;
    state[result->origin] = 0;
    for (size_t i = 0; i < count; i++)
      if (state[i] == result->origin + LV4_TRANSFER_BASE) state[i] = 0;
  }
  if (ordinary_nullable && lv4_is(term, "op", "return")) {
    hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
    /* A live handle result transfers the lease to the caller; the Void
       sentinel (or any borrowed value) owns nothing and hands over
       cleanly. A spent or non-handle value is not a valid result. */
    if (faulted || !result || result->kind != LV4_HANDLE) return 0;
    if (result->origin >= 0) {
      if (state[result->origin] != LV4_LIVE) return 0;
      state[result->origin] = 0;
    }
  }
  if (birth && lv4_is(term, "op", "return") &&
      (faulted || !lv4_birth_finish(in, fn, state))) return 0;
  for (size_t i = 0; i < count; i++) if (state[i]) return 0;
  if (lv4_is(term, "op", "return_fault")) return faulted;
  hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
  return !faulted && result &&
      result->kind == (birth ? LV4_UNIT : ordinary_map ? LV4_MAP :
          ordinary_nullable ? LV4_HANDLE : LV4_I64);
}
