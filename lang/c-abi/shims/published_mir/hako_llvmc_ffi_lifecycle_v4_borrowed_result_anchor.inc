/* Dispatch the result support from its original Call, never from a type or
 * an attempted proof's success. Unknown/malformed slot support stays closed. */
static yyjson_val* lv4_borrowed_result_operation(hako_lv4_function_index* fn, hako_lv4_value* value) {
  value = lv4_value_alias_root(fn, value);
  if (!value || value->kind != LV4_BORROWED_OBJECT ||
      !lv4_is(value->instruction, "op", "invoke_normal_result")) return NULL;
  hako_lv4_block* block = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  return block ? yyjson_obj_get(lv4_term(block->block), "operation") : NULL;
}

/* A receiver slot result borrows its parent and unchanged slot after THIS
 * Call.Normal. Its Text key was checked at Call; it may already have ended. */
static int lv4_slot_result_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value,
    const hako_lv4_state* state, uint32_t use, uint32_t object, int check_object) {
  value = lv4_value_alias_root(fn, value);
  if (!value || value->kind != LV4_BORROWED_OBJECT ||
      !lv4_is(value->instruction, "op", "invoke_normal_result") || !value->definition_block) return 0;
  hako_lv4_block* block = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  yyjson_val* term = block ? lv4_term(block->block) : NULL;
  yyjson_val* op = yyjson_obj_get(term, "operation");
  yyjson_val* call = yyjson_obj_get(op, "call");
  uint32_t target;
  hako_physical_slot_support support;
  if (!block || value->origin != (int)block->ordinal || !lv4_is(term, "op", "invoke") ||
      !lv4_is(op, "kind", "ordinary_call") || !lv4_is(op, "result", "borrowed_nullable_handle") ||
      !hako_physical_result_slot(op, &support) ||
      !hako_physical_u32(yyjson_obj_get(call, "target"), &target) || !target || target >= in->index_count ||
      !hako_physical_result_slot_equal(op, in->index[target].function) ||
      in->index[target].return_object_state != 2 || in->index[target].return_object != support.payload_object ||
      (check_object && object != support.payload_object) ||
      lv4_u(term, "normal") != lv4_u(value->definition_block, "id") ||
      lv4_pred_count(fn, lv4_u(term, "normal")) != 1) return 0;
  hako_lv4_value* receiver = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(call, "receiver")));
  return receiver && receiver->kind == LV4_HANDLE &&
      lv4_readable_object(in, fn, receiver->id, state, use, support.receiver_object, 1) &&
      lv4_map_slot_interval(fn, use, lv4_u(term, "normal"), receiver, support.receiver_object, support.field);
}

/* Resolve the SAME fixed-formal actual through original borrowed Call edges.
 * No LIVE slot is created. Original Copy roots and the finite value inventory
 * bound this walk; an invalid relation never retries a different source. */
static hako_lv4_value* lv4_borrowed_result_anchor(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value) {
  for (size_t steps = 0; steps <= fn->value_count; steps++) {
    value = lv4_value_alias_root(fn, value);
    if (!value || value->kind != LV4_BORROWED_OBJECT) return value;
    if (!lv4_is(value->instruction, "op", "invoke_normal_result")) return NULL;
    hako_lv4_block* block = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
    yyjson_val* term = block ? lv4_term(block->block) : NULL;
    yyjson_val* op = yyjson_obj_get(term, "operation");
    yyjson_val* call = yyjson_obj_get(op, "call");
    uint32_t target, ordinal, declared;
    if (!block || value->origin != (int)block->ordinal ||
        !value->definition_block || lv4_u(term, "normal") != lv4_u(value->definition_block, "id") ||
        lv4_pred_count(fn, lv4_u(term, "normal")) != 1 ||
        !lv4_is(term, "op", "invoke") || !lv4_is(op, "kind", "ordinary_call") ||
        !lv4_is(op, "result", "borrowed_nullable_handle") ||
        !hako_physical_u32(yyjson_obj_get(call, "target"), &target) ||
        !target || target >= in->index_count) return NULL;
    yyjson_val* callee = in->index[target].function;
    if (hako_physical_result_receiver(op)) {
      uint32_t receiver;
      if (!hako_physical_result_receiver(callee) ||
          !lv4_is(callee, "role", "ordinary_borrowed_nullable_handle") ||
          !hako_physical_u32(yyjson_obj_get(call, "receiver"), &receiver)) return NULL;
      value = lv4_index_value(fn, receiver);
      continue;
    }
    if (yyjson_obj_get(op, "result_support") || yyjson_obj_get(callee, "result_support") ||
        yyjson_is_true(yyjson_obj_get(op, "result_read")) != yyjson_is_true(yyjson_obj_get(callee, "result_read")) ||
        !hako_physical_u32(yyjson_obj_get(op, "result_formal_ordinal"), &ordinal)) return NULL;
    yyjson_val* args = yyjson_obj_get(call, "args");
    yyjson_val* params = yyjson_obj_get(callee, "params");
    if (!lv4_is(callee, "role", "ordinary_borrowed_nullable_handle") ||
        !hako_physical_u32(yyjson_obj_get(callee, "result_formal_ordinal"), &declared) ||
        declared != ordinal || !yyjson_is_arr(args) ||
        yyjson_arr_size(args) != yyjson_arr_size(params) || ordinal >= yyjson_arr_size(params) ||
        !lv4_is(yyjson_arr_get(params, ordinal), "representation", "borrowed_kind_payload_v1")) return NULL;
    uint32_t actual;
    if (!hako_physical_u32(yyjson_obj_get(yyjson_arr_get(args, ordinal), "value"), &actual)) return NULL;
    value = lv4_index_value(fn, actual);
  }
  return NULL;
}

/* The explicit read qualifier belongs to both the original function and Call.
 * All body blocks, including Fault/unreachable blocks, obey the support law. */
static int lv4_declared_read_effects(hako_lv4_function_index* fn) {
  if (!yyjson_obj_get(fn->function, "result_read")) return 1;
  if (!yyjson_is_true(yyjson_obj_get(fn->function, "result_read")) ||
      !lv4_is(fn->function, "role", "ordinary_borrowed_nullable_handle")) return 0;
  hako_lv4_value* support;
  if (hako_physical_result_receiver(fn->function)) {
    support = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(fn->function, "receiver")));
    if (!support || !lv4_canonical_receiver(fn, support)) return 0;
  } else {
    uint32_t ordinal;
    yyjson_val* params = yyjson_obj_get(fn->function, "params");
    if (!hako_physical_u32(yyjson_obj_get(fn->function, "result_formal_ordinal"), &ordinal) ||
        ordinal >= yyjson_arr_size(params)) return 0;
    support = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(yyjson_arr_get(params, ordinal), "value")));
    if (!support || support->kind != LV4_TAGGED) return 0;
  }
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_arr_iter rows = yyjson_arr_iter_with(yyjson_obj_get(fn->blocks[bi].block, "instructions"));
    yyjson_val* row;
    while ((row = yyjson_arr_iter_next(&rows)))
      if (!lv4_read_support_effect(fn, yyjson_obj_get(row, "instruction"), support, 0)) return 0;
    yyjson_val* op = yyjson_obj_get(lv4_term(fn->blocks[bi].block), "operation");
    if (op && !lv4_read_support_effect(fn, op, support, 1)) return 0;
  }
  return 1;
}

static int lv4_read_result_interval(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, uint32_t use, size_t position) {
  yyjson_val* op = lv4_borrowed_result_operation(fn, value);
  if (!yyjson_obj_get(op, "result_read")) return 1;
  yyjson_val* call = yyjson_obj_get(op, "call");
  uint32_t target = lv4_u(call, "target");
  hako_lv4_value* original = lv4_value_alias_root(fn, value);
  hako_lv4_value* support = lv4_borrowed_result_anchor(in, fn, value);
  if (!yyjson_is_true(yyjson_obj_get(op, "result_read")) || !target || target >= in->index_count ||
      !yyjson_is_true(yyjson_obj_get(in->index[target].function, "result_read")) || !support ||
      !original || !original->definition_block) return 0;
  return lv4_field_interval_at(fn, use, position, lv4_u(original->definition_block, "id"),
      support, 0, 0, 0, LV4_FIELD_READ_SUPPORT);
}

/* Legacy inferred borrowing keeps its forwarding law. An explicit read
 * result cannot lose that capability at an unqualified argument/Return edge. */
static int lv4_borrowed_result_forwardable(hako_lv4_function_index* fn, hako_lv4_value* value) {
  return !yyjson_obj_get(lv4_borrowed_result_operation(fn, value), "result_read");
}

static int lv4_borrowed_result_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state, uint32_t use, size_t position) {
  if (!lv4_read_result_interval(in, fn, value, use, position)) return 0;
  if (hako_physical_result_slot(lv4_borrowed_result_operation(fn, value), NULL))
    return lv4_slot_result_readable(in, fn, value, state, use, 0, 0);
  hako_lv4_value* anchor = lv4_borrowed_result_anchor(in, fn, value);
  if (!anchor) return 0;
  if (anchor->kind == LV4_TAGGED) return lv4_borrowed_tagged_formal(fn, anchor);
  if (anchor->kind != LV4_HANDLE) return 0;
  if (lv4_is(anchor->instruction, "op", "const_null")) return 1;
  return lv4_nullable_field_loan_readable(in, fn, anchor->id, state, use, 0, 0);
}

/* Identity forwarding is a separate nonnull proof, not Return permission.
 * A finite fixed point requires every completed Return to preserve its formal;
 * literal Null and unproved call cycles never gain a nonnull fact. */
static int lv4_declared_formal_identity(const hako_lv4_input* in, uint32_t target) {
  unsigned char* proven = (unsigned char*)calloc(in->index_count, 1);
  if (!proven) return 0;
  int changed = 1;
  while (changed && !proven[target]) {
    changed = 0;
    for (size_t fi = 1; fi < in->index_count; fi++) {
      hako_lv4_function_index* fn = &in->index[fi];
      if (proven[fi] || !yyjson_is_true(yyjson_obj_get(fn->function, "result_read")) ||
          yyjson_obj_get(fn->function, "result_support") ||
          !lv4_is(fn->function, "role", "ordinary_borrowed_nullable_handle")) continue;
      size_t returns = 0;
      int valid = 1;
      for (size_t bi = 0; bi < fn->block_count && valid; bi++) {
        yyjson_val* term = lv4_term(fn->blocks[bi].block);
        if (!lv4_is(term, "op", "return")) continue;
        returns++;
        yyjson_val* origin = yyjson_obj_get(term, "result_origin");
        if (!lv4_borrowed_return_origin(in, fn, term)) { valid = 0; break; }
        if (lv4_is(origin, "kind", "borrowed_formal")) continue;
        if (!lv4_is(origin, "kind", "borrowed_call")) { valid = 0; break; }
        yyjson_val* invoke = lv4_result_coordinate(fn, yyjson_obj_get(origin, "invoke"), NULL);
        uint32_t inner = lv4_u(yyjson_obj_get(yyjson_obj_get(invoke, "operation"), "call"), "target");
        if (!inner || inner >= in->index_count || !proven[inner]) valid = 0;
      }
      if (returns && valid) { proven[fi] = 1; changed = 1; }
    }
  }
  int result = proven[target];
  free(proven);
  return result;
}

/* Without an exact result guard, EVERY original callee Return must prove
 * nonnull. A guard excludes Null only on its dominated consumer path. */
static int lv4_declared_read_object(const hako_lv4_input* in, hako_lv4_function_index* fn,
    hako_lv4_value* value, const hako_lv4_state* state, uint32_t use, size_t position, uint32_t object, uint32_t field) {
  yyjson_val* op = lv4_borrowed_result_operation(fn, value);
  if (!yyjson_is_true(yyjson_obj_get(op, "result_read"))) return 0;
  yyjson_val* stored = yyjson_arr_get(yyjson_obj_get(lv4_layout(in, object), "fields"), field);
  if (!stored || lv4_u(stored, "storage_kind") != HAKO_LLVMC_LIFECYCLE_STORAGE_I64) return 0;
  uint32_t target = lv4_u(yyjson_obj_get(op, "call"), "target");
  if (!target || target >= in->index_count) return 0;
  hako_lv4_function_index* callee = &in->index[target];
  hako_lv4_value* original = lv4_value_alias_root(fn, value);
  hako_lv4_block* producer = original && lv4_is(original->instruction, "op", "invoke_normal_result")
      ? lv4_index_block(fn, lv4_u(original->instruction, "invoke_block")) : NULL;
  int guarded = lv4_nullable_slot_guarded(fn, original, producer, use);
  size_t returns = 0;
  for (size_t bi = 0; bi < callee->block_count; bi++) {
    yyjson_val* term = lv4_term(callee->blocks[bi].block);
    if (!lv4_is(term, "op", "return")) continue;
    returns++;
    yyjson_val* origin = yyjson_obj_get(term, "result_origin");
    if (lv4_is(origin, "kind", "null") && guarded) continue;
    if (!hako_physical_result_receiver(op)) {
      if (!lv4_is(origin, "kind", "borrowed_formal") &&
          (!lv4_is(origin, "kind", "borrowed_call") || (!guarded && !lv4_declared_formal_identity(in, target)))) return 0;
    } else {
      if (!hako_physical_result_receiver(callee->function)) return 0;
      if (lv4_is(origin, "kind", "borrowed_receiver")) {
        if (lv4_u(callee->function, "receiver_object") != object) return 0;
      } else if (lv4_is(origin, "kind", "borrowed_receiver_field")) {
        yyjson_val* invoke = lv4_result_coordinate(callee, yyjson_obj_get(origin, "invoke"), NULL);
        yyjson_val* operation = yyjson_obj_get(invoke, "operation");
        /* Normal of a checked nonnullable borrow guarantees the child value;
           a nullable borrow requires its own result guard, not a Parent guard. */
        if (lv4_u(origin, "object_id") != object ||
            (!guarded && !yyjson_is_false(yyjson_obj_get(operation, "nullable")))) return 0;
      } else return 0;
    }
  }
  hako_lv4_value* anchor = lv4_borrowed_result_anchor(in, fn, value);
  return returns && lv4_read_support_new(fn, anchor) &&
      lv4_borrowed_result_readable(in, fn, value, state, use, position) &&
      lv4_readable_object(in, fn, anchor->id, state, use,
          hako_physical_result_receiver(op) ? lv4_u(callee->function, "receiver_object") : object, 1);
}
