/* Original physical identity, separate from kind unions and lease state.
 * Structural roots lend no kind or Home. Full kind closure precedes flow. */
static hako_lv4_value* lv4_index_alias_root(hako_lv4_function_index* fn,
    hako_lv4_value* value, size_t remaining) {
  if (!value || !remaining || value->alias_state == 1 || value->alias_state == 3) return NULL;
  if (value->alias_state == 2) return &fn->values[value->alias_root];
  value->alias_state = 1;
  hako_lv4_value* root = value;
  yyjson_val* ins = value->instruction;
  if (lv4_is(ins, "op", "copy")) {
    root = lv4_index_alias_root(fn, lv4_index_value(fn, lv4_u(ins, "src")), remaining - 1);
  } else if (lv4_is(ins, "op", "phi")) {
    yyjson_val* inputs = yyjson_obj_get(ins, "inputs");
    if (!value->definition_block || !yyjson_arr_size(inputs) ||
        !hako_physical_phi_predecessors_exact(fn->function,
            lv4_u(value->definition_block, "id"), inputs)) root = NULL;
    else {
      root = NULL;
      for (size_t i = 0; i < yyjson_arr_size(inputs); i++) {
        hako_lv4_value* next = lv4_index_alias_root(fn,
            lv4_index_value(fn, lv4_u(yyjson_arr_get(inputs, i), "value")), remaining - 1);
        /* A differing or cyclic scalar Phi remains its own scalar producer.
         * Such a Phi is never a transparent HANDLE alias. */
        if (!next || (root && root != next)) { root = value; break; }
        root = next;
      }
    }
  }
  if (!root) { value->alias_state = 3; return NULL; }
  value->alias_root = (size_t)(root - fn->values);
  value->alias_state = 2;
  return root;
}

static hako_lv4_value* lv4_value_alias_root(hako_lv4_function_index* fn,
    hako_lv4_value* value) {
  hako_lv4_value* root = lv4_index_alias_root(fn, value, fn->value_count + 1);
  if (root && root->kind == LV4_HANDLE && lv4_is(root->instruction, "op", "phi")) return NULL;
  return root;
}

/* A non-unanimous result join is a location, not one original Call. */
static hako_lv4_value* lv4_read_result_join_root(hako_lv4_function_index* fn, hako_lv4_value* value) {
  hako_lv4_value* root = lv4_value_alias_root(fn, value);
  return root && root->kind == LV4_BORROWED_OBJECT && lv4_is(root->instruction, "op", "phi") ? root : NULL;
}

/* Readonly Copy/Phi structure is separate from unanimous alias identity.
 * Visit every original Call once; reject cycles and incomplete predecessors.
 * This issues only a kind candidate. Consumers still check each use interval. */
typedef int (*lv4_read_call_leaf_fn)(hako_lv4_function_index*, hako_lv4_value*, void*);
typedef struct { hako_lv4_value* value; size_t next; } lv4_read_graph_frame;
static int lv4_read_call_graph(hako_lv4_function_index* fn, hako_lv4_value* value,
    lv4_read_call_leaf_fn visit, void* context) {
  if (!value || !fn->value_count) return 0;
  unsigned char* seen = (unsigned char*)calloc(fn->value_count, 1);
  lv4_read_graph_frame* stack = (lv4_read_graph_frame*)calloc(fn->value_count, sizeof(*stack));
  if (!seen || !stack) { free(seen); free(stack); return 0; }
  size_t depth = 1, leaves = 0;
  int valid = 1;
  stack[0].value = value;
  while (depth && valid) {
    lv4_read_graph_frame* frame = &stack[depth - 1];
    value = frame->value;
    size_t slot = (size_t)(value - fn->values);
    yyjson_val* row = value->instruction;
    if (!seen[slot]) {
      if (value->kind != LV4_BORROWED_OBJECT) { valid = 0; break; }
      seen[slot] = 1;
      if (lv4_is(row, "op", "invoke_normal_result")) {
        yyjson_val* op = lv4_borrowed_result_operation(fn, value);
        uint32_t target = lv4_u(yyjson_obj_get(op, "call"), "target");
        if (!lv4_is(op, "kind", "ordinary_call") || !lv4_is(op, "result", "borrowed_nullable_handle") ||
            !yyjson_is_true(yyjson_obj_get(op, "result_read")) || !target || target >= fn->peer_count ||
            !yyjson_is_true(yyjson_obj_get(fn->peers[target].function, "result_read")) ||
            !lv4_is(fn->peers[target].function, "role", "ordinary_borrowed_nullable_handle") ||
            (visit && !visit(fn, value, context))) { valid = 0; break; }
        leaves++; seen[slot] = 2; depth--; continue;
      }
      if (lv4_is(row, "op", "phi")) {
        yyjson_val* inputs = yyjson_obj_get(row, "inputs");
        if (!value->definition_block || !yyjson_arr_size(inputs) ||
            !hako_physical_phi_predecessors_exact(fn->function, lv4_u(value->definition_block, "id"), inputs)) { valid = 0; break; }
      } else if (!lv4_is(row, "op", "copy")) { valid = 0; break; }
    }
    yyjson_val* inputs = yyjson_obj_get(row, "inputs");
    size_t count = lv4_is(row, "op", "copy") ? 1 : yyjson_arr_size(inputs);
    if (frame->next == count) { seen[slot] = 2; depth--; continue; }
    uint32_t id = lv4_is(row, "op", "copy") ? lv4_u(row, "src")
        : lv4_u(yyjson_arr_get(inputs, frame->next), "value");
    frame->next++;
    hako_lv4_value* child = lv4_index_value(fn, id);
    if (!child) { valid = 0; break; }
    size_t child_slot = (size_t)(child - fn->values);
    if (seen[child_slot] == 1 || depth == fn->value_count) { valid = 0; break; }
    if (seen[child_slot] == 2) continue;
    stack[depth++] = (lv4_read_graph_frame){child, 0};
  }
  free(seen); free(stack);
  return valid && leaves;
}
