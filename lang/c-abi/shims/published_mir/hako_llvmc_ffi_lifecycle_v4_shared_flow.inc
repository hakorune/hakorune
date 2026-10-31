/* Shared handle flow: borrowed-view support and the per-step state transition. */

/* A Shared payload view is supported by its exact handle, not by any other
 * clone of the same cell. Only the field_get lane consumes this view. */
static hako_lv4_value* lv4_shared_borrow_support(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* view, const hako_lv4_state* state) {
  if (!view || view->kind != LV4_SHARED_BORROW || view->origin < 0 ||
      state[view->origin] != LV4_SHARED_BORROW_LIVE ||
      !view->object_id_set || !lv4_is(view->instruction, "op", "invoke_normal_result")) return NULL;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(view->instruction, "invoke_block"));
  yyjson_val* term = producer ? lv4_term(producer->block) : NULL;
  yyjson_val* op = term ? yyjson_obj_get(term, "operation") : NULL;
  if (!lv4_is(op, "kind", "shared_borrow") || lv4_u(op, "object_id") != view->object_id ||
      !view->definition_block || lv4_u(term, "normal") != lv4_u(view->definition_block, "id") ||
      lv4_pred_count(fn, lv4_u(view->definition_block, "id")) != 1 || !lv4_complete_end_body(in, op)) return NULL;
  hako_lv4_value* handle = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "value")));
  if (!handle || handle->kind != LV4_SHARED_HANDLE || handle->origin < 0 ||
      !handle->object_id_set || handle->object_id != view->object_id || state[handle->origin] != LV4_LIVE) return NULL;
  return handle;
}

static int lv4_shared_kind(yyjson_val* op) {
  return lv4_is(op, "kind", "shared_new") || lv4_is(op, "kind", "shared_clone") ||
         lv4_is(op, "kind", "shared_borrow") || lv4_is(op, "kind", "shared_release");
}

/* One Shared step on a flow block: New moves the payload on Normal, Clone adds a
 * token on Normal, Borrow only views, Release consumes even if the End faults. */
static int lv4_shared_step(const hako_lv4_input* in, hako_lv4_function_index* fn, yyjson_val* op,
    yyjson_val* block, int birth, int faulted, hako_lv4_state* state, hako_lv4_state* normal, size_t bi) {
  if (lv4_is(op, "kind", "shared_new")) {
    hako_lv4_value* payload = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "value")));
    if (birth || faulted || state[bi] || !payload || payload->kind != LV4_HANDLE || payload->origin < 0 ||
        !payload->object_id_set || payload->object_id != lv4_u(op, "object_id") ||
        !lv4_indexed_live(fn, payload->id, LV4_HANDLE, state, payload->object_id, 1) ||
        !lv4_owned_value_object(in, fn, payload, 0) ||
        !lv4_original_birth_completed(in, fn, payload, payload->object_id, lv4_u(block, "id")) ||
        !lv4_complete_end_body(in, op)) return 0;
    /* Shared New only moves the payload on the Normal edge. */
    normal[payload->origin] = 0;
    normal[bi] = LV4_LIVE;
  } else if (lv4_is(op, "kind", "shared_clone")) {
    hako_lv4_value* handle = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "value")));
    if (birth || faulted || state[bi] || !handle || handle->kind != LV4_SHARED_HANDLE || handle->origin < 0 ||
        !handle->object_id_set || handle->object_id != lv4_u(op, "object_id") ||
        state[handle->origin] != LV4_LIVE || !lv4_complete_end_body(in, op)) return 0;
    /* The original stays live on both edges; only Normal gets a token. */
    normal[bi] = LV4_LIVE;
  } else if (lv4_is(op, "kind", "shared_borrow")) {
    hako_lv4_value* handle = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "value")));
    if (birth || faulted || state[bi] || !handle || handle->kind != LV4_SHARED_HANDLE || handle->origin < 0 ||
        !handle->object_id_set || handle->object_id != lv4_u(op, "object_id") ||
        state[handle->origin] != LV4_LIVE || !lv4_complete_end_body(in, op)) return 0;
    normal[bi] = LV4_SHARED_BORROW_LIVE;
  } else if (lv4_is(op, "kind", "shared_release")) {
    hako_lv4_value* handle = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "value")));
    if (birth || !handle || handle->kind != LV4_SHARED_HANDLE || handle->origin < 0 ||
        !handle->object_id_set || handle->object_id != lv4_u(op, "object_id") ||
        state[handle->origin] != LV4_LIVE || !lv4_complete_end_body(in, op)) return 0;
    for (size_t vi = 0; vi < fn->value_count; vi++) {
      hako_lv4_value* view = &fn->values[vi];
      if (view->kind != LV4_SHARED_BORROW || view->origin < 0 || state[view->origin] != LV4_SHARED_BORROW_LIVE) continue;
      hako_lv4_value* support = lv4_shared_borrow_support(in, fn, view, state);
      if (!support || support->origin == handle->origin) return 0;
    }
    /* Release consumes even if the final payload End faults. */
    normal[handle->origin] = state[handle->origin] = 0;
  }
  return 1;
}
