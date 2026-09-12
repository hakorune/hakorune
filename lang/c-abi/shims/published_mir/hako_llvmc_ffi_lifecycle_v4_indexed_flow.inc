/* Exact physical obligations on supplied CFG edges. Not a source Home. */
enum { LV4_INDEXED_SAFE_MUTEX = 1, LV4_LIVE = 1, LV4_TRANSFER_BASE = 2 };
enum { LV4_FLOW_RECEIVER_MISMATCH = -2 };

static int lv4_indexed_live(hako_lv4_function_index* fn, uint32_t id, int kind,
    const uint16_t* state, uint32_t object, int check_object) {
  hako_lv4_value* value = lv4_index_value(fn, id);
  if (!value || value->kind != kind || (check_object && value->object_id != object)) return 0;
  if (value->origin == -2) return kind == LV4_HANDLE;
  return value->origin >= 0 && state[value->origin] == LV4_LIVE;
}

static int lv4_indexed_rows(const hako_lv4_input* in, hako_lv4_function_index* fn,
    yyjson_val* block, uint16_t* state, int faulted, int birth) {
  yyjson_val* rows = yyjson_obj_get(block, "instructions");
  int after_phi = 0;
  if (in->measurement) in->measurement->v4_flow_rows += yyjson_arr_size(rows);
  for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
    yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
    if (lv4_is(ins, "op", "phi")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(ins, "dst"));
      if (after_phi || !value || (value->kind != LV4_I64 && value->kind != LV4_BOOL)) return 0;
      continue;
    }
    after_phi = 1;
    if (lv4_is(ins, "op", "fault_frame_enter")) {
      if (lv4_u(block, "id") != lv4_u(fn->function, "entry") || ri || faulted) return 0;
    } else if (lv4_is(ins, "op", "const_i64") || lv4_is(ins, "op", "const_bool") ||
        lv4_is(ins, "op", "const_string")) {
      /* Text constants have no runtime lane; their uses remain unsupported. */
    } else if (lv4_is(ins, "op", "const_unit")) {
      if (!birth) return 0;
    } else if (lv4_is(ins, "op", "copy")) {
      hako_lv4_value* source = lv4_index_value(fn, lv4_u(ins, "src"));
      if (!source || (source->kind != LV4_I64 && source->kind != LV4_BOOL &&
          source->kind != LV4_HANDLE && source->kind != LV4_TAGGED)) return 0;
      if (source->kind == LV4_HANDLE && !lv4_indexed_live(fn, source->id,
          LV4_HANDLE, state, 0, 0)) return 0;
    } else if (lv4_is(ins, "op", "add") || lv4_is(ins, "op", "compare")) {
      hako_lv4_value *lhs = lv4_index_value(fn, lv4_u(ins, "lhs")),
          *rhs = lv4_index_value(fn, lv4_u(ins, "rhs"));
      if (!lhs || !rhs || lhs->kind != LV4_I64 || rhs->kind != LV4_I64) return 0;
    } else if (lv4_is(ins, "op", "object_field_get")) {
      if (faulted || birth || lv4_slot(in, ins) < 0 ||
          !lv4_indexed_live(fn, lv4_u(ins, "base"), LV4_HANDLE, state,
              lv4_u(ins, "object_id"), 1)) return 0;
    } else if (lv4_is(ins, "op", "invoke_normal_result")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(ins, "dst"));
      if (birth || faulted || ri || !value || value->origin < 0 ||
          state[value->origin] != LV4_LIVE) return 0;
      yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
      if (value->kind == LV4_KEY && (!(lv4_is(op, "kind", "map_install_indexed") || lv4_is(op, "kind", "map_install_value")) ||
          lv4_u(op, "key") != value->id)) return 0;
      if (value->kind == LV4_OUTCOME && (!lv4_is(op, "kind", "map_end_outcome") ||
          lv4_u(op, "outcome") != value->id)) return 0;
      /* Ordinary calls produce a scalar result, not a live storage lease. */
      if (value->kind == LV4_I64) state[value->origin] = 0;
    } else return 0;
  }
  return 1;
}

typedef struct {
  uint16_t* states;
  signed char faults[256];
  size_t queue[256], head, tail, count, entry;
} hako_lv4_worklist;

static int lv4_indexed_deliver(hako_lv4_worklist* work, size_t target,
    const uint16_t* state, int faulted) {
  if (target >= work->count || target == work->entry) return 0;
  uint16_t* saved = work->states + target * work->count;
  if (work->faults[target] >= 0)
    return work->faults[target] == faulted &&
        !memcmp(saved, state, work->count * sizeof(*state));
  if (work->tail >= work->count) return 0;
  memcpy(saved, state, work->count * sizeof(*state));
  work->faults[target] = (signed char)faulted;
  work->queue[work->tail++] = target;
  return 1;
}

static int lv4_indexed_flow_block(const hako_lv4_input* in, size_t fi, size_t bi,
    hako_lv4_worklist* work) {
  hako_lv4_function_index* fn = &in->index[fi];
  yyjson_val* blocks = hako_physical_blocks(fn->function);
  yyjson_val* block = yyjson_arr_get(blocks, bi);
  yyjson_val* term = lv4_term(block);
  size_t count = fn->block_count;
  int faulted = work->faults[bi];
  if (in->measurement) in->measurement->v4_flow_blocks++;
  uint16_t state[256], normal[256];
  memcpy(state, work->states + bi * count, count * sizeof(*state));
  int birth = lv4_is(fn->function, "role", "birth_unit");
  int ordinary = lv4_is(fn->function, "role", "ordinary_i64");
  if (!lv4_indexed_rows(in, fn, block, state, faulted, birth)) return 0;
  if (lv4_is(term, "op", "return") || lv4_is(term, "op", "return_fault")) {
    for (size_t i = 0; i < count; i++) if (state[i]) return 0;
    if (lv4_is(term, "op", "return_fault")) return faulted;
    hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
    return !faulted && result && result->kind == (birth ? LV4_UNIT : LV4_I64);
  }
  memcpy(normal, state, count * sizeof(*state));
  int failed = faulted;
  if (lv4_is(term, "op", "invoke")) {
    yyjson_val* op = yyjson_obj_get(term, "operation");
    failed = 1;
    if (lv4_is(op, "kind", "ordinary_call")) {
      yyjson_val* call = yyjson_obj_get(op, "call");
      yyjson_val* args = yyjson_obj_get(call, "args");
      uint32_t target = lv4_u(call, "target");
      yyjson_val* callee = (target < in->index_count) ? in->index[target].function : NULL;
      yyjson_val* receiver = yyjson_obj_get(call, "receiver");
      int callee_has_receiver = callee && yyjson_is_uint(yyjson_obj_get(callee, "receiver"));
      if (fi || faulted || state[bi] || !target || target >= in->index_count ||
          !lv4_is(callee, "role", "ordinary_i64") ||
          (callee_has_receiver != (receiver && !yyjson_is_null(receiver))) ||
          yyjson_arr_size(args) != yyjson_arr_size(yyjson_obj_get(in->index[target].function, "params"))) return 0;
      if (callee_has_receiver) {
        if (!yyjson_is_uint(receiver) || !in->index[target].receiver_object_set) return 0;
        if (!lv4_indexed_live(fn, lv4_u(call, "receiver"), LV4_HANDLE, state,
                in->index[target].receiver_object, 1))
          return LV4_FLOW_RECEIVER_MISMATCH;
      }
      for (size_t i = 0; i < yyjson_arr_size(args); i++) {
        yyjson_val* arg = yyjson_arr_get(args, i);
        hako_lv4_value* value = lv4_index_value(fn, lv4_u(arg, "value"));
        if (!value || value->kind != LV4_I64) return 0;
      }
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "new_box") || lv4_is(op, "kind", "map_new") ||
        lv4_is(op, "kind", "map_prepare_key")) {
      if ((fi && !ordinary) || faulted || state[bi]) return 0;
      if (lv4_is(op, "kind", "new_box") && !lv4_layout(in, lv4_u(op, "object_id"))) return 0;
      if (hako_physical_map_kind(yyjson_get_str(yyjson_obj_get(op, "kind"))) &&
          in->profile != LV4_INDEXED_SAFE_MUTEX) return 0;
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "birth_call")) {
      yyjson_val* call = yyjson_obj_get(op, "call");
      yyjson_val* args = yyjson_obj_get(call, "args");
      uint32_t target = lv4_u(call, "target");
      if ((fi && !ordinary) || faulted || !target || target >= in->index_count ||
          !lv4_is(in->index[target].function, "role", "birth_unit") ||
          !lv4_indexed_live(fn, lv4_u(call, "receiver"), LV4_HANDLE, state,
              in->index[target].receiver_object, 1)) return 0;
      for (size_t i = 0; i < yyjson_arr_size(args); i++) {
        yyjson_val* arg = yyjson_arr_get(args, i);
        hako_lv4_value* value = lv4_index_value(fn, lv4_u(arg, "value"));
        if (!value || value->kind != (lv4_u(arg, "kind") == 1 ? LV4_I64 : LV4_BOOL)) return 0;
      }
    } else if (lv4_is(op, "kind", "field_set")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
      if (!fi || faulted || lv4_slot(in, op) < 0 || !value ||
          (value->kind != LV4_I64 && value->kind != LV4_TAGGED) ||
          !lv4_indexed_live(fn, lv4_u(op, "base"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), 1)) return 0;
    } else if (lv4_is(op, "kind", "home_release") || lv4_is(op, "kind", "reclaim_unpublished")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
      if ((fi && !ordinary) || !lv4_indexed_live(fn, lv4_u(op, "value"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), 1) || value->origin < 0) return 0;
      normal[value->origin] = state[value->origin] = 0;
    } else if (lv4_is(op, "kind", "map_install_indexed") || lv4_is(op, "kind", "map_install_value")) {
      int scalar = lv4_is(op, "kind", "map_install_value");
      hako_lv4_value *map = lv4_index_value(fn, lv4_u(op, "map")),
          *key = lv4_index_value(fn, lv4_u(op, "key")),
          *value = lv4_index_value(fn, lv4_u(op, "value"));
      if ((fi && !ordinary) || faulted || state[bi] ||
          in->profile != LV4_INDEXED_SAFE_MUTEX ||
          !lv4_indexed_live(fn, lv4_u(op, "map"), LV4_MAP, state, 0, 0) ||
          !lv4_indexed_live(fn, lv4_u(op, "key"), LV4_KEY, state, 0, 0) || !value) return 0;
      if (scalar) {
        uint32_t kind = lv4_u(op, "value_kind");
        if (!((kind == NYRT_MAP_VALUE_I64 && value->kind == LV4_I64) ||
              (kind == NYRT_MAP_VALUE_BOOL && value->kind == LV4_BOOL))) return 0;
      } else {
        if (!lv4_indexed_live(fn, value->id, LV4_HANDLE, state,
                lv4_u(op, "object_id"), 1) || value->origin < 0) return 0;
        normal[value->origin] = (uint16_t)(map->origin + LV4_TRANSFER_BASE);
      }
      normal[key->origin] = state[key->origin] = 0;
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "map_end") || lv4_is(op, "kind", "map_end_outcome")) {
      int map_end = lv4_is(op, "kind", "map_end");
      uint32_t id = lv4_u(op, map_end ? "map" : "outcome");
      hako_lv4_value* value = lv4_index_value(fn, id);
      if ((fi && !ordinary) ||
          !lv4_indexed_live(fn, id, map_end ? LV4_MAP : LV4_OUTCOME, state, 0, 0)) return 0;
      normal[value->origin] = state[value->origin] = 0;
      if (map_end) for (size_t i = 0; i < count; i++)
        if (state[i] == value->origin + LV4_TRANSFER_BASE) normal[i] = state[i] = 0;
    } else return 0;
  } else if (lv4_is(term, "op", "branch")) {
    hako_lv4_value* condition = lv4_index_value(fn, lv4_u(term, "condition"));
    if (!condition || condition->kind != LV4_BOOL) return 0;
  } else if (!lv4_is(term, "op", "jump")) return 0;
  yyjson_val* edges = yyjson_obj_get(block, "edges");
  for (size_t ei = 0; ei < yyjson_arr_size(edges); ei++) {
    yyjson_val* edge = yyjson_arr_get(edges, ei);
    hako_lv4_block* next = lv4_index_block(fn, lv4_u(edge, "target"));
    if (!next || !yyjson_is_null(yyjson_obj_get(edge, "args"))) return 0;
    if (!lv4_indexed_deliver(work, next->ordinal,
        ei ? state : normal, ei ? failed : faulted)) return 0;
  }
  return 1;
}

static int lv4_indexed_admit(hako_lv4_input* in) {
  if (!lv4_index_open(in)) return 0;
  /* Birth callers bind borrowed receivers; ordinary callees already carry the
   * prepared object identity in their physical function row. */
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* caller = &in->index[fi];
    int ordinary = lv4_is(caller->function, "role", "ordinary_i64");
    yyjson_val* blocks = hako_physical_blocks(caller->function);
    for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val* op = yyjson_obj_get(lv4_term(yyjson_arr_get(blocks, bi)), "operation");
      if (!lv4_is(op, "kind", "birth_call")) continue;
      if (fi && !ordinary) return 0;
      yyjson_val* call = yyjson_obj_get(op, "call");
      uint32_t target = lv4_u(call, "target");
      hako_lv4_value* receiver = lv4_index_value(caller, lv4_u(call, "receiver"));
      if (!target || target >= in->index_count ||
          !lv4_is(in->index[target].function, "role", "birth_unit") ||
          !receiver || receiver->kind != LV4_HANDLE || receiver->origin < 0) return 0;
      hako_lv4_function_index* birth = &in->index[target];
      if (birth->receiver_object_set) return 0;
      birth->receiver_object = receiver->object_id; birth->receiver_object_set = 1;
      for (size_t vi = 0; vi < birth->value_count; vi++)
        if (birth->values[vi].origin == -2) birth->values[vi].object_id = receiver->object_id;
    }
  }
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    size_t count = fn->block_count, frames = 0;
    hako_lv4_worklist work = {0};
    if (lv4_is(fn->function, "role", "birth_unit") && !fn->receiver_object_set) return 0;
    memset(work.faults, -1, sizeof(work.faults));
    hako_lv4_block* entry = lv4_index_block(fn, lv4_u(fn->function, "entry"));
    for (size_t bi = 0; bi < count; bi++) {
      yyjson_val* rows = yyjson_obj_get(fn->blocks[bi].block, "instructions");
      for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++)
        frames += lv4_is(yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction"), "op", "fault_frame_enter");
      yyjson_val* term = lv4_term(fn->blocks[bi].block);
      if (lv4_operation_result(yyjson_obj_get(term, "operation")) != LV4_BAD) {
        uint32_t id = fn->blocks[bi].id, normal = lv4_u(term, "normal");
        size_t projections = 0;
        if (hako_physical_predecessor_count(fn->function, normal) != 1) return 0;
        for (size_t vi = 0; vi < fn->value_count; vi++) {
          yyjson_val* ins = fn->values[vi].instruction;
          projections += lv4_is(ins, "op", "invoke_normal_result") && lv4_u(ins, "invoke_block") == id;
        }
        if (projections != 1) return 0;
      }
    }
    /* Root initializes its frame. A Birth with no frame uses may omit the
       borrowed-frame projection; SSA validation still rejects undefined uses. */
    if (!entry || frames > 1 || (!fi && frames != 1) ||
        hako_physical_predecessor_count(fn->function, entry->id)) return 0;
    work.count = count; work.entry = entry->ordinal;
    work.states = calloc(count * count, sizeof(*work.states));
    if (!work.states) return 0;
    work.faults[work.entry] = 0;
    work.queue[work.tail++] = work.entry;
    int valid = 1;
    while (valid == 1 && work.head < work.tail) {
      size_t bi = work.queue[work.head++];
      valid = lv4_indexed_flow_block(in, fi, bi, &work);
    }
    free(work.states);
    if (valid == LV4_FLOW_RECEIVER_MISMATCH) return valid;
    for (size_t bi = 0; valid && bi < count; bi++) if (work.faults[bi] < 0) valid = 0;
    if (!valid) return 0;
  }
  return 1;
}
