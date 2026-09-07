/* Physical cohort admission only. Source names never participate. */
typedef struct {
  yyjson_val *root, *functions, *layout;
  uint32_t profile, object_id, field_ordinals[2];
  int64_t type_id;
  uint64_t result_site;
} hako_lv4_input;

enum { LV4_BAD, LV4_I64, LV4_HANDLE, LV4_FRAME, LV4_UNIT, LV4_TEXT };

static uint32_t lv4_u(yyjson_val* object, const char* key) {
  return (uint32_t)yyjson_get_uint(yyjson_obj_get(object, key));
}
static int lv4_is(yyjson_val* object, const char* key, const char* text) {
  return hako_physical_string(yyjson_obj_get(object, key), text);
}
static yyjson_val* lv4_term(yyjson_val* block) {
  return yyjson_obj_get(yyjson_obj_get(block, "terminator"), "instruction");
}
static yyjson_val* lv4_def(yyjson_val* fn, uint32_t value) {
  yyjson_val* blocks = hako_physical_blocks(fn);
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* rows = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
    for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
      yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, i), "instruction");
      yyjson_val* dst = hako_physical_instruction_dst(ins);
      if (dst && yyjson_get_uint(dst) == value) return ins;
    }
  }
  return NULL;
}
static int lv4_type(yyjson_val* fn, uint32_t value, size_t budget) {
  yyjson_val *params = yyjson_obj_get(fn, "params"), *def;
  if (!budget) return LV4_BAD;
  for (size_t i = 0; i < yyjson_arr_size(params); i++)
    if (yyjson_get_uint(yyjson_arr_get(params, i)) == value)
      return i == 0 ? LV4_HANDLE : LV4_I64;
  def = lv4_def(fn, value);
  if (!def) return LV4_BAD;
  if (lv4_is(def, "op", "const_i64") || lv4_is(def, "op", "add") ||
      lv4_is(def, "op", "object_field_get")) return LV4_I64;
  if (lv4_is(def, "op", "invoke_normal_result")) return LV4_HANDLE;
  if (lv4_is(def, "op", "fault_frame_enter")) return LV4_FRAME;
  if (lv4_is(def, "op", "const_unit")) return LV4_UNIT;
  if (lv4_is(def, "op", "const_string")) return LV4_TEXT;
  if (lv4_is(def, "op", "copy")) return lv4_type(fn, lv4_u(def, "src"), budget - 1);
  return LV4_BAD;
}
static int lv4_slot(const hako_lv4_input* in, yyjson_val* op) {
  if (lv4_u(op, "object_id") != in->object_id) return -1;
  for (int i = 0; i < 2; i++)
    if (lv4_u(op, "field_ordinal") == in->field_ordinals[i]) return i;
  return -1;
}
static size_t lv4_value_budget(yyjson_val* fn) {
  yyjson_val* blocks = hako_physical_blocks(fn);
  size_t count = yyjson_arr_size(yyjson_obj_get(fn, "params")) + 1;
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++)
    count += yyjson_arr_size(yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions"));
  return count;
}

/* Root states: start=0, unpublished=1, Home=2, normal-clean=3,
 * Fault-clean=4, Fault-unpublished=5. Birth states: written-slot bits / Fault=8.
 * These check supplied edges; they never manufacture a successor. */
static int lv4_flow(const hako_lv4_input* in, yyjson_val* fn, int birth,
    size_t bi, int state, int* states, unsigned char* active, size_t budget) {
  yyjson_val* blocks = hako_physical_blocks(fn);
  yyjson_val* block = yyjson_arr_get(blocks, bi);
  yyjson_val* rows = yyjson_obj_get(block, "instructions");
  yyjson_val* term = lv4_term(block);
  int normal = state, fault = state;
  if (active[bi]) return 0;
  if (states[bi] >= 0) return states[bi] == state;
  states[bi] = state; active[bi] = 1;
  for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
    yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, i), "instruction");
    if (lv4_is(ins, "op", "fault_frame_enter")) {
      if (lv4_u(block, "id") != lv4_u(fn, "entry") || i != 0) return 0;
    } else if (lv4_is(ins, "op", "const_i64") || lv4_is(ins, "op", "const_string")) {
      /* Unused text constants have no emitted runtime value. Any use rejects below. */
    } else if (lv4_is(ins, "op", "const_unit")) {
      if (!birth) return 0;
    } else if (lv4_is(ins, "op", "copy")) {
      int kind = lv4_type(fn, lv4_u(ins, "src"), budget);
      if (kind != LV4_I64 && kind != LV4_HANDLE) return 0;
    } else if (lv4_is(ins, "op", "add")) {
      if (lv4_type(fn, lv4_u(ins, "lhs"), budget) != LV4_I64 ||
          lv4_type(fn, lv4_u(ins, "rhs"), budget) != LV4_I64) return 0;
    } else if (lv4_is(ins, "op", "object_field_get")) {
      if (birth || state != 2 || lv4_slot(in, ins) < 0 ||
          lv4_type(fn, lv4_u(ins, "base"), budget) != LV4_HANDLE) return 0;
    } else if (lv4_is(ins, "op", "invoke_normal_result")) {
      if (birth || state != 1) return 0;
    } else return 0;
  }
  if (lv4_is(term, "op", "return")) {
    active[bi] = 0;
    return state == 3 && lv4_type(fn, lv4_u(term, "value"), budget) ==
        (birth ? LV4_UNIT : LV4_I64);
  }
  if (lv4_is(term, "op", "return_fault")) {
    active[bi] = 0;
    return state == (birth ? 8 : 4);
  }
  if (lv4_is(term, "op", "invoke")) {
    yyjson_val* op = yyjson_obj_get(term, "operation");
    if (lv4_is(op, "kind", "birth_call")) {
      yyjson_val* call = yyjson_obj_get(op, "call");
      yyjson_val* args = yyjson_obj_get(call, "args");
      if (birth || state != 1 || lv4_u(call, "target") != 1 ||
          lv4_type(fn, lv4_u(call, "receiver"), budget) != LV4_HANDLE ||
          yyjson_arr_size(args) != 2) return 0;
      for (size_t i = 0; i < 2; i++)
        if (lv4_type(fn, (uint32_t)yyjson_get_uint(yyjson_arr_get(args, i)), budget) != LV4_I64)
          return 0;
      normal = 2; fault = 5;
    } else {
      if (lv4_u(op, "object_id") != in->object_id) return 0;
      if (lv4_is(op, "kind", "new_box")) {
        if (birth || state != 0) return 0;
        normal = 1; fault = 4;
      } else if (lv4_is(op, "kind", "field_set")) {
        int slot = lv4_slot(in, op);
        if (!birth || slot < 0 || state >= 4 || (state & (1 << slot)) ||
            lv4_type(fn, lv4_u(op, "base"), budget) != LV4_HANDLE ||
            lv4_type(fn, lv4_u(op, "value"), budget) != LV4_I64) return 0;
        normal = state | (1 << slot); fault = 8;
      } else if (lv4_is(op, "kind", "home_release")) {
        if (birth || state != 2 || lv4_type(fn, lv4_u(op, "value"), budget) != LV4_HANDLE) return 0;
        normal = 3; fault = 4;
      } else if (lv4_is(op, "kind", "reclaim_unpublished")) {
        if (birth || state != 5 || lv4_type(fn, lv4_u(op, "value"), budget) != LV4_HANDLE) return 0;
        normal = fault = 4;
      } else return 0;
    }
  } else if (!lv4_is(term, "op", "jump")) return 0;
  yyjson_val* edges = yyjson_obj_get(block, "edges");
  for (size_t ei = 0; ei < yyjson_arr_size(edges); ei++) {
    uint32_t target = lv4_u(yyjson_arr_get(edges, ei), "target");
    size_t next;
    if (!yyjson_is_null(yyjson_obj_get(yyjson_arr_get(edges, ei), "args"))) return 0;
    for (next = 0; next < yyjson_arr_size(blocks); next++)
      if (lv4_u(yyjson_arr_get(blocks, next), "id") == target) break;
    if (next == yyjson_arr_size(blocks) || !lv4_flow(in, fn, birth, next,
        ei == 0 ? normal : fault, states, active, budget)) return 0;
  }
  active[bi] = 0;
  return 1;
}

static int lv4_admit(yyjson_val* root, hako_lv4_input* out, char** error) {
  yyjson_val *layouts = yyjson_obj_get(root, "layouts"), *fields;
  out->root = root; out->functions = yyjson_obj_get(root, "functions");
  if (yyjson_arr_size(out->functions) != 2 || yyjson_arr_size(layouts) != 1) goto unsupported;
  out->layout = yyjson_arr_get(layouts, 0);
  fields = yyjson_obj_get(out->layout, "fields");
  if (lv4_u(out->layout, "field_count") != 2 || yyjson_arr_size(fields) != 2) goto unsupported;
  out->profile = lv4_u(root, "storage_profile");
  out->object_id = lv4_u(out->layout, "object_id");
  out->type_id = (int64_t)yyjson_get_uint(yyjson_obj_get(out->layout, "runtime_type_id"));
  out->result_site = yyjson_get_uint(yyjson_obj_get(root, "process_result_site"));
  for (size_t i = 0; i < 2; i++) {
    yyjson_val* field = yyjson_arr_get(fields, i);
    if (lv4_u(field, "storage_kind") != 1 || lv4_u(field, "runtime_slot") >= 2) goto unsupported;
    out->field_ordinals[lv4_u(field, "runtime_slot")] = lv4_u(field, "declaration_ordinal");
  }
  for (size_t fi = 0; fi < 2; fi++) {
    yyjson_val* fn = yyjson_arr_get(out->functions, fi);
    yyjson_val* blocks = hako_physical_blocks(fn);
    size_t count = yyjson_arr_size(blocks), entry = count, frames = 0;
    if (count > 256 || lv4_value_budget(fn) > 4096) goto unsupported;
    int* states = malloc(count * sizeof(*states));
    unsigned char* active = calloc(count, 1);
    int valid = states && active;
    if (fi && yyjson_arr_size(yyjson_obj_get(fn, "params")) != 3) valid = 0;
    for (size_t bi = 0; bi < count && valid; bi++) {
      yyjson_val* block = yyjson_arr_get(blocks, bi);
      yyjson_val* rows = yyjson_obj_get(block, "instructions");
      states[bi] = -1;
      if (lv4_u(block, "id") == lv4_u(fn, "entry")) entry = bi;
      for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++)
        frames += lv4_is(yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction"), "op", "fault_frame_enter");
    }
    if (valid) valid = frames == 1 && entry < count &&
        lv4_flow(out, fn, fi != 0, entry, 0, states, active, lv4_value_budget(fn));
    for (size_t bi = 0; bi < count && valid; bi++) if (states[bi] < 0) valid = 0;
    free(states); free(active);
    if (!valid) goto unsupported;
  }
  return 0;
unsupported:
  return set_err_owned(error, "[freeze:contract][published-lifecycle-v4/unsupported-cohort]");
}
