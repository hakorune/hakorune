/* The existing per-origin flow owns these states. No source authority or
 * second ownership registry is issued by this physical projection. */
enum { LV4_OWNED_ARRAY_LIVE = 513 };

static int lv4_owned_array_kind(yyjson_val* op) {
  return lv4_is(op, "kind", "array_owned_claim") ||
      lv4_is(op, "kind", "array_owned_append") ||
      lv4_is(op, "kind", "array_owned_get") ||
      lv4_is(op, "kind", "array_owned_end");
}

static int lv4_carrier_array_kind(yyjson_val* op) {
  return lv4_is(op, "kind", "array_carrier_claim") ||
      lv4_is(op, "kind", "array_carrier_append") ||
      lv4_is(op, "kind", "array_carrier_end");
}

static hako_lv4_value* lv4_owned_array_root(hako_lv4_function_index* fn, uint32_t id);
static yyjson_val* lv4_owned_array_claim(hako_lv4_function_index* fn,
    hako_lv4_value* root);

static yyjson_val* lv4_carrier_array_claim(hako_lv4_function_index* fn,
    hako_lv4_value* root) {
  yyjson_val* found = NULL;
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* op = yyjson_obj_get(lv4_term(fn->blocks[bi].block), "operation");
    if (lv4_is(op, "kind", "array_carrier_claim") &&
        lv4_owned_array_root(fn, lv4_u(op, "array")) == root) {
      if (found) return NULL;
      found = op;
    }
  }
  return found;
}

static int lv4_carrier_array_holder_live(hako_lv4_function_index* fn,
    uint32_t id, const hako_lv4_state* state) {
  hako_lv4_value* holder = lv4_owned_array_root(fn, id);
  return holder && lv4_carrier_array_claim(fn, holder) &&
      state[holder->origin] == LV4_CARRIER_ARRAY_LIVE;
}

/* Candidate state stays unchanged on the invoke Fault edge and is consumed
 * only in the Normal projection after the runtime Array commits it. */
static int lv4_carrier_array_step(hako_lv4_function_index* fn, yyjson_val* op,
    int faulted, hako_lv4_state* state, hako_lv4_state* normal) {
  hako_lv4_value* holder = lv4_owned_array_root(fn, lv4_u(op, "array"));
  yyjson_val* claim = holder ? lv4_carrier_array_claim(fn, holder) : NULL;
  if (!holder || !claim) return 0;
  int is_claim = lv4_is(op, "kind", "array_carrier_claim");
  int is_end = lv4_is(op, "kind", "array_carrier_end");
  if (faulted && !is_end) return 0;
  if (is_claim) {
    if (state[holder->origin] != LV4_LIVE || lv4_owned_array_claim(fn, holder)) return 0;
    normal[holder->origin] = LV4_CARRIER_ARRAY_LIVE;
    return 1;
  }
  if (state[holder->origin] != LV4_CARRIER_ARRAY_LIVE) return 0;
  if (lv4_is(op, "kind", "array_carrier_append")) {
    hako_lv4_value* value = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "value")));
    if (!value || value->kind != LV4_DYNAMIC_CARRIER || value->origin < 0 ||
        state[value->origin] != LV4_DYNAMIC_CARRIER_LIVE) return 0;
    normal[value->origin] = 0;
    return 1;
  }
  if (!is_end) return 0;
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* append = yyjson_obj_get(lv4_term(fn->blocks[bi].block), "operation");
    if (!lv4_is(append, "kind", "array_carrier_append") ||
        lv4_owned_array_root(fn, lv4_u(append, "array")) != holder) continue;
    hako_lv4_value* value = lv4_value_alias_root(fn,
        lv4_index_value(fn, lv4_u(append, "value")));
    if (value && value->origin >= 0 &&
        state[value->origin] == LV4_DYNAMIC_CARRIER_LIVE) return 0;
  }
  normal[holder->origin] = state[holder->origin] = 0;
  return 1;
}

static int lv4_owned_descriptor_equal(yyjson_val* a, yyjson_val* b) {
  return lv4_u(a, "object_id") == lv4_u(b, "object_id") &&
      yyjson_get_bool(yyjson_obj_get(a, "nullable")) ==
          yyjson_get_bool(yyjson_obj_get(b, "nullable"));
}

static hako_lv4_value* lv4_owned_array_root(hako_lv4_function_index* fn, uint32_t id) {
  hako_lv4_value* root = lv4_value_alias_root(fn, lv4_index_value(fn, id));
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
    uint32_t id, const hako_lv4_state* state) {
  hako_lv4_value* holder = lv4_owned_array_root(fn, id);
  return holder && lv4_owned_array_claim(fn, holder) &&
      state[holder->origin] == LV4_OWNED_ARRAY_LIVE;
}

static int lv4_owned_array_borrow_live(hako_lv4_function_index* fn,
    hako_lv4_value* value, const hako_lv4_state* state) {
  value = lv4_value_alias_root(fn, value);
  if (!value || value->kind != LV4_SLOT_BORROW ||
      !lv4_is(value->instruction, "op", "invoke_normal_result")) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  yyjson_val* op = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
  hako_lv4_value* holder = op ? lv4_owned_array_root(fn, lv4_u(op, "array")) : NULL;
  yyjson_val* claim = holder ? lv4_owned_array_claim(fn, holder) : NULL;
  return claim && lv4_is(op, "kind", "array_owned_get") &&
      state[holder->origin] == LV4_OWNED_ARRAY_LIVE && lv4_owned_descriptor_equal(op, claim);
}

/* The existing value index retains the exact producer; no second borrow
 * registry, fake Array descriptor or payload Home is introduced. */
static int lv4_birth_dynamic(const hako_lv4_input*, yyjson_val*);
static int lv4_slot_borrow_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value,
    const hako_lv4_state* state, uint32_t use) {
  value = lv4_value_alias_root(fn, value);
  if (!value || value->kind != LV4_SLOT_BORROW ||
      !lv4_is(value->instruction, "op", "invoke_normal_result")) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  yyjson_val* term = producer ? lv4_term(producer->block) : NULL;
  yyjson_val* op = yyjson_obj_get(term, "operation");
  if (!producer || value->origin != (int)producer->ordinal || !value->definition_block ||
      !lv4_is(term, "op", "invoke") || lv4_u(term, "normal") != lv4_u(value->definition_block, "id") ||
      hako_physical_predecessor_count(fn->function, lv4_u(term, "normal")) != 1) return 0;
  if (lv4_is(op, "kind", "array_owned_get")) return lv4_owned_array_borrow_live(fn, value, state);
  if (!lv4_is(op, "kind", "map_field_get_indexed") || in->profile != LV4_INDEXED_SAFE_MUTEX ||
      !value->object_id_set || value->object_id != lv4_u(op, "payload_object_id") ||
      !lv4_layout(in, value->object_id) || !lv4_birth_dynamic(in, op) || lv4_slot(in, op) < 0) return 0;
  hako_lv4_value* base = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "base")));
  return base && base->kind == LV4_HANDLE &&
      lv4_readable_object(in, fn, base->id, state, use, lv4_u(op, "object_id"), 1) &&
      lv4_field_interval(fn, use, lv4_u(term, "normal"), base,
          lv4_u(op, "object_id"), lv4_u(op, "field_ordinal"), 0, LV4_FIELD_MAP_SLOT);
}

static int lv4_owned_array_step(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* op, int faulted,
    hako_lv4_state* state, hako_lv4_state* normal, size_t bi) {
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
    hako_lv4_value* root = lv4_value_alias_root(fn, candidate);
    if (!root || root->kind != LV4_HANDLE) return 0;
    if (lv4_is(root->instruction, "op", "const_null"))
      return yyjson_get_bool(yyjson_obj_get(op, "nullable"));
    if (root->origin < 0 || root == holder || state[root->origin] != LV4_LIVE ||
        !lv4_owned_value_object(in, fn, candidate, yyjson_get_bool(yyjson_obj_get(op, "nullable"))) ||
        !root->object_id_set || root->object_id != lv4_u(op, "object_id")) return 0;
    /* Existing candidate lease stays live on Fault; only successful slot
       commit transfers it. End subsequently owns every committed child. */
    normal[root->origin] = 0;
    return 1;
  }
  if (lv4_is(op, "kind", "array_owned_get")) {
    hako_lv4_value* index = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "index")));
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
    hako_lv4_value* candidate = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(append, "value")));
    if (candidate && candidate->origin >= 0 && state[candidate->origin] == LV4_LIVE) return 0;
  }
  normal[holder->origin] = state[holder->origin] = 0;
  return 1;
}
