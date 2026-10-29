/* Physical cohort admission only. Source names never participate. */
typedef struct {
  yyjson_val *root, *functions;
  uint32_t profile;
  int native;
  uint64_t result_site;
  struct hako_lv4_function_index* index;
  size_t index_count;
  struct hako_lv4_layout_index* layouts;
  size_t layout_count;
  hako_llvmc_lifecycle_measurement* measurement;
} hako_lv4_input;

enum {
  LV4_BAD, LV4_I64, LV4_HANDLE, LV4_FRAME, LV4_UNIT, LV4_TEXT, LV4_BOOL,
  LV4_F64, LV4_TAGGED, LV4_MAP, LV4_KEY, LV4_OUTCOME, LV4_MAP_VIEW, LV4_TEXT_VIEW, LV4_OWNED_STRING, LV4_SLOT_BORROW, LV4_BORROWED_OBJECT, LV4_MIXED_OBJECT, LV4_DYNAMIC_CARRIER, LV4_TEXT_OPERAND
};

static uint32_t lv4_u(yyjson_val* object, const char* key) {
  return (uint32_t)yyjson_get_uint(yyjson_obj_get(object, key));
}
static int lv4_is(yyjson_val* object, const char* key, const char* text) {
  return hako_physical_string(yyjson_obj_get(object, key), text);
}
static int lv4_is_ordinary(yyjson_val* function) {
  return hako_physical_ordinary_function(function);
}
#include "hako_llvmc_ffi_lifecycle_v4_state.inc"
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
  if (yyjson_is_uint(yyjson_obj_get(fn, "receiver")) && lv4_u(fn, "receiver") == value)
    return LV4_HANDLE;
  for (size_t i = 0; i < yyjson_arr_size(params); i++)
    if (lv4_u(yyjson_arr_get(params, i), "value") == value)
      return LV4_TAGGED;
  def = lv4_def(fn, value);
  if (!def) return LV4_BAD;
  if (lv4_is(def, "op", "const_i64") || lv4_is(def, "op", "add") ||
      lv4_is(def, "op", "mul") || lv4_is(def, "op", "sub") ||
      lv4_is(def, "op", "div") || lv4_is(def, "op", "mod") ||
      lv4_is(def, "op", "array_get") || lv4_is(def, "op", "array_length") ||
      lv4_is(def, "op", "object_field_get")) return LV4_I64;
  if (lv4_is(def, "op", "const_bool")) return LV4_BOOL;
  if (lv4_is(def, "op", "const_f64_bits")) return LV4_F64;
  if (lv4_is(def, "op", "invoke_normal_result")) {
    yyjson_val* origin = hako_physical_block_by_id(fn, lv4_u(def, "invoke_block"));
    yyjson_val* op = yyjson_obj_get(lv4_term(origin), "operation");
    if (lv4_is(op, "kind", "ordinary_call") && lv4_is(op, "result", "text"))
      return LV4_OWNED_STRING;
    if (lv4_is(op, "kind", "ordinary_call") && lv4_is(op, "result", "nullable_text"))
      return LV4_DYNAMIC_CARRIER;
    int result = hako_physical_string_result(yyjson_get_str(yyjson_obj_get(yyjson_obj_get(lv4_term(origin), "operation"), "kind")));
    return result == 1 ? LV4_OWNED_STRING : result == 2 ? LV4_I64 : result == 3 ? LV4_BOOL : LV4_HANDLE;
  }
  if (lv4_is(def, "op", "fault_frame_enter")) return LV4_FRAME;
  if (lv4_is(def, "op", "const_unit")) return LV4_UNIT;
  if (lv4_is(def, "op", "const_string")) return LV4_TEXT;
  if (lv4_is(def, "op", "copy")) return lv4_type(fn, lv4_u(def, "src"), budget - 1);
  return LV4_BAD;
}
static size_t lv4_value_budget(yyjson_val* fn) {
  yyjson_val* blocks = hako_physical_blocks(fn);
  size_t count = yyjson_arr_size(yyjson_obj_get(fn, "params"));
  if (count == SIZE_MAX) return SIZE_MAX;
  count++;
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    size_t rows = yyjson_arr_size(yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions"));
    if (rows > SIZE_MAX - count) return SIZE_MAX;
    count += rows;
  }
  return count;
}

#include "hako_llvmc_ffi_lifecycle_v4_index.inc"
#include "hako_llvmc_ffi_lifecycle_v4_native_admission.inc"

/* Descriptor availability never creates a Home. Only Phi edge transfer may
 * move an existing obligation into its dedicated carrier slot. */
static int lv4_text_readable(hako_lv4_function_index* fn, hako_lv4_value* value,
    const hako_lv4_state* state) {
  if (!value || (value->kind != LV4_TEXT && value->kind != LV4_OWNED_STRING && value->kind != LV4_TEXT_OPERAND)) return 0;
  int origin = lv4_text_origin(fn, value);
  if (origin == -1) return 1;
  if (origin < 0 || (size_t)origin >= lv4_state_count(fn)) return 0;
  hako_lv4_state rank = lv4_string_rank(state[origin]);
  return rank > LV4_STRING_RANK_BASE && rank < LV4_STRING_LIMIT;
}

/* Formal read admission borrows the completed value index, independently of flow. */
static yyjson_val* lv4_borrowed_formal_parameter(hako_lv4_function_index* fn, hako_lv4_value* value) {
  if (!value || (value->kind != LV4_TAGGED && value->kind != LV4_I64 && value->kind != LV4_BOOL) || !(lv4_is_ordinary(fn->function) || lv4_is(fn->function, "role", "birth_unit"))) return NULL;
  hako_lv4_value* root = lv4_value_alias_root(fn, value);
  yyjson_val* params = yyjson_obj_get(fn->function, "params");
  for (size_t i = 0; root && i < yyjson_arr_size(params); i++) {
    yyjson_val* param = yyjson_arr_get(params, i);
    if (lv4_u(param, "value") == root->id && lv4_is(param, "representation", "borrowed_kind_payload_v1") &&
        (value->kind == LV4_TAGGED || (value->kind == LV4_I64 && yyjson_is_true(yyjson_obj_get(param, "integer_view"))) ||
         (value->kind == LV4_BOOL && yyjson_is_true(yyjson_obj_get(param, "bool_view"))))) return param;
  }
  return NULL;
}
static int lv4_borrowed_formal_domain(hako_lv4_function_index* fn, hako_lv4_value* value, int text) {
  yyjson_val* param = lv4_borrowed_formal_parameter(fn, value);
  return value && value->kind == LV4_TAGGED && param && yyjson_is_true(yyjson_obj_get(param, "text_view")) == text;
}

static int lv4_borrowed_tagged_formal(hako_lv4_function_index* fn, hako_lv4_value* value) {
  return lv4_borrowed_formal_domain(fn, value, 0);
}
static int lv4_borrowed_text_formal(hako_lv4_function_index* fn, hako_lv4_value* value) {
  return lv4_borrowed_formal_domain(fn, value, 1);
}

/* A guarded `formal.field` read rides the same borrowed kind/payload
   formal: the copy root must be a borrowed param whose sealed `object_view`
   names exactly this canonical object. The emitted checked get re-validates
   type and slot at the site, so a null-state or foreign payload faults
   inside its own row instead of guessing a layout. */
static int lv4_borrowed_object_formal(hako_lv4_function_index* fn,
    hako_lv4_value* value, uint32_t object_id) {
  if (!lv4_borrowed_tagged_formal(fn, value)) return 0;
  hako_lv4_value* root = lv4_value_alias_root(fn, value);
  yyjson_val* params = yyjson_obj_get(fn->function, "params");
  for (size_t i = 0; root && i < yyjson_arr_size(params); i++) {
    yyjson_val* param = yyjson_arr_get(params, i);
    yyjson_val* view = yyjson_obj_get(param, "object_view");
    if (lv4_u(param, "value") == root->id)
      return yyjson_is_uint(view) && yyjson_get_uint(view) == object_id;
  }
  return 0;
}

static int lv4_nonnull_object_formal(hako_lv4_function_index* fn,
    hako_lv4_value* value, uint32_t object) {
  return lv4_borrowed_object_formal(fn, value, object) &&
      yyjson_is_true(yyjson_obj_get(lv4_borrowed_formal_parameter(fn, value), "object_nonnull_view"));
}

/* The null sentinel's other admitted lanes: the function carries at
   least one `borrowed_null_compare` row, or a call edge whose borrowed
   actual spells tag 0 — the arg's own `value` field still names the
   exact const_null producer at the flow arm below. Like the other const
   lanes this is a function-scope admission — dead const projections
   stay admitted and the compare/arg arms pin their exact siblings. */
static int lv4_null_lane_present(hako_lv4_function_index* fn) {
  yyjson_val* blocks = hako_physical_blocks(fn->function);
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* rows = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
    yyjson_val* op;
    for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
      yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
      if (lv4_is(ins, "op", "borrowed_null_compare")) return 1;
    }
    op = yyjson_obj_get(lv4_term(yyjson_arr_get(blocks, bi)), "operation");
    if (op && lv4_is(op, "kind", "array_owned_append") &&
        yyjson_get_bool(yyjson_obj_get(op, "nullable"))) return 1;
    if (op && (lv4_is(op, "kind", "ordinary_call") ||
        lv4_is(op, "kind", "birth_call"))) {
      yyjson_val* args = yyjson_obj_get(yyjson_obj_get(op, "call"), "args");
      for (size_t i = 0; args && i < yyjson_arr_size(args); i++) {
        yyjson_val* kind = yyjson_obj_get(yyjson_arr_get(args, i), "kind");
        if (yyjson_is_uint(kind) && yyjson_get_uint(kind) == 0) return 1;
      }
    }
  }
  return 0;
}

static int lv4_dynamic_operand_live(hako_lv4_function_index* fn, yyjson_val* operand,
    const hako_lv4_state* state);

static int lv4_string_indexed_operand(hako_lv4_function_index* fn, yyjson_val* operand,
    const hako_lv4_state* state) {
  if (lv4_is(operand, "kind", "static_text")) return 1;
  if (lv4_is(operand, "kind", "dynamic_carrier")) return lv4_dynamic_operand_live(fn, operand, state);
  hako_lv4_value* value = lv4_index_value(fn, lv4_u(operand, "value"));
  if (lv4_is(operand, "kind", "text_operand")) return lv4_text_readable(fn, value, state);
  if (lv4_is(operand, "kind", "normal_integer")) return value && value->kind == LV4_I64;
  if (lv4_is(operand, "kind", "normal_bool")) return value && value->kind == LV4_BOOL;
  if (lv4_is(operand, "kind", "borrowed_text")) return lv4_borrowed_text_formal(fn, value);
  if (lv4_is(operand, "kind", "tagged_dynamic"))
    return hako_physical_finite_domain(lv4_borrowed_formal_parameter(fn, value)) != 0;
  return value && value->kind == LV4_OWNED_STRING && value->origin >= 0 &&
      state[value->origin] > LV4_STRING_RANK_BASE;
}

static int lv4_string_indexed_integer_operand(hako_lv4_function_index* fn,
    yyjson_val* operand) {
  hako_lv4_value* value = lv4_index_value(fn, lv4_u(operand, "value"));
  if (lv4_is(operand, "kind", "normal_integer")) return value && value->kind == LV4_I64;
  return lv4_is(operand, "kind", "tagged_dynamic") && value &&
      value->kind == LV4_TAGGED && lv4_borrowed_tagged_formal(fn, value);
}

static int lv4_dynamic_operand_live(hako_lv4_function_index* fn, yyjson_val* operand,
    const hako_lv4_state* state) {
  if (lv4_is(operand, "kind", "null")) return 1;
  hako_lv4_value* value = lv4_index_value(fn, lv4_u(operand, "value"));
  if (!value) return 0;
  if (lv4_is(operand, "kind", "integer")) return value->kind == LV4_I64;
  if (lv4_is(operand, "kind", "bool")) return value->kind == LV4_BOOL;
  if (lv4_is(operand, "kind", "tagged_dynamic"))
    return value->kind == LV4_TAGGED && lv4_borrowed_tagged_formal(fn, value) &&
        !(hako_physical_finite_domain(lv4_borrowed_formal_parameter(fn, value)) & (1u << 4));
  return lv4_is(operand, "kind", "dynamic_carrier") &&
      value->kind == LV4_DYNAMIC_CARRIER && value->origin >= 0 &&
      state[value->origin] == LV4_DYNAMIC_CARRIER_LIVE;
}


#include "hako_llvmc_ffi_lifecycle_v4_indexed_flow.inc"

static int lv4_indexed_admit(hako_lv4_input* in, char** error) {
  if (!lv4_index_open(in)) return 0;
  /* Every independent producer in every function precedes any dependent
   * Field classification or recursive owned-result correspondence. A caller
   * cannot demand class from a callee whose original producer is unseeded. */
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    for (size_t vi = 0; vi < fn->value_count; vi++) {
      hako_lv4_value* value = &fn->values[vi];
      if (lv4_is(value->instruction, "op", "object_field_get")) continue;
      if (!lv4_index_seed(in, fn, value)) return 0;
    }
  }
  /* Birth callers bind borrowed receivers; ordinary callees already carry the
   * prepared object identity in their physical function row. */
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* caller = &in->index[fi];
    int ordinary = lv4_is_ordinary(caller->function);
    yyjson_val* blocks = hako_physical_blocks(caller->function);
    for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val* op = yyjson_obj_get(lv4_term(yyjson_arr_get(blocks, bi)), "operation");
      if (!lv4_is(op, "kind", "birth_call")) continue;
      int birth_caller = lv4_is(caller->function, "role", "birth_unit");
      if (fi && !ordinary && !birth_caller) return 0;
      yyjson_val* call = yyjson_obj_get(op, "call");
      uint32_t target = lv4_u(call, "target");
      hako_lv4_value* receiver = lv4_value_alias_root(caller,
          lv4_index_value(caller, lv4_u(call, "receiver")));
      if (!target || target >= in->index_count ||
          !lv4_is(in->index[target].function, "role", "birth_unit") ||
          !receiver || receiver->kind != LV4_HANDLE || !receiver->object_id_set ||
          receiver->origin < 0) return 0;
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
        if (birth->values[vi].origin == -2) {
          birth->values[vi].object_id = receiver->object_id;
          birth->values[vi].object_id_set = receiver->object_id_set;
        }
    }
  }
  for (size_t fi = 0; fi < in->index_count; fi++)
    if (!lv4_index_kinds(in, &in->index[fi])) return 0;
  if (!lv4_index_owned_results(in)) return 0;
  for (size_t fi = 0; fi < in->index_count; fi++) {
    hako_lv4_function_index* fn = &in->index[fi];
    size_t count = fn->block_count, frames = 0;
    hako_lv4_worklist work = {0};
    if (lv4_is(fn->function, "role", "birth_unit") && !fn->receiver_object_set) return 0;
    hako_lv4_block* entry = lv4_index_block(fn, lv4_u(fn->function, "entry"));
    for (size_t bi = 0; bi < count; bi++) {
      yyjson_val* rows = yyjson_obj_get(fn->blocks[bi].block, "instructions");
      for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++)
        frames += lv4_is(yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction"), "op", "fault_frame_enter");
      yyjson_val* term = lv4_term(fn->blocks[bi].block);
      if (lv4_operation_result(yyjson_obj_get(term, "operation")) != LV4_BAD) {
        uint32_t id = fn->blocks[bi].id, normal = lv4_u(term, "normal");
        size_t projections = 0;
        if (lv4_pred_count(fn, normal) != 1) return 0;
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
        lv4_pred_count(fn, entry->id)) return 0;
    if (!lv4_work_open(&work, count, lv4_state_count(fn), entry->ordinal)) return 0;
    int valid = 1;
    size_t failed_block = entry->ordinal;
    const char* failed_phase = "flow";
    while (valid == 1 && work.pending) {
      size_t bi = work.queue[work.head]; work.head = (work.head + 1) % work.count;
      work.queued[bi] = 0; work.pending--;
      failed_block = bi;
      valid = lv4_indexed_flow_block(in, fi, bi, &work);
    }
    if (valid == LV4_FLOW_RECEIVER_MISMATCH) {
      if (getenv("HAKO_PHYSICAL_DEBUG")) {
        const char* name = yyjson_get_str(yyjson_obj_get(fn->function, "name"));
        char* json = yyjson_val_write(lv4_term(fn->blocks[failed_block].block), 0, NULL);
        fprintf(stderr, "[lifecycle-v4-receiver-mismatch] function=%zu name=%s block=%u %s\n", fi,
            name ? name : "?", fn->blocks[failed_block].id, json ? json : "?");
        free(json);
      }
      lv4_work_close(&work);
      return valid;
    }
    for (size_t bi = 0; valid && bi < count; bi++) if (work.faults[bi] < 0) {
      failed_block = bi; failed_phase = "unreachable"; valid = 0;
    }
    lv4_work_close(&work);
    if (!valid) {
      char message[320];
      const char* name = yyjson_get_str(yyjson_obj_get(fn->function, "name"));
      const char* op = yyjson_get_str(yyjson_obj_get(lv4_term(fn->blocks[failed_block].block), "op"));
      const char* kind = yyjson_get_str(yyjson_obj_get(yyjson_obj_get(lv4_term(fn->blocks[failed_block].block), "operation"), "kind"));
      snprintf(message, sizeof(message),
          "[freeze:contract][published-lifecycle-v4/unsupported-cohort] function=%zu name=%.120s block=%u phase=%s terminator=%.40s operation=%.40s",
          fi, name ? name : "<invalid>", fn->blocks[failed_block].id, failed_phase, op ? op : "<invalid>", kind ? kind : "-");
      if (getenv("HAKO_PHYSICAL_DEBUG")) {
        char* json = yyjson_val_write(lv4_term(fn->blocks[failed_block].block), 0, NULL);
        char* edges = yyjson_val_write(yyjson_obj_get(fn->blocks[failed_block].block, "edges"), 0, NULL);
        fprintf(stderr, "[lifecycle-v4-refused] %s edges=%s\n", json ? json : "?", edges ? edges : "?");
        free(json); free(edges);
      }
      set_err_owned(error, message);
      return 0;
    }
  }
  return 1;
}

static int lv4_admit(yyjson_val* root, hako_lv4_input* out, char** error) {
  int result;
  out->root = root; out->functions = yyjson_obj_get(root, "functions");
  out->native = yyjson_obj_get(root, "runtime_requirements") != NULL;
  out->result_site = yyjson_get_uint(yyjson_obj_get(root, "process_result_site"));
  out->profile = lv4_u(root, "storage_profile");
  result = out->native ? lv4_native_admit(out) : lv4_indexed_admit(out, error);
  if (result) {
    if (result == LV4_FLOW_RECEIVER_MISMATCH)
      return set_err_owned(error,
          "[freeze:contract][published-lifecycle-v4/receiver-object-mismatch]");
    return 0;
  }
  if (error && *error) return -1;
  return set_err_owned(error, "[freeze:contract][published-lifecycle-v4/unsupported-cohort]");
}
