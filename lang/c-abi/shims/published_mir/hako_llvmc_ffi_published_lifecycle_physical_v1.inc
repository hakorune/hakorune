/* Call-local decoder for hako.published-lifecycle-physical-program.v1.
 * It validates final-view transport before LLVM is touched. It never resolves
 * source names or supplements a missing physical relation. */

#define HAKO_PHYSICAL_SCHEMA "hako.published-lifecycle-physical-program.v1"

static int hako_physical_fail(char** err_out, const char* reason) {
  char message[160];
  snprintf(message, sizeof(message),
      "[freeze:contract][published-lifecycle-physical-parser/%s]", reason);
  return set_err_owned(err_out, message);
}

static int hako_physical_u32(yyjson_val* value, uint32_t* out) {
  uint64_t raw;
  if (!(value && yyjson_is_uint(value))) return 0;
  raw = yyjson_get_uint(value);
  if (raw > UINT32_MAX) return 0;
  if (out) *out = (uint32_t)raw;
  return 1;
}

static int hako_physical_string(yyjson_val* value, const char* expected) {
  const char* text = (value && yyjson_is_str(value)) ? yyjson_get_str(value) : NULL;
  size_t expected_len = expected ? strlen(expected) : 0u;
  return text && expected && yyjson_get_len(value) == expected_len &&
      memcmp(text, expected, expected_len) == 0;
}

static int hako_physical_i64(yyjson_val* value) {
  return (value && yyjson_is_sint(value)) ||
      (value && yyjson_is_uint(value) && yyjson_get_uint(value) <= INT64_MAX);
}

static int hako_physical_exact_keys(yyjson_val* object, const char* const* names, size_t count) {
  yyjson_obj_iter iter;
  yyjson_val* key;
  size_t seen[8] = {0};
  if (!(count <= 8 && object && yyjson_is_obj(object) && yyjson_obj_size(object) == count)) return 0;
  iter = yyjson_obj_iter_with(object);
  while ((key = yyjson_obj_iter_next(&iter))) {
    const char* text = yyjson_get_str(key);
    size_t i;
    int known = 0;
    for (i = 0; i < count; i++) {
      if (hako_physical_string(key, names[i])) { known = 1; break; }
    }
    if (!known) return 0;
    seen[i]++;
  }
  for (size_t i = 0; i < count; i++) if (seen[i] != 1) return 0;
  return 1;
}

static int hako_physical_array_u32(yyjson_val* array) {
  size_t i;
  if (!(array && yyjson_is_arr(array))) return 0;
  for (i = 0; i < yyjson_arr_size(array); i++)
    if (!hako_physical_u32(yyjson_arr_get(array, i), NULL)) return 0;
  return 1;
}

static yyjson_val* hako_physical_blocks(yyjson_val* function) {
  return function ? yyjson_obj_get(function, "blocks") : NULL;
}

static yyjson_val* hako_physical_block_by_id(yyjson_val* function, uint32_t wanted) {
  yyjson_val* blocks = hako_physical_blocks(function);
  size_t i;
  if (!(blocks && yyjson_is_arr(blocks))) return NULL;
  for (i = 0; i < yyjson_arr_size(blocks); i++) {
    yyjson_val* block = yyjson_arr_get(blocks, i);
    uint32_t id;
    if (hako_physical_u32(block ? yyjson_obj_get(block, "id") : NULL, &id) && id == wanted)
      return block;
  }
  return NULL;
}

static int hako_physical_has_block(yyjson_val* function, uint32_t id) {
  return hako_physical_block_by_id(function, id) != NULL;
}

static int hako_physical_block_has_edge_to(yyjson_val* function, uint32_t from, uint32_t target) {
  yyjson_val* block = hako_physical_block_by_id(function, from);
  yyjson_val* edges = block ? yyjson_obj_get(block, "edges") : NULL;
  size_t i;
  for (i = 0; edges && i < yyjson_arr_size(edges); i++) {
    uint32_t edge_target;
    if (hako_physical_u32(yyjson_obj_get(yyjson_arr_get(edges, i), "target"), &edge_target) && edge_target == target)
      return 1;
  }
  return 0;
}

static yyjson_val* hako_physical_block_invoke_terminator(yyjson_val* function, uint32_t block_id) {
  yyjson_val* block = hako_physical_block_by_id(function, block_id);
  yyjson_val* term = block ? yyjson_obj_get(block, "terminator") : NULL;
  yyjson_val* instruction = term ? yyjson_obj_get(term, "instruction") : NULL;
  return hako_physical_string(instruction ? yyjson_obj_get(instruction, "op") : NULL, "invoke") ? instruction : NULL;
}

static int hako_physical_unique_block_ids(yyjson_val* function) {
  yyjson_val* blocks = hako_physical_blocks(function);
  size_t i, j;
  if (!(blocks && yyjson_is_arr(blocks))) return 0;
  for (i = 0; i < yyjson_arr_size(blocks); i++) {
    uint32_t left;
    yyjson_val* block = yyjson_arr_get(blocks, i);
    if (!hako_physical_u32(block ? yyjson_obj_get(block, "id") : NULL, &left)) return 0;
    for (j = i + 1; j < yyjson_arr_size(blocks); j++) {
      uint32_t right;
      yyjson_val* other = yyjson_arr_get(blocks, j);
      if (!hako_physical_u32(other ? yyjson_obj_get(other, "id") : NULL, &right) || left == right)
        return 0;
    }
  }
  return 1;
}

static yyjson_val* hako_physical_instruction_dst(yyjson_val* instruction) {
  const char* op = instruction ? yyjson_get_str(yyjson_obj_get(instruction, "op")) : NULL;
  yyjson_val* call;
  if (!op) return NULL;
  if (strcmp(op, "const_i64") == 0 || strcmp(op, "const_string") == 0 ||
      strcmp(op, "const_unit") == 0 || strcmp(op, "add") == 0 ||
      strcmp(op, "copy") == 0 || strcmp(op, "phi") == 0 ||
      strcmp(op, "object_field_get") == 0 || strcmp(op, "invoke_normal_result") == 0 ||
      strcmp(op, "fault_frame_enter") == 0)
    return yyjson_obj_get(instruction, "dst");
  if (strcmp(op, "birth_call") != 0) return NULL;
  call = yyjson_obj_get(instruction, "call");
  return call ? yyjson_obj_get(call, "dst") : NULL;
}

static int hako_physical_value_def_count(yyjson_val* function, uint32_t wanted) {
  yyjson_val* params = function ? yyjson_obj_get(function, "params") : NULL;
  yyjson_val* blocks = hako_physical_blocks(function);
  size_t bi;
  int count = 0;
  if (params && yyjson_is_arr(params)) {
    size_t i;
    for (i = 0; i < yyjson_arr_size(params); i++) {
      uint32_t value;
      if (hako_physical_u32(yyjson_arr_get(params, i), &value) && value == wanted) count++;
    }
  }
  if (!(blocks && yyjson_is_arr(blocks))) return count;
  for (bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* rows = block ? yyjson_obj_get(block, "instructions") : NULL;
    yyjson_val* term = block ? yyjson_obj_get(block, "terminator") : NULL;
    size_t i;
    if (rows && yyjson_is_arr(rows)) for (i = 0; i < yyjson_arr_size(rows); i++) {
      yyjson_val* row = yyjson_arr_get(rows, i);
      uint32_t value;
      yyjson_val* dst = hako_physical_instruction_dst(row ? yyjson_obj_get(row, "instruction") : NULL);
      if (hako_physical_u32(dst, &value) && value == wanted) count++;
    }
    if (term) {
      uint32_t value;
      yyjson_val* dst = hako_physical_instruction_dst(yyjson_obj_get(term, "instruction"));
      if (hako_physical_u32(dst, &value) && value == wanted) count++;
    }
  }
  return count;
}

static int hako_physical_has_value(yyjson_val* function, uint32_t value) {
  return hako_physical_value_def_count(function, value) == 1;
}

static int hako_physical_layout_has_object(yyjson_val* layouts, uint32_t object_id) {
  size_t i;
  int count = 0;
  for (i = 0; layouts && i < yyjson_arr_size(layouts); i++) {
    uint32_t id;
    yyjson_val* layout = yyjson_arr_get(layouts, i);
    if (hako_physical_u32(layout ? yyjson_obj_get(layout, "object_id") : NULL, &id) && id == object_id)
      count++;
  }
  return count == 1;
}

static int hako_physical_layout_has_field(yyjson_val* layouts, uint32_t object_id, uint32_t ordinal) {
  size_t i;
  for (i = 0; layouts && i < yyjson_arr_size(layouts); i++) {
    yyjson_val* layout = yyjson_arr_get(layouts, i);
    uint32_t id;
    if (!hako_physical_u32(layout ? yyjson_obj_get(layout, "object_id") : NULL, &id) || id != object_id)
      continue;
    {
      yyjson_val* fields = yyjson_obj_get(layout, "fields");
      size_t fi;
      for (fi = 0; fields && fi < yyjson_arr_size(fields); fi++) {
        uint32_t field_ordinal;
        yyjson_val* field = yyjson_arr_get(fields, fi);
        if (hako_physical_u32(field ? yyjson_obj_get(field, "declaration_ordinal") : NULL, &field_ordinal) &&
            field_ordinal == ordinal) return 1;
      }
    }
  }
  return 0;
}

static int hako_physical_values_exist(yyjson_val* function, yyjson_val* values);

static int hako_physical_edge_args(yyjson_val* value) {
  static const char* const keys[] = {"layout", "values"};
  const char* layout;
  if (yyjson_is_null(value)) return 1;
  if (!hako_physical_exact_keys(value, keys, 2)) return 0;
  layout = yyjson_get_str(yyjson_obj_get(value, "layout"));
  return layout && (strcmp(layout, "carriers_only") == 0 ||
      strcmp(layout, "expr_result_plus_carriers") == 0) &&
      hako_physical_array_u32(yyjson_obj_get(value, "values"));
}

static int hako_physical_edge_args_equal(yyjson_val* left, yyjson_val* right) {
  yyjson_val *left_values, *right_values;
  size_t i;
  if (yyjson_is_null(left) || yyjson_is_null(right)) return yyjson_is_null(left) && yyjson_is_null(right);
  if (!hako_physical_edge_args(left) || !hako_physical_edge_args(right) ||
      !hako_physical_string(yyjson_obj_get(left, "layout"), yyjson_get_str(yyjson_obj_get(right, "layout")))) return 0;
  left_values = yyjson_obj_get(left, "values"); right_values = yyjson_obj_get(right, "values");
  if (yyjson_arr_size(left_values) != yyjson_arr_size(right_values)) return 0;
  for (i = 0; i < yyjson_arr_size(left_values); i++)
    if (yyjson_get_uint(yyjson_arr_get(left_values, i)) != yyjson_get_uint(yyjson_arr_get(right_values, i))) return 0;
  return 1;
}

static int hako_physical_validate_edges(yyjson_val* function, yyjson_val* edges) {
  static const char* const keys[] = {"target", "args"};
  size_t i;
  for (i = 0; edges && i < yyjson_arr_size(edges); i++) {
    yyjson_val* edge = yyjson_arr_get(edges, i); uint32_t target;
    if (!hako_physical_exact_keys(edge, keys, 2) ||
        !hako_physical_u32(yyjson_obj_get(edge, "target"), &target) ||
        !hako_physical_has_block(function, target) ||
        !hako_physical_edge_args(yyjson_obj_get(edge, "args"))) return 0;
    if (!yyjson_is_null(yyjson_obj_get(edge, "args")) &&
        !hako_physical_values_exist(function, yyjson_obj_get(yyjson_obj_get(edge, "args"), "values"))) return 0;
  }
  return 1;
}

static int hako_physical_validate_terminal_edges(yyjson_val* terminal, yyjson_val* edges) {
  yyjson_val* instruction = terminal ? yyjson_obj_get(terminal, "instruction") : NULL;
  const char* op = instruction ? yyjson_get_str(yyjson_obj_get(instruction, "op")) : NULL;
  uint32_t target;
  if (!op) return 0;
  if (!strcmp(op, "return") || !strcmp(op, "return_fault")) return yyjson_arr_size(edges) == 0;
  if (!strcmp(op, "jump")) {
    yyjson_val* edge;
    return yyjson_arr_size(edges) == 1 && (edge = yyjson_arr_get(edges, 0)) &&
      hako_physical_u32(yyjson_obj_get(instruction, "target"), &target) &&
      hako_physical_u32(yyjson_obj_get(edge, "target"), NULL) &&
      yyjson_get_uint(yyjson_obj_get(edge, "target")) == target &&
      hako_physical_edge_args_equal(yyjson_obj_get(instruction, "args"), yyjson_obj_get(edge, "args"));
  }
  if (!strcmp(op, "branch")) {
    yyjson_val *then_edge, *else_edge;
    return yyjson_arr_size(edges) == 2 && (then_edge = yyjson_arr_get(edges, 0)) && (else_edge = yyjson_arr_get(edges, 1)) &&
      hako_physical_u32(yyjson_obj_get(instruction, "then"), &target) && yyjson_get_uint(yyjson_obj_get(then_edge, "target")) == target &&
      hako_physical_u32(yyjson_obj_get(instruction, "else"), &target) && yyjson_get_uint(yyjson_obj_get(else_edge, "target")) == target &&
      hako_physical_edge_args_equal(yyjson_obj_get(instruction, "then_args"), yyjson_obj_get(then_edge, "args")) &&
      hako_physical_edge_args_equal(yyjson_obj_get(instruction, "else_args"), yyjson_obj_get(else_edge, "args"));
  }
  if (!strcmp(op, "invoke")) {
    yyjson_val *normal_edge, *fault_edge;
    return yyjson_arr_size(edges) == 2 && (normal_edge = yyjson_arr_get(edges, 0)) && (fault_edge = yyjson_arr_get(edges, 1)) &&
      hako_physical_u32(yyjson_obj_get(instruction, "normal"), &target) && yyjson_get_uint(yyjson_obj_get(normal_edge, "target")) == target &&
      hako_physical_u32(yyjson_obj_get(instruction, "fault"), &target) && yyjson_get_uint(yyjson_obj_get(fault_edge, "target")) == target &&
      yyjson_is_null(yyjson_obj_get(normal_edge, "args")) && yyjson_is_null(yyjson_obj_get(fault_edge, "args"));
  }
  return 0;
}

static int hako_physical_value_def_block(yyjson_val* function, uint32_t wanted, uint32_t* out) {
  yyjson_val* params = yyjson_obj_get(function, "params"); yyjson_val* blocks = hako_physical_blocks(function); size_t bi, i; int count = 0; uint32_t block = 0;
  for (i = 0; params && i < yyjson_arr_size(params); i++) { uint32_t value;
    if (hako_physical_u32(yyjson_arr_get(params, i), &value) && value == wanted) { count++; if (!hako_physical_u32(yyjson_obj_get(function, "entry"), &block)) return 0; }
  }
  for (bi = 0; blocks && bi < yyjson_arr_size(blocks); bi++) { yyjson_val* item = yyjson_arr_get(blocks, bi); yyjson_val* rows = yyjson_obj_get(item, "instructions"); uint32_t id;
    if (!hako_physical_u32(yyjson_obj_get(item, "id"), &id)) return 0;
    for (i = 0; rows && i < yyjson_arr_size(rows); i++) { uint32_t value; yyjson_val* dst = hako_physical_instruction_dst(yyjson_obj_get(yyjson_arr_get(rows, i), "instruction"));
      if (hako_physical_u32(dst, &value) && value == wanted) { count++; block = id; }
    }
  }
  if (count != 1) return 0; if (out) *out = block; return 1;
}

static int hako_physical_block_dominates(yyjson_val* function, uint32_t def, uint32_t use) {
  yyjson_val* blocks = hako_physical_blocks(function); uint32_t entry; size_t count, head = 0, tail = 0, i; unsigned char* seen; uint32_t* queue;
  if (def == use) return 1;
  if (!blocks || !hako_physical_u32(yyjson_obj_get(function, "entry"), &entry)) return 0;
  count = yyjson_arr_size(blocks); seen = calloc(count, 1u); queue = calloc(count, sizeof(*queue)); if (!seen || !queue) { free(seen); free(queue); return 0; }
  queue[tail++] = entry;
  while (head < tail) { uint32_t current = queue[head++]; yyjson_val* block; yyjson_val* edges; size_t bi, ei;
    if (current == def) continue; if (current == use) { free(seen); free(queue); return 0; }
    block = hako_physical_block_by_id(function, current); edges = block ? yyjson_obj_get(block, "edges") : NULL;
    for (ei = 0; edges && ei < yyjson_arr_size(edges); ei++) { uint32_t next; int known = 0;
      if (!hako_physical_u32(yyjson_obj_get(yyjson_arr_get(edges, ei), "target"), &next)) { free(seen); free(queue); return 0; }
      for (bi = 0; bi < tail; bi++) if (queue[bi] == next) known = 1;
      if (!known && tail < count) queue[tail++] = next;
    }
  }
  free(seen); free(queue); return 1;
}

static int hako_physical_value_available_on_edge(yyjson_val* function, uint32_t value, uint32_t predecessor) {
  uint32_t def;
  return hako_physical_value_def_block(function, value, &def) && hako_physical_block_dominates(function, def, predecessor);
}

static size_t hako_physical_predecessor_count(yyjson_val* function, uint32_t target) {
  yyjson_val* blocks = hako_physical_blocks(function); size_t bi, count = 0;
  for (bi = 0; blocks && bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi); yyjson_val* edges = yyjson_obj_get(block, "edges"); size_t ei;
    for (ei = 0; edges && ei < yyjson_arr_size(edges); ei++) { uint32_t edge_target;
      if (!hako_physical_u32(yyjson_obj_get(yyjson_arr_get(edges, ei), "target"), &edge_target)) return 0;
      if (edge_target == target) count++;
    }
  }
  return count;
}

static int hako_physical_phi_predecessors_exact(yyjson_val* function, uint32_t target, yyjson_val* inputs) {
  size_t i, j;
  if (!inputs || !yyjson_is_arr(inputs) || yyjson_arr_size(inputs) != hako_physical_predecessor_count(function, target)) return 0;
  for (i = 0; i < yyjson_arr_size(inputs); i++) { uint32_t pred;
    yyjson_val* input = yyjson_arr_get(inputs, i);
    if (!hako_physical_u32(yyjson_obj_get(input, "block"), &pred) || !hako_physical_block_has_edge_to(function, pred, target)) return 0;
    for (j = i + 1; j < yyjson_arr_size(inputs); j++) { uint32_t other;
      if (!hako_physical_u32(yyjson_obj_get(yyjson_arr_get(inputs, j), "block"), &other) || other == pred) return 0;
    }
  }
  return 1;
}

static int hako_physical_values_exist(yyjson_val* function, yyjson_val* values) {
  size_t i;
  if (!hako_physical_array_u32(values)) return 0;
  for (i = 0; i < yyjson_arr_size(values); i++) {
    uint32_t value;
    if (!hako_physical_u32(yyjson_arr_get(values, i), &value) || !hako_physical_has_value(function, value))
      return 0;
  }
  return 1;
}

static int hako_physical_frame_mode(yyjson_val* function, uint32_t frame, const char* expected) {
  yyjson_val* blocks = hako_physical_blocks(function);
  size_t bi;
  for (bi = 0; blocks && bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* rows = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
    size_t i;
    for (i = 0; rows && i < yyjson_arr_size(rows); i++) {
      yyjson_val* instruction = yyjson_obj_get(yyjson_arr_get(rows, i), "instruction");
      uint32_t dst;
      if (hako_physical_string(instruction ? yyjson_obj_get(instruction, "op") : NULL, "fault_frame_enter") &&
          hako_physical_u32(yyjson_obj_get(instruction, "dst"), &dst) && dst == frame)
        return hako_physical_string(yyjson_obj_get(instruction, "mode"), expected);
    }
  }
  return 0;
}

static int hako_physical_validate_call(yyjson_val* function, yyjson_val* functions, yyjson_val* call) {
  static const char* const keys[] = {"target", "receiver", "args", "dst"};
  uint32_t target, receiver;
  yyjson_val* callee;
  if (!hako_physical_exact_keys(call, keys, 4) ||
      !hako_physical_u32(yyjson_obj_get(call, "target"), &target) ||
      !hako_physical_u32(yyjson_obj_get(call, "receiver"), &receiver) ||
      !hako_physical_values_exist(function, yyjson_obj_get(call, "args")) ||
      !hako_physical_has_value(function, receiver) || target == 0 || target >= yyjson_arr_size(functions))
    return 0;
  callee = yyjson_arr_get(functions, target);
  return hako_physical_string(callee ? yyjson_obj_get(callee, "role") : NULL, "birth_unit") &&
      yyjson_arr_size(yyjson_obj_get(callee, "params")) == yyjson_arr_size(yyjson_obj_get(call, "args")) + 1 &&
      yyjson_is_null(yyjson_obj_get(call, "dst"));
}

static int hako_physical_validate_operation(yyjson_val* function, yyjson_val* functions,
    yyjson_val* layouts, yyjson_val* operation) {
  static const char* const call_keys[] = {"kind", "call"};
  static const char* const new_keys[] = {"kind", "object_id"};
  static const char* const field_keys[] = {"kind", "object_id", "field_ordinal", "base", "value"};
  static const char* const object_value_keys[] = {"kind", "object_id", "value"};
  const char* kind = operation ? yyjson_get_str(yyjson_obj_get(operation, "kind")) : NULL;
  uint32_t object_id, ordinal, value;
  if (!kind) return 0;
  if (strcmp(kind, "birth_call") == 0)
    return hako_physical_exact_keys(operation, call_keys, 2) && hako_physical_validate_call(function, functions, yyjson_obj_get(operation, "call"));
  if (strcmp(kind, "new_box") == 0)
    return hako_physical_exact_keys(operation, new_keys, 2) && hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) && hako_physical_layout_has_object(layouts, object_id);
  if (strcmp(kind, "field_set") == 0)
    return hako_physical_exact_keys(operation, field_keys, 5) && hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) && hako_physical_u32(yyjson_obj_get(operation, "field_ordinal"), &ordinal) && hako_physical_u32(yyjson_obj_get(operation, "base"), &value) && hako_physical_has_value(function, value) && hako_physical_u32(yyjson_obj_get(operation, "value"), &value) && hako_physical_has_value(function, value) && hako_physical_layout_has_field(layouts, object_id, ordinal);
  if (strcmp(kind, "home_release") == 0 || strcmp(kind, "reclaim_unpublished") == 0)
    return hako_physical_exact_keys(operation, object_value_keys, 3) && hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) && hako_physical_u32(yyjson_obj_get(operation, "value"), &value) && hako_physical_layout_has_object(layouts, object_id) && hako_physical_has_value(function, value);
  return 0;
}

static int hako_physical_validate_instruction(yyjson_val* function, yyjson_val* functions,
    yyjson_val* layouts, yyjson_val* instruction, int terminal, const char* frame_mode,
    uint32_t current_block) {
  static const char* const const_i64[] = {"op", "dst", "value"};
  static const char* const const_unit[] = {"op", "dst"};
  static const char* const binary[] = {"op", "dst", "lhs", "rhs"};
  static const char* const copy[] = {"op", "dst", "src"};
  static const char* const phi[] = {"op", "dst", "inputs"};
  static const char* const field_get[] = {"op", "dst", "base", "object_id", "field_ordinal"};
  static const char* const invoke[] = {"op", "operation", "fault_frame", "normal", "fault"};
  static const char* const normal_result[] = {"op", "invoke_block", "dst"};
  static const char* const return_fault[] = {"op", "fault_frame"};
  static const char* const frame_enter[] = {"op", "dst", "mode"};
  static const char* const branch[] = {"op", "condition", "then", "else", "then_args", "else_args"};
  static const char* const jump[] = {"op", "target", "args"};
  static const char* const ret[] = {"op", "value"};
  static const char* const birth[] = {"op", "call"};
  const char* op = instruction ? yyjson_get_str(yyjson_obj_get(instruction, "op")) : NULL;
  uint32_t a, b, c;
  if (!op) return 0;
  if (terminal && strcmp(op, "invoke") && strcmp(op, "branch") && strcmp(op, "jump") && strcmp(op, "return") && strcmp(op, "return_fault")) return 0;
  if (!terminal && (!strcmp(op, "invoke") || !strcmp(op, "branch") || !strcmp(op, "jump") || !strcmp(op, "return") || !strcmp(op, "return_fault"))) return 0;
  if (!strcmp(op, "const_i64")) return hako_physical_exact_keys(instruction, const_i64, 3) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && hako_physical_i64(yyjson_obj_get(instruction, "value"));
  if (!strcmp(op, "const_string")) return hako_physical_exact_keys(instruction, const_i64, 3) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && yyjson_is_str(yyjson_obj_get(instruction, "value"));
  if (!strcmp(op, "const_unit")) return hako_physical_exact_keys(instruction, const_unit, 2) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a);
  if (!strcmp(op, "add")) return hako_physical_exact_keys(instruction, binary, 4) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && hako_physical_u32(yyjson_obj_get(instruction, "lhs"), &b) && hako_physical_u32(yyjson_obj_get(instruction, "rhs"), &c) && hako_physical_has_value(function, b) && hako_physical_has_value(function, c);
  if (!strcmp(op, "copy")) return hako_physical_exact_keys(instruction, copy, 3) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && hako_physical_u32(yyjson_obj_get(instruction, "src"), &b) && hako_physical_has_value(function, b);
  if (!strcmp(op, "phi")) {
    yyjson_val* inputs = yyjson_obj_get(instruction, "inputs"); size_t i;
    if (!hako_physical_exact_keys(instruction, phi, 3) || !hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) || !hako_physical_phi_predecessors_exact(function, current_block, inputs)) return 0;
    for (i = 0; i < yyjson_arr_size(inputs); i++) { yyjson_val* item = yyjson_arr_get(inputs, i); static const char* const keys[] = {"block", "value"};
      if (!hako_physical_exact_keys(item, keys, 2) || !hako_physical_u32(yyjson_obj_get(item, "block"), &b) || !hako_physical_block_has_edge_to(function, b, current_block) || !hako_physical_u32(yyjson_obj_get(item, "value"), &c) || !hako_physical_value_available_on_edge(function, c, b)) return 0; }
    return 1;
  }
  if (!strcmp(op, "object_field_get")) return hako_physical_exact_keys(instruction, field_get, 5) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && hako_physical_u32(yyjson_obj_get(instruction, "base"), &b) && hako_physical_has_value(function, b) && hako_physical_u32(yyjson_obj_get(instruction, "object_id"), &b) && hako_physical_u32(yyjson_obj_get(instruction, "field_ordinal"), &c) && hako_physical_layout_has_field(layouts, b, c);
  if (!strcmp(op, "invoke")) return hako_physical_exact_keys(instruction, invoke, 5) && hako_physical_u32(yyjson_obj_get(instruction, "fault_frame"), &a) && hako_physical_frame_mode(function, a, frame_mode) && hako_physical_u32(yyjson_obj_get(instruction, "normal"), &b) && hako_physical_has_block(function, b) && hako_physical_u32(yyjson_obj_get(instruction, "fault"), &c) && hako_physical_has_block(function, c) && hako_physical_validate_operation(function, functions, layouts, yyjson_obj_get(instruction, "operation"));
  if (!strcmp(op, "invoke_normal_result")) { yyjson_val* invoke;
    return hako_physical_exact_keys(instruction, normal_result, 3) && hako_physical_u32(yyjson_obj_get(instruction, "invoke_block"), &a) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &b) && (invoke = hako_physical_block_invoke_terminator(function, a)) && hako_physical_u32(yyjson_obj_get(invoke, "normal"), &c) && c == current_block && hako_physical_string(yyjson_obj_get(yyjson_obj_get(invoke, "operation"), "kind"), "new_box");
  }
  if (!strcmp(op, "fault_frame_enter")) return hako_physical_exact_keys(instruction, frame_enter, 3) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && hako_physical_string(yyjson_obj_get(instruction, "mode"), frame_mode);
  if (!strcmp(op, "birth_call")) return hako_physical_exact_keys(instruction, birth, 2) && hako_physical_validate_call(function, functions, yyjson_obj_get(instruction, "call"));
  if (!strcmp(op, "return_fault")) return hako_physical_exact_keys(instruction, return_fault, 2) && hako_physical_u32(yyjson_obj_get(instruction, "fault_frame"), &a) && hako_physical_frame_mode(function, a, frame_mode);
  if (!strcmp(op, "return")) return hako_physical_exact_keys(instruction, ret, 2) &&
      ((hako_physical_string(yyjson_obj_get(function, "role"), "root_i64") && hako_physical_u32(yyjson_obj_get(instruction, "value"), &a) && hako_physical_has_value(function, a)) ||
       (hako_physical_string(yyjson_obj_get(function, "role"), "birth_unit") && yyjson_is_null(yyjson_obj_get(instruction, "value"))));
  if (!strcmp(op, "branch")) return hako_physical_exact_keys(instruction, branch, 6) && hako_physical_u32(yyjson_obj_get(instruction, "condition"), &a) && hako_physical_has_value(function, a) && hako_physical_u32(yyjson_obj_get(instruction, "then"), &b) && hako_physical_has_block(function, b) && hako_physical_u32(yyjson_obj_get(instruction, "else"), &c) && hako_physical_has_block(function, c) && hako_physical_edge_args(yyjson_obj_get(instruction, "then_args")) && hako_physical_edge_args(yyjson_obj_get(instruction, "else_args"));
  if (!strcmp(op, "jump")) return hako_physical_exact_keys(instruction, jump, 3) && hako_physical_u32(yyjson_obj_get(instruction, "target"), &a) && hako_physical_has_block(function, a) && hako_physical_edge_args(yyjson_obj_get(instruction, "args"));
  return 0;
}

static int hako_physical_validate_layouts(yyjson_val* layouts) {
  static const char* const layout_keys[] = {"object_id", "runtime_type_id", "field_count", "fields"};
  static const char* const field_keys[] = {"declaration_ordinal", "runtime_slot", "storage_kind"};
  size_t i, j;
  if (!(layouts && yyjson_is_arr(layouts) && yyjson_arr_size(layouts))) return 0;
  for (i = 0; i < yyjson_arr_size(layouts); i++) {
    yyjson_val* layout = yyjson_arr_get(layouts, i); yyjson_val* fields; uint32_t object, count;
    if (!hako_physical_exact_keys(layout, layout_keys, 4) || !hako_physical_u32(yyjson_obj_get(layout, "object_id"), &object) || !hako_physical_u32(yyjson_obj_get(layout, "runtime_type_id"), NULL) || !hako_physical_u32(yyjson_obj_get(layout, "field_count"), &count) || !(fields = yyjson_obj_get(layout, "fields")) || !yyjson_is_arr(fields) || yyjson_arr_size(fields) != count) return 0;
    for (j = 0; j < yyjson_arr_size(fields); j++) { uint32_t ordinal, storage; yyjson_val* field = yyjson_arr_get(fields, j);
      if (!hako_physical_exact_keys(field, field_keys, 3) || !hako_physical_u32(yyjson_obj_get(field, "declaration_ordinal"), &ordinal) || ordinal != j || !hako_physical_u32(yyjson_obj_get(field, "runtime_slot"), &storage) || storage != ordinal || !hako_physical_u32(yyjson_obj_get(field, "storage_kind"), &storage) || storage != 1u) return 0; }
    for (j = i + 1; j < yyjson_arr_size(layouts); j++) { uint32_t other; if (!hako_physical_u32(yyjson_obj_get(yyjson_arr_get(layouts, j), "object_id"), &other) || object == other) return 0; }
  }
  return 1;
}

static int hako_physical_storage_profile(yyjson_val* value) {
  uint32_t profile;
  return hako_physical_u32(value, &profile) && (profile == 1u || profile == 2u);
}

static int hako_physical_validate_function(yyjson_val* function, yyjson_val* functions, yyjson_val* layouts, size_t ordinal) {
  static const char* const keys[] = {"name", "role", "params", "entry", "blocks"};
  yyjson_val* blocks; yyjson_val* params; uint32_t entry; const char* mode; size_t bi, i, j;
  if (!hako_physical_exact_keys(function, keys, 5) || !(yyjson_get_str(yyjson_obj_get(function, "name"))) || !(params = yyjson_obj_get(function, "params")) || !hako_physical_array_u32(params) || !hako_physical_u32(yyjson_obj_get(function, "entry"), &entry) || !(blocks = hako_physical_blocks(function)) || !yyjson_is_arr(blocks) || !yyjson_arr_size(blocks) || !hako_physical_unique_block_ids(function) || !hako_physical_has_block(function, entry)) return 0;
  if (ordinal == 0) { if (!hako_physical_string(yyjson_obj_get(function, "role"), "root_i64") || yyjson_arr_size(params)) return 0; mode = "root_owned"; }
  else { if (!hako_physical_string(yyjson_obj_get(function, "role"), "birth_unit") || !yyjson_arr_size(params)) return 0; mode = "borrowed"; }
  for (i = 0; i < yyjson_arr_size(params); i++) for (j = i + 1; j < yyjson_arr_size(params); j++) { uint32_t a, b; if (!hako_physical_u32(yyjson_arr_get(params, i), &a) || !hako_physical_u32(yyjson_arr_get(params, j), &b) || a == b) return 0; }
  for (bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    static const char* const block_keys[] = {"id", "instructions", "terminator", "edges"};
    yyjson_val* block = yyjson_arr_get(blocks, bi); yyjson_val* rows; yyjson_val* term; yyjson_val* edges; uint32_t block_id;
    if (!hako_physical_exact_keys(block, block_keys, 4) || !hako_physical_u32(yyjson_obj_get(block, "id"), &block_id) || !(rows = yyjson_obj_get(block, "instructions")) || !yyjson_is_arr(rows) || !(term = yyjson_obj_get(block, "terminator")) || !yyjson_is_obj(term) || !(edges = yyjson_obj_get(block, "edges")) || !yyjson_is_arr(edges)) return 0;
    for (i = 0; i < yyjson_arr_size(rows); i++) { static const char* const row_keys[] = {"index", "instruction"}; yyjson_val* row = yyjson_arr_get(rows, i); uint32_t index;
      if (!hako_physical_exact_keys(row, row_keys, 2) || !hako_physical_u32(yyjson_obj_get(row, "index"), &index) || index != i || !hako_physical_validate_instruction(function, functions, layouts, yyjson_obj_get(row, "instruction"), 0, mode, block_id)) return 0; }
    { static const char* const row_keys[] = {"index", "instruction"}; uint32_t index;
      if (!hako_physical_exact_keys(term, row_keys, 2) || !hako_physical_u32(yyjson_obj_get(term, "index"), &index) || index != yyjson_arr_size(rows) || !hako_physical_validate_instruction(function, functions, layouts, yyjson_obj_get(term, "instruction"), 1, mode, block_id)) return 0; }
    if (!hako_physical_validate_edges(function, edges) || !hako_physical_validate_terminal_edges(term, edges)) return 0;
    for (i = 0; i < yyjson_arr_size(rows); i++) { yyjson_val* dst = hako_physical_instruction_dst(yyjson_obj_get(yyjson_arr_get(rows, i), "instruction")); uint32_t value; if (hako_physical_u32(dst, &value) && !hako_physical_has_value(function, value)) return 0; }
  }
  return 1;
}

static int hako_llvmc_validate_published_lifecycle_physical_v1_doc(yyjson_val* root, char** err_out) {
  static const char* const root_keys[] = {"schema", "functions", "fault_abi_version", "storage_profile", "layouts"};
  yyjson_val *functions, *layouts; size_t i;
  if (!hako_physical_exact_keys(root, root_keys, 5) || !hako_physical_string(yyjson_obj_get(root, "schema"), HAKO_PHYSICAL_SCHEMA)) return hako_physical_fail(err_out, "schema");
  functions = yyjson_obj_get(root, "functions"); layouts = yyjson_obj_get(root, "layouts");
  if (!(functions && yyjson_is_arr(functions) && yyjson_arr_size(functions) >= 2u)) return hako_physical_fail(err_out, "functions");
  if (!hako_physical_u32(yyjson_obj_get(root, "fault_abi_version"), NULL) || yyjson_get_uint(yyjson_obj_get(root, "fault_abi_version")) != 1u || !hako_physical_storage_profile(yyjson_obj_get(root, "storage_profile")) || !hako_physical_validate_layouts(layouts)) return hako_physical_fail(err_out, "abi-layout");
  for (i = 0; i < yyjson_arr_size(functions); i++) if (!hako_physical_validate_function(yyjson_arr_get(functions, i), functions, layouts, i)) return hako_physical_fail(err_out, "function-body");
  return 0;
}
