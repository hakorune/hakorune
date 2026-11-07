static int lv4_declared_read_graph(const hako_lv4_input*, hako_lv4_function_index*,
    hako_lv4_value*, const hako_lv4_state*, uint32_t, size_t, uint32_t, int);

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

/* One validated Call support edge. Both liveness and nonnull analysis use
 * this relation; neither substitutes the final anchor for the payload. */
static hako_lv4_value* lv4_borrowed_result_step(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value) {
  value = lv4_value_alias_root(fn, value);
  if (!value || value->kind != LV4_BORROWED_OBJECT ||
      !lv4_is(value->instruction, "op", "invoke_normal_result")) return NULL;
  hako_lv4_block* block = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
  yyjson_val* term = block ? lv4_term(block->block) : NULL;
  yyjson_val* op = yyjson_obj_get(term, "operation");
  yyjson_val* call = yyjson_obj_get(op, "call");
  uint32_t target, ordinal, declared;
  if (!block || value->origin != (int)block->ordinal || !value->definition_block ||
      lv4_u(term, "normal") != lv4_u(value->definition_block, "id") ||
      lv4_pred_count(fn, lv4_u(term, "normal")) != 1 || !lv4_is(term, "op", "invoke") ||
      !lv4_is(op, "kind", "ordinary_call") || !lv4_is(op, "result", "borrowed_nullable_handle") ||
      !hako_physical_u32(yyjson_obj_get(call, "target"), &target) || !target || target >= in->index_count) return NULL;
  yyjson_val* callee = in->index[target].function;
  if (!lv4_is(callee, "role", "ordinary_borrowed_nullable_handle") ||
      yyjson_is_true(yyjson_obj_get(op, "result_read")) != yyjson_is_true(yyjson_obj_get(callee, "result_read"))) return NULL;
  if (hako_physical_result_receiver(op)) {
    uint32_t receiver;
    if (!hako_physical_result_receiver(callee) || !hako_physical_u32(yyjson_obj_get(call, "receiver"), &receiver)) return NULL;
    return lv4_index_value(fn, receiver);
  }
  if (yyjson_obj_get(op, "result_support") || yyjson_obj_get(callee, "result_support") ||
      !hako_physical_u32(yyjson_obj_get(op, "result_formal_ordinal"), &ordinal)) return NULL;
  yyjson_val* args = yyjson_obj_get(call, "args");
  yyjson_val* params = yyjson_obj_get(callee, "params");
  if (!hako_physical_u32(yyjson_obj_get(callee, "result_formal_ordinal"), &declared) || declared != ordinal ||
      !yyjson_is_arr(args) || yyjson_arr_size(args) != yyjson_arr_size(params) || ordinal >= yyjson_arr_size(params) ||
      !lv4_is(yyjson_arr_get(params, ordinal), "representation", "borrowed_kind_payload_v1")) return NULL;
  uint32_t actual;
  if (!hako_physical_u32(yyjson_obj_get(yyjson_arr_get(args, ordinal), "value"), &actual)) return NULL;
  return lv4_index_value(fn, actual);
}

/* Structural support fold only: state, interval and payload proofs stay with
 * their use owners. One active/done set also catches Call -> Phi -> Call cycles. */
typedef struct { hako_lv4_value* value; size_t next; hako_lv4_value* anchor; } lv4_anchor_frame;
static hako_lv4_value* lv4_borrowed_result_anchor(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value) {
  value = lv4_value_alias_root(fn, value);
  if (!value || value->kind != LV4_BORROWED_OBJECT) return value;
  if (!fn->value_count) return NULL;
  hako_lv4_value* initial = value;
  int joined = 0;
  for (size_t steps = 0; steps <= fn->value_count; steps++) {
    value = lv4_value_alias_root(fn, value);
    if (!value || value->kind != LV4_BORROWED_OBJECT) return value;
    if (lv4_is(value->instruction, "op", "phi")) { joined = 1; break; }
    value = lv4_borrowed_result_step(in, fn, value);
  }
  if (!joined) return NULL;
  value = initial; /* Preserve allocation-free linear queries; fold all edges after a join. */
  unsigned char* seen = (unsigned char*)calloc(fn->value_count, 1);
  hako_lv4_value** memo = (hako_lv4_value**)calloc(fn->value_count, sizeof(*memo));
  lv4_anchor_frame* stack = (lv4_anchor_frame*)calloc(fn->value_count, sizeof(*stack));
  if (!seen || !memo || !stack) { free(seen); free(memo); free(stack); return NULL; }
  size_t depth = 1;
  int valid = 1, graph = 0, legacy = 0;
  hako_lv4_value* root = value;
  stack[0].value = value;
  while (depth && valid) {
    lv4_anchor_frame* frame = &stack[depth - 1];
    value = frame->value;
    size_t slot = (size_t)(value - fn->values);
    if (slot >= fn->value_count) { valid = 0; break; }
    yyjson_val* row = value->instruction;
    if (!seen[slot]) {
      seen[slot] = 1;
      if (value->kind != LV4_BORROWED_OBJECT) {
        memo[slot] = value; seen[slot] = 2; depth--; continue;
      }
      if (lv4_is(row, "op", "phi")) {
        yyjson_val* inputs = yyjson_obj_get(row, "inputs");
        graph = 1;
        if (!value->definition_block || !yyjson_arr_size(inputs) ||
            !hako_physical_phi_predecessors_exact(fn->function, lv4_u(value->definition_block, "id"), inputs)) { valid = 0; break; }
      } else if (lv4_is(row, "op", "invoke_normal_result")) {
        yyjson_val* op = lv4_borrowed_result_operation(fn, value);
        legacy |= !yyjson_is_true(yyjson_obj_get(op, "result_read"));
      } else { valid = 0; break; }
    }
    int phi = lv4_is(row, "op", "phi");
    yyjson_val* inputs = yyjson_obj_get(row, "inputs");
    size_t count = phi ? yyjson_arr_size(inputs) : 1;
    if (frame->next == count) {
      if (!frame->anchor) { valid = 0; break; }
      memo[slot] = frame->anchor; seen[slot] = 2; depth--; continue;
    }
    hako_lv4_value* child = phi ? lv4_index_value(fn, lv4_u(yyjson_arr_get(inputs, frame->next), "value"))
        : lv4_borrowed_result_step(in, fn, value);
    child = lv4_value_alias_root(fn, child);
    if (!child) { valid = 0; break; }
    size_t child_slot = (size_t)(child - fn->values);
    if (child_slot >= fn->value_count || seen[child_slot] == 1) { valid = 0; break; }
    if (seen[child_slot] == 2) {
      if (!memo[child_slot] || (frame->anchor && frame->anchor != memo[child_slot])) { valid = 0; break; }
      frame->anchor = memo[child_slot]; frame->next++; continue;
    }
    if (depth == fn->value_count) { valid = 0; break; }
    stack[depth++] = (lv4_anchor_frame){child, 0, NULL};
  }
  /* Legacy linear support keeps its old contract. Joined support never
     obtains a read capability from an unqualified Call edge. */
  hako_lv4_value* result = valid && !(graph && legacy) ? memo[(size_t)(root - fn->values)] : NULL;
  free(seen); free(memo); free(stack);
  return result;
}

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
  /* A graph has no single interval. Only the all-leaf owner may validate it. */
  if (lv4_read_result_join_root(fn, value)) return 0;
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
  if (lv4_read_result_join_root(fn, value)) return 0;
  return !yyjson_obj_get(lv4_borrowed_result_operation(fn, value), "result_read");
}

static int lv4_borrowed_result_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state, uint32_t use, size_t position) {
  if (lv4_read_result_join_root(fn, value))
    return lv4_declared_read_graph(in, fn, value, state, use, position, 0, 0);
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

/* Payload class and nonnull are separate from support liveness and Return
 * permission. Every completed Return must prove the requested properties;
 * unproved call cycles never gain a fact. */
static int lv4_declared_result_proves(const hako_lv4_input* in, uint32_t target,
    uint32_t object, int check_object, int nonnull) {
  unsigned char* proven = (unsigned char*)calloc(in->index_count, 1);
  if (!proven) return 0;
  int changed = 1;
  while (changed && !proven[target]) {
    changed = 0;
    for (size_t fi = 1; fi < in->index_count; fi++) {
      hako_lv4_function_index* fn = &in->index[fi];
      if (proven[fi] || !yyjson_is_true(yyjson_obj_get(fn->function, "result_read")) ||
          hako_physical_result_slot(fn->function, NULL) ||
          !lv4_is(fn->function, "role", "ordinary_borrowed_nullable_handle")) continue;
      size_t returns = 0;
      int valid = 1;
      for (size_t bi = 0; bi < fn->block_count && valid; bi++) {
        yyjson_val* term = lv4_term(fn->blocks[bi].block);
        if (!lv4_is(term, "op", "return")) continue;
        returns++;
        yyjson_val* origin = yyjson_obj_get(term, "result_origin");
        if (!lv4_borrowed_return_origin(in, fn, term)) { valid = 0; break; }
        if (lv4_is(origin, "kind", "null") && !nonnull) continue;
        if (lv4_is(origin, "kind", "borrowed_formal")) {
          yyjson_val* param = yyjson_arr_get(yyjson_obj_get(fn->function, "params"), lv4_u(origin, "formal_ordinal"));
          if (check_object && (!hako_physical_u32(yyjson_obj_get(param, "object_view"), NULL) || lv4_u(param, "object_view") != object)) valid = 0;
          continue;
        }
        if (lv4_is(origin, "kind", "borrowed_receiver")) {
          if (check_object && (!hako_physical_u32(yyjson_obj_get(fn->function, "receiver_object"), NULL) ||
              lv4_u(fn->function, "receiver_object") != object)) valid = 0;
          continue;
        }
        if (lv4_is(origin, "kind", "borrowed_receiver_field")) {
          yyjson_val* invoke = lv4_result_coordinate(fn, yyjson_obj_get(origin, "invoke"), NULL);
          if ((check_object && lv4_u(origin, "object_id") != object) ||
              (nonnull && !yyjson_is_false(yyjson_obj_get(yyjson_obj_get(invoke, "operation"), "nullable")))) valid = 0;
          continue;
        }
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

/* Value nonnull is separate from lifetime-anchor nonnull. A conditional
 * function fact assumes only its declared support, never all formals. */
static int lv4_declared_value_nonnull(const hako_lv4_input* in, hako_lv4_function_index* fn,
    hako_lv4_value* value, hako_lv4_value* assumed,
    const unsigned char* absolute, const unsigned char* conditional) {
  assumed = lv4_value_alias_root(fn, assumed);
  for (size_t steps = 0; steps <= fn->value_count; steps++) {
    value = lv4_value_alias_root(fn, value);
    if (!value || lv4_is(value->instruction, "op", "const_null")) return 0;
    if ((assumed && value == assumed) || lv4_canonical_receiver(fn, value) || lv4_read_support_new(fn, value)) return 1;
    if (value->kind == LV4_TAGGED) {
      yyjson_val* param = lv4_borrowed_formal_parameter(fn, value);
      return hako_physical_u32(yyjson_obj_get(param, "object_view"), NULL) &&
          lv4_nonnull_object_formal(fn, value, lv4_u(param, "object_view"));
    }
    yyjson_val* field = lv4_object_field_borrow_operation(fn, value);
    if (field) return yyjson_is_false(yyjson_obj_get(field, "nullable"));
    yyjson_val* op = lv4_borrowed_result_operation(fn, value);
    uint32_t target = lv4_u(yyjson_obj_get(op, "call"), "target");
    if (!yyjson_is_true(yyjson_obj_get(op, "result_read")) || !target || target >= in->index_count) return 0;
    if (absolute[target]) return 1;
    if (!conditional[target]) return 0;
    value = lv4_borrowed_result_step(in, fn, value);
  }
  return 0;
}

static int lv4_declared_read_nonnull(const hako_lv4_input* in,
    hako_lv4_function_index* caller, hako_lv4_value* value) {
  unsigned char* absolute = (unsigned char*)calloc(in->index_count, 1);
  unsigned char* conditional = (unsigned char*)calloc(in->index_count, 1);
  if (!absolute || !conditional) { free(absolute); free(conditional); return 0; }
  int changed = 1;
  while (changed) {
    changed = 0;
    for (size_t fi = 1; fi < in->index_count; fi++) {
      hako_lv4_function_index* fn = &in->index[fi];
      if ((absolute[fi] && conditional[fi]) || !yyjson_is_true(yyjson_obj_get(fn->function, "result_read")) ||
          hako_physical_result_slot(fn->function, NULL) || !lv4_is(fn->function, "role", "ordinary_borrowed_nullable_handle")) continue;
      hako_lv4_value* support = NULL;
      if (hako_physical_result_receiver(fn->function)) support = lv4_index_value(fn, lv4_u(fn->function, "receiver"));
      else {
        uint32_t ordinal;
        yyjson_val* params = yyjson_obj_get(fn->function, "params");
        if (hako_physical_u32(yyjson_obj_get(fn->function, "result_formal_ordinal"), &ordinal) && ordinal < yyjson_arr_size(params))
          support = lv4_index_value(fn, lv4_u(yyjson_arr_get(params, ordinal), "value"));
      }
      size_t returns = 0;
      int all_absolute = 1, all_conditional = support != NULL;
      for (size_t bi = 0; bi < fn->block_count; bi++) {
        yyjson_val* term = lv4_term(fn->blocks[bi].block);
        if (!lv4_is(term, "op", "return")) continue;
        returns++;
        if (!lv4_borrowed_return_origin(in, fn, term)) { all_absolute = all_conditional = 0; break; }
        hako_lv4_value* returned = lv4_index_value(fn, lv4_u(term, "value"));
        all_absolute &= lv4_declared_value_nonnull(in, fn, returned, NULL, absolute, conditional);
        all_conditional &= lv4_declared_value_nonnull(in, fn, returned, support, absolute, conditional);
      }
      if (returns && all_absolute && !absolute[fi]) { absolute[fi] = 1; changed = 1; }
      if (returns && all_conditional && !conditional[fi]) { conditional[fi] = 1; changed = 1; }
    }
  }
  int result = lv4_declared_value_nonnull(in, caller, value, NULL, absolute, conditional);
  free(absolute); free(conditional);
  return result;
}

/* Without an exact result guard, EVERY original callee Return must prove
 * nonnull. A guard excludes Null only on its dominated consumer path. */
static int lv4_declared_read_leaf_payload(const hako_lv4_input* in, hako_lv4_function_index* fn,
    hako_lv4_value* value, const hako_lv4_state* state, uint32_t use, size_t position, uint32_t object, int guarded, hako_lv4_value** retained_anchor) {
  yyjson_val* op = lv4_borrowed_result_operation(fn, value);
  if (!yyjson_is_true(yyjson_obj_get(op, "result_read"))) return 0;
  uint32_t target = lv4_u(yyjson_obj_get(op, "call"), "target");
  if (!target || target >= in->index_count) return 0;
  hako_lv4_function_index* callee = &in->index[target];
  hako_lv4_value* original = lv4_value_alias_root(fn, value);
  hako_lv4_block* producer = original && lv4_is(original->instruction, "op", "invoke_normal_result")
      ? lv4_index_block(fn, lv4_u(original->instruction, "invoke_block")) : NULL;
  guarded |= lv4_nullable_slot_guarded(fn, original, producer, use);
  size_t returns = 0;
  for (size_t bi = 0; bi < callee->block_count; bi++) {
    yyjson_val* term = lv4_term(callee->blocks[bi].block);
    if (!lv4_is(term, "op", "return")) continue;
    returns++;
    yyjson_val* origin = yyjson_obj_get(term, "result_origin");
    if (lv4_is(origin, "kind", "null") && guarded) continue;
    if (!hako_physical_result_receiver(op)) {
      if (!lv4_is(origin, "kind", "borrowed_formal") &&
          (!lv4_is(origin, "kind", "borrowed_call") || (!guarded && !lv4_declared_result_proves(in, target, 0, 0, 1)))) return 0;
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
      } else if (lv4_is(origin, "kind", "borrowed_call")) {
        if (!lv4_declared_result_proves(in, target, object, 1, !guarded)) return 0;
      } else return 0;
    }
  }
  hako_lv4_value* anchor = lv4_borrowed_result_anchor(in, fn, value);
  if (!returns || !lv4_declared_result_proves(in, target, object, 1, 0) ||
      !(guarded || lv4_declared_read_nonnull(in, fn, value)) ||
      !lv4_borrowed_result_readable(in, fn, value, state, use, position)) return 0;
  /* Caller entry support has no local New/Home. It lends lifetime only;
     payload class and nonnull remain the producer/guard proofs above. */
  int valid = anchor && anchor->kind == LV4_TAGGED ? lv4_borrowed_tagged_formal(fn, anchor)
      : anchor && (lv4_read_support_new(fn, anchor) || lv4_canonical_receiver(fn, anchor)) &&
        anchor->object_id_set && lv4_readable_object(in, fn, anchor->id, state, use, anchor->object_id, 1);
  if (valid && retained_anchor) *retained_anchor = anchor;
  return valid;
}

/* A use retains all Call.Normal intervals and one common caller anchor.
 * Payload class is checked only when a consumer requests it; a BO kind or
 * null comparison never issues a layout or an ownership capability. */
typedef struct {
  const hako_lv4_input* input;
  const hako_lv4_state* state;
  uint32_t use, object;
  size_t position;
  int check_object, guarded;
  hako_lv4_value* anchor;
} lv4_read_graph_use;
static int lv4_read_graph_leaf(hako_lv4_function_index* fn, hako_lv4_value* leaf, void* data) {
  lv4_read_graph_use* query = (lv4_read_graph_use*)data;
  hako_lv4_value* anchor = NULL;
  if (!lv4_borrowed_result_step(query->input, fn, leaf)) return 0;
  if (query->check_object) {
    if (!lv4_declared_read_leaf_payload(query->input, fn, leaf, query->state, query->use,
        query->position, query->object, query->guarded, &anchor)) return 0;
  } else {
    if (!lv4_borrowed_result_readable(query->input, fn, leaf, query->state, query->use, query->position)) return 0;
    anchor = lv4_borrowed_result_anchor(query->input, fn, leaf);
  }
  if (!anchor || (query->anchor && query->anchor != anchor)) return 0;
  query->anchor = anchor;
  return 1;
}
static int lv4_declared_read_graph(const hako_lv4_input* in, hako_lv4_function_index* fn,
    hako_lv4_value* value, const hako_lv4_state* state, uint32_t use, size_t position,
    uint32_t object, int check_object) {
  hako_lv4_value* root = lv4_read_result_join_root(fn, value);
  if (!root || !root->definition_block) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->definition_block, "id"));
  lv4_read_graph_use query = {in, state, use, object, position, check_object,
      check_object && lv4_nullable_slot_guarded(fn, root, producer, use), NULL};
  return lv4_read_call_graph(fn, value, lv4_read_graph_leaf, &query) && query.anchor;
}
static int lv4_declared_read_payload(const hako_lv4_input* in, hako_lv4_function_index* fn,
    hako_lv4_value* value, const hako_lv4_state* state, uint32_t use, size_t position, uint32_t object) {
  if (lv4_read_result_join_root(fn, value))
    return lv4_declared_read_graph(in, fn, value, state, use, position, object, 1);
  return lv4_declared_read_leaf_payload(in, fn, value, state, use, position, object, 0, NULL);
}

/* Scalar field readability adds its storage contract to the same payload proof. */
static int lv4_declared_read_object(const hako_lv4_input* in, hako_lv4_function_index* fn,
    hako_lv4_value* value, const hako_lv4_state* state, uint32_t use, size_t position, uint32_t object, uint32_t field) {
  yyjson_val* stored = yyjson_arr_get(yyjson_obj_get(lv4_layout(in, object), "fields"), field);
  return stored && lv4_u(stored, "storage_kind") == HAKO_LLVMC_LIFECYCLE_STORAGE_I64 &&
      lv4_declared_read_payload(in, fn, value, state, use, position, object);
}

/* A read argument retains its capability at a source-qualified nonmutating
 * Call. Payload class comes from the producer; the anchor is lifetime only. */
static int lv4_declared_read_argument(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, yyjson_val* consumer,
    uint32_t object, int check_object) {
  yyjson_val* producer = lv4_borrowed_result_operation(fn, value);
  uint32_t target = lv4_u(yyjson_obj_get(producer, "call"), "target");
  return yyjson_is_true(yyjson_obj_get(producer, "result_read")) &&
      target && target < in->index_count && lv4_readonly_call(fn, consumer, 1) &&
      (!check_object || lv4_declared_result_proves(in, target, object, 1, 0));
}

typedef struct {
  const hako_lv4_input* input;
  yyjson_val* consumer;
  uint32_t object;
  int check_object;
} lv4_read_argument_query;
static int lv4_read_argument_leaf(hako_lv4_function_index* fn, hako_lv4_value* leaf, void* data) {
  lv4_read_argument_query* query = (lv4_read_argument_query*)data;
  return lv4_declared_read_argument(query->input, fn, leaf, query->consumer,
      query->object, query->check_object);
}

/* Shared borrowed argument permission: explicit read proves payload class;
 * legacy forwarding retains its existing anchor/slot class obligation. */
static int lv4_borrowed_argument_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, yyjson_val* consumer,
    const hako_lv4_state* state, uint32_t use, size_t position,
    uint32_t object, int check_object) {
  if (!lv4_borrowed_result_readable(in, fn, value, state, use, position)) return 0;
  if (lv4_read_result_join_root(fn, value)) {
    /* Read arguments may remain nullable: the callee owns its own guard.
       The shared readability fold above checks EVERY lifetime interval. */
    lv4_read_argument_query query = {in, consumer, object, check_object};
    return lv4_read_call_graph(fn, value, lv4_read_argument_leaf, &query);
  }
  if (yyjson_obj_get(lv4_borrowed_result_operation(fn, value), "result_read"))
    return lv4_declared_read_argument(in, fn, value, consumer, object, check_object);
  if (!lv4_borrowed_result_forwardable(fn, value)) return 0;
  if (!check_object) return 1;
  hako_lv4_value* anchor = lv4_borrowed_result_anchor(in, fn, value);
  if (!anchor) return lv4_slot_result_readable(in, fn, value, state, use, object, 1);
  return !(anchor->object_id_set && anchor->object_id != object) &&
      !(anchor->kind == LV4_TAGGED && !lv4_borrowed_object_formal(fn, anchor, object));
}

/* A declared read used as receiver keeps payload class, nonnull proof and
 * readonly consumer permission separate from the lifetime anchor. */
static int lv4_borrowed_receiver_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, yyjson_val* consumer,
    const hako_lv4_state* state, uint32_t use, size_t position, uint32_t object) {
  if (lv4_read_result_join_root(fn, value))
    return lv4_readonly_call(fn, consumer, 1) &&
        lv4_declared_read_payload(in, fn, value, state, use, position, object);
  hako_lv4_value* original = lv4_value_alias_root(fn, value);
  if (original && original->kind == LV4_BORROWED_OBJECT &&
      yyjson_obj_get(lv4_borrowed_result_operation(fn, original), "result_read"))
    return lv4_readonly_call(fn, consumer, 1) &&
        lv4_declared_read_payload(in, fn, original, state, use, position, object);
  /* Owned nullable Call results retain their original object and lease;
     Call's checked dispatch excludes Null before entering the callee. This
     grants no permission to field loans, literal null, or ended receivers. */
  if (original && lv4_is(original->instruction, "op", "invoke_normal_result")) {
    hako_lv4_block* producer = lv4_index_block(fn, lv4_u(original->instruction, "invoke_block"));
    yyjson_val* op = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
    if (lv4_is(op, "kind", "ordinary_call") && lv4_is(op, "result", "nullable_handle")) {
      yyjson_val* params = yyjson_obj_get(yyjson_arr_get(in->functions,
          lv4_u(yyjson_obj_get(consumer, "call"), "target")), "params");
      for (size_t i = 0; i < yyjson_arr_size(params); i++)
        if (lv4_is(yyjson_arr_get(params, i), "ownership", "take")) return 0;
      return lv4_owned_value_object(in, fn, value, 1) && value->object_id_set && value->object_id == object &&
          lv4_indexed_live(fn, value->id, LV4_HANDLE, state, object, 1) &&
          lv4_object_field_dependencies_live(in, fn, value, state, use);
    }
  }
  return lv4_nonnull_object_formal(fn, value, object) ||
      (value && lv4_readable_object(in, fn, value->id, state, use, object, 1));
}
