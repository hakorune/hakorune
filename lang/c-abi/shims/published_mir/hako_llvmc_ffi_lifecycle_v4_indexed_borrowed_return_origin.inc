/* Same existing field interval owner, defined after the result index. */
static int lv4_map_slot_interval(hako_lv4_function_index*, uint32_t, uint32_t,
    const hako_lv4_value*, uint32_t, uint32_t);

static int lv4_slot_return_origin(const hako_lv4_input* in,
    hako_lv4_function_index* fn, yyjson_val* returned) {
  static const char* const keys[] = {"kind", "producer", "invoke", "field", "key_ordinal", "object_id", "copies", "present"};
  static const char* const installed_keys[] = {"kind", "producer", "invoke", "field", "key_ordinal", "object_id", "copies", "installed"};
  static const char* const field_keys[] = {"object_id", "field_ordinal"};
  static const char* const present_keys[] = {"invoke", "normal", "copies", "branch"};
  static const char* const install_keys[] = {"normal", "end"};
  hako_physical_slot_support support;
  yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
  yyjson_val* installed = yyjson_obj_get(origin, "installed");
  yyjson_val* field = yyjson_obj_get(origin, "field");
  uint32_t producer_block = 0, invoke_block = 0, result;
  if (!hako_physical_result_slot(fn->function, &support) ||
      !lv4_is(fn->function, "role", "ordinary_borrowed_nullable_handle") ||
      !lv4_is(origin, "kind", "borrowed_map_slot") ||
      !hako_physical_exact_keys(origin, installed ? installed_keys : keys, 8) ||
      !hako_physical_exact_keys(field, field_keys, 2) ||
      !hako_physical_u32(yyjson_obj_get(field, "object_id"), NULL) ||
      !hako_physical_u32(yyjson_obj_get(field, "field_ordinal"), NULL) ||
      !hako_physical_u32(yyjson_obj_get(origin, "key_ordinal"), NULL) ||
      !hako_physical_u32(yyjson_obj_get(origin, "object_id"), NULL) ||
      lv4_u(field, "object_id") != support.receiver_object || lv4_u(field, "field_ordinal") != support.field ||
      lv4_u(origin, "key_ordinal") != support.key_ordinal || lv4_u(origin, "object_id") != support.payload_object ||
      !hako_physical_u32(yyjson_obj_get(returned, "value"), &result)) return 0;
  yyjson_val* producer = lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), &producer_block);
  yyjson_val* invoke = lv4_result_coordinate(fn, yyjson_obj_get(origin, "invoke"), &invoke_block);
  hako_lv4_block* block = lv4_index_block(fn, invoke_block);
  yyjson_val* op = yyjson_obj_get(invoke, "operation");
  hako_lv4_value* receiver = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(op, "base")));
  yyjson_val* formal = yyjson_arr_get(yyjson_obj_get(fn->function, "params"), support.key_ordinal);
  hako_lv4_value* key = lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(yyjson_obj_get(op, "key"), "value")));
  if (!producer || !block || lv4_term(block->block) != invoke || !lv4_is(invoke, "op", "invoke") ||
      !receiver || !lv4_canonical_receiver(fn, receiver) ||
      lv4_u(op, "object_id") != support.receiver_object || lv4_u(op, "field_ordinal") != support.field ||
      lv4_u(op, "payload_object_id") != support.payload_object || !lv4_layout(in, support.payload_object) ||
      !lv4_is(yyjson_obj_get(op, "key"), "kind", "borrowed_text") ||
      !key || key->id != lv4_u(formal, "value") || !lv4_is(formal, "representation", "borrowed_kind_payload_v1") ||
      !yyjson_is_true(yyjson_obj_get(formal, "text_view"))) return 0;
  uint32_t value = lv4_u(producer, "dst"), born = lv4_u(invoke, "normal");
  uint32_t return_block = 0;
  int found_return = 0;
  for (size_t bi = 0; bi < fn->block_count; bi++) if (lv4_term(fn->blocks[bi].block) == returned) {
    return_block = fn->blocks[bi].id; found_return = 1;
  }
  if (!found_return || !hako_physical_block_dominates(fn->function, producer_block, return_block) ||
      !lv4_result_copy_chain(fn, origin, value, result)) return 0;
  if (!installed) {
    yyjson_val* present = yyjson_obj_get(origin, "present");
    uint32_t has_block = 0, normal_block = 0, branch_block = 0;
    yyjson_val* has = lv4_result_coordinate(fn, yyjson_obj_get(present, "invoke"), &has_block);
    yyjson_val* normal = lv4_result_coordinate(fn, yyjson_obj_get(present, "normal"), &normal_block);
    yyjson_val* branch = lv4_result_coordinate(fn, yyjson_obj_get(present, "branch"), &branch_block);
    yyjson_val* has_op = yyjson_obj_get(has, "operation");
    hako_lv4_block* hb = lv4_index_block(fn, has_block);
    hako_lv4_block* bb = lv4_index_block(fn, branch_block);
    if (!hako_physical_exact_keys(present, present_keys, 4) || !hb || !bb ||
        lv4_term(hb->block) != has || lv4_term(bb->block) != branch ||
        !lv4_is(has, "op", "invoke") || !lv4_is(has_op, "kind", "map_field_has") ||
        lv4_u(has_op, "object_id") != support.receiver_object || lv4_u(has_op, "field_ordinal") != support.field ||
        lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(has_op, "base"))) != receiver ||
        !lv4_is(yyjson_obj_get(has_op, "key"), "kind", "borrowed_text") ||
        lv4_value_alias_root(fn, lv4_index_value(fn, lv4_u(yyjson_obj_get(has_op, "key"), "value"))) != key ||
        !lv4_is(normal, "op", "invoke_normal_result") || lv4_u(normal, "invoke_block") != has_block ||
        lv4_u(has, "normal") != normal_block || lv4_pred_count(fn, normal_block) != 1 ||
        !lv4_is(branch, "op", "branch") ||
        !lv4_result_copy_chain(fn, present, lv4_u(normal, "dst"), lv4_u(branch, "condition")) ||
        !hako_physical_block_dominates(fn->function, normal_block, branch_block) ||
        !hako_physical_block_dominates(fn->function, branch_block, invoke_block) ||
        !hako_physical_block_dominates(fn->function, lv4_u(branch, "then"), invoke_block) ||
        !lv4_map_slot_interval(fn, invoke_block, normal_block, receiver, support.receiver_object, support.field) ||
        !lv4_is(op, "kind", "map_field_get_indexed") ||
        !lv4_is(producer, "op", "invoke_normal_result") || lv4_u(producer, "invoke_block") != invoke_block ||
        producer_block != born || lv4_pred_count(fn, born) != 1) return 0;
  } else {
    uint32_t normal_block = 0, end_block = 0;
    yyjson_val* normal = lv4_result_coordinate(fn, yyjson_obj_get(installed, "normal"), &normal_block);
    yyjson_val* end = lv4_result_coordinate(fn, yyjson_obj_get(installed, "end"), &end_block);
    hako_lv4_block* eb = lv4_index_block(fn, end_block);
    hako_lv4_value* candidate = lv4_index_value(fn, lv4_u(op, "value"));
    if (!hako_physical_exact_keys(installed, install_keys, 2) || !eb || lv4_term(eb->block) != end ||
        !lv4_is(op, "kind", "map_field_install_indexed") || !lv4_is(producer, "op", "copy") ||
        lv4_u(producer, "src") != lv4_u(op, "value") || value == lv4_u(op, "value") ||
        !lv4_owned_value_object(in, fn, candidate, 0) || candidate->object_id != support.payload_object ||
        !lv4_is(normal, "op", "invoke_normal_result") || lv4_u(normal, "invoke_block") != invoke_block ||
        normal_block != born || lv4_pred_count(fn, born) != 1 ||
        !lv4_is(end, "op", "invoke") || !lv4_is(yyjson_obj_get(end, "operation"), "kind", "map_end_outcome") ||
        lv4_u(yyjson_obj_get(end, "operation"), "outcome") != lv4_u(normal, "dst") ||
        lv4_u(end, "fault_frame") != lv4_u(invoke, "fault_frame") ||
        !hako_physical_block_dominates(fn->function, born, end_block) ||
        lv4_pred_count(fn, lv4_u(end, "normal")) != 1 ||
        !hako_physical_block_dominates(fn->function, lv4_u(end, "normal"), producer_block)) return 0;
    hako_lv4_value* acquired = lv4_value_alias_root(fn, candidate);
    if (!lv4_original_birth_completed(in, fn, acquired, support.payload_object, invoke_block)) return 0;
  }
  return lv4_map_slot_interval(fn, producer_block, born, receiver, support.receiver_object, support.field) &&
      lv4_map_slot_interval(fn, return_block, born, receiver, support.receiver_object, support.field);
}

/* The existing nullable-forward law is Null or the SAME fixed logical formal.
 * Physical corroboration adds no Home or incoming-call permission. The
 * dedicated Call/Normal/anchor consumers remain independently mandatory. */
static int lv4_borrowed_return_origin(const hako_lv4_input* in, hako_lv4_function_index* fn, yyjson_val* returned) {
  if (yyjson_obj_get(fn->function, "result_support")) return lv4_slot_return_origin(in, fn, returned);
  static const char* const null_keys[] = {"kind", "producer", "copies"};
  static const char* const formal_keys[] = {"kind", "formal_ordinal", "copies"};
  yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
  uint32_t ordinal, returned_value, value;
  yyjson_val* params = yyjson_obj_get(fn->function, "params");
  if (!lv4_is(fn->function, "role", "ordinary_borrowed_nullable_handle") ||
      !hako_physical_u32(yyjson_obj_get(fn->function, "result_formal_ordinal"), &ordinal) ||
      ordinal >= yyjson_arr_size(params) ||
      !hako_physical_u32(yyjson_obj_get(returned, "value"), &returned_value)) return 0;
  yyjson_val* formal = yyjson_arr_get(params, ordinal);
  if (!lv4_is(formal, "representation", "borrowed_kind_payload_v1") ||
      !hako_physical_u32(yyjson_obj_get(formal, "value"), &value)) return 0;
  if (lv4_is(origin, "kind", "null")) {
    yyjson_val* producer = lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), NULL);
    if (!hako_physical_exact_keys(origin, null_keys, 3) ||
        !lv4_is(producer, "op", "const_null") ||
        !hako_physical_u32(yyjson_obj_get(producer, "dst"), &value)) return 0;
  } else {
    uint32_t original_ordinal;
    if (!hako_physical_exact_keys(origin, formal_keys, 3) ||
        !lv4_is(origin, "kind", "borrowed_formal") ||
        !hako_physical_u32(yyjson_obj_get(origin, "formal_ordinal"), &original_ordinal) ||
        original_ordinal != ordinal) return 0;
  }
  return lv4_result_copy_chain(fn, origin, value, returned_value);
}

/* Only original post-install Return aliases, never the acquired candidate.
 * These retain HANDLE's spent origin; no consumer gains a new live token. */
static yyjson_val* lv4_installed_return_alias(hako_lv4_function_index* fn, uint32_t value,
    yyjson_val* exact_copy) {
  for (size_t bi = 0; bi < fn->block_count; bi++) {
    yyjson_val* returned = lv4_term(fn->blocks[bi].block);
    yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
    if (!lv4_is(returned, "op", "return") || !lv4_is(origin, "kind", "borrowed_map_slot") ||
        !yyjson_obj_get(origin, "installed")) continue;
    yyjson_val* producer = lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), NULL);
    if (producer && lv4_u(producer, "dst") == value && (!exact_copy || producer == exact_copy)) return returned;
    yyjson_val* copies = yyjson_obj_get(origin, "copies");
    for (size_t ci = 0; ci < yyjson_arr_size(copies); ci++) {
      yyjson_val* copy = lv4_result_coordinate(fn, yyjson_arr_get(copies, ci), NULL);
      if (copy && lv4_u(copy, "dst") == value && (!exact_copy || copy == exact_copy)) return returned;
    }
  }
  return NULL;
}
