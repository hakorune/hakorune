/* Normal-committed Birth field markers reuse store block origin slots. */
enum { LV4_BIRTH_FIELD = 258, LV4_BIRTH_PROVIDED_FIELD = 259 }; /* Map tags occupy a separate high-word domain. */
static int lv4_birth_marker(hako_lv4_state state) {
  return state == LV4_BIRTH_FIELD || state == LV4_BIRTH_PROVIDED_FIELD;
}
_Static_assert(LV4_BIRTH_FIELD < LV4_TRANSFER_BASE, "Birth/Map marker domains overlap");

static int lv4_birth_dynamic(const hako_lv4_input* in, yyjson_val* op) {
  yyjson_val* layout = lv4_layout(in, lv4_u(op, "object_id"));
  return lv4_u(yyjson_arr_get(yyjson_obj_get(layout, "fields"), lv4_u(op, "field_ordinal")),
      "storage_kind") == HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC;
}

static int lv4_birth_owned(const hako_lv4_input* in, yyjson_val* op, int object) {
  if (!object && lv4_birth_dynamic(in, op)) return 1;
  yyjson_val* layout = lv4_layout(in, lv4_u(op, "object_id"));
  yyjson_val* rows = yyjson_obj_get(layout,
      object ? "owned_object_residences" : "owned_residences");
  for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
    yyjson_val* row = yyjson_arr_get(rows, i);
    if (object ? (lv4_u(row, "field_ordinal") == lv4_u(op, "field_ordinal") &&
        lv4_u(row, "child_object_id") == lv4_u(op, "child_object_id")) :
        yyjson_get_uint(row) == lv4_u(op, "field_ordinal")) return 1;
  }
  return 0;
}

/* "Reserved" names any metadata-declared owned slot — array or object —
 * in any caller role; Birth commit/discharge is a separate gate. */
static int lv4_birth_reserved(const hako_lv4_input* in, yyjson_val* op) {
  if (lv4_birth_dynamic(in, op)) return 1;
  yyjson_val* layout = lv4_layout(in, lv4_u(op, "object_id"));
  for (int object = 0; object <= 1; object++) {
    yyjson_val* rows = yyjson_obj_get(layout,
        object ? "owned_object_residences" : "owned_residences");
    for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
      yyjson_val* row = yyjson_arr_get(rows, i);
      if ((object ? lv4_u(row, "field_ordinal") : yyjson_get_uint(row)) ==
          lv4_u(op, "field_ordinal")) return 1;
    }
  }
  return 0;
}

static int lv4_birth_array_value(hako_lv4_function_index* fn, hako_lv4_value* value) {
  return value && value->origin >= 0 && (size_t)value->origin < fn->block_count &&
      lv4_is(yyjson_obj_get(lv4_term(yyjson_arr_get(hako_physical_blocks(fn->function),
          value->origin)), "operation"), "kind", "array_new");
}

static int lv4_birth_base(hako_lv4_function_index* fn, yyjson_val* op) {
  hako_lv4_value* base = lv4_index_value(fn, lv4_u(op, "base"));
  hako_lv4_value* root = lv4_value_alias_root(fn, base);
  return base && root && base->kind == LV4_HANDLE && base->origin == -2 &&
      fn->receiver_object_set && root->id == lv4_u(fn->function, "receiver") &&
      root->object_id == fn->receiver_object &&
      lv4_u(op, "object_id") == fn->receiver_object;
}

static yyjson_val* lv4_birth_store(hako_lv4_function_index* fn, size_t origin) {
  return yyjson_obj_get(lv4_term(yyjson_arr_get(
      hako_physical_blocks(fn->function), origin)), "operation");
}

static int lv4_birth_same_slot(yyjson_val* a, yyjson_val* b, int object) {
  return lv4_u(a, "object_id") == lv4_u(b, "object_id") &&
      lv4_u(a, "field_ordinal") == lv4_u(b, "field_ordinal") &&
      (!object || lv4_u(a, "child_object_id") == lv4_u(b, "child_object_id"));
}

static int lv4_birth_commit(const hako_lv4_input* in, hako_lv4_function_index* fn,
    yyjson_val* op, hako_lv4_state* normal, size_t origin, int object) {
  if (!lv4_birth_base(fn, op) || !lv4_birth_owned(in, op, object) || normal[origin]) return 0;
  hako_lv4_value* value = lv4_value_alias_root(fn,
      lv4_index_value(fn, lv4_u(op, "value")));
  for (size_t i = 0; i < fn->block_count; i++) {
    if (!lv4_birth_marker(normal[i])) continue;
    yyjson_val* prior = lv4_birth_store(fn, i);
    if (lv4_birth_same_slot(prior, op, 0) ||
        (object && value && value->origin == -3 &&
         normal[i] == LV4_BIRTH_PROVIDED_FIELD &&
         lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(prior, "value"))) == value)) return 0;
  }
  normal[origin] = object && value && value->origin == -3 ?
      LV4_BIRTH_PROVIDED_FIELD : LV4_BIRTH_FIELD;
  return 1;
}

static int lv4_birth_discharge(const hako_lv4_input* in, hako_lv4_function_index* fn,
    yyjson_val* op, hako_lv4_state* state, hako_lv4_state* normal, int faulted, int object) {
  if (!faulted || !lv4_birth_base(fn, op) || !lv4_birth_owned(in, op, object)) return 0;
  size_t found = fn->block_count;
  for (size_t i = 0; i < fn->block_count; i++) {
    yyjson_val* store = lv4_birth_store(fn, i);
    if (state[i] != LV4_BIRTH_FIELD ||
        !lv4_is(store, "kind", object ? "object_field_set" : "field_set") ||
        !lv4_birth_base(fn, store) || !lv4_birth_same_slot(store, op, object)) continue;
    if (found != fn->block_count) return 0;
    found = i;
  }
  if (found == fn->block_count) return 0;
  /* Partial cleanup follows actual commit order, not declaration order.
   * A still-live later commit must drain before an earlier one. */
  for (size_t i = 0; i < fn->block_count; i++) {
    if (i != found && state[i] == LV4_BIRTH_FIELD &&
        hako_physical_block_dominates(fn->function,
            lv4_u(fn->blocks[found].block, "id"), lv4_u(fn->blocks[i].block, "id"))) return 0;
  }
  normal[found] = state[found] = 0;
  return 1;
}

static int lv4_birth_finish(const hako_lv4_input* in, hako_lv4_function_index* fn,
    hako_lv4_state* state) {
  yyjson_val* layout = lv4_layout(in, fn->receiver_object);
  size_t expected = 0, actual = 0;
  for (int object = 0; object <= 1; object++) {
    yyjson_val* rows = yyjson_obj_get(layout,
        object ? "owned_object_residences" : "owned_residences");
    for (size_t j = 0; j < yyjson_arr_size(rows); j++) {
      yyjson_val* row = yyjson_arr_get(rows, j);
      uint32_t ordinal = object ? lv4_u(row, "field_ordinal") : yyjson_get_uint(row);
      size_t matched = 0;
      for (size_t i = 0; i < fn->block_count; i++) {
        yyjson_val* store = lv4_birth_store(fn, i);
        if (lv4_birth_marker(state[i]) &&
            lv4_is(store, "kind", object ? "object_field_set" : "field_set") &&
            lv4_birth_base(fn, store) && lv4_birth_owned(in, store, object) &&
            lv4_u(store, "field_ordinal") == ordinal) matched++;
      }
      if (matched != 1) return 0;
      expected++;
    }
  }
  yyjson_val* fields = yyjson_obj_get(layout, "fields");
  for (size_t ordinal = 0; ordinal < yyjson_arr_size(fields); ordinal++) {
    if (lv4_u(yyjson_arr_get(fields, ordinal), "storage_kind") != HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC) continue;
    size_t matched = 0;
    for (size_t i = 0; i < fn->block_count; i++) {
      yyjson_val* store = lv4_birth_store(fn, i);
      if (state[i] == LV4_BIRTH_FIELD && lv4_is(store, "kind", "field_set") &&
          lv4_birth_base(fn, store) && lv4_u(store, "field_ordinal") == ordinal &&
          (lv4_is(store, "transaction", "dynamic_i64_initialize") ||
           lv4_is(store, "transaction", "dynamic_map_initialize") ||
           lv4_is(store, "transaction", "dynamic_text_initialize"))) matched++;
    }
    if (matched != 1) return 0;
    expected++;
  }
  for (size_t i = 0; i < fn->block_count; i++)
    if (lv4_birth_marker(state[i])) { actual++; state[i] = 0; }
  return actual == expected;
}

/* Provided slots are staged references until Birth Normal. On Fault the
 * caller still owns each actual; storage-only reclaim never ends that child. */
static int lv4_birth_drop_provided(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_state* state) {
  for (size_t i = 0; i < fn->block_count; i++) {
    if (state[i] != LV4_BIRTH_PROVIDED_FIELD) continue;
    yyjson_val* store = lv4_birth_store(fn, i);
    hako_lv4_value* value = lv4_value_alias_root(fn,
        lv4_index_value(fn, lv4_u(store, "value")));
    if (!lv4_is(store, "kind", "object_field_set") || !lv4_birth_base(fn, store) ||
        !lv4_birth_owned(in, store, 1) || !value || value->origin != -3 ||
        value->kind != LV4_HANDLE) return 0;
    state[i] = 0;
  }
  return 1;
}

/* A direct stored-child loan during Birth, lent by the same Normal commit
 * marker. It never turns the child into an always-live formal or a lease.
 * The provider's unique completed Birth must precede that exact store. */
static int lv4_birth_field_borrow_live(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* edge, const hako_lv4_state* state, uint32_t use) {
  if (!edge || edge->origin != LV4_OWNED_FIELD_BORROW || !edge->definition_block ||
      !edge->object_id_set || !lv4_is(edge->instruction, "op", "object_field_get") ||
      !lv4_birth_base(fn, edge->instruction)) return 0;
  yyjson_val* read = edge->instruction;
  hako_lv4_value* base = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(read, "base")));
  size_t found = fn->block_count;
  for (size_t i = 0; i < fn->block_count; i++) {
    yyjson_val* store = lv4_birth_store(fn, i);
    if (state[i] != LV4_BIRTH_FIELD || !lv4_is(store, "kind", "object_field_set") ||
        !lv4_birth_base(fn, store) || !lv4_birth_owned(in, store, 1) ||
        !lv4_birth_same_slot(store, read, 0) || lv4_u(store, "child_object_id") != edge->object_id) continue;
    if (found != fn->block_count) return 0;
    found = i;
  }
  if (found == fn->block_count) return 0;
  yyjson_val* store = lv4_birth_store(fn, found);
  hako_lv4_value* child = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(store, "value")));
  if (!child || child->origin < 0 || !child->object_id_set || child->object_id != edge->object_id ||
      !lv4_is(child->instruction, "op", "invoke_normal_result")) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(child->instruction, "invoke_block"));
  yyjson_val* created = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
  if (!lv4_is(created, "kind", "new_box") || lv4_u(created, "object_id") != child->object_id) return 0;
  uint32_t born = 0; size_t births = 0;
  for (size_t i = 0; i < fn->block_count; i++) {
    yyjson_val* term = lv4_term(fn->blocks[i].block);
    yyjson_val* op = yyjson_obj_get(term, "operation");
    if (!lv4_is(op, "kind", "birth_call")) continue;
    yyjson_val* call = yyjson_obj_get(op, "call");
    if (lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(call, "receiver"))) != child) continue;
    uint32_t target = lv4_u(call, "target");
    if (++births != 1 || target >= in->index_count ||
        !lv4_is(in->index[target].function, "role", "birth_unit") ||
        !in->index[target].receiver_object_set || in->index[target].receiver_object != child->object_id) return 0;
    born = lv4_u(term, "normal");
  }
  yyjson_val* store_block = fn->blocks[found].block;
  uint32_t committed = lv4_u(lv4_term(store_block), "normal");
  return births == 1 && hako_physical_predecessor_count(fn->function, born) == 1 &&
      hako_physical_block_dominates(fn->function, born, lv4_u(store_block, "id")) &&
      hako_physical_predecessor_count(fn->function, committed) == 1 &&
      lv4_field_borrow_suffix(fn, lv4_u(edge->definition_block, "id"), committed, base,
          base->object_id, lv4_u(read, "field_ordinal")) &&
      lv4_field_borrow_suffix(fn, use, committed, base, base->object_id, lv4_u(read, "field_ordinal"));
}

/* Reuse original owned result correspondence, then join the exact provided
 * formal's sole store to that child. A class or nullable representation alone
 * never supplies the caller's acquisition or the callee's destination. */
static int lv4_birth_actual_object(const hako_lv4_input* in,
    hako_lv4_function_index* caller, uint32_t target, size_t ordinal,
    hako_lv4_value* value, int nullable) {
  if (!target || target >= in->index_count ||
      !lv4_owned_value_object(in, caller, value, nullable)) return 0;
  hako_lv4_function_index* callee = &in->index[target];
  yyjson_val* param = yyjson_arr_get(yyjson_obj_get(callee->function, "params"), ordinal);
  hako_lv4_value* formal = lv4_index_value(callee, lv4_u(param, "value"));
  if (!lv4_is(callee->function, "role", "birth_unit") ||
      !lv4_is(param, "representation", "nullable_kind_payload_v1") ||
      !formal || formal->origin != -3 || formal->kind != LV4_HANDLE) return 0;
  size_t stores = 0;
  for (size_t i = 0; i < callee->block_count; i++) {
    yyjson_val* store = lv4_birth_store(callee, i);
    if (!lv4_is(store, "kind", "object_field_set") ||
        lv4_value_alias_root(callee, lv4_index_value(callee, lv4_u(store, "value"))) != formal) continue;
    if (++stores != 1 || !lv4_birth_base(callee, store) || !lv4_birth_owned(in, store, 1) ||
        lv4_u(store, "child_object_id") != value->object_id) return 0;
  }
  return stores == 1;
}
