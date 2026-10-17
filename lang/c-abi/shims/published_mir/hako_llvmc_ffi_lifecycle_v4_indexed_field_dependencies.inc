/* Borrowed JSON groups corroborate the SAME physical dependency. Rust keeps
 * all source witnesses; this owner checks every Return and every substitution,
 * without inventing witness IDs or collapsing dependencies into class caches. */
static int lv4_field_borrow_suffix(hako_lv4_function_index* fn, uint32_t use,
    uint32_t born, const hako_lv4_value* base, uint32_t object, uint32_t field);

static int lv4_field_dependency_same(yyjson_val* a, yyjson_val* b) {
  static const char* names[] = {"object_id", "field_ordinal", "child_object_id", "constructor_formal_ordinal"};
  for (size_t i = 0; i < 4; i++)
    if (lv4_u(a, names[i]) != lv4_u(b, names[i])) return 0;
  return 1;
}

static int lv4_field_dependency_layout(const hako_lv4_input* in, yyjson_val* field) {
  static const char* const keys[] = {"object_id", "field_ordinal", "child_object_id", "constructor_formal_ordinal"};
  if (!hako_physical_exact_keys(field, keys, 4)) return 0;
  for (size_t i = 0; i < 4; i++)
    if (!hako_physical_u32(yyjson_obj_get(field, keys[i]), NULL)) return 0;
  yyjson_val* layout = lv4_layout(in, lv4_u(field, "object_id"));
  if (!layout || !lv4_layout(in, lv4_u(field, "child_object_id"))) return 0;
  yyjson_val* rows = yyjson_obj_get(layout, "owned_object_residences");
  size_t found = 0;
  for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
    yyjson_val* row = yyjson_arr_get(rows, i);
    if (lv4_u(row, "field_ordinal") == lv4_u(field, "field_ordinal") &&
        lv4_u(row, "child_object_id") == lv4_u(field, "child_object_id")) found++;
  }
  return found == 1;
}

/* A constructor's exact canonical Provided store, including original Copies.
 * This observes the existing Birth owner; it grants no new formal use. */
static int lv4_field_dependency_store_tuple(hako_lv4_function_index* fn,
    uint32_t object, uint32_t field, uint32_t child, uint32_t ordinal) {
  yyjson_val* params = yyjson_obj_get(fn->function, "params");
  if (!lv4_is(fn->function, "role", "birth_unit") || !fn->receiver_object_set ||
      fn->receiver_object != object || ordinal >= yyjson_arr_size(params)) return 0;
  yyjson_val* param = yyjson_arr_get(params, ordinal);
  if (!lv4_is(param, "representation", "nullable_kind_payload_v1")) return 0;
  hako_lv4_value* formal = lv4_index_value(fn, lv4_u(param, "value"));
  size_t found = 0;
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* instructions = yyjson_obj_get(fn->blocks[bi].block, "instructions");
    for (size_t ri = 0; ri < yyjson_arr_size(instructions); ri++) {
      yyjson_val* row = yyjson_obj_get(yyjson_arr_get(instructions, ri), "instruction");
      if (lv4_is(row, "op", "field_set") && lv4_u(row, "object_id") == object &&
          lv4_u(row, "field_ordinal") == field) return 0;
    }
    yyjson_val* op = yyjson_obj_get(lv4_term(fn->blocks[bi].block), "operation");
    if (!(lv4_is(op, "kind", "object_field_set") || lv4_is(op, "kind", "field_set")) ||
        lv4_u(op, "object_id") != object ||
        lv4_u(op, "field_ordinal") != field) continue;
    hako_lv4_value* base = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "base")));
    if (++found != 1 || !lv4_is(op, "kind", "object_field_set") || !base ||
        base->origin != -2 || base->id != lv4_u(fn->function, "receiver") ||
        lv4_u(op, "child_object_id") != child ||
        lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "value"))) != formal) return 0;
  }
  return formal && found == 1;
}

static int lv4_field_dependency_store(hako_lv4_function_index* fn, yyjson_val* field) {
  return lv4_field_dependency_store_tuple(fn, lv4_u(field, "object_id"),
      lv4_u(field, "field_ordinal"), lv4_u(field, "child_object_id"),
      lv4_u(field, "constructor_formal_ordinal"));
}

static int lv4_field_dependency_group(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* origin, yyjson_val* group) {
  static const char* const keys[] = {"field", "formal_ordinal", "entry", "copies", "call", "construction"};
  static const char* const call_keys[] = {"invoke", "producer", "formal_ordinal"};
  yyjson_val* field = yyjson_obj_get(group, "field");
  yyjson_val* relation = yyjson_obj_get(group, "call");
  uint32_t outer, inner, entry, invoke_block, producer_block, target;
  if (!hako_physical_exact_keys(group, keys, 6) || !lv4_field_dependency_layout(in, field) ||
      !hako_physical_exact_keys(relation, call_keys, 3) ||
      !hako_physical_u32(yyjson_obj_get(group, "formal_ordinal"), &outer) ||
      !hako_physical_u32(yyjson_obj_get(relation, "formal_ordinal"), &inner) ||
      !hako_physical_u32(yyjson_obj_get(group, "entry"), &entry)) return 0;
  yyjson_val* params = yyjson_obj_get(fn->function, "params");
  yyjson_val* param = yyjson_arr_get(params, outer);
  if (outer >= yyjson_arr_size(params) || !lv4_is(param, "representation", "borrowed_kind_payload_v1") ||
      lv4_u(param, "value") != entry) return 0;
  yyjson_val* invoke = lv4_result_coordinate(fn, yyjson_obj_get(relation, "invoke"), &invoke_block);
  yyjson_val* producer = lv4_result_coordinate(fn, yyjson_obj_get(relation, "producer"), &producer_block);
  if (!invoke || !producer) return 0;
  hako_lv4_block* block = lv4_index_block(fn, invoke_block);
  yyjson_val* op = yyjson_obj_get(invoke, "operation");
  yyjson_val* call = yyjson_obj_get(op, "call");
  yyjson_val* args = yyjson_obj_get(call, "args");
  if (!block || lv4_term(block->block) != invoke || !lv4_is(invoke, "op", "invoke") ||
      !lv4_is(producer, "op", "invoke_normal_result") || lv4_u(producer, "invoke_block") != invoke_block ||
      lv4_u(invoke, "normal") != producer_block ||
      hako_physical_predecessor_count(fn->function, producer_block) != 1 ||
      !lv4_is(op, "kind", "ordinary_call") ||
      !hako_physical_u32(yyjson_obj_get(call, "target"), &target) || !target || target >= in->index_count ||
      !yyjson_is_arr(args) || inner >= yyjson_arr_size(args) ||
      !lv4_result_copy_chain(fn, group, entry, lv4_u(yyjson_arr_get(args, inner), "value"))) return 0;
  yyjson_val* construction = yyjson_obj_get(group, "construction");
  if (!yyjson_is_null(construction)) {
    uint32_t declared, birth_block;
    if (!lv4_is(origin, "kind", "fresh") || !lv4_is(op, "result", "mixed_handle") ||
        !hako_physical_u32(yyjson_obj_get(op, "result_formal_ordinal"), &declared) || declared != inner ||
        !lv4_is(in->index[target].function, "role", "ordinary_mixed_handle") ||
        !hako_physical_u32(yyjson_obj_get(in->index[target].function, "result_formal_ordinal"), &declared) || declared != inner) return 0;
    yyjson_val* birth = lv4_result_coordinate(fn, construction, &birth_block);
    if (!birth) return 0;
    hako_lv4_block* birth_index = lv4_index_block(fn, birth_block);
    yyjson_val* birth_op = yyjson_obj_get(birth, "operation");
    yyjson_val* birth_call = yyjson_obj_get(birth_op, "call");
    yyjson_val* birth_args = yyjson_obj_get(birth_call, "args");
    yyjson_val* fresh = lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), NULL);
    uint32_t constructor, ordinal = lv4_u(field, "constructor_formal_ordinal");
    if (!birth_index || lv4_term(birth_index->block) != birth || !lv4_is(birth, "op", "invoke") ||
        !lv4_is(birth_op, "kind", "birth_call") || !fresh ||
        hako_physical_predecessor_count(fn->function, lv4_u(birth, "normal")) != 1 ||
        lv4_u(birth_call, "receiver") != lv4_u(fresh, "dst") ||
        !hako_physical_u32(yyjson_obj_get(birth_call, "target"), &constructor) ||
        !constructor || constructor >= in->index_count || !yyjson_is_arr(birth_args) ||
        ordinal >= yyjson_arr_size(birth_args) ||
        lv4_u(yyjson_arr_get(birth_args, ordinal), "value") != lv4_u(producer, "dst") ||
        !lv4_field_dependency_store(&in->index[constructor], field)) return 0;
    return 1;
  }
  /* Relay uses the callee's FIELD support ordinal, never its result ordinal. */
  if (!lv4_is(origin, "kind", "owned_call") ||
      invoke != lv4_result_coordinate(fn, yyjson_obj_get(origin, "invoke"), NULL) ||
      producer != lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), NULL) ||
      !(lv4_is(op, "result", "handle") || lv4_is(op, "result", "nullable_handle"))) return 0;
  hako_lv4_function_index* callee = &in->index[target];
  size_t found = 0;
  for (size_t bi = 0; bi < callee->block_count; bi++) {
    yyjson_val* groups = yyjson_obj_get(yyjson_obj_get(lv4_term(callee->blocks[bi].block), "result_origin"), "field_supports");
    for (size_t i = 0; i < yyjson_arr_size(groups); i++) {
      yyjson_val* inherited = yyjson_arr_get(groups, i);
      if (lv4_field_dependency_same(field, yyjson_obj_get(inherited, "field")) &&
          inner == lv4_u(inherited, "formal_ordinal")) found++;
    }
  }
  return found != 0;
}

static int lv4_return_field_dependencies(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* returned) {
  yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
  yyjson_val* groups = yyjson_obj_get(origin, "field_supports");
  if (groups && (!yyjson_is_arr(groups) || !yyjson_arr_size(groups))) return 0;
  for (size_t i = 0; i < yyjson_arr_size(groups); i++) {
    yyjson_val* group = yyjson_arr_get(groups, i);
    if (!lv4_field_dependency_group(in, fn, origin, group)) return 0;
    yyjson_val* source = lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), NULL);
    hako_lv4_value* base = source ? lv4_index_value(fn, lv4_u(source, "dst")) : NULL;
    yyjson_val* construction = yyjson_obj_get(group, "construction");
    yyjson_val* boundary = lv4_result_coordinate(fn, yyjson_is_null(construction) ?
        yyjson_obj_get(origin, "invoke") : construction, NULL);
    uint32_t returned_block = 0;
    int found = 0;
    for (size_t bi = 0; bi < fn->block_count; bi++)
      if (lv4_term(fn->blocks[bi].block) == returned) { returned_block = fn->blocks[bi].id; found = 1; }
    yyjson_val* field = yyjson_obj_get(group, "field");
    if (!found || !base || !boundary ||
        !lv4_field_borrow_suffix(fn, returned_block, lv4_u(boundary, "normal"), base,
            lv4_u(field, "object_id"), lv4_u(field, "field_ordinal"))) return 0;
  }
  /* Relay must preserve every physical dependency from every callee Return. */
  if (lv4_is(origin, "kind", "owned_call")) {
    yyjson_val* invoke = lv4_result_coordinate(fn, yyjson_obj_get(origin, "invoke"), NULL);
    uint32_t target = lv4_u(yyjson_obj_get(yyjson_obj_get(invoke, "operation"), "call"), "target");
    if (!target || target >= in->index_count) return 0;
    hako_lv4_function_index* callee = &in->index[target];
    for (size_t bi = 0; bi < callee->block_count; bi++) {
      yyjson_val* required = yyjson_obj_get(yyjson_obj_get(lv4_term(callee->blocks[bi].block), "result_origin"), "field_supports");
      for (size_t i = 0; i < yyjson_arr_size(required); i++) {
        yyjson_val* dependency = yyjson_arr_get(required, i);
        int found = 0;
        for (size_t j = 0; j < yyjson_arr_size(groups); j++) {
          yyjson_val* group = yyjson_arr_get(groups, j);
          if (yyjson_is_null(yyjson_obj_get(group, "construction")) &&
              lv4_field_dependency_same(yyjson_obj_get(dependency, "field"), yyjson_obj_get(group, "field")) &&
              lv4_u(dependency, "formal_ordinal") == lv4_u(yyjson_obj_get(group, "call"), "formal_ordinal")) found = 1;
        }
        if (!found) return 0;
      }
    }
  }
  return 1;
}

/* Every Mixed actual committed into every canonical Provided field needs its
 * original returned-New group. Removing all/one group cannot erase the loan. */
static int lv4_mixed_birth_dependencies(const hako_lv4_input* in, hako_lv4_function_index* fn) {
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* birth = lv4_term(fn->blocks[bi].block);
    yyjson_val* op = yyjson_obj_get(birth, "operation");
    if (!lv4_is(op, "kind", "birth_call")) continue;
    yyjson_val* call = yyjson_obj_get(op, "call");
    yyjson_val* args = yyjson_obj_get(call, "args");
    uint32_t target = lv4_u(call, "target");
    if (!target || target >= in->index_count) return 0;
    hako_lv4_function_index* constructor = &in->index[target];
    yyjson_val* params = yyjson_obj_get(constructor->function, "params");
    for (size_t ai = 0; ai < yyjson_arr_size(args); ai++) {
      hako_lv4_value* value = lv4_value_alias_root(fn,
          lv4_index_value(fn, lv4_u(yyjson_arr_get(args, ai), "value")));
      if (!value || !lv4_is(value->instruction, "op", "invoke_normal_result")) continue;
      hako_lv4_block* producer = lv4_index_block(fn, lv4_u(value->instruction, "invoke_block"));
      yyjson_val* acquired = producer ? yyjson_obj_get(lv4_term(producer->block), "operation") : NULL;
      if (!lv4_is(acquired, "kind", "ordinary_call") || !lv4_is(acquired, "result", "mixed_handle")) continue;
      if (ai >= yyjson_arr_size(params)) return 0;
      yyjson_val* param = yyjson_arr_get(params, ai);
      if (!lv4_is(param, "representation", "nullable_kind_payload_v1")) return 0;
      hako_lv4_value* formal = lv4_index_value(constructor, lv4_u(param, "value"));
      size_t stores = 0;
      for (size_t si = 0; si < constructor->block_count; si++) {
        yyjson_val* store = yyjson_obj_get(lv4_term(constructor->blocks[si].block), "operation");
        if (!lv4_is(store, "kind", "object_field_set") ||
            lv4_value_alias_root(constructor, lv4_index_value(constructor, lv4_u(store, "value"))) != formal) continue;
        stores++;
        size_t returns = 0;
        for (size_t ri = 0; ri < fn->block_count; ri++) {
          yyjson_val* returned = lv4_term(fn->blocks[ri].block);
          if (!lv4_is(returned, "op", "return")) continue;
          yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
          yyjson_val* fresh = lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), NULL);
          if (!lv4_is(origin, "kind", "fresh") || !fresh ||
              lv4_u(fresh, "dst") != lv4_u(call, "receiver")) continue;
          returns++;
          yyjson_val* groups = yyjson_obj_get(origin, "field_supports");
          int found = 0;
          for (size_t gi = 0; gi < yyjson_arr_size(groups); gi++) {
            yyjson_val* group = yyjson_arr_get(groups, gi);
            yyjson_val* field = yyjson_obj_get(group, "field");
            yyjson_val* relation = yyjson_obj_get(group, "call");
            if (lv4_result_coordinate(fn, yyjson_obj_get(group, "construction"), NULL) == birth &&
                lv4_result_coordinate(fn, yyjson_obj_get(relation, "producer"), NULL) == value->instruction &&
                lv4_u(field, "constructor_formal_ordinal") == ai &&
                lv4_u(field, "object_id") == lv4_u(store, "object_id") &&
                lv4_u(field, "field_ordinal") == lv4_u(store, "field_ordinal") &&
                lv4_u(field, "child_object_id") == lv4_u(store, "child_object_id")) found = 1;
          }
          if (!found) return 0;
        }
        /* The selected source proof permits returned construction only;
         * local Mixed field storage needs its own source lifetime contract. */
        if (!returns) return 0;
      }
      if (!stores) return 0;
    }
  }
  return 1;
}
