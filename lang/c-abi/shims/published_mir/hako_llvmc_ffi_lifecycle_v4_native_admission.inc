/* Native residence validation over supplied CFG/SSA only. No source Home issuance. */
static int lv4_native_origin(yyjson_val* fn, uint32_t value, size_t budget) {
  yyjson_val* def;
  if (!budget || !(def = lv4_def(fn, value))) return -1;
  if (lv4_is(def, "op", "copy"))
    return lv4_native_origin(fn, lv4_u(def, "src"), budget - 1);
  if (!lv4_is(def, "op", "invoke_normal_result")) return -1;
  yyjson_val* blocks = hako_physical_blocks(fn);
  for (size_t i = 0; i < yyjson_arr_size(blocks); i++)
    if (lv4_u(yyjson_arr_get(blocks, i), "id") == lv4_u(def, "invoke_block")) return (int)i;
  return -1;
}

static int lv4_native_live(yyjson_val* fn, uint32_t value,
    const hako_lv4_state* state, size_t budget) {
  int origin = lv4_native_origin(fn, value, budget);
  return origin >= 0 && state[origin] != 0;
}

static int lv4_string_native_operand(yyjson_val* fn, yyjson_val* operand,
    const hako_lv4_state* state, size_t budget) {
  if (lv4_is(operand, "kind", "static_text")) return 1;
  uint32_t value = lv4_u(operand, "value");
  int kind = lv4_type(fn, value, budget);
  if (lv4_is(operand, "kind", "normal_integer")) return kind == LV4_I64;
  int origin = lv4_native_origin(fn, value, budget);
  return kind == LV4_OWNED_STRING && origin >= 0 && state[origin] > LV4_STRING_RANK_BASE;
}

static int lv4_native_flow(yyjson_val* fn, size_t bi, const hako_lv4_state* incoming,
    int faulted, hako_lv4_state* saved, signed char* faults, unsigned char* active,
    size_t count, size_t budget) {
  yyjson_val* blocks = hako_physical_blocks(fn);
  yyjson_val* block = yyjson_arr_get(blocks, bi);
  yyjson_val* rows = yyjson_obj_get(block, "instructions");
  yyjson_val* term = lv4_term(block);
  hako_lv4_state state[256], normal[256];
  if (active[bi]) return 0;
  memcpy(state, incoming, count * sizeof(*state));
  /* A Fault path can only release, never claim/append. Claim failure and append
   * failure legitimately share cleanup: retain exact live identities, not the
   * now-unusable claim capability. */
  if (faulted) for (size_t i = 0; i < count; i++) if (state[i] && state[i] < LV4_STRING_RANK_BASE) state[i] = 1;
  if (faults[bi] >= 0)
    return faults[bi] == faulted && memcmp(saved + bi * count, state, count * sizeof(*state)) == 0;
  faults[bi] = (signed char)faulted;
  memcpy(saved + bi * count, state, count * sizeof(*state));
  active[bi] = 1;
  for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
    yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
    if (lv4_is(ins, "op", "fault_frame_enter")) {
      if (lv4_u(block, "id") != lv4_u(fn, "entry") || ri || faulted) return 0;
    } else if (lv4_is(ins, "op", "const_i64") || lv4_is(ins, "op", "const_bool") ||
        lv4_is(ins, "op", "const_f64_bits") || lv4_is(ins, "op", "const_unit")) {
      /* Distinct primitive kinds remain in lv4_type; Unit has no value lane. */
    } else if (lv4_is(ins, "op", "copy")) {
      uint32_t src = lv4_u(ins, "src");
      int kind = lv4_type(fn, src, budget);
      if (kind != LV4_I64 && kind != LV4_BOOL && kind != LV4_F64 && kind != LV4_HANDLE) return 0;
      if (kind == LV4_HANDLE && !lv4_native_live(fn, src, state, budget)) return 0;
    } else if (lv4_is(ins, "op", "invoke_normal_result")) {
      int origin = lv4_native_origin(fn, lv4_u(ins, "dst"), budget);
      if (faulted || origin < 0 || ri != 0 ||
          (lv4_type(fn, lv4_u(ins, "dst"), budget) == LV4_OWNED_STRING
              ? state[origin] <= LV4_STRING_RANK_BASE : state[origin] != 1)) return 0;
      int result_kind = lv4_type(fn, lv4_u(ins, "dst"), budget);
      if (result_kind == LV4_I64 || result_kind == LV4_BOOL) state[origin] = 0;
    } else if (lv4_is(ins, "op", "string_residence_install")) {
      uint32_t value = lv4_u(ins, "value");
      if (faulted || lv4_type(fn, value, budget) != LV4_OWNED_STRING ||
          !lv4_string_install(state, count, lv4_native_origin(fn, value, budget))) return 0;
    } else if (lv4_is(ins, "op", "array_residence_release")) {
      int origin = lv4_native_origin(fn, lv4_u(ins, "value"), budget);
      if (origin < 0 || !state[origin] ||
          lv4_type(fn, lv4_u(ins, "value"), budget) != LV4_HANDLE) return 0;
      state[origin] = 0;
    } else return 0;
  }
  if (lv4_is(term, "op", "return") || lv4_is(term, "op", "return_fault")) {
    for (size_t i = 0; i < count; i++) if (state[i]) return 0;
    active[bi] = 0;
    if (lv4_is(term, "op", "return_fault")) return faulted;
    return !faulted && (lv4_is(fn, "role", "root_unit") ||
        lv4_type(fn, lv4_u(term, "value"), budget) == LV4_I64);
  }
  memcpy(normal, state, count * sizeof(*state));
  int normal_fault = faulted, failure_fault = faulted;
  if (lv4_is(term, "op", "invoke")) {
    yyjson_val* op = yyjson_obj_get(term, "operation");
    int string_end = lv4_is(op, "kind", "string_end");
    if (faulted && !string_end) return 0;
    failure_fault = 1;
    if (string_end) {
      uint32_t value = lv4_u(op, "value");
      int origin = lv4_native_origin(fn, value, budget);
      if (lv4_type(fn, value, budget) != LV4_OWNED_STRING || origin < 0 ||
          state[origin] <= LV4_STRING_RANK_BASE || state[origin] != lv4_string_top(state, count)) return 0;
      normal[origin] = state[origin] = 0;
    } else if (hako_physical_string_kind(yyjson_get_str(yyjson_obj_get(op, "kind")))) {
      if (state[bi]) return 0;
      for (size_t role = 0; role < hako_physical_string_operand_count(op); role++)
        if (!lv4_string_native_operand(fn, hako_physical_string_operand_at(op, role), state, budget)) return 0;
      if (lv4_is(op, "kind", "string_substring") &&
          (lv4_type(fn, lv4_u(op, "start"), budget) != LV4_I64 || lv4_type(fn, lv4_u(op, "end"), budget) != LV4_I64)) return 0;
      if (hako_physical_string_result(yyjson_get_str(yyjson_obj_get(op, "kind"))) == 1 &&
          !lv4_string_push(state, count, &normal[bi])) return 0;
      if (hako_physical_string_result(yyjson_get_str(yyjson_obj_get(op, "kind"))) >= 2) normal[bi] = 1;
    } else if (lv4_is(op, "kind", "array_new")) {
      if (state[bi]) return 0;
      normal[bi] = 1;
    } else {
      int origin = lv4_native_origin(fn, lv4_u(op, "array"), budget);
      if (origin < 0 || !state[origin]) return 0;
      if (lv4_is(op, "kind", "array_claim")) {
        if (state[origin] != 1) return 0;
        normal[origin] = 2;
      } else if (lv4_is(op, "kind", "array_append")) {
        int expected = lv4_is(op, "representation", "i64") ? LV4_I64 :
            lv4_is(op, "representation", "bool") ? LV4_BOOL : LV4_F64;
        if (state[origin] != 2 || lv4_type(fn, lv4_u(op, "value"), budget) != expected) return 0;
      } else return 0;
    }
  } else if (lv4_is(term, "op", "branch")) {
    if (lv4_type(fn, lv4_u(term, "condition"), budget) != LV4_BOOL) return 0;
  } else if (!lv4_is(term, "op", "jump")) return 0;
  yyjson_val* edges = yyjson_obj_get(block, "edges");
  for (size_t ei = 0; ei < yyjson_arr_size(edges); ei++) {
    yyjson_val* edge = yyjson_arr_get(edges, ei);
    size_t next;
    if (!yyjson_is_null(yyjson_obj_get(edge, "args"))) return 0;
    for (next = 0; next < count; next++)
      if (lv4_u(yyjson_arr_get(blocks, next), "id") == lv4_u(edge, "target")) break;
    if (next == count || !lv4_native_flow(fn, next, ei ? state : normal,
        ei ? failure_fault : normal_fault, saved, faults, active, count, budget)) return 0;
  }
  active[bi] = 0;
  return 1;
}

static int lv4_native_admit(hako_lv4_input* in) {
  yyjson_val* fn = yyjson_arr_get(in->functions, 0);
  yyjson_val* blocks = hako_physical_blocks(fn);
  size_t count = yyjson_arr_size(blocks), budget = lv4_value_budget(fn);
  size_t entry = count, frames = 0;
  hako_lv4_state initial[256] = {0};
  unsigned char active[256] = {0};
  signed char faults[256];
  if (yyjson_arr_size(in->functions) != 1 || !count || count > 256 || budget > 4096) return 0;
  memset(faults, -1, sizeof(faults));
  for (size_t bi = 0; bi < count; bi++) {
    yyjson_val* block = yyjson_arr_get(blocks, bi);
    yyjson_val* rows = yyjson_obj_get(block, "instructions");
    yyjson_val* term = lv4_term(block);
    if (lv4_u(block, "id") == lv4_u(fn, "entry")) entry = bi;
    for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++)
      frames += lv4_is(yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction"), "op", "fault_frame_enter");
    if (!lv4_is(term, "op", "invoke") || !hako_physical_result_operation(yyjson_obj_get(term, "operation"), 1)) continue;
    yyjson_val* landing = hako_physical_block_by_id(fn, lv4_u(term, "normal"));
    yyjson_val* landing_rows = yyjson_obj_get(landing, "instructions");
    size_t results = 0;
    if (hako_physical_predecessor_count(fn, lv4_u(term, "normal")) != 1) return 0;
    for (size_t ri = 0; ri < yyjson_arr_size(landing_rows); ri++) {
      yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(landing_rows, ri), "instruction");
      results += lv4_is(ins, "op", "invoke_normal_result") && lv4_u(ins, "invoke_block") == lv4_u(block, "id");
    }
    if (results != 1) return 0;
  }
  if (frames != 1 || entry == count) return 0;
  hako_lv4_state* saved = calloc(count * count, sizeof(*saved));
  if (!saved) return 0;
  int valid = lv4_native_flow(fn, entry, initial, 0, saved, faults, active, count, budget);
  for (size_t bi = 0; valid && bi < count; bi++) if (faults[bi] < 0) valid = 0;
  free(saved);
  return valid;
}
