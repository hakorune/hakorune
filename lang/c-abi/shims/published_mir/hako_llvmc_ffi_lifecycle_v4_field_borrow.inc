/* A read-only loan of existing indexed New/Birth/field obligations. No
 * resource slot, ownership permission, source class, or runtime lease is issued. */
static int lv4_birth_field_borrow_live(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* edge, const hako_lv4_state* state, uint32_t use);

/* One current writer census for provider and Provided residence proofs. */
static yyjson_val* lv4_unique_field_writer(const hako_lv4_input* in,
    uint32_t object, uint32_t field, hako_lv4_function_index** owner) {
  yyjson_val* selected = NULL;
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    for (size_t bi = 0; bi < fn->block_count; bi++) {
      yyjson_val* block = fn->blocks[bi].block;
      yyjson_val* instructions = yyjson_obj_get(block, "instructions");
      for (size_t ri = 0; ri < yyjson_arr_size(instructions); ri++) {
        yyjson_val* row = yyjson_obj_get(yyjson_arr_get(instructions, ri), "instruction");
        if (lv4_is(row, "op", "field_set") && lv4_u(row, "object_id") == object &&
            lv4_u(row, "field_ordinal") == field) return NULL;
      }
      yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
      if (!(lv4_is(op, "kind", "field_set") || lv4_is(op, "kind", "object_field_set")) ||
          lv4_u(op, "object_id") != object || lv4_u(op, "field_ordinal") != field) continue;
      if (selected) return NULL;
      selected = op; *owner = fn;
    }
  }
  return selected;
}

static int lv4_field_provider_writer(const hako_lv4_input* in,
    uint32_t object, uint32_t field, uint32_t child, int array) {
  hako_lv4_function_index* fn = NULL;
  yyjson_val* op = lv4_unique_field_writer(in, object, field, &fn);
  if (!op) return 0;
  hako_lv4_value* base = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "base")));
  hako_lv4_value* value = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "value")));
  if (!lv4_is(fn->function, "role", "birth_unit") || !fn->receiver_object_set ||
      fn->receiver_object != object || !base || base->id != lv4_u(fn->function, "receiver") ||
      base->origin != -2 || !(array ? lv4_is(op, "kind", "field_set") :
          (lv4_is(op, "kind", "object_field_set") && lv4_u(op, "child_object_id") == child)) ||
      !value || value->origin < 0 || (!array && (!value->object_id_set || value->object_id != child)) ||
      !lv4_is(value->instruction, "op", "invoke_normal_result")) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  yyjson_val* created = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
  return array ? lv4_is(created, "kind", "array_new") :
      (lv4_is(created, "kind", "new_box") && lv4_u(created, "object_id") == child);
}

static int lv4_field_borrow_writer(const hako_lv4_input* in,
    uint32_t object, uint32_t field, uint32_t child) {
  return lv4_field_provider_writer(in, object, field, child, 0);
}

#include "hako_llvmc_ffi_lifecycle_v4_received_field_residence.inc"

/* Distinct FieldGets can name the same stable owning path. Compare original
 * Copy roots and each canonical edge, ending at the same New identity. */
static int lv4_field_borrow_same_path(hako_lv4_function_index* fn,
    hako_lv4_value* left, const hako_lv4_value* right) {
  for (size_t depth = 0; left && right; depth++) {
    if (left == right) return 1;
    if (depth >= fn->value_count) return -1;
    /* Different slot reads may still denote the same stored object. Without
     * a common producer proof they cannot establish distinct paths. */
    if (left->kind == LV4_SLOT_BORROW || right->kind == LV4_SLOT_BORROW) return -1;
    if (left->origin != LV4_OWNED_FIELD_BORROW || right->origin != LV4_OWNED_FIELD_BORROW) return 0;
    if (!lv4_is(left->instruction, "op", "object_field_get") ||
        !lv4_is(right->instruction, "op", "object_field_get")) return -1;
    if (lv4_u(left->instruction, "object_id") != lv4_u(right->instruction, "object_id") ||
        lv4_u(left->instruction, "field_ordinal") != lv4_u(right->instruction, "field_ordinal")) return 0;
    left = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(left->instruction, "base")));
    right = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(right->instruction, "base")));
  }
  return -1;
}

/* An owned candidate cannot be stored into itself or one of its owned
 * descendants. Copy/transparent Phi aliases preserve that same root.
 * Slot-borrow receivers remain outside the readable HANDLE lane. */
static int lv4_field_install_disjoint(hako_lv4_function_index* fn,
    uint32_t base_id, uint32_t candidate_id) {
  hako_lv4_value* candidate = lv4_value_alias_root(fn, lv4_index_value(fn, candidate_id));
  hako_lv4_value* base = lv4_value_alias_root(fn, lv4_index_value(fn, base_id));
  if (!candidate || candidate->kind != LV4_HANDLE || candidate->origin < 0) return 0;
  for (size_t depth = 0; base && depth < fn->value_count; depth++) {
    if (base == candidate || base->kind != LV4_HANDLE) return 0;
    if (base->origin != LV4_OWNED_FIELD_BORROW) return 1;
    if (!lv4_is(base->instruction, "op", "object_field_get")) return 0;
    base = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(base->instruction, "base")));
  }
  return 0;
}

#include "hako_llvmc_ffi_lifecycle_v4_field_interval.inc"

static int lv4_object_base_readable_mode(const hako_lv4_input* in, hako_lv4_function_index* fn,
    uint32_t id, const hako_lv4_state* state, uint32_t use, uint32_t object, int check_object,
    int allow_nullable) {
  hako_lv4_value* value = lv4_index_value(fn, id);
  if (!value || value->kind != LV4_HANDLE ||
      (check_object && (!value->object_id_set || value->object_id != object))) return 0;
  if (value->origin != LV4_OWNED_FIELD_BORROW)
    return lv4_indexed_live(fn, id, LV4_HANDLE, state, object, check_object);
  hako_lv4_value* edge = lv4_value_alias_root(fn, value);
  for (size_t depth = 0; edge && edge->origin == LV4_OWNED_FIELD_BORROW; depth++) {
    if (depth >= fn->value_count || !lv4_is(edge->instruction, "op", "object_field_get")) return 0;
    yyjson_val* read = edge->instruction;
    hako_lv4_value* base = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(read, "base")));
    if (!base || !base->object_id_set || base->object_id != lv4_u(read, "object_id")) return 0;
    if (lv4_canonical_receiver(fn, base) && lv4_is(fn->function, "role", "birth_unit"))
      return lv4_field_borrow_writer(in, base->object_id, lv4_u(read, "field_ordinal"), edge->object_id) &&
          lv4_birth_field_borrow_live(in, fn, edge, state, use);
    hako_lv4_value* root = base;
    for (size_t steps = 0; root && root->origin == LV4_OWNED_FIELD_BORROW; steps++) {
      if (steps >= fn->value_count || !lv4_is(root->instruction, "op", "object_field_get")) return 0;
      root = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(root->instruction, "base")));
    }
    uint32_t born;
    int nullable = 0;
    int received = root && lv4_received_field_parent(in, fn, root);
    if (received && base == root) {
      uint32_t target = lv4_received_field_parent(in, fn, root);
      if (!lv4_received_field_residence(in, &in->index[target], base->object_id,
          lv4_u(read, "field_ordinal"), edge->object_id, in->index_count + 1, &nullable) ||
          (nullable && !allow_nullable)) return 0;
    } else if (!lv4_field_borrow_writer(in, base->object_id, lv4_u(read, "field_ordinal"), edge->object_id)) return 0;
    if (lv4_canonical_receiver(fn, root) && lv4_is_ordinary(fn->function)) {
      born = lv4_u(fn->function, "entry");
    } else {
      if (!root || root->origin < 0 || !root->object_id_set ||
          !lv4_indexed_live(fn, root->id, LV4_HANDLE, state, root->object_id, 1)) return 0;
      if (received) {
        hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
        if (!producer) return 0;
        born = lv4_u(lv4_term(producer->block), "normal");
      } else {
      born = 0; size_t births = 0;
      for (size_t bi = 0; bi < fn->block_count; bi++) {
        yyjson_val* term = lv4_term(fn->blocks[bi].block);
        yyjson_val* op = yyjson_obj_get(term, "operation");
        if (!lv4_is(op, "kind", "birth_call")) continue;
        yyjson_val* call = yyjson_obj_get(op, "call");
        if (lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(call, "receiver"))) != root) continue;
        uint32_t target = lv4_u(call, "target");
        if (++births != 1 || target >= in->index_count || !in->index[target].receiver_object_set ||
            in->index[target].receiver_object != root->object_id) return 0;
        born = lv4_u(term, "normal");
      }
      if (births != 1 || lv4_pred_count(fn, born) != 1) return 0;
      }
    }
    if (!edge->definition_block ||
        !lv4_field_borrow_suffix(fn, lv4_u(edge->definition_block, "id"), born, base,
            base->object_id, lv4_u(read, "field_ordinal")) ||
        !lv4_field_borrow_suffix(fn, use, born, base,
            base->object_id, lv4_u(read, "field_ordinal"))) return 0;
    edge = base;
  }
  return edge && (edge->origin >= 0 ||
      (lv4_is_ordinary(fn->function) && lv4_canonical_receiver(fn, edge)));
}

static int lv4_object_base_readable(const hako_lv4_input* in, hako_lv4_function_index* fn,
    uint32_t id, const hako_lv4_state* state, uint32_t use, uint32_t object, int check_object) {
  return lv4_object_base_readable_mode(in, fn, id, state, use, object, check_object, 0);
}

static int lv4_object_field_dependencies_live(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state, uint32_t use);

static int lv4_slot_borrow_readable(const hako_lv4_input*, hako_lv4_function_index*,
    hako_lv4_value*, const hako_lv4_state*, uint32_t);

static int lv4_slot_result_readable(const hako_lv4_input*, hako_lv4_function_index*,
    hako_lv4_value*, const hako_lv4_state*, uint32_t, uint32_t, int);

/* Classify the already-indexed producer only. Readability and source
 * membership remain independent prerequisites at each consumer. */
static yyjson_val* lv4_object_field_borrow_operation(hako_lv4_function_index* fn,
    hako_lv4_value* value) {
  if (!value || value->kind != LV4_SLOT_BORROW) return NULL;
  hako_lv4_value* root = lv4_value_alias_root(fn, value);
  hako_lv4_block* producer = root && lv4_is(root->instruction, "op", "invoke_normal_result") ?
      lv4_index_block(fn, lv4_u(root->instruction, "invoke_block")) : NULL;
  yyjson_val* op = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
  return lv4_is(op, "kind", "object_field_borrow") ? op : NULL;
}

/* A path query over the indexed physical CFG. A negative result is a proven
 * absence, while malformed edges or allocation failure remain unknown. */
static int lv4_guard_path(hako_lv4_function_index* fn, uint32_t start, uint32_t target,
    int cut, uint32_t guard, uint32_t use) {
  hako_lv4_block* first = lv4_index_block(fn, start);
  if (!first || !lv4_index_block(fn, target) || !fn->block_count) return -1;
  unsigned char* seen = lv4_checked_calloc(fn->block_count, sizeof(*seen));
  size_t* queue = lv4_checked_calloc(fn->block_count, sizeof(*queue));
  if (!seen || !queue) { free(seen); free(queue); return -1; }
  size_t head = 0, tail = 0, ordinal = (size_t)(first - fn->blocks);
  seen[ordinal] = 1; queue[tail++] = ordinal;
  int result = 0;
  while (head < tail) {
    hako_lv4_block* current = &fn->blocks[queue[head++]];
    if (current->id == target) { result = 1; break; }
    if (cut && (current->id == guard || current->id == use)) continue;
    yyjson_val* edges = yyjson_obj_get(current->block, "edges");
    if (!yyjson_is_arr(edges)) { result = -1; break; }
    yyjson_arr_iter iter = yyjson_arr_iter_with(edges); yyjson_val* edge;
    while ((edge = yyjson_arr_iter_next(&iter))) {
      uint32_t id;
      if (!hako_physical_u32(yyjson_obj_get(edge, "target"), &id)) { result = -1; break; }
      hako_lv4_block* next = lv4_index_block(fn, id);
      if (!next) { result = -1; break; }
      ordinal = (size_t)(next - fn->blocks);
      if (!seen[ordinal]) { seen[ordinal] = 1; queue[tail++] = ordinal; }
    }
    if (result < 0) break;
  }
  free(seen); free(queue); return result;
}

/* Only an actual comparison and its nonnull edge lend a Map payload view.
 * This checks no lifetime or callee effects; the slot owner still does so. */
static int lv4_map_slot_guarded(hako_lv4_function_index* fn, hako_lv4_value* value, uint32_t use) {
  hako_lv4_value* root = lv4_value_alias_root(fn, value);
  if (!root || root->kind != LV4_SLOT_BORROW ||
      !lv4_is(root->instruction, "op", "invoke_normal_result")) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
  yyjson_val* op = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
  if (!lv4_is(op, "kind", "map_field_get_indexed") ||
      lv4_guard_path(fn, lv4_u(fn->function, "entry"), use, 0, 0, 0) != 1) return 0;
  for (size_t i = 0; i < fn->block_count; i++) {
    hako_lv4_block* block = &fn->blocks[i]; yyjson_val* term = lv4_term(block->block);
    if (!lv4_is(term, "op", "branch")) continue;
    hako_lv4_value* condition = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(term, "condition")));
    if (!condition || condition->definition_block != block->block ||
        !lv4_is(condition->instruction, "op", "borrowed_null_compare")) continue;
    yyjson_val* cmp = condition->instruction;
    hako_lv4_value* left = lv4_index_value(fn, lv4_u(cmp, "lhs"));
    hako_lv4_value* right = lv4_index_value(fn, lv4_u(cmp, "rhs"));
    if (!left || !right) continue;
    int left_null = lv4_is(left->instruction, "op", "const_null");
    int right_null = lv4_is(right->instruction, "op", "const_null");
    if (left_null == right_null || lv4_value_alias_root(fn, left_null ? right : left) != root) continue;
    int eq = lv4_is(cmp, "predicate", "eq"), ne = lv4_is(cmp, "predicate", "ne");
    if (eq == ne) continue;
    uint32_t present = lv4_u(term, eq ? "else" : "then");
    if (lv4_u(term, "then") == lv4_u(term, "else") ||
        lv4_pred_count(fn, present) != 1 ||
        !hako_physical_block_dominates(fn->function, producer->id, block->id) ||
        !hako_physical_block_dominates(fn->function, present, use) ||
        lv4_guard_path(fn, present, producer->id, 1, block->id, use) != 0) continue;
    return 1;
  }
  return 0;
}

static int lv4_map_present_guarded(hako_lv4_function_index*, hako_lv4_value*, uint32_t);

static int lv4_readable_object(const hako_lv4_input* in, hako_lv4_function_index* fn,
    uint32_t id, const hako_lv4_state* state, uint32_t use, uint32_t object, int check_object) {
  hako_lv4_value* value = lv4_index_value(fn, id);
  if (value && value->kind == LV4_BORROWED_OBJECT)
    return lv4_slot_result_readable(in, fn, value, state, use, object, check_object);
  if (value && value->kind == LV4_SLOT_BORROW) {
    yyjson_val* op = lv4_object_field_borrow_operation(fn, value);
    if (!op && (lv4_map_present_guarded(fn, value, use) || lv4_map_slot_guarded(fn, value, use)))
      return (!check_object || (value->object_id_set && value->object_id == object)) &&
          lv4_slot_borrow_readable(in, fn, value, state, use);
    return op && yyjson_is_false(yyjson_obj_get(op, "nullable")) &&
        (!check_object || (value->object_id_set && value->object_id == object)) &&
        lv4_slot_borrow_readable(in, fn, value, state, use);
  }
  return lv4_object_base_readable(in, fn, id, state, use, object, check_object) &&
      lv4_object_field_dependencies_live(in, fn, lv4_index_value(fn, id), state, use);
}

static int lv4_map_slot_interval(hako_lv4_function_index*, uint32_t, uint32_t,
    const hako_lv4_value*, uint32_t, uint32_t);

/* A Map element borrow whose Get is dominated by the Then edge of a Has branch
 * over the same Map slot and key, with no slot mutation in between. */
static int lv4_map_present_guarded(hako_lv4_function_index* fn, hako_lv4_value* value, uint32_t use) {
  hako_lv4_value* root = lv4_value_alias_root(fn, value);
  if (!root || root->kind != LV4_SLOT_BORROW || !lv4_is(root->instruction, "op", "invoke_normal_result")) return 0;
  uint32_t get_block = lv4_u(root->instruction, "invoke_block");
  hako_lv4_block* producer = lv4_index_block(fn, get_block);
  yyjson_val* get = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
  if (!lv4_is(get, "kind", "map_field_get_indexed")) return 0;
  hako_lv4_value* receiver = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(get, "base")));
  yyjson_val* key = yyjson_obj_get(get, "key");
  hako_lv4_value* key_root = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(key, "value")));
  if (!receiver || !key_root) return 0;
  for (size_t i = 0; i < fn->block_count; i++) {
    hako_lv4_block* branch = &fn->blocks[i]; yyjson_val* term = lv4_term(branch->block);
    if (!lv4_is(term, "op", "branch")) continue;
    hako_lv4_value* condition = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(term, "condition")));
    if (!condition || !lv4_is(condition->instruction, "op", "invoke_normal_result")) continue;
    hako_lv4_block* has_block = lv4_index_block(fn, lv4_u(condition->instruction, "invoke_block"));
    yyjson_val* has_term = has_block ? lv4_term(has_block->block) : NULL;
    yyjson_val* has = has_term ? yyjson_obj_get(has_term, "operation") : NULL;
    yyjson_val* has_key = yyjson_obj_get(has, "key");
    if (!lv4_is(has, "kind", "map_field_has") ||
        lv4_u(has, "object_id") != lv4_u(get, "object_id") || lv4_u(has, "field_ordinal") != lv4_u(get, "field_ordinal") ||
        lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(has, "base"))) != receiver ||
        !lv4_is(has_key, "kind", yyjson_get_str(yyjson_obj_get(key, "kind"))) ||
        lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(has_key, "value"))) != key_root) continue;
    uint32_t normal = lv4_u(has_term, "normal"), present = lv4_u(term, "then");
    if (lv4_pred_count(fn, normal) != 1 ||
        lv4_pred_count(fn, present) != 1 ||
        !hako_physical_block_dominates(fn->function, normal, branch->id) ||
        !hako_physical_block_dominates(fn->function, present, get_block) ||
        !hako_physical_block_dominates(fn->function, present, use) ||
        !lv4_map_slot_interval(fn, get_block, normal, receiver, lv4_u(get, "object_id"), lv4_u(get, "field_ordinal"))) continue;
    return 1;
  }
  return 0;
}

/* Nullable residence payload read/borrow only. No receiver, move or End grant. */
static int lv4_nullable_field_loan_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, uint32_t id, const hako_lv4_state* state,
    uint32_t use, uint32_t object, int check_object) {
  return lv4_object_base_readable_mode(in, fn, id, state, use, object, check_object, 1) &&
      lv4_object_field_dependencies_live(in, fn, lv4_index_value(fn, id), state, use);
}
