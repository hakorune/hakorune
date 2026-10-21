/* Call-local decoder for hako.published-lifecycle-physical-program.v2.
 * It validates final-view transport before LLVM is touched. It never resolves
 * source names or supplements a missing physical relation. */

#define HAKO_PHYSICAL_SCHEMA "hako.published-lifecycle-physical-program.v2"

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

static int hako_physical_u64(yyjson_val* value, uint64_t* out) {
  if (!(value && yyjson_is_uint(value))) return 0;
  if (out) *out = yyjson_get_uint(value);
  return 1;
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

#include "hako_llvmc_ffi_published_lifecycle_result_transport.inc"
#include "hako_llvmc_ffi_published_lifecycle_formal_transport.inc"

static yyjson_val* hako_physical_blocks(yyjson_val* function) {
  return function ? yyjson_obj_get(function, "blocks") : NULL;
}

static yyjson_val* hako_physical_block_by_id(yyjson_val* function, uint32_t wanted) {
  yyjson_val* blocks = hako_physical_blocks(function);
  yyjson_arr_iter iter;
  yyjson_val* block;
  if (!(blocks && yyjson_is_arr(blocks))) return NULL;
  iter = yyjson_arr_iter_with(blocks);
  while ((block = yyjson_arr_iter_next(&iter))) {
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
  if (!(blocks && yyjson_is_arr(blocks))) return 0;
  yyjson_arr_iter outer = yyjson_arr_iter_with(blocks);
  yyjson_val* block;
  while ((block = yyjson_arr_iter_next(&outer))) {
    uint32_t left;
    if (!hako_physical_u32(yyjson_obj_get(block, "id"), &left)) return 0;
    /* Copy the cursor after left: preserve the same ordered pair census. */
    yyjson_arr_iter inner = outer;
    yyjson_val* other;
    while ((other = yyjson_arr_iter_next(&inner))) {
      uint32_t right;
      if (!hako_physical_u32(yyjson_obj_get(other, "id"), &right) || left == right) return 0;
    }
  }
  return 1;
}

static yyjson_val* hako_physical_instruction_dst(yyjson_val* instruction) {
  const char* op = instruction ? yyjson_get_str(yyjson_obj_get(instruction, "op")) : NULL;
  yyjson_val* call;
  if (!op) return NULL;
  if (strcmp(op, "const_i64") == 0 || strcmp(op, "const_f64_bits") == 0 ||
      strcmp(op, "const_bool") == 0 || strcmp(op, "const_string") == 0 ||
      strcmp(op, "const_unit") == 0 || strcmp(op, "const_null") == 0 ||
      strcmp(op, "add") == 0 || strcmp(op, "mul") == 0 ||
      strcmp(op, "sub") == 0 || strcmp(op, "div") == 0 || strcmp(op, "mod") == 0 ||
      strcmp(op, "compare") == 0 ||
      strcmp(op, "borrowed_null_compare") == 0 ||
      strcmp(op, "copy") == 0 || strcmp(op, "phi") == 0 ||
      strcmp(op, "object_field_get") == 0 || strcmp(op, "invoke_normal_result") == 0 ||
      strcmp(op, "array_get") == 0 || strcmp(op, "array_length") == 0 ||
      strcmp(op, "fault_frame_enter") == 0)
    return yyjson_obj_get(instruction, "dst");
  if (strcmp(op, "birth_call") != 0) return NULL;
  call = yyjson_obj_get(instruction, "call");
  return call ? yyjson_obj_get(call, "dst") : NULL;
}

static int hako_physical_value_def_count(yyjson_val* function, uint32_t wanted) {
  yyjson_val* params = function ? yyjson_obj_get(function, "params") : NULL;
  yyjson_val* blocks = hako_physical_blocks(function);
  int count = 0;
  if (params && yyjson_is_arr(params)) {
    size_t i;
    for (i = 0; i < hako_physical_formal_count(function); i++) {
      uint32_t value;
      if (hako_physical_u32(hako_physical_formal_value(function, i), &value) && value == wanted) count++;
    }
  }
  if (!(blocks && yyjson_is_arr(blocks))) return count;
  yyjson_arr_iter block_iter = yyjson_arr_iter_with(blocks);
  yyjson_val* block;
  while ((block = yyjson_arr_iter_next(&block_iter))) {
    yyjson_val* term = yyjson_obj_get(block, "terminator");
    yyjson_arr_iter row_iter = yyjson_arr_iter_with(yyjson_obj_get(block, "instructions"));
    yyjson_val* row;
    while ((row = yyjson_arr_iter_next(&row_iter))) {
      uint32_t value;
      yyjson_val* dst = hako_physical_instruction_dst(yyjson_obj_get(row, "instruction"));
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

/* Exact field storage in the supplied layout; zero is never a valid field kind. */
static uint32_t hako_physical_field_storage(yyjson_val* layouts, uint32_t object, uint32_t ordinal) {
  for (size_t i = 0; i < yyjson_arr_size(layouts); i++) {
    yyjson_val* layout = yyjson_arr_get(layouts, i);
    if (yyjson_get_uint(yyjson_obj_get(layout, "object_id")) != object) continue;
    yyjson_val* fields = yyjson_obj_get(layout, "fields");
    if (ordinal >= yyjson_arr_size(fields)) return 0;
    return (uint32_t)yyjson_get_uint(yyjson_obj_get(yyjson_arr_get(fields, ordinal), "storage_kind"));
  }
  return 0;
}

/* Every drain names exactly the supplied residence, never a numeric payload. */
static int hako_physical_drain_matches(yyjson_val* layouts, yyjson_val* op) {
  uint32_t object = (uint32_t)yyjson_get_uint(yyjson_obj_get(op, "object_id"));
  uint32_t ordinal = (uint32_t)yyjson_get_uint(yyjson_obj_get(op, "field_ordinal"));
  int field = hako_physical_string(yyjson_obj_get(op, "kind"), "field_residence_release");
  uint32_t storage = hako_physical_field_storage(layouts, object, ordinal);
  if (storage == HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC)
    return field && hako_physical_string(yyjson_obj_get(op, "transaction"), "take_dynamic");
  if (storage != HAKO_LLVMC_LIFECYCLE_STORAGE_I64 || yyjson_obj_get(op, "transaction")) return 0;
  for (size_t i = 0; i < yyjson_arr_size(layouts); i++) {
    yyjson_val* layout = yyjson_arr_get(layouts, i);
    if (yyjson_get_uint(yyjson_obj_get(layout, "object_id")) != object) continue;
    yyjson_val* rows = yyjson_obj_get(layout, field ? "owned_residences" : "owned_object_residences");
    for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
      yyjson_val* row = yyjson_arr_get(rows, ri);
      if (field ? yyjson_get_uint(row) == ordinal :
          yyjson_get_uint(yyjson_obj_get(row, "field_ordinal")) == ordinal &&
          yyjson_get_uint(yyjson_obj_get(row, "child_object_id")) == yyjson_get_uint(yyjson_obj_get(op, "child_object_id"))) return 1;
    }
  }
  return 0;
}

static int hako_physical_values_available_at(yyjson_val* function, yyjson_val* values,
    uint32_t block_id, uint32_t index);

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
  yyjson_val* params = yyjson_obj_get(function, "params"); yyjson_val* blocks = hako_physical_blocks(function); size_t i; int count = 0; uint32_t block = 0;
  for (i = 0; params && i < hako_physical_formal_count(function); i++) { uint32_t value;
    if (hako_physical_u32(hako_physical_formal_value(function, i), &value) && value == wanted) { count++; if (!hako_physical_u32(yyjson_obj_get(function, "entry"), &block)) return 0; }
  }
  yyjson_arr_iter block_iter = yyjson_arr_iter_with(blocks);
  yyjson_val* item;
  while ((item = yyjson_arr_iter_next(&block_iter))) {
    uint32_t id;
    if (!hako_physical_u32(yyjson_obj_get(item, "id"), &id)) return 0;
    yyjson_arr_iter row_iter = yyjson_arr_iter_with(yyjson_obj_get(item, "instructions"));
    yyjson_val* row;
    while ((row = yyjson_arr_iter_next(&row_iter))) {
      uint32_t value;
      yyjson_val* dst = hako_physical_instruction_dst(yyjson_obj_get(row, "instruction"));
      if (hako_physical_u32(dst, &value) && value == wanted) { count++; block = id; }
    }
  }
  if (count != 1) return 0; if (out) *out = block; return 1;
}

/* Query-local geometry borrows the same function/edges; it issues no facts. */
typedef struct { uint32_t id; yyjson_val* block; } hako_physical_dom_block;
static int hako_physical_dom_block_compare(const void* left, const void* right) {
  uint32_t a = ((const hako_physical_dom_block*)left)->id;
  uint32_t b = ((const hako_physical_dom_block*)right)->id;
  return (a > b) - (a < b);
}
static hako_physical_dom_block* hako_physical_dom_find(
    hako_physical_dom_block* blocks, size_t count, uint32_t id) {
  hako_physical_dom_block key = {id, NULL};
  return bsearch(&key, blocks, count, sizeof(key), hako_physical_dom_block_compare);
}
static int hako_physical_block_dominates(yyjson_val* function, uint32_t def, uint32_t use) {
  yyjson_val* blocks = hako_physical_blocks(function);
  uint32_t entry;
  size_t count, head = 0, tail = 0;
  int result = 0;
  hako_physical_dom_block* indexed = NULL;
  unsigned char* seen = NULL;
  size_t* queue = NULL;
  if (def == use) return 1;
  if (!blocks || !hako_physical_u32(yyjson_obj_get(function, "entry"), &entry)) return 0;
  count = yyjson_arr_size(blocks);
  if (!count) return 0;
  indexed = calloc(count, sizeof(*indexed));
  seen = calloc(count, sizeof(*seen));
  queue = calloc(count, sizeof(*queue));
  if (!indexed || !seen || !queue) goto done;
  yyjson_arr_iter block_iter = yyjson_arr_iter_with(blocks);
  yyjson_val* block;
  size_t block_ordinal = 0;
  while ((block = yyjson_arr_iter_next(&block_iter))) {
    indexed[block_ordinal].block = block;
    if (!hako_physical_u32(yyjson_obj_get(block, "id"), &indexed[block_ordinal].id)) goto done;
    block_ordinal++;
  }
  qsort(indexed, count, sizeof(*indexed), hako_physical_dom_block_compare);
  for (size_t i = 1; i < count; i++) if (indexed[i - 1].id == indexed[i].id) goto done;
  hako_physical_dom_block* first = hako_physical_dom_find(indexed, count, entry);
  if (!first) goto done;
  queue[tail++] = (size_t)(first - indexed);
  seen[first - indexed] = 1;
  while (head < tail) {
    size_t ordinal = queue[head++];
    hako_physical_dom_block* current = &indexed[ordinal];
    if (current->id == def) continue;
    if (current->id == use) goto done;
    yyjson_val* edges = yyjson_obj_get(current->block, "edges");
    yyjson_arr_iter iter = yyjson_arr_iter_with(edges);
    yyjson_val* edge;
    while ((edge = yyjson_arr_iter_next(&iter))) {
      uint32_t next;
      if (!hako_physical_u32(yyjson_obj_get(edge, "target"), &next)) goto done;
      hako_physical_dom_block* successor = hako_physical_dom_find(indexed, count, next);
      if (!successor) goto done;
      size_t next_ordinal = (size_t)(successor - indexed);
      if (!seen[next_ordinal]) {
        seen[next_ordinal] = 1;
        queue[tail++] = next_ordinal;
      }
    }
  }
  /* As before, unreachable uses have no avoiding path and satisfy dominance. */
  result = 1;
done:
  free(indexed); free(seen); free(queue);
  return result;
}

static int hako_physical_value_available_on_edge(yyjson_val* function, uint32_t value, uint32_t predecessor) {
  uint32_t def;
  return hako_physical_value_def_block(function, value, &def) && hako_physical_block_dominates(function, def, predecessor);
}

static int hako_physical_is_const_unit(yyjson_val* function, uint32_t value) {
  yyjson_val* blocks = hako_physical_blocks(function); size_t bi, i;
  for (bi = 0; blocks && bi < yyjson_arr_size(blocks); bi++) { yyjson_val* rows = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
    for (i = 0; rows && i < yyjson_arr_size(rows); i++) { yyjson_val* instruction = yyjson_obj_get(yyjson_arr_get(rows, i), "instruction"); uint32_t dst;
      if (hako_physical_string(yyjson_obj_get(instruction, "op"), "const_unit") && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &dst) && dst == value) return 1;
    }
  }
  return 0;
}

static int hako_physical_value_available_at(yyjson_val* function, uint32_t value, uint32_t block_id, uint32_t index) {
  yyjson_val* block = hako_physical_block_by_id(function, block_id); yyjson_val* rows = block ? yyjson_obj_get(block, "instructions") : NULL;
  uint32_t def; size_t i;
  yyjson_val* params = yyjson_obj_get(function, "params");
  if (!hako_physical_value_def_block(function, value, &def)) return 0;
  for (i = 0; params && i < hako_physical_formal_count(function); i++) { uint32_t param;
    if (hako_physical_u32(hako_physical_formal_value(function, i), &param) && param == value) return 1;
  }
  if (def != block_id) return hako_physical_block_dominates(function, def, block_id);
  for (i = 0; rows && i < index; i++) { uint32_t dst;
    if (hako_physical_u32(hako_physical_instruction_dst(yyjson_obj_get(yyjson_arr_get(rows, i), "instruction")), &dst) && dst == value) return 1;
  }
  return 0;
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

static int hako_physical_values_available_at(yyjson_val* function, yyjson_val* values,
    uint32_t block_id, uint32_t index) {
  size_t i;
  if (!values || yyjson_is_null(values)) return 1;
  if (!hako_physical_array_u32(values)) return 0;
  for (i = 0; i < yyjson_arr_size(values); i++) {
    uint32_t value;
    if (!hako_physical_u32(yyjson_arr_get(values, i), &value) ||
        !hako_physical_value_available_at(function, value, block_id, index)) return 0;
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

#include "hako_llvmc_ffi_published_lifecycle_call_transport.inc"

#include "hako_llvmc_ffi_published_lifecycle_map_operations.inc"
#include "hako_llvmc_ffi_published_lifecycle_instruction_schema.inc"

static int hako_physical_validate_layouts(yyjson_val* layouts) {
  static const char* const layout_keys[] = {"object_id", "runtime_type_id", "field_count", "fields", "owned_residences"};
  static const char* const owned_layout_keys[] = {"object_id", "runtime_type_id", "field_count", "fields", "owned_residences", "owned_object_residences"};
  static const char* const field_keys[] = {"declaration_ordinal", "runtime_slot", "storage_kind"};
  size_t i, j;
  if (!(layouts && yyjson_is_arr(layouts))) return 0;
  for (i = 0; i < yyjson_arr_size(layouts); i++) {
    yyjson_val* layout = yyjson_arr_get(layouts, i); yyjson_val* fields; yyjson_val* owned; uint32_t object, count;
    if (!(hako_physical_exact_keys(layout, layout_keys, 5) || hako_physical_exact_keys(layout, owned_layout_keys, 6)) || !hako_physical_u32(yyjson_obj_get(layout, "object_id"), &object) || !hako_physical_u32(yyjson_obj_get(layout, "runtime_type_id"), NULL) || !hako_physical_u32(yyjson_obj_get(layout, "field_count"), &count) || !(fields = yyjson_obj_get(layout, "fields")) || !yyjson_is_arr(fields) || yyjson_arr_size(fields) != count || !(owned = yyjson_obj_get(layout, "owned_residences")) || !yyjson_is_arr(owned)) return 0;
    for (j = 0; j < yyjson_arr_size(fields); j++) { uint32_t ordinal, storage; yyjson_val* field = yyjson_arr_get(fields, j);
      if (!hako_physical_exact_keys(field, field_keys, 3) || !hako_physical_u32(yyjson_obj_get(field, "declaration_ordinal"), &ordinal) || ordinal != j || !hako_physical_u32(yyjson_obj_get(field, "runtime_slot"), &storage) || storage != ordinal || !hako_physical_u32(yyjson_obj_get(field, "storage_kind"), &storage) || (storage != HAKO_LLVMC_LIFECYCLE_STORAGE_I64 && storage != HAKO_LLVMC_LIFECYCLE_STORAGE_DYNAMIC)) return 0; }
    { uint32_t previous = 0;
      for (j = 0; j < yyjson_arr_size(owned); j++) { uint32_t ordinal;
        /* Owned residences are declared `ArrayBox` field ordinals in
           declaration order: unique, ascending, resolving in this
           layout's own field rows — the release consumer walks them
           newest-first. */
        if (!hako_physical_u32(yyjson_arr_get(owned, j), &ordinal) || ordinal >= count || (j && ordinal <= previous) ||
            hako_physical_field_storage(layouts, object, ordinal) != HAKO_LLVMC_LIFECYCLE_STORAGE_I64) return 0;
        previous = ordinal; } }
    if (!hako_physical_owned_object_layout(layout, layouts, owned, count)) return 0;
    for (j = i + 1; j < yyjson_arr_size(layouts); j++) { uint32_t other; if (!hako_physical_u32(yyjson_obj_get(yyjson_arr_get(layouts, j), "object_id"), &other) || object == other) return 0; }
  }
  return 1;
}

static int hako_physical_storage_profile(yyjson_val* value) {
  uint32_t profile;
  return hako_physical_u32(value, &profile) && (profile == 1u || profile == 2u);
}

static int hako_physical_validate_function(yyjson_val* function, yyjson_val* functions, yyjson_val* layouts, size_t ordinal, int native) {
  static const char* const keys[] = {"name", "role", "receiver", "receiver_object", "params", "entry", "blocks"};
  static const char* const borrowed_keys[] = {"name", "role", "receiver", "receiver_object", "params", "entry", "blocks", "result_formal_ordinal"};
  int borrowed = (hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_borrowed_nullable_handle") ||
         hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_mixed_handle"));
  yyjson_val* blocks; yyjson_val* params; uint32_t entry; const char* mode; size_t bi, i, j;
  if ((borrowed && native) || !hako_physical_exact_keys(function, borrowed ? borrowed_keys : keys, borrowed ? 8 : 7) || !(yyjson_get_str(yyjson_obj_get(function, "name"))) || !(params = yyjson_obj_get(function, "params")) || !hako_physical_params(function, layouts) || !hako_physical_u32(yyjson_obj_get(function, "entry"), &entry) || !(blocks = hako_physical_blocks(function)) || !yyjson_is_arr(blocks) || !yyjson_arr_size(blocks) || !hako_physical_unique_block_ids(function) || !hako_physical_has_block(function, entry)) return 0;
  if (borrowed) {
    uint32_t support;
    if (!hako_physical_u32(yyjson_obj_get(function, "result_formal_ordinal"), &support) ||
        support >= yyjson_arr_size(params) ||
        !hako_physical_string(yyjson_obj_get(yyjson_arr_get(params, support), "representation"),
            "borrowed_kind_payload_v1")) return 0;
  }
  if (ordinal == 0) { if (!(hako_physical_string(yyjson_obj_get(function, "role"), "root_i64") || (native && hako_physical_string(yyjson_obj_get(function, "role"), "root_unit"))) || yyjson_arr_size(params) || !yyjson_is_null(yyjson_obj_get(function, "receiver")) || !yyjson_is_null(yyjson_obj_get(function, "receiver_object"))) return 0; mode = "root_owned"; }
  else if (hako_physical_string(yyjson_obj_get(function, "role"), "birth_unit")) {
    if (!hako_physical_u32(yyjson_obj_get(function, "receiver"), NULL) || !yyjson_is_null(yyjson_obj_get(function, "receiver_object"))) return 0;
    mode = "borrowed";
  } else if (hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_unit") ||
        hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_bool") ||
      hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_i64") ||
      hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_map") ||
      hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_text") ||
      hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_handle") ||
      hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_nullable_handle") || borrowed) {
    yyjson_val* receiver = yyjson_obj_get(function, "receiver");
    yyjson_val* receiver_object = yyjson_obj_get(function, "receiver_object");
    if (!yyjson_is_null(receiver) &&
        (!hako_physical_u32(receiver, NULL) || !hako_physical_u32(receiver_object, NULL) ||
         !hako_physical_layout_has_object(layouts, yyjson_get_uint(receiver_object)))) return 0;
    if (yyjson_is_null(receiver) && !yyjson_is_null(receiver_object)) return 0;
    mode = "borrowed";
  } else return 0;
  for (i = 0; i < hako_physical_formal_count(function); i++) for (j = i + 1; j < hako_physical_formal_count(function); j++) { uint32_t a, b; if (!hako_physical_u32(hako_physical_formal_value(function, i), &a) || !hako_physical_u32(hako_physical_formal_value(function, j), &b) || a == b) return 0; }
  for (bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    static const char* const block_keys[] = {"id", "instructions", "terminator", "edges"};
    yyjson_val* block = yyjson_arr_get(blocks, bi); yyjson_val* rows; yyjson_val* term; yyjson_val* edges; uint32_t block_id;
    if (!hako_physical_exact_keys(block, block_keys, 4) || !hako_physical_u32(yyjson_obj_get(block, "id"), &block_id) || !(rows = yyjson_obj_get(block, "instructions")) || !yyjson_is_arr(rows) || !(term = yyjson_obj_get(block, "terminator")) || !yyjson_is_obj(term) || !(edges = yyjson_obj_get(block, "edges")) || !yyjson_is_arr(edges)) return 0;
    for (i = 0; i < yyjson_arr_size(rows); i++) { static const char* const row_keys[] = {"index", "instruction"}; yyjson_val* row = yyjson_arr_get(rows, i); uint32_t index;
      if (!hako_physical_exact_keys(row, row_keys, 2) || !hako_physical_u32(yyjson_obj_get(row, "index"), &index) || index != i || !hako_physical_validate_instruction(function, functions, layouts, yyjson_obj_get(row, "instruction"), 0, mode, block_id, index, native)) return 0; }
    { static const char* const row_keys[] = {"index", "instruction"}; uint32_t index;
      if (!hako_physical_exact_keys(term, row_keys, 2) || !hako_physical_u32(yyjson_obj_get(term, "index"), &index) || index != yyjson_arr_size(rows) || !hako_physical_validate_instruction(function, functions, layouts, yyjson_obj_get(term, "instruction"), 1, mode, block_id, index, native)) return 0; }
    if (!hako_physical_validate_edges(function, edges) || !hako_physical_validate_terminal_edges(term, edges)) return 0;
    for (i = 0; i < yyjson_arr_size(rows); i++) { yyjson_val* dst = hako_physical_instruction_dst(yyjson_obj_get(yyjson_arr_get(rows, i), "instruction")); uint32_t value; if (hako_physical_u32(dst, &value) && !hako_physical_has_value(function, value)) return 0; }
  }
  return 1;
}

#include "hako_llvmc_ffi_physical_diagnostic_sites.inc"

static int hako_llvmc_validate_published_lifecycle_physical_v2_doc(yyjson_val* root, char** err_out) {
  static const char* const object_keys[] = {"schema", "functions", "fault_abi_version", "storage_profile", "layouts", "process_result_site"};
  static const char* const native_keys[] = {"schema", "functions", "fault_abi_version", "runtime_requirements", "process_result_site"};
  static const char* const requirement_keys[] = {"kind", "abi_version"};
  yyjson_val *functions, *layouts, *requirements = yyjson_obj_get(root, "runtime_requirements");
  int native = requirements != NULL;
  uint64_t process_result_site;
  if (!hako_physical_exact_keys(root, native ? native_keys : object_keys, native ? 5 : 6) ||
      !hako_physical_string(yyjson_obj_get(root, "schema"), HAKO_PHYSICAL_SCHEMA))
    return hako_physical_fail(err_out, "schema");
  if (native && (!hako_physical_exact_keys(requirements, requirement_keys, 2) ||
      !hako_physical_string(yyjson_obj_get(requirements, "kind"), "native_array") ||
      !hako_physical_u32(yyjson_obj_get(requirements, "abi_version"), NULL) ||
      yyjson_get_uint(yyjson_obj_get(requirements, "abi_version")) != 1))
    return hako_physical_fail(err_out, "runtime-requirements");
  functions = yyjson_obj_get(root, "functions"); layouts = yyjson_obj_get(root, "layouts");
  if (!yyjson_is_arr(functions) || (native ? yyjson_arr_size(functions) != 1 : yyjson_arr_size(functions) < 1))
    return hako_physical_fail(err_out, "functions");
  if (!hako_physical_u32(yyjson_obj_get(root, "fault_abi_version"), NULL) ||
      yyjson_get_uint(yyjson_obj_get(root, "fault_abi_version")) != 1 ||
      (!native && (!hako_physical_storage_profile(yyjson_obj_get(root, "storage_profile")) ||
       !hako_physical_validate_layouts(layouts)))) return hako_physical_fail(err_out, "abi-layout");
  for (size_t i = 0; i < yyjson_arr_size(functions); i++)
    if (!hako_physical_validate_function(yyjson_arr_get(functions, i), functions, layouts, i, native))
      return hako_physical_fail(err_out, "function-body");
  if (!hako_physical_u64(yyjson_obj_get(root, "process_result_site"), &process_result_site) ||
      !hako_physical_checked_sites_unique(functions, process_result_site))
    return hako_physical_fail(err_out, "diagnostic-sites");
  return 0;
}
