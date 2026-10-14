/* The existing per-origin flow owns these states. No source authority or
 * second ownership registry is issued by this physical projection. */
enum { LV4_OWNED_ARRAY_LIVE = 513 };

static int lv4_owned_array_kind(yyjson_val* op) {
  return lv4_is(op, "kind", "array_owned_claim") ||
      lv4_is(op, "kind", "array_owned_append") ||
      lv4_is(op, "kind", "array_owned_get") ||
      lv4_is(op, "kind", "array_owned_end");
}

static int lv4_owned_descriptor_equal(yyjson_val* a, yyjson_val* b) {
  return lv4_u(a, "object_id") == lv4_u(b, "object_id") &&
      yyjson_get_bool(yyjson_obj_get(a, "nullable")) ==
          yyjson_get_bool(yyjson_obj_get(b, "nullable"));
}

static hako_lv4_value* lv4_owned_array_root(hako_lv4_function_index* fn, uint32_t id) {
  hako_lv4_value* root = lv4_borrowed_copy_root(fn, lv4_index_value(fn, id));
  if (!root || root->kind != LV4_HANDLE || root->origin < 0 ||
      !lv4_is(root->instruction, "op", "invoke_normal_result")) return NULL;
  hako_lv4_block* origin = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
  return origin && lv4_is(yyjson_obj_get(lv4_term(origin->block), "operation"),
      "kind", "array_new") ? root : NULL;
}

static yyjson_val* lv4_owned_array_claim(hako_lv4_function_index* fn,
    hako_lv4_value* root) {
  yyjson_val* found = NULL;
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* op = yyjson_obj_get(lv4_term(fn->blocks[bi].block), "operation");
    if (lv4_is(op, "kind", "array_owned_claim") &&
        lv4_owned_array_root(fn, lv4_u(op, "array")) == root) {
      if (found) return NULL;
      found = op;
    }
  }
  return found;
}

static int lv4_owned_array_holder_live(hako_lv4_function_index* fn,
    uint32_t id, const uint16_t* state) {
  hako_lv4_value* holder = lv4_owned_array_root(fn, id);
  return holder && lv4_owned_array_claim(fn, holder) &&
      state[holder->origin] == LV4_OWNED_ARRAY_LIVE;
}

static int lv4_owned_array_borrow_live(hako_lv4_function_index* fn,
    hako_lv4_value* value, const uint16_t* state) {
  value = lv4_borrowed_copy_root(fn, value);
  if (!value || value->kind != LV4_ARRAY_BORROW ||
      !lv4_is(value->instruction, "op", "invoke_normal_result")) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  yyjson_val* op = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
  hako_lv4_value* holder = op ? lv4_owned_array_root(fn, lv4_u(op, "array")) : NULL;
  yyjson_val* claim = holder ? lv4_owned_array_claim(fn, holder) : NULL;
  return claim && lv4_is(op, "kind", "array_owned_get") &&
      state[holder->origin] == LV4_OWNED_ARRAY_LIVE && lv4_owned_descriptor_equal(op, claim);
}

static int lv4_owned_array_step(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* op, int faulted,
    uint16_t* state, uint16_t* normal, size_t bi) {
  hako_lv4_value* holder = lv4_owned_array_root(fn, lv4_u(op, "array"));
  yyjson_val* claim = holder ? lv4_owned_array_claim(fn, holder) : NULL;
  if (!holder || !claim || !lv4_layout(in, lv4_u(op, "object_id")) ||
      !lv4_owned_descriptor_equal(op, claim)) return 0;
  int end = lv4_is(op, "kind", "array_owned_end");
  if (faulted && !end) return 0;
  if (lv4_is(op, "kind", "array_owned_claim")) {
    if (state[holder->origin] != LV4_LIVE) return 0;
    normal[holder->origin] = LV4_OWNED_ARRAY_LIVE;
    return 1;
  }
  if (state[holder->origin] != LV4_OWNED_ARRAY_LIVE) return 0;
  if (lv4_is(op, "kind", "array_owned_append")) {
    hako_lv4_value* candidate = lv4_index_value(fn, lv4_u(op, "value"));
    hako_lv4_value* root = lv4_borrowed_copy_root(fn, candidate);
    if (!root || root->kind != LV4_HANDLE) return 0;
    if (lv4_is(root->instruction, "op", "const_null"))
      return yyjson_get_bool(yyjson_obj_get(op, "nullable"));
    if (root->origin < 0 || root == holder || state[root->origin] != LV4_LIVE ||
        !root->object_id_set || root->object_id != lv4_u(op, "object_id")) return 0;
    /* Existing candidate lease stays live on Fault; only successful slot
       commit transfers it. End subsequently owns every committed child. */
    normal[root->origin] = 0;
    return 1;
  }
  if (lv4_is(op, "kind", "array_owned_get")) {
    hako_lv4_value* index = lv4_borrowed_copy_root(fn, lv4_index_value(fn, lv4_u(op, "index")));
    if (!index || index->kind != LV4_I64 ||
        !lv4_is(index->instruction, "op", "const_i64") || state[bi]) return 0;
    /* A projection marker is consumed by its exclusive Normal landing;
       the borrowed child itself never becomes a live lease. */
    normal[bi] = LV4_LIVE;
    return 1;
  }
  if (!end) return 0;
  for (size_t i = 0; i < fn->block_count; i++) {
    yyjson_val* append = yyjson_obj_get(lv4_term(fn->blocks[i].block), "operation");
    if (!lv4_is(append, "kind", "array_owned_append") ||
        lv4_owned_array_root(fn, lv4_u(append, "array")) != holder) continue;
    hako_lv4_value* candidate = lv4_borrowed_copy_root(fn, lv4_index_value(fn, lv4_u(append, "value")));
    if (candidate && candidate->origin >= 0 && state[candidate->origin] == LV4_LIVE) return 0;
  }
  normal[holder->origin] = state[holder->origin] = 0;
  return 1;
}
