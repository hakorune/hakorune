/* Instruction-row state transfer owned by the indexed flow consumer.
 * Extracted without changing admission or lease transitions. */
static int lv4_handle_alias_readable(const hako_lv4_input* in,
    hako_lv4_function_index* fn, hako_lv4_value* value, const hako_lv4_state* state,
    uint32_t use) {
  return lv4_nullable_field_loan_readable(in, fn, value->id, state, use, 0, 0) ||
      lv4_owned_array_holder_live(fn, value->id, state);
}

static int lv4_indexed_rows(const hako_lv4_input* in, hako_lv4_function_index* fn,
    yyjson_val* block, hako_lv4_state* state, int faulted, int birth) {
  yyjson_val* rows = yyjson_obj_get(block, "instructions");
  int after_phi = 0;
  if (in->measurement) in->measurement->v4_flow_rows += yyjson_arr_size(rows);
  for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
    yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, ri), "instruction");
    if (lv4_is(ins, "op", "phi")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(ins, "dst"));
      if (after_phi || !value) return 0;
      if (value->kind == LV4_HANDLE) {
        if (!lv4_value_alias_root(fn, value) ||
            !lv4_handle_alias_readable(in, fn, value, state, lv4_u(block, "id"))) return 0;
      } else if (value->kind != LV4_I64 && value->kind != LV4_BOOL) return 0;
      continue;
    }
    int first_non_phi = !after_phi;
    after_phi = 1;
    if (lv4_is(ins, "op", "fault_frame_enter")) {
      if (lv4_u(block, "id") != lv4_u(fn->function, "entry") || ri || faulted) return 0;
    } else if (lv4_is(ins, "op", "keep_alive")) {
      /* Schema proves every SSA operand available. No payload read or lease
         transition: even a released value may remain an optimizer hint. */
    } else if (lv4_is(ins, "op", "string_residence_install")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(ins, "value"));
      yyjson_val* previous = yyjson_obj_get(ins, "replaces");
      hako_lv4_value* old = previous ? lv4_index_value(fn, lv4_u(ins, "replaces")) : NULL;
      if (faulted || birth || !value || value->kind != LV4_OWNED_STRING ||
          (previous && (!old || old->kind != LV4_OWNED_STRING || old->origin < 0)) ||
          (yyjson_is_true(yyjson_obj_get(ins, "returning"))
              ? (previous || !lv4_is(fn->function, "role", "ordinary_text") ||
                  !lv4_string_hold(state, fn->block_count, value->origin))
              : !lv4_string_install(state, fn->block_count, value->origin, old ? old->origin : -1))) return 0;
    } else if (lv4_is(ins, "op", "array_residence_release")) {
      hako_lv4_value* root = lv4_script_arguments_root(fn, lv4_u(ins, "value"));
      if (!root) {
        root = lv4_owned_array_root(fn, lv4_u(ins, "value"));
        if (!root || !lv4_owned_array_claim(fn, root)) return 0;
      }
      if (state[root->origin] != LV4_LIVE) return 0;
      state[root->origin] = 0;
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
          !lv4_is(fn->function, "role", "ordinary_borrowed_nullable_handle") &&
          !lv4_is(fn->function, "role", "ordinary_mixed_handle") &&
          !lv4_null_lane_present(fn)) return 0;
    } else if (lv4_is(ins, "op", "copy")) {
      hako_lv4_value* source = lv4_index_value(fn, lv4_u(ins, "src"));
      if (!source || (source->kind != LV4_I64 && source->kind != LV4_BOOL &&
          source->kind != LV4_HANDLE && source->kind != LV4_TAGGED && source->kind != LV4_SLOT_BORROW &&
          source->kind != LV4_BORROWED_OBJECT && source->kind != LV4_MIXED_OBJECT)) return 0;
      if (source->kind == LV4_MIXED_OBJECT &&
          (faulted || !lv4_mixed_result_readable(in, fn, source, state, lv4_u(block, "id")))) return 0;
      if (source->kind == LV4_BORROWED_OBJECT &&
          (faulted || !lv4_borrowed_result_readable(in, fn, source, state, lv4_u(block, "id")))) return 0;
      if (source->kind == LV4_SLOT_BORROW && (faulted || !lv4_slot_borrow_readable(in, fn, source, state, lv4_u(block, "id")))) return 0;
      if (source->kind == LV4_HANDLE &&
          !lv4_handle_alias_readable(in, fn, source, state, lv4_u(block, "id"))) return 0;
    } else if (lv4_is(ins, "op", "add") || lv4_is(ins, "op", "mul")) {
      hako_lv4_value *lhs = lv4_index_value(fn, lv4_u(ins, "lhs")),
          *rhs = lv4_index_value(fn, lv4_u(ins, "rhs"));
      /* The borrowed kind/payload lane may serve a dominated arithmetic operand
         only; the emitted kind==1 branch proves the i64 read at the use
         site, uniform with the checked compare. */
      if (!lhs || !rhs ||
          (lhs->kind != LV4_I64 && !(lhs->kind == LV4_TAGGED && !lv4_borrowed_text_formal(fn, lhs))) ||
          (rhs->kind != LV4_I64 && !(rhs->kind == LV4_TAGGED && !lv4_borrowed_text_formal(fn, rhs)))) return 0;
    } else if (lv4_is(ins, "op", "sub") || lv4_is(ins, "op", "div") || lv4_is(ins, "op", "mod")) {
      hako_lv4_value *lhs = lv4_index_value(fn, lv4_u(ins, "lhs")),
          *rhs = lv4_index_value(fn, lv4_u(ins, "rhs"));
      if (!lhs || !rhs || lhs->kind != LV4_I64 || rhs->kind != LV4_I64 ||
          (!lv4_is(ins, "op", "sub") &&
           !hako_physical_positive_divisor(fn->function, lv4_u(ins, "rhs")))) return 0;
    } else if (lv4_is(ins, "op", "compare")) {
      hako_lv4_value *lhs = lv4_index_value(fn, lv4_u(ins, "lhs")),
          *rhs = lv4_index_value(fn, lv4_u(ins, "rhs"));
      /* Bool equality uses its canonical 0/1 carrier, never Integer or
         tagged-payload coercion. Bool ordering remains outside this lane. */
      if (lhs && rhs && lhs->kind == LV4_BOOL && rhs->kind == LV4_BOOL) {
        if (!lv4_is(ins, "predicate", "eq") && !lv4_is(ins, "predicate", "ne")) return 0;
        continue;
      }
      /* The borrowed kind/payload lane may serve a checked compare only;
         the emitted kind==1 branch proves the i64 read at the use site. */
      if (!lhs || !rhs ||
          (lhs->kind != LV4_I64 && !(lhs->kind == LV4_TAGGED && !lv4_borrowed_text_formal(fn, lhs))) ||
          (rhs->kind != LV4_I64 && !(rhs->kind == LV4_TAGGED && !lv4_borrowed_text_formal(fn, rhs)))) return 0;
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
      if (lv4_borrowed_text_formal(fn, carrier)) return 0;
      if (carrier->kind != LV4_TAGGED && carrier->kind != LV4_HANDLE &&
          carrier->kind != LV4_I64 && carrier->kind != LV4_BOOL && carrier->kind != LV4_BORROWED_OBJECT && carrier->kind != LV4_MIXED_OBJECT) return 0;
      if (carrier->kind == LV4_MIXED_OBJECT &&
          (faulted || !lv4_mixed_result_readable(in, fn, carrier, state, lv4_u(block, "id")))) return 0;
      if (carrier->kind == LV4_BORROWED_OBJECT &&
          (faulted || !lv4_borrowed_result_readable(in, fn, carrier, state, lv4_u(block, "id")))) return 0;
    } else if (lv4_is(ins, "op", "array_set")) {
      hako_lv4_value *array = lv4_index_value(fn, lv4_u(ins, "array")),
          *index = lv4_index_value(fn, lv4_u(ins, "index")),
          *value = lv4_index_value(fn, lv4_u(ins, "value"));
      /* The element write is a checked kernel call: the array operand is a
         raw slot-loaded handle lane, the index a committed i64, and the
         element value may read the same lent Integer view — the emitted
         kind==1 branch proves that read at this exact site, uniform with
         the checked compare and the dominated `+`. */
      if (faulted || birth || !array ||
          !lv4_field_array_readable(in, fn, array->id, state, lv4_u(block, "id")) ||
          !index || index->kind != LV4_I64 ||
          !value || (value->kind != LV4_I64 && !(value->kind == LV4_TAGGED && !lv4_borrowed_text_formal(fn, value)))) return 0;
    } else if (lv4_is(ins, "op", "array_length")) {
      hako_lv4_value* array = lv4_index_value(fn, lv4_u(ins, "array"));
      if (faulted || birth || !array ||
          !lv4_field_array_readable(in, fn, array->id, state, lv4_u(block, "id"))) return 0;
    } else if (lv4_is(ins, "op", "array_get")) {
      /* A routed `.get` element read: the array operand is the same
         slot-loaded handle lane and the index a committed i64 — the read
         consumes neither and needs no site (miss → the null sentinel 0);
         the dst is the i64-or-handle carrier. */
      hako_lv4_value *array = lv4_index_value(fn, lv4_u(ins, "array")),
          *index = lv4_index_value(fn, lv4_u(ins, "index"));
      if (faulted || birth || !array ||
          !lv4_field_array_readable(in, fn, array->id, state, lv4_u(block, "id")) ||
          !index || index->kind != LV4_I64) return 0;
    } else if (lv4_is(ins, "op", "object_field_get")) {
      /* A borrowed formal's guarded read carries no owned lease: the
         tagged pair's payload names the object, and the param's sealed
         `object_view` must equal this row's canonical object — the
         publisher's dominance proof, not this arm, places the row in the
         non-null cone. */
      hako_lv4_value* base = lv4_index_value(fn, lv4_u(ins, "base"));
      hako_lv4_value* read = lv4_index_value(fn, lv4_u(ins, "dst"));
      if (faulted || (birth && !lv4_birth_field_borrow_live(in, fn,
              lv4_value_alias_root(fn, read), state, lv4_u(block, "id"))) ||
          lv4_slot(in, ins) < 0 || !base ||
          (!lv4_readable_object(in, fn, base->id, state, lv4_u(block, "id"),
              lv4_u(ins, "object_id"), 1) &&
           !lv4_borrowed_object_formal(fn, base, lv4_u(ins, "object_id")))) return 0;
      if (read && read->origin == LV4_OWNED_FIELD_BORROW &&
          !lv4_nullable_field_loan_readable(in, fn, read->id, state, lv4_u(block, "id"), read->object_id, 1)) return 0;
    } else if (lv4_is(ins, "op", "field_set")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(ins, "value"));
      yyjson_val* check = yyjson_obj_get(ins, "exact_numeric_runtime_check");
      /* A routed scalar store consumes the same checked helper as the
         invoke form: the base is the exact live object and the value is a
         proven i64 lane only — tagged carriers and handle leases never
         reach this row. A dynamic-integer-range obligation is discharged by
         the emitted bounds check at this row, not by the kernel. */
      /* Birth source stores are Invoke terminators with issued Fault
         cleanup. A reserved (metadata-owned) slot never takes this plain
         scalar row in any role. */
      if (birth || lv4_birth_reserved(in, ins) ||
          lv4_is(fn->function, "role", "root_unit") || faulted ||
          lv4_slot(in, ins) < 0 || !value || value->kind != LV4_I64 ||
          !check ||
          !(lv4_is(check, "kind", "none") ||
            (lv4_is(check, "kind", "dynamic_integer_range") &&
             lv4_is(check, "declared_type", "usize"))) ||
          !lv4_indexed_live(fn, lv4_u(ins, "base"), LV4_HANDLE, state,
              lv4_u(ins, "object_id"), 1)) return 0;
    } else if (lv4_is(ins, "op", "invoke_normal_result")) {
      hako_lv4_value* value = lv4_index_value(fn, lv4_u(ins, "dst"));
      int result_requires_live = value && value->kind != LV4_TEXT_VIEW && value->kind != LV4_BORROWED_OBJECT && value->kind != LV4_MIXED_OBJECT;
      /* Birth units carry one result projection: the proven provider
         `new ArrayBox()` normal landing. It is the first non-PHI row of a
         block reached only by the invoke's Normal edge. */
      if (faulted || !first_non_phi || !value || value->origin < 0 ||
          (result_requires_live && (value->kind == LV4_OWNED_STRING
              ? state[value->origin] <= LV4_STRING_RANK_BASE
              : value->kind == LV4_DYNAMIC_CARRIER
                  ? state[value->origin] != LV4_DYNAMIC_CARRIER_LIVE
                  : state[value->origin] != LV4_LIVE))) return 0;
      if (value->kind == LV4_MIXED_OBJECT &&
          !lv4_mixed_result_readable(in, fn, value, state, lv4_u(block, "id"))) return 0;
      if (value->kind == LV4_BORROWED_OBJECT &&
          !lv4_borrowed_result_readable(in, fn, value, state, lv4_u(block, "id"))) return 0;
      if (value->kind == LV4_SLOT_BORROW &&
          !lv4_slot_borrow_readable(in, fn, value, state, lv4_u(block, "id"))) return 0;
      yyjson_val* op = yyjson_obj_get(lv4_term(block), "operation");
      if (value->kind == LV4_KEY && (!(lv4_is(op, "kind", "map_install_indexed") ||
          lv4_is(op, "kind", "map_install_value") || lv4_is(op, "kind", "map_install_text") ||
          lv4_is(op, "kind", "map_install_empty_array") ||
          lv4_is(op, "kind", "map_install_borrowed_array")) ||
          lv4_u(op, "key") != value->id)) return 0;
      if (value->kind == LV4_OUTCOME && (!lv4_is(op, "kind", "map_end_outcome") ||
          lv4_u(op, "outcome") != value->id)) return 0;
      /* Ordinary calls produce a scalar result, not a live storage lease. */
      if (value->kind == LV4_I64 || value->kind == LV4_BOOL || value->kind == LV4_SLOT_BORROW) state[value->origin] = 0;
    } else return 0;
  }
  return 1;
}
