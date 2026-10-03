/* Exact physical obligations on supplied CFG edges. Not a source Home. */
enum { LV4_INDEXED_SAFE_MUTEX = 1, LV4_LIVE = 1, LV4_TRANSFER_BASE = 2 };
enum { LV4_FLOW_RECEIVER_MISMATCH = -2 };

static int lv4_indexed_live(hako_lv4_function_index* fn, uint32_t id, int kind,
    const uint16_t* state, uint32_t object, int check_object) {
  hako_lv4_value* value = lv4_index_value(fn, id);
  if (!value || value->kind != kind || (check_object && value->object_id != object)) return 0;
  /* Borrowed operand (-2): a receiver handle lives for the whole call. */
  if (value->origin == -2) return kind == LV4_HANDLE;
  return value->origin >= 0 && state[value->origin] == LV4_LIVE;
}

/* A checked-map read operand: a live owned lease, or a caller-owned MapBox
 * formal (origin -2) borrowed for the whole call. Only the read lane uses
 * this predicate — install and end lanes keep lv4_indexed_live, so borrowed
 * storage is never mutated or ended. */
static int lv4_indexed_map_readable(hako_lv4_function_index* fn, uint32_t id,
    const uint16_t* state) {
  hako_lv4_value* value = lv4_index_value(fn, id);
  if (!value || value->kind != LV4_MAP) return 0;
  if (value->origin == -2) return 1;
  return value->origin >= 0 && state[value->origin] == LV4_LIVE;
}

/* Type unions do not prove provenance. Follow the exact Copy producer instead. */
static hako_lv4_value* lv4_borrowed_copy_root(hako_lv4_function_index* fn,
    hako_lv4_value* value) {
  for (size_t steps = 0; value && lv4_is(value->instruction, "op", "copy"); steps++) {
    if (steps >= fn->value_count) return NULL;
    value = lv4_index_value(fn, lv4_u(value->instruction, "src"));
  }
  return value;
}

static int lv4_borrowed_typed_live(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const uint16_t* state) {
  if (!value || value->kind != LV4_HANDLE || !lv4_layout(in, value->object_id) ||
      !lv4_indexed_live(fn, value->id, LV4_HANDLE, state, value->object_id, 1)) return 0;
  hako_lv4_value* root = lv4_borrowed_copy_root(fn, value);
  if (!root || root->object_id != value->object_id) return 0;
  if (root->origin == -2)
    return fn->receiver_object_set && yyjson_is_uint(yyjson_obj_get(fn->function, "receiver")) &&
        root->id == lv4_u(fn->function, "receiver") && root->object_id == fn->receiver_object;
  if (!lv4_is(root->instruction, "op", "invoke_normal_result")) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
  yyjson_val* op = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
  return lv4_is(op, "kind", "new_box") && lv4_u(op, "object_id") == root->object_id;
}

static int lv4_borrowed_tagged_formal(hako_lv4_function_index* fn, hako_lv4_value* value) {
  if (!value || value->kind != LV4_TAGGED || !fn->receiver_object_set ||
      !lv4_is(fn->function, "role", "ordinary_i64")) return 0;
  hako_lv4_value* root = lv4_borrowed_copy_root(fn, value);
  yyjson_val* params = yyjson_obj_get(fn->function, "params");
  for (size_t i = 0; root && i < yyjson_arr_size(params); i++) {
    yyjson_val* param = yyjson_arr_get(params, i);
    if (lv4_u(param, "value") == root->id && lv4_is(param, "representation", "borrowed_kind_payload_v1")) return 1;
  }
  return 0;
}

/* The null sentinel's other admitted lane: the function carries at
   least one `borrowed_null_compare` row. Like the other const lanes this
   is a function-scope admission — dead const projections stay admitted
   and the compare arm still pins the exact const_null sibling. */
static int lv4_null_compare_present(hako_lv4_function_index* fn) {
  yyjson_val* blocks = hako_physical_blocks(fn->function);
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* rows = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
    for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
      yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
      if (lv4_is(ins, "op", "borrowed_null_compare")) return 1;
    }
  }
  return 0;
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
      /* A missing else projects a dead Unit row; it carries no lane and
         no operand position accepts the UNIT kind. */
    } else if (lv4_is(ins, "op", "const_null")) {
      /* The null sentinel materializes only inside a nullable-result
         callee or inside a function carrying the admitted borrowed/null
         equality; it seeds a handle-lane slot that never owns a lease. */
      if (!lv4_is(fn->function, "role", "ordinary_nullable_handle") &&
          !lv4_null_compare_present(fn)) return 0;
    } else if (lv4_is(ins, "op", "copy")) {
      hako_lv4_value* source = lv4_index_value(fn, lv4_u(ins, "src"));
      if (!source || (source->kind != LV4_I64 && source->kind != LV4_BOOL &&
          source->kind != LV4_HANDLE && source->kind != LV4_TAGGED)) return 0;
      if (source->kind == LV4_HANDLE && !lv4_indexed_live(fn, source->id,
          LV4_HANDLE, state, 0, 0)) return 0;
    } else if (lv4_is(ins, "op", "add")) {
      hako_lv4_value *lhs = lv4_index_value(fn, lv4_u(ins, "lhs")),
          *rhs = lv4_index_value(fn, lv4_u(ins, "rhs"));
      /* The borrowed kind/payload lane may serve a dominated `+` operand
         only; the emitted kind==1 branch proves the i64 read at the use
         site, uniform with the checked compare. */
      if (!lhs || !rhs ||
          (lhs->kind != LV4_I64 && lhs->kind != LV4_TAGGED) ||
          (rhs->kind != LV4_I64 && rhs->kind != LV4_TAGGED)) return 0;
    } else if (lv4_is(ins, "op", "compare")) {
      hako_lv4_value *lhs = lv4_index_value(fn, lv4_u(ins, "lhs")),
          *rhs = lv4_index_value(fn, lv4_u(ins, "rhs"));
      /* The borrowed kind/payload lane may serve a checked compare only;
         the emitted kind==1 branch proves the i64 read at the use site. */
      if (!lhs || !rhs ||
          (lhs->kind != LV4_I64 && lhs->kind != LV4_TAGGED) ||
          (rhs->kind != LV4_I64 && rhs->kind != LV4_TAGGED)) return 0;
    } else if (lv4_is(ins, "op", "borrowed_null_compare")) {
      hako_lv4_value *lhs = lv4_index_value(fn, lv4_u(ins, "lhs")),
          *rhs = lv4_index_value(fn, lv4_u(ins, "rhs"));
      hako_lv4_value* carrier;
      int lnull, rnull;
      /* The borrowed/null equality admits exactly one carrier operand and
         the exact const_null producer as its sibling. Equality reads the
         carrier's kind or payload lane — never the Integer view — so a
         well-formed non-null kind answers false instead of faulting. */
      if (!lhs || !rhs) return 0;
      lnull = lv4_is(lhs->instruction, "op", "const_null");
      rnull = lv4_is(rhs->instruction, "op", "const_null");
      if (lnull == rnull) return 0;
      carrier = lnull ? rhs : lhs;
      if (carrier->kind != LV4_TAGGED && carrier->kind != LV4_HANDLE &&
          carrier->kind != LV4_I64 && carrier->kind != LV4_BOOL) return 0;
    } else if (lv4_is(ins, "op", "array_set")) {
      hako_lv4_value *array = lv4_index_value(fn, lv4_u(ins, "array")),
          *index = lv4_index_value(fn, lv4_u(ins, "index")),
          *value = lv4_index_value(fn, lv4_u(ins, "value"));
      /* The element write is a checked kernel call: the array operand is a
         raw slot-loaded handle lane, the index a committed i64, and the
         element value may read the same lent Integer view — the emitted
         kind==1 branch proves that read at this exact site, uniform with
         the checked compare and the dominated `+`. */
      if (faulted || birth || !array || array->kind != LV4_I64 ||
          !index || index->kind != LV4_I64 ||
          !value || (value->kind != LV4_I64 && value->kind != LV4_TAGGED)) return 0;
    } else if (lv4_is(ins, "op", "object_field_get")) {
      if (faulted || birth || lv4_slot(in, ins) < 0 ||
          !lv4_indexed_live(fn, lv4_u(ins, "base"), LV4_HANDLE, state,
              lv4_u(ins, "object_id"), 1)) return 0;
    } else if (lv4_is(ins, "op", "field_set")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(ins, "value"));
      yyjson_val* check = yyjson_obj_get(ins, "exact_numeric_runtime_check");
      /* A routed scalar store consumes the same checked helper as the
         invoke form: the base is the exact live object and the value is a
         proven i64 lane only — tagged carriers and handle leases never
         reach this row. A dynamic-integer-range obligation is discharged by
         the emitted bounds check at this row, not by the kernel. */
      if (lv4_is(fn->function, "role", "root_unit") || faulted ||
          lv4_slot(in, ins) < 0 || !value || value->kind != LV4_I64 ||
          !check ||
          !(lv4_is(check, "kind", "none") ||
            (lv4_is(check, "kind", "dynamic_integer_range") &&
             lv4_is(check, "declared_type", "usize"))) ||
          !lv4_indexed_live(fn, lv4_u(ins, "base"), LV4_HANDLE, state,
              lv4_u(ins, "object_id"), 1)) return 0;
    } else if (lv4_is(ins, "op", "invoke_normal_result")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(ins, "dst"));
      int result_requires_live = value && value->kind != LV4_TEXT_VIEW;
      /* Birth units carry one result projection: the proven provider
         `new ArrayBox()` normal landing. It is still the first row of a
         block reached only by the invoke's Normal edge. */
      if (faulted || ri || !value || value->origin < 0 ||
          (result_requires_live && state[value->origin] != LV4_LIVE)) return 0;
      yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
      if (value->kind == LV4_KEY && (!(lv4_is(op, "kind", "map_install_indexed") ||
          lv4_is(op, "kind", "map_install_value") || lv4_is(op, "kind", "map_install_text") ||
          lv4_is(op, "kind", "map_install_empty_array") ||
          lv4_is(op, "kind", "map_install_borrowed_array")) ||
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
  int ordinary = lv4_is(fn->function, "role", "ordinary_i64") ||
      lv4_is(fn->function, "role", "ordinary_map") ||
      lv4_is(fn->function, "role", "ordinary_nullable_handle");
  int ordinary_map = lv4_is(fn->function, "role", "ordinary_map");
  int ordinary_nullable = lv4_is(fn->function, "role", "ordinary_nullable_handle");
  if (!lv4_indexed_rows(in, fn, block, state, faulted, birth)) return 0;
  if (lv4_is(term, "op", "return") || lv4_is(term, "op", "return_fault")) {
    if (ordinary_map && lv4_is(term, "op", "return")) {
      hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
      /* A live LV4_MAP lease transfers into caller-owned storage. A spent
         lease or any other value is not a valid ordinary_map result. */
      if (faulted || !result || result->kind != LV4_MAP || result->origin < 0 ||
          state[result->origin] != LV4_LIVE) return 0;
      state[result->origin] = 0;
      for (size_t i = 0; i < count; i++)
        if (state[i] == result->origin + LV4_TRANSFER_BASE) state[i] = 0;
    }
    if (ordinary_nullable && lv4_is(term, "op", "return")) {
      hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
      /* A live handle result transfers the lease to the caller; the Void
         sentinel (or any borrowed value) owns nothing and hands over
         cleanly. A spent or non-handle value is not a valid result. */
      if (faulted || !result || result->kind != LV4_HANDLE) return 0;
      if (result->origin >= 0) {
        if (state[result->origin] != LV4_LIVE) return 0;
        state[result->origin] = 0;
      }
    }
    for (size_t i = 0; i < count; i++) if (state[i]) return 0;
    if (lv4_is(term, "op", "return_fault")) return faulted;
    hako_lv4_value* result = lv4_index_value(fn, lv4_u(term, "value"));
    return !faulted && result &&
        result->kind == (birth ? LV4_UNIT : ordinary_map ? LV4_MAP :
            ordinary_nullable ? LV4_HANDLE : LV4_I64);
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
      int want_map = lv4_is(op, "result", "map");
      int want_nullable = lv4_is(op, "result", "nullable_handle");
      if ((fi && !ordinary) || faulted || state[bi] || !target || target >= in->index_count ||
          !lv4_is(callee, "role", want_map ? "ordinary_map" :
              want_nullable ? "ordinary_nullable_handle" : "ordinary_i64") ||
          (callee_has_receiver != (receiver && !yyjson_is_null(receiver))) ||
          yyjson_arr_size(args) != yyjson_arr_size(yyjson_obj_get(in->index[target].function, "params"))) return 0;
      if (callee_has_receiver) {
        if (!yyjson_is_uint(receiver) || !in->index[target].receiver_object_set) return 0;
        if (!lv4_indexed_live(fn, lv4_u(call, "receiver"), LV4_HANDLE, state,
                in->index[target].receiver_object, 1))
          return LV4_FLOW_RECEIVER_MISMATCH;
      }
      {
        yyjson_val* cparams = yyjson_obj_get(in->index[target].function, "params");
        for (size_t i = 0; i < yyjson_arr_size(args); i++) {
          yyjson_val* arg = yyjson_arr_get(args, i);
          hako_lv4_value* value = lv4_index_value(fn, lv4_u(arg, "value"));
          /* Corroborated pair per ordinal: a caller-owned live map lease
             spelled "map" rides the edge borrowed against a "map" formal;
             the loan consumes nothing and the caller still owns End. Every
             other pair stays i64, and a map claim on either side alone is
             ABI drift, never a silent scalar fill. */
          const char* repr = yyjson_get_str(yyjson_obj_get(
              yyjson_arr_get(cparams, i), "representation"));
          const char* arg_kind = yyjson_get_str(yyjson_obj_get(arg, "kind"));
          if (repr && !strcmp(repr, "borrowed_kind_payload_v1")) {
            yyjson_val* kind = yyjson_obj_get(arg, "kind");
            if (arg_kind && !strcmp(arg_kind, "tagged")) {
              if (!lv4_borrowed_tagged_formal(fn, value)) return 0;
            } else if (yyjson_is_uint(kind)) {
              uint64_t tag = yyjson_get_uint(kind);
              if (!value || (tag == 1 && value->kind != LV4_I64) ||
                  (tag == 2 && value->kind != LV4_BOOL) ||
                  (tag == 3 && !lv4_borrowed_typed_live(in, fn, value, state)) ||
                  tag < 1 || tag > 3) return 0;
            } else return 0;
            continue; /* BorrowForCall: no lease state is consumed. */
          }
          int want = repr && !strcmp(repr, "i64") ? LV4_I64
              : repr && !strcmp(repr, "map") ? LV4_MAP : LV4_BAD;
          if (!value || value->kind != want) return 0;
          if (want == LV4_MAP) {
            if (!arg_kind || strcmp(arg_kind, "map") ||
                !lv4_indexed_live(fn, value->id, LV4_MAP, state, 0, 0))
              return 0;
          } else if (!arg_kind || strcmp(arg_kind, "i64")) return 0;
        }
      }
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "new_box") || lv4_is(op, "kind", "map_new") ||
        lv4_is(op, "kind", "map_prepare_key")) {
      /* A provider `new <Child>` inside a birth unit issues the child
         allocation on the shared frame, so `new_box` is admitted there:
         the produced lease is still a caller-owned in-flight handle. */
      if ((fi && !ordinary && !birth) || faulted || state[bi]) return 0;
      if (lv4_is(op, "kind", "new_box") && !lv4_layout(in, lv4_u(op, "object_id"))) return 0;
      if (hako_physical_map_kind(yyjson_get_str(yyjson_obj_get(op, "kind"))) &&
          in->profile != LV4_INDEXED_SAFE_MUTEX) return 0;
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "array_new")) {
      /* A provider `new ArrayBox()` inside a birth unit (or ordinary
         function) issues a live handle lease; a following field store
         moves it into the object slot or a release discharges it. */
      if ((fi && !ordinary && !birth) || faulted || state[bi]) return 0;
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "birth_call")) {
      yyjson_val* call = yyjson_obj_get(op, "call");
      yyjson_val* args = yyjson_obj_get(call, "args");
      uint32_t target = lv4_u(call, "target");
      /* A birth caller invokes its child's own birth unit on the in-flight
         handle; the lease survives the call for the field store below. */
      if ((fi && !ordinary && !birth) || faulted || !target || target >= in->index_count ||
          !lv4_is(in->index[target].function, "role", "birth_unit") ||
          !lv4_indexed_live(fn, lv4_u(call, "receiver"), LV4_HANDLE, state,
              in->index[target].receiver_object, 1)) return 0;
      for (size_t i = 0; i < yyjson_arr_size(args); i++) {
        yyjson_val* arg = yyjson_arr_get(args, i);
        hako_lv4_value* value = lv4_index_value(fn, lv4_u(arg, "value"));
        /* A dominated `new`-argument view spells "tagged": the value must
           be this caller's own borrowed kind/payload formal — the same
           proof the checked-compare lane owns — and the emitted kind==1
           branch discharges the i64 read at the call edge. */
        const char* arg_kind = yyjson_get_str(yyjson_obj_get(arg, "kind"));
        if (arg_kind) {
          if (strcmp(arg_kind, "tagged") || !lv4_borrowed_tagged_formal(fn, value))
            return 0;
          continue;
        }
        /* arg kind is exact: 1 is i64, 2 is bool — anything else,
           including a missing field, rejects rather than defaulting. */
        uint32_t numeric_kind = lv4_u(arg, "kind");
        if (!value || (numeric_kind != 1 && numeric_kind != 2) ||
            value->kind != (numeric_kind == 1 ? LV4_I64 : LV4_BOOL)) return 0;
      }
    } else if (lv4_is(op, "kind", "field_set")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
      if (!fi || faulted || lv4_slot(in, op) < 0 || !value) return 0;
      if (value->kind == LV4_HANDLE) {
        /* A live handle stored into an owned field moves the lease into
           the object slot; a `field_residence_release` on the base
           discharges it later — the store consumes the lease here. */
        if (value->origin < 0 ||
            !lv4_indexed_live(fn, lv4_u(op, "value"), LV4_HANDLE, state,
                0, 0)) return 0;
        normal[value->origin] = state[value->origin] = 0;
      } else if (value->kind != LV4_I64 && !(birth && value->kind == LV4_TAGGED)) return 0;
      if (!lv4_indexed_live(fn, lv4_u(op, "base"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), 1)) return 0;
    } else if (lv4_is(op, "kind", "field_residence_release")) {
      /* An owned field residence release discharges only the child slot.
         The base Home stays live until its own release consumes it, so
         the operand corroborates the exact object without ending it. */
      hako_lv4_value* base = lv4_index_value(fn, lv4_u(op, "base"));
      if ((fi && !ordinary) || lv4_slot(in, op) < 0 || !base ||
          base->origin < 0 ||
          !lv4_indexed_live(fn, lv4_u(op, "base"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), 1)) return 0;
    } else if (lv4_is(op, "kind", "object_field_set")) {
      /* A checked user-object field store moves the live child handle
         into the parent's slot. Normal owns the lease in the slot; Fault
         keeps it live so the discharge block's own `home_release` on the
         child still has a lease to consume. */
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
      if (!fi || faulted || lv4_slot(in, op) < 0 || !value ||
          value->kind != LV4_HANDLE || value->origin < 0 ||
          !lv4_layout(in, lv4_u(op, "child_object_id")) ||
          !lv4_indexed_live(fn, value->id, LV4_HANDLE, state,
              lv4_u(op, "child_object_id"), 1) ||
          !lv4_indexed_live(fn, lv4_u(op, "base"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), 1)) return 0;
      normal[value->origin] = 0;
    } else if (lv4_is(op, "kind", "object_field_release")) {
      /* Same child-slot discharge as a field residence release, keyed on
         the canonical child object — the emitted helper owns the child's
         own teardown under the exact-child corroboration here. */
      hako_lv4_value* base = lv4_index_value(fn, lv4_u(op, "base"));
      if ((fi && !ordinary) || lv4_slot(in, op) < 0 || !base ||
          base->origin < 0 ||
          !lv4_layout(in, lv4_u(op, "child_object_id")) ||
          !lv4_indexed_live(fn, lv4_u(op, "base"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), 1)) return 0;
    } else if (lv4_is(op, "kind", "home_release") || lv4_is(op, "kind", "home_release_if_live") ||
        lv4_is(op, "kind", "reclaim_unpublished")) {
      /* `home_release_if_live` releases a value that may carry the Void
         sentinel: the lease slot is consumed either way, but call results
         carry no object_id, so the checked release verifies liveness
         without the exact-object corroboration. */
      int nullable_release = lv4_is(op, "kind", "home_release_if_live");
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
      /* Birth units admit these only for an in-flight provider child
         (origin >= 0); the borrowed `me` receiver still fails the origin
         check, so self-release can never slip through this lane. */
      if ((fi && !ordinary && !birth) || !lv4_indexed_live(fn, lv4_u(op, "value"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), !nullable_release) || value->origin < 0) return 0;
      normal[value->origin] = state[value->origin] = 0;
    } else if (lv4_is(op, "kind", "map_install_text") ||
        lv4_is(op, "kind", "map_install_empty_array")) {
      hako_lv4_value *key = lv4_index_value(fn, lv4_u(op, "key"));
      if ((fi && !ordinary) || faulted || state[bi] ||
          in->profile != LV4_INDEXED_SAFE_MUTEX ||
          !lv4_indexed_live(fn, lv4_u(op, "map"), LV4_MAP, state, 0, 0) ||
          !lv4_indexed_live(fn, lv4_u(op, "key"), LV4_KEY, state, 0, 0)) return 0;
      normal[key->origin] = state[key->origin] = 0;
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "map_install_borrowed_array")) {
      yyjson_val* elements = yyjson_obj_get(op, "elements");
      hako_lv4_value *key = lv4_index_value(fn, lv4_u(op, "key"));
      if ((fi && !ordinary) || faulted || state[bi] ||
          in->profile != LV4_INDEXED_SAFE_MUTEX ||
          !lv4_indexed_live(fn, lv4_u(op, "map"), LV4_MAP, state, 0, 0) ||
          !lv4_indexed_live(fn, lv4_u(op, "key"), LV4_KEY, state, 0, 0) ||
          !elements || !yyjson_is_arr(elements) || !yyjson_arr_size(elements)) return 0;
      for (size_t i = 0; i < yyjson_arr_size(elements); i++) {
        hako_lv4_value* value = lv4_index_value(fn, (uint32_t)yyjson_get_uint(yyjson_arr_get(elements, i)));
        if (!value || !lv4_indexed_map_readable(fn, value->id, state)) return 0;
      }
      normal[key->origin] = state[key->origin] = 0;
      normal[bi] = LV4_LIVE;
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
              (kind == NYRT_MAP_VALUE_BOOL && value->kind == LV4_BOOL) ||
              (kind == NYRT_MAP_VALUE_BORROWED_HANDLE && value->kind == LV4_I64))) return 0;
      } else {
        if (!lv4_indexed_live(fn, value->id, LV4_HANDLE, state,
                lv4_u(op, "object_id"), 1) || value->origin < 0) return 0;
        normal[value->origin] = (uint16_t)(map->origin + LV4_TRANSFER_BASE);
      }
      normal[key->origin] = state[key->origin] = 0;
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "map_checked_get") || lv4_is(op, "kind", "map_array_length")) {
      /* Read-only scalar lookup: the map operand must be a live owned lease
         or a borrowed formal; the read consumes neither. */
      if ((fi && !ordinary) || faulted || state[bi] ||
          in->profile != LV4_INDEXED_SAFE_MUTEX ||
          !lv4_indexed_map_readable(fn, lv4_u(op, "map"), state)) return 0;
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "map_array_index_map")) {
      /* ArrayIndex creates one borrowed view; the parent Map remains live. */
      if ((fi && !ordinary) || faulted || state[bi] ||
          in->profile != LV4_INDEXED_SAFE_MUTEX ||
          !lv4_indexed_map_readable(fn, lv4_u(op, "map"), state)) return 0;
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "map_get_text")) {
      /* Text lookup consumes the immediately-produced view on both Normal
         and Fault; TextView itself has no cleanup state or escape lane. */
      hako_lv4_value* view = lv4_index_value(fn, lv4_u(op, "map"));
      if ((fi && !ordinary) || faulted || state[bi] || !view ||
          view->kind != LV4_MAP_VIEW || view->origin < 0 ||
          state[view->origin] != LV4_LIVE) return 0;
      normal[view->origin] = state[view->origin] = 0;
    } else if (lv4_is(op, "kind", "map_end") || lv4_is(op, "kind", "map_end_outcome")) {
      int map_end = lv4_is(op, "kind", "map_end");
      uint32_t id = lv4_u(op, map_end ? "map" : "outcome");
      hako_lv4_value* value = lv4_index_value(fn, id);
      /* Borrowed storage (-2) is never ended: only an owned live lease or
         outcome may reach this lane and index the state arrays. */
      if ((fi && !ordinary) || !value || value->origin < 0 ||
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
    yyjson_val* edge_args = yyjson_obj_get(edge, "args");
    /* An empty carriers_only edge transports nothing and is the same
       physical shape as a null args row. */
    if (!yyjson_is_null(edge_args)) {
      yyjson_val* values = yyjson_obj_get(edge_args, "values");
      if (!lv4_is(edge_args, "layout", "carriers_only") || !values ||
          !yyjson_is_arr(values) || yyjson_arr_size(values)) return 0;
    }
    if (!next) return 0;
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
    int ordinary = lv4_is(caller->function, "role", "ordinary_i64") ||
        lv4_is(caller->function, "role", "ordinary_map") ||
        lv4_is(caller->function, "role", "ordinary_nullable_handle");
    yyjson_val* blocks = hako_physical_blocks(caller->function);
    for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val* op = yyjson_obj_get(lv4_term(yyjson_arr_get(blocks, bi)), "operation");
      if (!lv4_is(op, "kind", "birth_call")) continue;
      int birth_caller = lv4_is(caller->function, "role", "birth_unit");
      if (fi && !ordinary && !birth_caller) return 0;
      yyjson_val* call = yyjson_obj_get(op, "call");
      uint32_t target = lv4_u(call, "target");
      hako_lv4_value* receiver = lv4_index_value(caller, lv4_u(call, "receiver"));
      if (!target || target >= in->index_count ||
          !lv4_is(in->index[target].function, "role", "birth_unit") ||
          !receiver || receiver->kind != LV4_HANDLE || receiver->origin < 0) return 0;
      hako_lv4_function_index* birth = &in->index[target];
      /* One birth unit may serve several provider sites (two fields can
         construct the same class): a repeat binding is admitted only when
         it restates the already-bound child object exactly. */
      if (birth->receiver_object_set) {
        if (birth->receiver_object != receiver->object_id) return 0;
        continue;
      }
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
