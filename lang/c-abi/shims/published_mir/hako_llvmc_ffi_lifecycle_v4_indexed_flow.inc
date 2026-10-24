/* Exact physical obligations on supplied CFG edges. Not a source Home. */
enum { LV4_INDEXED_SAFE_MUTEX = 1, LV4_LIVE = 1 };
enum { LV4_FLOW_RECEIVER_MISMATCH = -2, LV4_CONDITIONAL_OBJECT = 260 };
_Static_assert(LV4_CONDITIONAL_OBJECT < LV4_TRANSFER_BASE, "Mixed/Map states overlap");

static int lv4_indexed_live(hako_lv4_function_index* fn, uint32_t id, int kind,
    const hako_lv4_state* state, uint32_t object, int check_object) {
  hako_lv4_value* value = lv4_index_value(fn, id);
  if (!value || value->kind != kind || (check_object && (!value->object_id_set || value->object_id != object))) return 0;
  /* Borrowed operand (-2): a receiver handle lives for the whole call. */
  if (value->origin == -2) return kind == LV4_HANDLE;
  return value->origin >= 0 && state[value->origin] == LV4_LIVE;
}

/* A checked-map read operand: a live owned lease, or a caller-owned MapBox
 * formal (origin -2) borrowed for the whole call. Only the read lane uses
 * this predicate — install and end lanes keep lv4_indexed_live, so borrowed
 * storage is never mutated or ended. */
static int lv4_indexed_map_readable(hako_lv4_function_index* fn, uint32_t id,
    const hako_lv4_state* state) {
  hako_lv4_value* value = lv4_index_value(fn, id);
  if (!value || value->kind != LV4_MAP) return 0;
  if (value->origin == -2) return 1;
  return value->origin >= 0 && state[value->origin] == LV4_LIVE;
}

#include "hako_llvmc_ffi_lifecycle_v4_field_borrow.inc"

#include "hako_llvmc_ffi_lifecycle_v4_owned_array_flow.inc"

#include "hako_llvmc_ffi_lifecycle_v4_indexed_birth_fields.inc"

/* Wrapper arguments never become an owned-object holder/descriptor. */
static hako_lv4_value* lv4_script_arguments_root(hako_lv4_function_index* fn, uint32_t id) {
  hako_lv4_value* root = lv4_value_alias_root(fn, lv4_index_value(fn, id));
  if (!root || root->kind != LV4_HANDLE || root->origin < 0 ||
      !lv4_is(root->instruction, "op", "invoke_normal_result")) return NULL;
  hako_lv4_block* origin = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
  return origin && lv4_is(yyjson_obj_get(lv4_term(origin->block), "operation"),
      "kind", "script_arguments_new") ? root : NULL;
}

static int lv4_borrowed_typed_live(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state) {
  if (!value || value->kind != LV4_HANDLE || !lv4_layout(in, value->object_id) ||
      !lv4_indexed_live(fn, value->id, LV4_HANDLE, state, value->object_id, 1)) return 0;
  hako_lv4_value* root = lv4_value_alias_root(fn, value);
  if (!root || root->object_id != value->object_id) return 0;
  if (root->origin == -2)
    return fn->receiver_object_set && yyjson_is_uint(yyjson_obj_get(fn->function, "receiver")) &&
        root->id == lv4_u(fn->function, "receiver") && root->object_id == fn->receiver_object;
  if (!lv4_is(root->instruction, "op", "invoke_normal_result")) return 0;
  hako_lv4_block* producer = lv4_index_block(fn, lv4_u(root->instruction, "invoke_block"));
  yyjson_val* op = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
  return lv4_is(op, "kind", "new_box") && lv4_u(op, "object_id") == root->object_id;
}

#include "hako_llvmc_ffi_lifecycle_v4_borrowed_result_anchor.inc"
#include "hako_llvmc_ffi_lifecycle_v4_field_dependency_liveness.inc"
#include "hako_llvmc_ffi_lifecycle_v4_mixed_result_flow.inc"
#include "hako_llvmc_ffi_lifecycle_v4_received_field_read.inc"
#include "hako_llvmc_ffi_lifecycle_v4_field_array_flow.inc"
#include "hako_llvmc_ffi_lifecycle_v4_indexed_rows.inc"

#include "hako_llvmc_ffi_lifecycle_v4_flow_workspace.inc"


#include "hako_llvmc_ffi_lifecycle_v4_indexed_return.inc"

static int lv4_indexed_flow_block(const hako_lv4_input* in, size_t fi, size_t bi,
    hako_lv4_worklist* work) {
  hako_lv4_function_index* fn = &in->index[fi];
  yyjson_val* blocks = hako_physical_blocks(fn->function);
  yyjson_val* block = yyjson_arr_get(blocks, bi);
  yyjson_val* term = lv4_term(block);
  size_t count = work->stride;
  int faulted = work->faults[bi];
  if (in->measurement) in->measurement->v4_flow_blocks++;
  hako_lv4_state *state = work->scratch, *normal = state + count;
  memcpy(state, work->states + bi * count, count * sizeof(*state));
  int birth = lv4_is(fn->function, "role", "birth_unit");
  int ordinary = lv4_is_ordinary(fn->function);
  if (!lv4_indexed_rows(in, fn, block, state, faulted, birth)) return 0;
  if (lv4_is(term, "op", "return") || lv4_is(term, "op", "return_fault"))
    return lv4_indexed_return(in, fn, term, state, faulted);
  memcpy(normal, state, count * sizeof(*state));
  int failed = faulted;
  if (lv4_is(term, "op", "invoke")) {
    yyjson_val* op = yyjson_obj_get(term, "operation");
    failed = 1;
    if (lv4_string_pending(state, count) && !(lv4_is(op, "kind", "string_end") || lv4_is(op, "kind", "string_end_if_owned"))) return 0;
    if ((lv4_string_held(state, count) || lv4_string_call_pending(state, count)) && (lv4_is(op, "kind", "ordinary_call") ||
        hako_physical_array_text_append(op) || hako_physical_map_field_text_read(op) || (hako_physical_string_kind(yyjson_get_str(yyjson_obj_get(op, "kind"))) && !(lv4_is(op, "kind", "string_end") || lv4_is(op, "kind", "string_end_if_owned"))))) return 0;
    if (lv4_owned_array_kind(op)) {
      if (birth || !lv4_owned_array_step(in, fn, op, faulted, state, normal, bi)) return 0;
    } else if (!hako_physical_array_text_append(op) && hako_physical_string_operand_count(op)) {
      if (birth || faulted || state[bi]) return 0;
      if (hako_physical_map_field_text_read(op) && (in->profile != LV4_INDEXED_SAFE_MUTEX ||
          !lv4_birth_dynamic(in, op) || lv4_slot(in, op) < 0 ||
          !lv4_readable_object(in, fn, lv4_u(op, "base"), state, lv4_u(block, "id"),
              lv4_u(op, "object_id"), 1))) return 0;
      for (size_t role = 0; role < hako_physical_string_operand_count(op); role++) {
        yyjson_val* operand = hako_physical_string_operand_at(op, role);
        int integer_index = lv4_is(op, "kind", "string_substring") && role > 0;
        if (!(integer_index ? lv4_string_indexed_integer_operand(fn, operand) :
            lv4_string_indexed_operand(fn, operand, state))) return 0;
      }
      if (hako_physical_string_result(yyjson_get_str(yyjson_obj_get(op, "kind"))) == 1 &&
          !lv4_string_push(state, count, &normal[bi])) return 0;
      if (hako_physical_string_result(yyjson_get_str(yyjson_obj_get(op, "kind"))) >= 2 || hako_physical_map_field_text_read(op)) normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "string_end") || lv4_is(op, "kind", "string_end_if_owned")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
      int conditional = lv4_is(op, "kind", "string_end_if_owned");
      if (birth || !value || (!conditional && value->kind != LV4_OWNED_STRING) ||
          (conditional && !lv4_text_readable(fn, value, state))) return 0;
      int origin = conditional ? lv4_text_origin(fn, value) : value->origin;
      if (origin != -1 && !lv4_string_end_paths(state, normal, count, origin, faulted)) return 0;
    } else if (lv4_is(op, "kind", "ordinary_call")) {
      yyjson_val* call = yyjson_obj_get(op, "call");
      yyjson_val* args = yyjson_obj_get(call, "args");
      uint32_t target = lv4_u(call, "target");
      yyjson_val* callee = (target < in->index_count) ? in->index[target].function : NULL;
      yyjson_val* receiver = yyjson_obj_get(call, "receiver");
      int callee_has_receiver = callee && yyjson_is_uint(yyjson_obj_get(callee, "receiver"));
      int want_unit = lv4_is(op, "result", "unit");
      /* Birth keeps the I64 provider lane. A zero-argument Unit child call
         additionally rechecks the exact committed field loan below. */
      int birth_i64 = birth && lv4_is(op, "result", "i64");
      hako_lv4_value* birth_receiver = receiver && yyjson_is_uint(receiver) ?
          lv4_index_value(fn, (uint32_t)yyjson_get_uint(receiver)) : NULL;
      int birth_unit = birth && want_unit && callee_has_receiver && !yyjson_arr_size(args) &&
          lv4_birth_field_borrow_live(in, fn, lv4_value_alias_root(fn, birth_receiver),
              state, lv4_u(block, "id"));
      if ((fi && !ordinary && !birth_i64 && !birth_unit) || faulted || state[bi] || !target || target >= in->index_count ||
          !hako_physical_string(yyjson_obj_get(callee, "role"),
              hako_physical_ordinary_result_role(yyjson_obj_get(op, "result"))) ||
          (callee_has_receiver != (receiver && !yyjson_is_null(receiver))) ||
          yyjson_arr_size(args) != yyjson_arr_size(yyjson_obj_get(in->index[target].function, "params"))) return 0;
      if (callee_has_receiver) {
        if (!yyjson_is_uint(receiver) || !in->index[target].receiver_object_set) return 0;
        if (!lv4_readable_object(in, fn, lv4_u(call, "receiver"), state, lv4_u(block, "id"),
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
            yyjson_val* view = yyjson_obj_get(yyjson_arr_get(cparams, i), "object_view");
            unsigned finite_domain = hako_physical_finite_domain(yyjson_arr_get(cparams, i));
            int class_domain = yyjson_is_uint(view);
            int text_domain = yyjson_is_true(yyjson_obj_get(yyjson_arr_get(cparams, i), "text_view"));
            int integer_domain = yyjson_is_true(yyjson_obj_get(yyjson_arr_get(cparams, i), "integer_view"));
            int bool_domain = yyjson_is_true(yyjson_obj_get(yyjson_arr_get(cparams, i), "bool_view"));
            if (bool_domain && (!yyjson_is_uint(kind) || yyjson_get_uint(kind) != 2 ||
                !value || value->kind != LV4_BOOL)) return 0;
            if (integer_domain && (!yyjson_is_uint(kind) || yyjson_get_uint(kind) != 1 ||
                !value || value->kind != LV4_I64)) return 0;
            if (arg_kind && !strcmp(arg_kind, "borrowed_text")) {
              yyjson_val* operand = yyjson_obj_get(arg, "operand");
              if (!(hako_physical_input_domain(yyjson_arr_get(cparams, i)) & (1u << 4)) || class_domain ||
                  !(lv4_is(operand, "kind", "static_text") || lv4_is(operand, "kind", "owned_string")) ||
                  !lv4_string_indexed_operand(fn, operand, state)) return 0;
              continue; /* Caller owns the pair through either Call outcome. */
            }
            if (arg_kind && !strcmp(arg_kind, "tagged")) {
              unsigned incoming = hako_physical_input_domain(lv4_borrowed_formal_parameter(fn, value));
              if (finite_domain && (!incoming || (incoming & ~finite_domain))) return 0;
              if (!finite_domain && !text_domain && (incoming & (1u << 4))) return 0;
              if ((!finite_domain && !(text_domain ? lv4_borrowed_text_formal(fn, value) : lv4_borrowed_tagged_formal(fn, value))) ||
                  (class_domain && !lv4_borrowed_object_formal(fn, value, (uint32_t)yyjson_get_uint(view)))) return 0;
            } else if (arg_kind && !strcmp(arg_kind, "nullable_typed_object")) {
              /* A caller-owned received nullable lends its handle lane
                 borrowed: the Void sentinel or a live lease — never a
                 spent origin, never a forwarded tagged carrier. The ABI
                 owner selects tag 0/3 at runtime. */
              if (finite_domain) return 0;
              int array_borrow = value && value->kind == LV4_SLOT_BORROW;
              if (value && value->kind == LV4_BORROWED_OBJECT) {
                if (!lv4_borrowed_result_readable(in, fn, value, state, lv4_u(block, "id"))) return 0;
                hako_lv4_value* anchor = lv4_borrowed_result_anchor(in, fn, value);
                if (class_domain && ((anchor->object_id_set && anchor->object_id != yyjson_get_uint(view)) ||
                    (anchor->kind == LV4_TAGGED && !lv4_borrowed_object_formal(fn, anchor,
                        (uint32_t)yyjson_get_uint(view))))) return 0;
              } else if (value && value->kind == LV4_MIXED_OBJECT) {
                /* Preserve the conditional obligation. Fresh and the SAME
                   fixed-formal actual both agree with the receiving domain;
                   nested Mixed anchors share one decreasing recursion budget. */
                if (!lv4_mixed_result_readable_depth(in, fn, value, state, lv4_u(block, "id"),
                    fn->value_count + 1, class_domain ? (uint32_t)yyjson_get_uint(view) : 0,
                    class_domain)) return 0;
              } else if (array_borrow) {
                if (!class_domain || !lv4_owned_array_borrow_live(fn, value, state) ||
                    !value->object_id_set || value->object_id != yyjson_get_uint(view)) return 0;
              } else if (value && value->kind == LV4_HANDLE && value->origin == LV4_OWNED_FIELD_BORROW) {
                if ((class_domain && (!value->object_id_set || value->object_id != yyjson_get_uint(view))) ||
                    !lv4_nullable_field_loan_readable(in, fn, value->id, state,
                        lv4_u(block, "id"), class_domain ? (uint32_t)yyjson_get_uint(view) : 0,
                        class_domain)) return 0;
              } else if (!value || value->kind != LV4_HANDLE || value->origin < 0 ||
                  state[value->origin] != LV4_LIVE) return 0;
              /* A nullable spelling cannot erase a known New/Copy class.
                 Unknown acquired call-result class is checked in prologue. */
              if (class_domain && lv4_borrowed_typed_live(in, fn, value, state) &&
                  value->object_id != yyjson_get_uint(view)) return 0;
            } else if (yyjson_is_uint(kind)) {
              uint64_t tag = yyjson_get_uint(kind);
              if (finite_domain && (tag > 4 || !(finite_domain & (1u << tag)))) return 0;
              if (class_domain && (tag != 0 && tag != 3)) return 0;
              /* Tag 0 admits the exact const_null producer only — a
                 forged zero payload never spells the null actual. */
              if (!value ||
                  (tag == 0 &&
                      !lv4_is(value->instruction, "op", "const_null")) ||
                  (tag == 1 && value->kind != LV4_I64) ||
                  (tag == 2 && value->kind != LV4_BOOL) ||
                  (tag == 3 && !lv4_borrowed_typed_live(in, fn, value, state)) ||
                  tag > 3) return 0;
              if (class_domain && tag == 3 && value->object_id != yyjson_get_uint(view)) return 0;
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
      if (lv4_is(op, "result", "text")) {
        yyjson_val* cleanup = yyjson_obj_get(op, "text_argument_cleanup");
        size_t size = yyjson_arr_size(cleanup);
        if (!size) {
          if (!lv4_string_push(state, count, &normal[bi])) return 0;
        } else {
          hako_lv4_state top = lv4_string_domain_top(state, count, LV4_STRING_RANK_BASE, LV4_STRING_HOME_BASE);
          if (size > top - LV4_STRING_RANK_BASE || top + 1 >= LV4_STRING_HOME_BASE) return 0;
          for (size_t i = 0; i < size; i++) {
            hako_lv4_value* arg = lv4_index_value(fn, (uint32_t)yyjson_get_uint(yyjson_arr_get(cleanup, i)));
            if (!arg || arg->kind != LV4_OWNED_STRING || arg->origin < 0 || (size_t)arg->origin >= count ||
                state[arg->origin] != top - i) return 0;
            normal[arg->origin] = state[arg->origin] + 1;
          }
          normal[bi] = (top - size + 1) | LV4_STRING_CALL_HELD;
        }
      } else if (!want_unit && !lv4_is(op, "result", "borrowed_nullable_handle"))
        normal[bi] = lv4_is(op, "result", "mixed_handle") ? LV4_CONDITIONAL_OBJECT : LV4_LIVE;
    } else if (lv4_is(op, "kind", "dynamic_invoke")) {
      yyjson_val* arguments = yyjson_obj_get(op, "arguments");
      if ((fi && !ordinary && !birth) || birth || faulted || state[bi] ||
          !lv4_dynamic_operand_live(fn, yyjson_obj_get(op, "receiver"), state)) return 0;
      for (size_t i = 0; i < yyjson_arr_size(arguments); i++)
        if (!lv4_dynamic_operand_live(fn, yyjson_arr_get(arguments, i), state)) return 0;
      normal[bi] = LV4_DYNAMIC_CARRIER_LIVE;
    } else if (lv4_is(op, "kind", "dynamic_carrier_integer")) {
      hako_lv4_value* carrier = lv4_index_value(fn, lv4_u(op, "carrier"));
      if (birth || faulted || state[bi] || !carrier || carrier->kind != LV4_DYNAMIC_CARRIER ||
          carrier->origin < 0 || state[carrier->origin] != LV4_DYNAMIC_CARRIER_LIVE) return 0;
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "dynamic_carrier_end")) {
      hako_lv4_value* carrier = lv4_index_value(fn, lv4_u(op, "carrier"));
      if (birth || !carrier || carrier->kind != LV4_DYNAMIC_CARRIER || carrier->origin < 0 ||
          state[carrier->origin] != LV4_DYNAMIC_CARRIER_LIVE) return 0;
      /* Runtime End consumes even when it records Fault; the Fault edge must
         never retry this same carrier. */
      normal[carrier->origin] = state[carrier->origin] = 0;
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
    } else if (lv4_is(op, "kind", "script_arguments_new")) {
      if (fi || faulted || state[bi]) return 0;
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "array_new")) {
      /* A provider `new ArrayBox()` inside a birth unit (or ordinary
         function) issues a live handle lease; a following field store
         moves it into the object slot or a release discharges it. */
      if ((fi && !ordinary && !birth) || faulted || state[bi]) return 0;
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "array_append")) {
      if (faulted || birth ||
          !lv4_field_array_readable(in, fn, lv4_u(op, "array"), state, lv4_u(block, "id"))) return 0;
      if (hako_physical_array_text_append(op)) {
        if (!lv4_string_indexed_operand(fn, yyjson_obj_get(op, "value"), state)) return 0;
      } else {
        hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
        if (!lv4_is(op, "representation", "i64") || !value || value->kind != LV4_I64) return 0;
      }
      /* Copy/borrow only: both outcomes keep parent and input obligations. */
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
        if (arg_kind && !strcmp(arg_kind, "nullable_typed_object")) {
          if (value && value->kind == LV4_MIXED_OBJECT) {
            if (value->origin < 0 || (size_t)value->origin >= count ||
                normal[value->origin] != LV4_CONDITIONAL_OBJECT ||
                !lv4_mixed_birth_actual(in, fn, target, i, value, state, lv4_u(block, "id"))) return 0;
            normal[value->origin] = 0; /* Fault retains the original obligation. */
            continue;
          }
          if (!value || value->origin < 0 ||
              !lv4_indexed_live(fn, value->id, LV4_HANDLE, state, 0, 0) ||
              normal[value->origin] != LV4_LIVE ||
              !lv4_birth_actual_object(in, fn, target, i, value, 1)) return 0;
          normal[value->origin] = 0;
          continue;
        }
        if (arg_kind) {
          if (strcmp(arg_kind, "tagged") || !lv4_borrowed_tagged_formal(fn, value))
            return 0;
          continue;
        }
        /* arg kind is exact: 0 is the const_null producer (the same pin
           the ordinary borrowed lane owns), 1 is i64, 2 is bool, 3 is an
           owned handle moving into the callee's nullable pair formal —
           the move completes on the invoke's Normal edge, so Fault keeps
           the lease live for the caller's own unwind discharge. Anything
           else, including a missing field or a forged zero payload,
           rejects rather than defaulting. */
        uint32_t numeric_kind = lv4_u(arg, "kind");
        if (!value ||
            (numeric_kind == 0 &&
                !lv4_is(value->instruction, "op", "const_null")) ||
            (numeric_kind != 0 && numeric_kind != 1 && numeric_kind != 2 &&
                numeric_kind != 3) ||
            (numeric_kind == 1 && value->kind != LV4_I64) ||
            (numeric_kind == 2 && value->kind != LV4_BOOL) ||
            (numeric_kind == 3 &&
                (value->origin < 0 ||
                 !lv4_indexed_live(fn, value->id, LV4_HANDLE, state, 0, 0) ||
                 !lv4_birth_actual_object(in, fn, target, i, value, 0))) ||
            (numeric_kind == 0 && value->kind != LV4_HANDLE)) return 0;
        if (numeric_kind == 3) {
          if (normal[value->origin] != LV4_LIVE) return 0;
          normal[value->origin] = 0;
        }
      }
    } else if (lv4_is(op, "kind", "field_get")) {
      if (faulted || birth || !lv4_received_field_readable(in, fn, op, state, lv4_u(block, "id"))) return 0;
      /* The receiver's original nullable/conditional obligation is untouched.
         The I64 result is available exclusively on Normal. */
      normal[bi] = LV4_LIVE;
    } else if (lv4_is(op, "kind", "field_set")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
      hako_lv4_value* base = lv4_index_value(fn, lv4_u(op, "base"));
      yyjson_val* range = yyjson_obj_get(op, "exact_numeric_runtime_check");
      if (lv4_is(op, "transaction", "dynamic_i64_initialize") ||
          lv4_is(op, "transaction", "dynamic_map_initialize")) {
        int map = lv4_is(op, "transaction", "dynamic_map_initialize");
        if (!birth || faulted || !value || !lv4_birth_dynamic(in, op) ||
            !lv4_is(range, "kind", "none") || !lv4_birth_commit(in, fn, op, normal, bi, 0)) return 0;
        if (map) {
          if (in->profile != LV4_INDEXED_SAFE_MUTEX || value->kind != LV4_MAP || value->origin < 0 ||
              !lv4_indexed_live(fn, value->id, LV4_MAP, state, 0, 0)) return 0;
          yyjson_val* producer = lv4_term(fn->blocks[value->origin].block);
          /* This source provider is bare new MapBox(), immediately stored.
           * No borrowed entries or unproved contents escape into a field. */
          if (!lv4_is(yyjson_obj_get(producer, "operation"), "kind", "map_new") ||
              lv4_u(producer, "normal") != lv4_u(block, "id") ||
              hako_physical_predecessor_count(fn->function, lv4_u(block, "id")) != 1) return 0;
          normal[value->origin] = 0; /* Fault still owns the in-flight candidate. */
        } else if (value->kind != LV4_I64) return 0;
      } else if (lv4_is(op, "transaction", "dynamic_i64_replace")) {
        /* This source family carries signed I64 only, never tagged payloads or Homes. */
        if (birth || faulted || !value || value->kind != LV4_I64 ||
            !lv4_is(range, "kind", "none") || !base ||
            !lv4_readable_object(in, fn, base->id, state, lv4_u(block, "id"), lv4_u(op, "object_id"), 1)) return 0;
      } else {
      /* The published source-owned usize write borrows two original tagged
         formals. The base retains its sealed class; neither operand owns a
         lease. Runtime kind/range/slot checks precede the store itself. */
      int borrowed_write = ordinary &&
          lv4_is(range, "kind", "dynamic_integer_range") &&
          lv4_is(range, "declared_type", "usize") &&
          lv4_borrowed_object_formal(fn, base, lv4_u(op, "object_id")) &&
          lv4_borrowed_tagged_formal(fn, value);
      /* A birth caller installs provider `new` results and scalar formals
         into its borrowed receiver's fields: the base is the stamped
         receiver handle (origin -2, object-corroborated), the value an
         in-flight live lease or a scalar lane exactly as below. */
      if ((fi && !ordinary && !birth) || faulted || lv4_slot(in, op) < 0 || !value) return 0;
      if (value->kind == LV4_HANDLE) {
        /* A live handle stored into an owned field moves the lease into
           the object slot; a `field_residence_release` on the base
           discharges it later — the store consumes the lease here. */
        if (value->origin < 0 ||
            !lv4_indexed_live(fn, lv4_u(op, "value"), LV4_HANDLE, state,
                0, 0)) return 0;
        /* A metadata-reserved slot accepts only its declared tuple: an
           `owned_residences` (array) slot takes an exact `array_new`
           lease — an ordinary handle store never reissues the sealed
           residence. Scalar values never land in a reserved slot at all:
           the sealed residence is owned state, not i64 storage. */
        if (lv4_birth_reserved(in, op) &&
            (!lv4_birth_owned(in, op, 0) || !lv4_birth_array_value(fn, value))) return 0;
        if (birth && (!lv4_birth_array_value(fn, value) ||
            !lv4_birth_commit(in, fn, op, normal, bi, 0))) return 0;
        normal[value->origin] = state[value->origin] = 0;
      } else if ((value->kind != LV4_I64 && !((birth || borrowed_write) && value->kind == LV4_TAGGED)) ||
          lv4_birth_reserved(in, op)) return 0;
      if (!borrowed_write && !lv4_indexed_live(fn, lv4_u(op, "base"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), 1)) return 0;
      }
    } else if (lv4_is(op, "kind", "field_residence_release")) {
      /* An owned field residence release discharges only the child slot.
         The base Home stays live until its own release consumes it, so
         the operand corroborates the exact object without ending it. */
      hako_lv4_value* base = lv4_index_value(fn, lv4_u(op, "base"));
      /* Owned bases retain their existing teardown. A borrowed Birth base
         additionally requires an exact Normal-committed slot marker. */
      if ((fi && !ordinary && !birth) || lv4_slot(in, op) < 0 || !base ||
          !lv4_indexed_live(fn, lv4_u(op, "base"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), 1)) return 0;
      if (lv4_is(op, "transaction", "take_dynamic") && base->origin < 0 && !birth) return 0;
      if (base->origin < 0 && (!birth ||
          !lv4_birth_discharge(in, fn, op, state, normal, faulted, 0))) return 0;
    } else if (lv4_is(op, "kind", "object_field_set")) {
      /* A checked user-object field store moves the live child handle
         into the parent's slot. Normal owns the lease in the slot; Fault
         keeps it live so the discharge block's own `home_release` on the
         child still has a lease to consume. */
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
      /* Inside a birth unit the checked store moves the provider child
         into the borrowed receiver's slot: the base is the stamped
         receiver (origin -2 admitted by lv4_indexed_live), the child is
         still an in-flight live lease (origin >= 0). A nullable formal
         (origin -3) carries a possibly-Void payload instead: the slot
         stores the pair's payload — zero means the slot stays Void and
         no lease changes hands. */
      int nullable_formal = value && value->origin == -3;
      if ((fi && !ordinary && !birth) || faulted || lv4_slot(in, op) < 0 || !value ||
          value->kind != LV4_HANDLE || (value->origin < 0 && !nullable_formal) ||
          !lv4_layout(in, lv4_u(op, "child_object_id")) ||
          (!nullable_formal &&
              !lv4_indexed_live(fn, value->id, LV4_HANDLE, state,
                  lv4_u(op, "child_object_id"), 1)) ||
          !lv4_indexed_live(fn, lv4_u(op, "base"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), 1)) return 0;
      /* A metadata-reserved object slot stores only the canonical child:
         the exact `owned_object_residences` tuple plus borrowed typed-live
         provenance — the receiver or the same object's `new_box` root.
         The nullable formal's provenance is its sealed representation,
         not a live lease, so it names no typed-live source. */
      if (lv4_birth_reserved(in, op) &&
          (!lv4_birth_owned(in, op, 1) ||
           (!nullable_formal && !lv4_borrowed_typed_live(in, fn, value, state)))) return 0;
      if (birth && !lv4_birth_commit(in, fn, op, normal, bi, 1)) return 0;
      if (!nullable_formal) normal[value->origin] = 0;
    } else if (lv4_is(op, "kind", "object_field_release")) {
      /* Same child-slot discharge as a field residence release, keyed on
         the canonical child object — the emitted helper owns the child's
         own teardown under the exact-child corroboration here. A birth
         caller's call-fault discharge on borrowed `me` consumes the exact
         source-owned slot marker; other callers keep owned-base teardown. */
      hako_lv4_value* base = lv4_index_value(fn, lv4_u(op, "base"));
      if ((fi && !ordinary && !birth) || lv4_slot(in, op) < 0 || !base ||
          (!birth && base->origin < 0) ||
          !lv4_layout(in, lv4_u(op, "child_object_id")) ||
          !lv4_indexed_live(fn, lv4_u(op, "base"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), 1)) return 0;
      if (base->origin < 0 && (!birth ||
          !lv4_birth_discharge(in, fn, op, state, normal, faulted, 1))) return 0;
    } else if (lv4_is(op, "kind", "object_end_if_owned")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
      if ((fi && !ordinary) || birth || !value ||
          !lv4_mixed_result_readable(in, fn, value, state, lv4_u(block, "id")) ||
          !lv4_mixed_owned_class(in, fn, value, lv4_u(op, "object_id")) ||
          !lv4_complete_end_body(in, op)) return 0;
      normal[value->origin] = state[value->origin] = 0; /* both cleanup exits */
    } else if (lv4_is(op, "kind", "home_release") || lv4_is(op, "kind", "home_release_if_live") ||
        lv4_is(op, "kind", "reclaim_unpublished")) {
      /* `home_release_if_live` releases a value that may carry the Void
         sentinel: the lease slot is consumed either way, but call results
         carry no object_id, so the checked release verifies liveness
         without the exact-object corroboration. */
      int nullable_release = lv4_is(op, "kind", "home_release_if_live");
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(op, "value"));
      /* Only a live caller-owned acquisition may end. Birth formals and
         their Copies remain caller-owned until this Birth returns Normal. */
      if ((fi && !ordinary && !birth) || !value || value->origin < 0 ||
          !lv4_indexed_live(fn, lv4_u(op, "value"), LV4_HANDLE, state,
              lv4_u(op, "object_id"), !nullable_release)) return 0;
      if (!lv4_is(op, "kind", "reclaim_unpublished") &&
          (!lv4_object_field_dependencies_released(in, fn, value, state, lv4_u(block, "id")) ||
           !lv4_complete_fields_discharged(in, fn, value, lv4_u(op, "object_id"), lv4_u(block, "id")))) return 0;
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
        normal[value->origin] = lv4_map_transfer(map->origin);
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
      if ((fi && !ordinary && !birth) || !value || value->origin < 0 ||
          !lv4_indexed_live(fn, id, map_end ? LV4_MAP : LV4_OUTCOME, state, 0, 0)) return 0;
      normal[value->origin] = state[value->origin] = 0;
      if (map_end) for (size_t i = 0; i < count; i++)
        if (state[i] == lv4_map_transfer(value->origin)) normal[i] = state[i] = 0;
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
    int edge_faulted = ei ? failed : faulted;
    const hako_lv4_state* outgoing = ei ? state : normal;
    hako_lv4_state* projected = normal + count;
    if (fn->has_text_carriers || (birth && edge_faulted)) {
      memcpy(projected, outgoing, count * sizeof(*projected));
      if (fn->has_text_carriers && !lv4_text_edge(fn, lv4_u(block, "id"), next->block, outgoing, projected)) return 0;
      outgoing = projected;
    }
    if (birth && edge_faulted) {
      if (!lv4_birth_drop_provided(in, fn, projected)) return 0;
      outgoing = projected;
    }
    if (!lv4_indexed_deliver(work, next->ordinal, outgoing, edge_faulted)) return 0;
  }
  return 1;
}

static int lv4_indexed_admit(hako_lv4_input* in) {
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
    if (!lv4_work_open(&work, count, lv4_state_count(fn), entry->ordinal)) return 0;
    int valid = 1;
    while (valid == 1 && work.pending) {
      size_t bi = work.queue[work.head]; work.head = (work.head + 1) % work.count;
      work.queued[bi] = 0; work.pending--;
      valid = lv4_indexed_flow_block(in, fi, bi, &work);
    }
    if (valid == LV4_FLOW_RECEIVER_MISMATCH) { lv4_work_close(&work); return valid; }
    for (size_t bi = 0; valid && bi < count; bi++) if (work.faults[bi] < 0) valid = 0;
    lv4_work_close(&work);
    if (!valid) return 0;
  }
  return 1;
}
