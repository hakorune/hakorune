/* Operation and instruction schema within the existing physical-v2 owner. */
/* One physical String classification/operand table, shared by all consumers. */
static int hako_physical_string_result(const char* kind) {
  if (!kind) return 0;
  if (!strcmp(kind, "string_add") || !strcmp(kind, "string_substring") || !strcmp(kind, "string_materialize")) return 1;
  if (!strcmp(kind, "string_materialize") || !strcmp(kind, "string_length") || !strcmp(kind, "string_index_of")) return 2;
  if (!strcmp(kind, "string_compare")) return 3; /* Bool, never I64 */
  return 0;
}
static int hako_physical_string_kind(const char* kind) {
  return hako_physical_string_result(kind) || (kind && (!strcmp(kind, "string_print") || !strcmp(kind, "string_end")));
}
static size_t hako_physical_string_operand_count(yyjson_val* op) {
  const char* kind = yyjson_get_str(yyjson_obj_get(op, "kind"));
  if (!hako_physical_string_kind(kind) || !strcmp(kind, "string_end")) return 0;
  return !strcmp(kind, "string_add") || !strcmp(kind, "string_compare") || !strcmp(kind, "string_index_of") ? 2 : 1;
}
static yyjson_val* hako_physical_string_operand_at(yyjson_val* op, size_t ordinal) {
  const char* kind = yyjson_get_str(yyjson_obj_get(op, "kind"));
  const char* field = kind && (!strcmp(kind, "string_add") || !strcmp(kind, "string_compare")) ? (ordinal ? "right" : "left") : (ordinal ? "needle" : "value");
  return yyjson_obj_get(op, field);
}
/* One comparison spelling/opcode table shared by validation and emission. */
static int hako_physical_string_comparison(yyjson_val* operation) {
  yyjson_val* value = yyjson_obj_get(operation, "comparison");
  const char* comparison = yyjson_is_str(value) && yyjson_get_len(value) == 2 ? yyjson_get_str(value) : NULL;
  static const char* const names[] = {"eq", "ne", "lt", "le", "gt", "ge"};
  if (comparison) for (int i = 0; i < 6; i++) if (!strcmp(comparison, names[i])) return i;
  return -1;
}
static int hako_physical_string_operand(yyjson_val* function, yyjson_val* operand,
    uint32_t block, uint32_t index, int text_only) {
  static const char* const text[] = {"kind", "utf8"};
  static const char* const value_keys[] = {"kind", "value"};
  uint32_t value;
  if (hako_physical_string(yyjson_obj_get(operand, "kind"), "static_text"))
    return hako_physical_exact_keys(operand, text, 2) && yyjson_is_str(yyjson_obj_get(operand, "utf8"));
  return (hako_physical_string(yyjson_obj_get(operand, "kind"), "owned_string") ||
      hako_physical_string(yyjson_obj_get(operand, "kind"), "borrowed_text") ||
      (!text_only && hako_physical_string(yyjson_obj_get(operand, "kind"), "normal_integer"))) &&
      hako_physical_exact_keys(operand, value_keys, 2) &&
      hako_physical_u32(yyjson_obj_get(operand, "value"), &value) &&
      hako_physical_value_available_at(function, value, block, index);
}

static int hako_physical_result_operation(yyjson_val* op, int native) {
  const char* kind = yyjson_get_str(yyjson_obj_get(op, "kind"));
  if (!kind) return 0;
  if (hako_physical_string_result(kind)) return 1;
  if (native) return !strcmp(kind, "array_new");
  if (!strcmp(kind, "field_get")) return 1;
  /* `array_new` outside the native lane is the proven provider `new
     ArrayBox()` inside a birth unit — its normal result is the store's
     installed handle. */
  return !strcmp(kind, "script_arguments_new") || !strcmp(kind, "new_box") || !strcmp(kind, "array_new") || !strcmp(kind, "map_new") ||
      !strcmp(kind, "map_prepare_key") || (!strcmp(kind, "map_install_indexed") || !strcmp(kind, "map_install_value")) ||
      !strcmp(kind, "map_install_text") || !strcmp(kind, "map_install_empty_array") ||
      !strcmp(kind, "map_install_borrowed_array") ||
      !strcmp(kind, "map_checked_get") || !strcmp(kind, "map_array_length") ||
      !strcmp(kind, "map_array_index_map") ||
      !strcmp(kind, "map_get_text") || !strcmp(kind, "array_owned_get") ||
      (!strcmp(kind, "ordinary_call") && !hako_physical_string(yyjson_obj_get(op, "result"), "unit"));
}

/* The check is explicit physical policy. Canonical declaration/contract
 * agreement remains the Rust issuer's responsibility, not a C name lookup. */
static int hako_physical_field_range_check(yyjson_val* check) {
  static const char* const none[] = {"kind"};
  static const char* const range[] = {"kind", "declared_type"};
  if (hako_physical_string(yyjson_obj_get(check, "kind"), "none"))
    return hako_physical_exact_keys(check, none, 1);
  return hako_physical_string(yyjson_obj_get(check, "kind"), "dynamic_integer_range") &&
      hako_physical_exact_keys(check, range, 2) &&
      hako_physical_string(yyjson_obj_get(check, "declared_type"), "usize");
}

static int hako_physical_validate_operation(yyjson_val* function, yyjson_val* functions,
    yyjson_val* layouts, yyjson_val* operation, uint32_t block, uint32_t index, int native) {
  static const char* const call_keys[] = {"kind", "call"};
  static const char* const ordinary_keys[] = {"kind", "call", "result"};
  static const char* const new_keys[] = {"kind", "object_id", "site"};
  static const char* const field_keys[] = {"kind", "object_id", "field_ordinal", "base", "value", "site", "exact_numeric_runtime_check"};
  static const char* const field_access_keys[] = {"kind", "object_id", "field_ordinal", "base", "site"};
  static const char* const object_field_release_keys[] = {"kind", "object_id", "field_ordinal", "base", "child_object_id", "site"};
  static const char* const object_field_set_keys[] = {"kind", "object_id", "field_ordinal", "base", "value", "child_object_id", "site"};
  static const char* const object_value_keys[] = {"kind", "object_id", "value", "site"};
  const char* kind = operation ? yyjson_get_str(yyjson_obj_get(operation, "kind")) : NULL;
  uint32_t object_id, ordinal, value;
  if (!kind) return 0;
  if (!strcmp(kind, "string_compare")) {
    static const char* const keys[] = {"kind", "comparison", "left", "right", "site"};
    return hako_physical_exact_keys(operation, keys, 5) && hako_physical_string_comparison(operation) >= 0 &&
        hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) &&
        hako_physical_string_operand(function, yyjson_obj_get(operation, "left"), block, index, 1) &&
        hako_physical_string_operand(function, yyjson_obj_get(operation, "right"), block, index, 1);
  }
  if (!strcmp(kind, "string_add")) {
    static const char* const keys[] = {"kind", "left", "right", "site"};
    return hako_physical_exact_keys(operation, keys, 4) &&
        hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) &&
        hako_physical_string_operand(function, yyjson_obj_get(operation, "left"), block, index, 1) &&
        hako_physical_string_operand(function, yyjson_obj_get(operation, "right"), block, index, 0);
  }
  if (!strcmp(kind, "string_materialize") || !strcmp(kind, "string_length") || !strcmp(kind, "string_substring") || !strcmp(kind, "string_index_of")) {
    static const char* const read[] = {"kind", "value", "site"};
    static const char* const slice[] = {"kind", "value", "start", "end", "site"};
    static const char* const search[] = {"kind", "value", "needle", "site"};
    int substring = !strcmp(kind, "string_substring"), index_of = !strcmp(kind, "string_index_of");
    if (!hako_physical_exact_keys(operation, substring ? slice : index_of ? search : read,
        substring ? 5 : index_of ? 4 : 3) || !hako_physical_u64(yyjson_obj_get(operation, "site"), NULL)) return 0;
    for (size_t role = 0; role < hako_physical_string_operand_count(operation); role++)
      if (!hako_physical_string_operand(function, hako_physical_string_operand_at(operation, role), block, index, 1)) return 0;
    if (substring) {
      const char* bounds[] = {"start", "end"};
      for (size_t i = 0; i < 2; i++)
        if (!hako_physical_u32(yyjson_obj_get(operation, bounds[i]), &value) ||
            !hako_physical_value_available_at(function, value, block, index)) return 0;
    }
    return 1;
  }
  if (!strcmp(kind, "string_print") || !strcmp(kind, "string_end")) {
    static const char* const keys[] = {"kind", "value", "site"};
    if (!hako_physical_exact_keys(operation, keys, 3) ||
        !hako_physical_u64(yyjson_obj_get(operation, "site"), NULL)) return 0;
    return !strcmp(kind, "string_print")
        ? hako_physical_string_operand(function, yyjson_obj_get(operation, "value"), block, index, 0)
        : hako_physical_u32(yyjson_obj_get(operation, "value"), &value) &&
            hako_physical_value_available_at(function, value, block, index);
  }
  if (!strcmp(kind, "array_owned_claim") || !strcmp(kind, "array_owned_append") ||
      !strcmp(kind, "array_owned_get") || !strcmp(kind, "array_owned_end")) {
    static const char* const holder[] = {"kind", "array", "object_id", "nullable", "site"};
    static const char* const append[] = {"kind", "array", "object_id", "nullable", "site", "value"};
    static const char* const get[] = {"kind", "array", "object_id", "nullable", "site", "index"};
    int push = !strcmp(kind, "array_owned_append"), read = !strcmp(kind, "array_owned_get");
    if (native || !hako_physical_exact_keys(operation, push ? append : read ? get : holder, push || read ? 6 : 5) ||
        !hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) ||
        !hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) ||
        !hako_physical_layout_has_object(layouts, object_id) ||
        !yyjson_is_bool(yyjson_obj_get(operation, "nullable")) ||
        !hako_physical_u32(yyjson_obj_get(operation, "array"), &value) ||
        !hako_physical_value_available_at(function, value, block, index)) return 0;
    return !(push || read) ||
        (hako_physical_u32(yyjson_obj_get(operation, push ? "value" : "index"), &value) &&
         hako_physical_value_available_at(function, value, block, index));
  }
  if (hako_physical_map_kind(kind))
    return !native && hako_physical_map_operation(function, layouts, operation, block, index);
  if (!strcmp(kind, "array_append")) {
    static const char* const keys[] = {"kind", "array", "value", "representation", "site"};
    yyjson_val* repr = yyjson_obj_get(operation, "representation");
    return hako_physical_exact_keys(operation, keys, 5) &&
        hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) &&
        hako_physical_u32(yyjson_obj_get(operation, "array"), &value) &&
        hako_physical_value_available_at(function, value, block, index) &&
        hako_physical_u32(yyjson_obj_get(operation, "value"), &value) &&
        hako_physical_value_available_at(function, value, block, index) &&
        (hako_physical_string(repr, "i64") || (native &&
         (hako_physical_string(repr, "bool") || hako_physical_string(repr, "f64"))));
  }
  if (native) {
    static const char* const array_new[] = {"kind", "site"};
    static const char* const claim[] = {"kind", "array", "element_tag", "site"};
    if (!hako_physical_u64(yyjson_obj_get(operation, "site"), NULL)) return 0;
    if (hako_physical_string(yyjson_obj_get(operation, "kind"), "array_new"))
      return hako_physical_exact_keys(operation, array_new, 2);
    if (!hako_physical_u32(yyjson_obj_get(operation, "array"), &value) ||
        !hako_physical_value_available_at(function, value, block, index)) return 0;
    if (hako_physical_string(yyjson_obj_get(operation, "kind"), "array_claim"))
      return hako_physical_exact_keys(operation, claim, 4) &&
          hako_physical_u32(yyjson_obj_get(operation, "element_tag"), &ordinal) &&
          ordinal >= 1 && ordinal <= 7;
    return 0;
  }
  if (!strcmp(kind, "script_arguments_new")) {
    static const char* const keys[] = {"kind", "values", "site"};
    yyjson_val* values = yyjson_obj_get(operation, "values");
    if (native || !hako_physical_exact_keys(operation, keys, 3) || !yyjson_is_arr(values) ||
        !hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) ||
        !(hako_physical_string(yyjson_obj_get(function, "role"), "root_i64") ||
          hako_physical_string(yyjson_obj_get(function, "role"), "root_unit")) ||
        !hako_physical_u32(yyjson_obj_get(function, "entry"), &value) || value != block) return 0;
    for (size_t i = 0; i < yyjson_arr_size(values); i++)
      if (!yyjson_is_str(yyjson_arr_get(values, i))) return 0;
    return 1;
  }
  if (strcmp(kind, "array_new") == 0) {
    static const char* const array_new_keys[] = {"kind", "site"};
    /* A provider `new ArrayBox()` inside a birth unit rides the same
       checked allocation op; the produced lease is consumed by the
       following field store or released by a residence step. */
    return hako_physical_exact_keys(operation, array_new_keys, 2) &&
        hako_physical_u64(yyjson_obj_get(operation, "site"), NULL);
  }
  if (strcmp(kind, "birth_call") == 0)
    return hako_physical_exact_keys(operation, call_keys, 2) && hako_physical_validate_call(function, functions, yyjson_obj_get(operation, "call"), block, index);
  if (strcmp(kind, "ordinary_call") == 0) {
    yyjson_val* result = yyjson_obj_get(operation, "result");
    static const char* const borrowed_keys[] = {"kind", "call", "result", "result_formal_ordinal"};
    int borrowed = hako_physical_string(result, "borrowed_nullable_handle") ||
        hako_physical_string(result, "mixed_handle");
    uint32_t support = 0, declared = 0;
    const char* want_role = hako_physical_ordinary_result_role(result);
    /* This structural pair is corroborated again by indexed flow. Neither
       classifier may silently fill an unknown result with the i64 role. */
    yyjson_val* call = yyjson_obj_get(operation, "call");
    return hako_physical_exact_keys(operation, borrowed ? borrowed_keys : ordinary_keys, borrowed ? 4 : 3) && want_role &&
        hako_physical_validate_ordinary_call(function, functions, call, block, index, want_role) &&
        (!borrowed || (hako_physical_u32(yyjson_obj_get(operation, "result_formal_ordinal"), &support) &&
            hako_physical_u32(yyjson_obj_get(yyjson_arr_get(functions,
                yyjson_get_uint(yyjson_obj_get(call, "target"))), "result_formal_ordinal"), &declared) &&
            support == declared));
  }
  if (strcmp(kind, "new_box") == 0)
    return hako_physical_exact_keys(operation, new_keys, 3) && hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) && hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) && hako_physical_layout_has_object(layouts, object_id);
  if (strcmp(kind, "field_get") == 0)
    return !native && hako_physical_exact_keys(operation, field_access_keys, 5) &&
        hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) &&
        hako_physical_u32(yyjson_obj_get(operation, "field_ordinal"), &ordinal) &&
        hako_physical_u32(yyjson_obj_get(operation, "base"), &value) &&
        hako_physical_value_available_at(function, value, block, index) &&
        hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) &&
        hako_physical_layout_has_field(layouts, object_id, ordinal);
  if (strcmp(kind, "field_set") == 0)
    return hako_physical_exact_keys(operation, field_keys, 7) && hako_physical_field_range_check(yyjson_obj_get(operation, "exact_numeric_runtime_check")) && hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) && hako_physical_u32(yyjson_obj_get(operation, "field_ordinal"), &ordinal) && hako_physical_u32(yyjson_obj_get(operation, "base"), &value) && hako_physical_value_available_at(function, value, block, index) && hako_physical_u32(yyjson_obj_get(operation, "value"), &value) && hako_physical_value_available_at(function, value, block, index) && hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) && hako_physical_layout_has_field(layouts, object_id, ordinal);
  if (strcmp(kind, "field_residence_release") == 0)
    return hako_physical_exact_keys(operation, field_access_keys, 5) && hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) && hako_physical_u32(yyjson_obj_get(operation, "field_ordinal"), &ordinal) && hako_physical_u32(yyjson_obj_get(operation, "base"), &value) && hako_physical_value_available_at(function, value, block, index) && hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) && hako_physical_layout_has_field(layouts, object_id, ordinal);
  if (strcmp(kind, "object_field_release") == 0) {
    uint32_t child_id;
    return hako_physical_exact_keys(operation, object_field_release_keys, 6) &&
        hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) &&
        hako_physical_u32(yyjson_obj_get(operation, "field_ordinal"), &ordinal) &&
        hako_physical_u32(yyjson_obj_get(operation, "base"), &value) &&
        hako_physical_value_available_at(function, value, block, index) &&
        hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) &&
        hako_physical_layout_has_field(layouts, object_id, ordinal) &&
        hako_physical_u32(yyjson_obj_get(operation, "child_object_id"), &child_id) &&
        hako_physical_layout_has_object(layouts, child_id);
  }
  if (strcmp(kind, "object_field_set") == 0) {
    uint32_t child_id;
    return hako_physical_exact_keys(operation, object_field_set_keys, 7) &&
        hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) &&
        hako_physical_u32(yyjson_obj_get(operation, "field_ordinal"), &ordinal) &&
        hako_physical_u32(yyjson_obj_get(operation, "base"), &value) &&
        hako_physical_value_available_at(function, value, block, index) &&
        hako_physical_u32(yyjson_obj_get(operation, "value"), &value) &&
        hako_physical_value_available_at(function, value, block, index) &&
        hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) &&
        hako_physical_layout_has_field(layouts, object_id, ordinal) &&
        hako_physical_u32(yyjson_obj_get(operation, "child_object_id"), &child_id) &&
        hako_physical_layout_has_object(layouts, child_id);
  }
  if (!strcmp(kind, "object_end_if_owned")) {
    static const char* const end_keys[] = {"kind", "object_id", "value", "end", "site"};
    return !native && hako_physical_exact_keys(operation, end_keys, 5) &&
        hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) &&
        hako_physical_layout_has_object(layouts, object_id) &&
        hako_physical_u32(yyjson_obj_get(operation, "value"), &value) &&
        hako_physical_value_available_at(function, value, block, index) &&
        hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) &&
        yyjson_is_arr(yyjson_obj_get(operation, "end"));
  }
  if (strcmp(kind, "home_release") == 0 || strcmp(kind, "home_release_if_live") == 0 ||
      strcmp(kind, "reclaim_unpublished") == 0)
    return hako_physical_exact_keys(operation, object_value_keys, 4) && hako_physical_u32(yyjson_obj_get(operation, "object_id"), &object_id) && hako_physical_u32(yyjson_obj_get(operation, "value"), &value) && hako_physical_u64(yyjson_obj_get(operation, "site"), NULL) && hako_physical_layout_has_object(layouts, object_id) && hako_physical_value_available_at(function, value, block, index);
  return 0;
}

/* Div is the selected positive literal profile, not a constant evaluator. */
static int hako_physical_positive_divisor(yyjson_val* function, uint32_t value) {
  if (hako_physical_value_def_count(function, value) != 1) return 0;
  yyjson_val* blocks = hako_physical_blocks(function);
  for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
    yyjson_val* rows = yyjson_obj_get(yyjson_arr_get(blocks, bi), "instructions");
    for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
      yyjson_val* ins = yyjson_obj_get(yyjson_arr_get(rows, i), "instruction");
      uint32_t dst;
      if (hako_physical_u32(hako_physical_instruction_dst(ins), &dst) && dst == value) {
        yyjson_val* literal = yyjson_obj_get(ins, "value");
        return hako_physical_string(yyjson_obj_get(ins, "op"), "const_i64") &&
            hako_physical_i64(literal) && yyjson_get_sint(literal) > 0;
      }
    }
  }
  return 0;
}

static int hako_physical_validate_instruction(yyjson_val* function, yyjson_val* functions,
    yyjson_val* layouts, yyjson_val* instruction, int terminal, const char* frame_mode,
    uint32_t current_block, uint32_t current_index, int native) {
  static const char* const const_i64[] = {"op", "dst", "value"};
  static const char* const const_unit[] = {"op", "dst"};
  static const char* const binary[] = {"op", "dst", "lhs", "rhs"};
  static const char* const compare[] = {"op", "dst", "lhs", "rhs", "predicate"};
  static const char* const copy[] = {"op", "dst", "src"};
  static const char* const phi[] = {"op", "dst", "inputs"};
  static const char* const field_get[] = {"op", "dst", "base", "object_id", "field_ordinal"};
  static const char* const invoke[] = {"op", "operation", "fault_frame", "normal", "fault"};
  static const char* const normal_result[] = {"op", "invoke_block", "dst"};
  static const char* const return_fault[] = {"op", "fault_frame"};
  static const char* const frame_enter[] = {"op", "dst", "mode"};
  static const char* const branch[] = {"op", "condition", "then", "else", "then_args", "else_args"};
  static const char* const jump[] = {"op", "target", "args"};
  static const char* const ret[] = {"op", "value"};
  static const char* const birth[] = {"op", "call"};
  const char* op = instruction ? yyjson_get_str(yyjson_obj_get(instruction, "op")) : NULL;
  uint32_t a, b, c;
  if (!op) return 0;
  if (native && strcmp(op, "const_i64") && strcmp(op, "const_bool") &&
      strcmp(op, "const_f64_bits") && strcmp(op, "const_unit") &&
      strcmp(op, "copy") && strcmp(op, "fault_frame_enter") &&
      strcmp(op, "invoke") && strcmp(op, "invoke_normal_result") &&
      strcmp(op, "array_residence_release") && strcmp(op, "string_residence_install") && strcmp(op, "branch") && strcmp(op, "jump") &&
      strcmp(op, "return") && strcmp(op, "return_fault")) return 0;
  if (terminal && strcmp(op, "invoke") && strcmp(op, "branch") && strcmp(op, "jump") && strcmp(op, "return") && strcmp(op, "return_fault")) return 0;
  if (!terminal && (!strcmp(op, "invoke") || !strcmp(op, "branch") || !strcmp(op, "jump") || !strcmp(op, "return") || !strcmp(op, "return_fault"))) return 0;
  if (native && !strcmp(op, "const_f64_bits")) {
    static const char* const keys[] = {"op", "dst", "bits"};
    return hako_physical_exact_keys(instruction, keys, 3) &&
        hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) &&
        hako_physical_u64(yyjson_obj_get(instruction, "bits"), NULL);
  }
  if (!strcmp(op, "string_residence_install") && yyjson_obj_get(instruction, "replaces")) {
    static const char* const keys[] = {"op", "value", "replaces"};
    return hako_physical_exact_keys(instruction, keys, 3) &&
        hako_physical_u32(yyjson_obj_get(instruction, "value"), &a) &&
        hako_physical_u32(yyjson_obj_get(instruction, "replaces"), &b) && a != b &&
        hako_physical_value_available_at(function, a, current_block, current_index) &&
        hako_physical_value_available_at(function, b, current_block, current_index);
  }
  if (!strcmp(op, "array_residence_release") || !strcmp(op, "string_residence_install"))
    return hako_physical_exact_keys(instruction, ret, 2) &&
        hako_physical_u32(yyjson_obj_get(instruction, "value"), &a) &&
        hako_physical_value_available_at(function, a, current_block, current_index);
  if (!strcmp(op, "const_i64")) return hako_physical_exact_keys(instruction, const_i64, 3) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && hako_physical_i64(yyjson_obj_get(instruction, "value"));
  if (!strcmp(op, "const_bool")) return hako_physical_exact_keys(instruction, const_i64, 3) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && yyjson_is_bool(yyjson_obj_get(instruction, "value"));
  if (!strcmp(op, "const_string")) return hako_physical_exact_keys(instruction, const_i64, 3) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && yyjson_is_str(yyjson_obj_get(instruction, "value"));
  if (!strcmp(op, "const_unit")) return hako_physical_exact_keys(instruction, const_unit, 2) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a);
  /* The nullable-result null sentinel: an i64 slot carrying Void(0), emitted
     only inside `ordinary_nullable_handle` callees by the flow admission. */
  if (!strcmp(op, "const_null")) return hako_physical_exact_keys(instruction, const_unit, 2) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a);
  if (!strcmp(op, "sub") || !strcmp(op, "div")) {
    return hako_physical_exact_keys(instruction, binary, 4) &&
        hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) &&
        hako_physical_u32(yyjson_obj_get(instruction, "lhs"), &b) &&
        hako_physical_u32(yyjson_obj_get(instruction, "rhs"), &c) &&
        hako_physical_value_available_at(function, b, current_block, current_index) &&
        hako_physical_value_available_at(function, c, current_block, current_index) &&
        (strcmp(op, "div") || hako_physical_positive_divisor(function, c));
  }
  if (!strcmp(op, "add") || !strcmp(op, "mul")) return hako_physical_exact_keys(instruction, binary, 4) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && hako_physical_u32(yyjson_obj_get(instruction, "lhs"), &b) && hako_physical_u32(yyjson_obj_get(instruction, "rhs"), &c) && hako_physical_value_available_at(function, b, current_block, current_index) && hako_physical_value_available_at(function, c, current_block, current_index);
  if (!strcmp(op, "compare")) {
    const char* p = yyjson_get_str(yyjson_obj_get(instruction, "predicate"));
    return p && (!strcmp(p, "eq") || !strcmp(p, "ne") || !strcmp(p, "slt") ||
        !strcmp(p, "sle") || !strcmp(p, "sgt") || !strcmp(p, "sge")) &&
        hako_physical_exact_keys(instruction, compare, 5) &&
        hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) &&
        hako_physical_u32(yyjson_obj_get(instruction, "lhs"), &b) &&
        hako_physical_u32(yyjson_obj_get(instruction, "rhs"), &c) &&
        hako_physical_value_available_at(function, b, current_block, current_index) &&
        hako_physical_value_available_at(function, c, current_block, current_index);
  }
  /* The borrowed/null equality: source order is preserved in lhs/rhs and
     the flow admission verifies exactly one side is the const_null
     producer — the row itself only pins the eq/ne predicate and shape. */
  if (!strcmp(op, "borrowed_null_compare")) {
    const char* p = yyjson_get_str(yyjson_obj_get(instruction, "predicate"));
    return p && (!strcmp(p, "eq") || !strcmp(p, "ne")) &&
        hako_physical_exact_keys(instruction, compare, 5) &&
        hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) &&
        hako_physical_u32(yyjson_obj_get(instruction, "lhs"), &b) &&
        hako_physical_u32(yyjson_obj_get(instruction, "rhs"), &c) &&
        hako_physical_value_available_at(function, b, current_block, current_index) &&
        hako_physical_value_available_at(function, c, current_block, current_index);
  }
  if (!strcmp(op, "keep_alive")) {
    static const char* const keys[] = {"op", "values"};
    yyjson_val* values = yyjson_obj_get(instruction, "values");
    if (!hako_physical_exact_keys(instruction, keys, 2) || !yyjson_is_arr(values)) return 0;
    for (size_t i = 0; i < yyjson_arr_size(values); i++) {
      if (!hako_physical_u32(yyjson_arr_get(values, i), &a) ||
          !hako_physical_value_available_at(function, a, current_block, current_index)) return 0;
    }
    return 1; /* SSA liveness hint, never runtime lease permission. */
  }
  if (!strcmp(op, "copy")) return hako_physical_exact_keys(instruction, copy, 3) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && hako_physical_u32(yyjson_obj_get(instruction, "src"), &b) && hako_physical_value_available_at(function, b, current_block, current_index);
  if (!strcmp(op, "phi")) {
    yyjson_val* inputs = yyjson_obj_get(instruction, "inputs"); size_t i;
    if (!hako_physical_exact_keys(instruction, phi, 3) || !hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) || !hako_physical_phi_predecessors_exact(function, current_block, inputs)) return 0;
    for (i = 0; i < yyjson_arr_size(inputs); i++) { yyjson_val* item = yyjson_arr_get(inputs, i); static const char* const keys[] = {"block", "value"};
      if (!hako_physical_exact_keys(item, keys, 2) || !hako_physical_u32(yyjson_obj_get(item, "block"), &b) || !hako_physical_block_has_edge_to(function, b, current_block) || !hako_physical_u32(yyjson_obj_get(item, "value"), &c) || !hako_physical_value_available_on_edge(function, c, b)) return 0; }
    return 1;
  }
  if (!strcmp(op, "object_field_get")) return hako_physical_exact_keys(instruction, field_get, 5) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && hako_physical_u32(yyjson_obj_get(instruction, "base"), &b) && hako_physical_value_available_at(function, b, current_block, current_index) && hako_physical_u32(yyjson_obj_get(instruction, "object_id"), &b) && hako_physical_u32(yyjson_obj_get(instruction, "field_ordinal"), &c) && hako_physical_layout_has_field(layouts, b, c);
  if (!strcmp(op, "field_set")) {
    static const char* const keys[] = {"op", "base", "object_id", "field_ordinal",
        "value", "site", "exact_numeric_runtime_check"};
    if (!hako_physical_exact_keys(instruction, keys, 7) ||
        !hako_physical_u32(yyjson_obj_get(instruction, "base"), &a) ||
        !hako_physical_value_available_at(function, a, current_block, current_index) ||
        !hako_physical_u32(yyjson_obj_get(instruction, "value"), &c) ||
        !hako_physical_value_available_at(function, c, current_block, current_index) ||
        !hako_physical_u32(yyjson_obj_get(instruction, "object_id"), &b) ||
        !hako_physical_u32(yyjson_obj_get(instruction, "field_ordinal"), &a) ||
        !hako_physical_layout_has_field(layouts, b, a) ||
        !hako_physical_u64(yyjson_obj_get(instruction, "site"), NULL))
      return 0;
    return hako_physical_field_range_check(yyjson_obj_get(instruction, "exact_numeric_runtime_check"));
  }
  if (!strcmp(op, "array_set")) {
    /* A routed `.set` element write: the array operand is a slot-loaded
       handle lane, index and value are committed i64 payloads, and the
       kernel records bounds/element faults at this exact site. */
    static const char* const keys[] = {"op", "array", "index", "value", "site"};
    return hako_physical_exact_keys(instruction, keys, 5) &&
        hako_physical_u32(yyjson_obj_get(instruction, "array"), &a) &&
        hako_physical_value_available_at(function, a, current_block, current_index) &&
        hako_physical_u32(yyjson_obj_get(instruction, "index"), &b) &&
        hako_physical_value_available_at(function, b, current_block, current_index) &&
        hako_physical_u32(yyjson_obj_get(instruction, "value"), &c) &&
        hako_physical_value_available_at(function, c, current_block, current_index) &&
        hako_physical_u64(yyjson_obj_get(instruction, "site"), NULL);
  }
  if (!strcmp(op, "array_length")) {
    static const char* const keys[] = {"op", "array", "dst"};
    return hako_physical_exact_keys(instruction, keys, 3) &&
        hako_physical_u32(yyjson_obj_get(instruction, "array"), &a) &&
        hako_physical_value_available_at(function, a, current_block, current_index) &&
        hako_physical_u32(yyjson_obj_get(instruction, "dst"), &c);
  }
  if (!strcmp(op, "array_get")) {
    /* A routed `.get` element read: the array operand is the same
       slot-loaded handle lane and the index a committed i64 — the emitted
       call returns the i64-or-handle carrier (miss → the lane's null
       sentinel 0), so the row carries no diagnostic site. */
    static const char* const keys[] = {"op", "array", "index", "dst"};
    return hako_physical_exact_keys(instruction, keys, 4) &&
        hako_physical_u32(yyjson_obj_get(instruction, "array"), &a) &&
        hako_physical_value_available_at(function, a, current_block, current_index) &&
        hako_physical_u32(yyjson_obj_get(instruction, "index"), &b) &&
        hako_physical_value_available_at(function, b, current_block, current_index) &&
        hako_physical_u32(yyjson_obj_get(instruction, "dst"), &c);
  }
  if (!strcmp(op, "invoke")) return hako_physical_exact_keys(instruction, invoke, 5) && hako_physical_u32(yyjson_obj_get(instruction, "fault_frame"), &a) && hako_physical_frame_mode(function, a, frame_mode) && hako_physical_value_available_at(function, a, current_block, current_index) && hako_physical_u32(yyjson_obj_get(instruction, "normal"), &b) && hako_physical_has_block(function, b) && hako_physical_u32(yyjson_obj_get(instruction, "fault"), &c) && hako_physical_has_block(function, c) && hako_physical_validate_operation(function, functions, layouts, yyjson_obj_get(instruction, "operation"), current_block, current_index, native);
  if (!strcmp(op, "invoke_normal_result")) { yyjson_val* invoke;
    return hako_physical_exact_keys(instruction, normal_result, 3) && hako_physical_u32(yyjson_obj_get(instruction, "invoke_block"), &a) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &b) && (invoke = hako_physical_block_invoke_terminator(function, a)) && hako_physical_u32(yyjson_obj_get(invoke, "normal"), &c) && c == current_block && hako_physical_result_operation(yyjson_obj_get(invoke, "operation"), native);
  }
  if (!strcmp(op, "fault_frame_enter")) return hako_physical_exact_keys(instruction, frame_enter, 3) && hako_physical_u32(yyjson_obj_get(instruction, "dst"), &a) && hako_physical_string(yyjson_obj_get(instruction, "mode"), frame_mode);
  if (!strcmp(op, "birth_call")) return hako_physical_exact_keys(instruction, birth, 2) && hako_physical_validate_call(function, functions, yyjson_obj_get(instruction, "call"), current_block, current_index);
  if (!strcmp(op, "return_fault")) return hako_physical_exact_keys(instruction, return_fault, 2) && hako_physical_u32(yyjson_obj_get(instruction, "fault_frame"), &a) && hako_physical_frame_mode(function, a, frame_mode) && hako_physical_value_available_at(function, a, current_block, current_index);
  if (!strcmp(op, "return")) {
    static const char* const object_ret[] = {"op", "value", "result_origin"};
    int owned = hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_handle") ||
        hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_nullable_handle") ||
        (hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_borrowed_nullable_handle") ||
         hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_mixed_handle"));
    return hako_physical_exact_keys(instruction, owned ? object_ret : ret, owned ? 3 : 2) &&
      (!owned || yyjson_is_obj(yyjson_obj_get(instruction, "result_origin"))) &&
      ((native && hako_physical_string(yyjson_obj_get(function, "role"), "root_unit") && yyjson_is_null(yyjson_obj_get(instruction, "value"))) ||
       (hako_physical_string(yyjson_obj_get(function, "role"), "root_i64") && hako_physical_u32(yyjson_obj_get(instruction, "value"), &a) && hako_physical_value_available_at(function, a, current_block, current_index)) ||
       (hako_physical_string(yyjson_obj_get(function, "role"), "birth_unit") && hako_physical_u32(yyjson_obj_get(instruction, "value"), &a) && hako_physical_value_available_at(function, a, current_block, current_index) && hako_physical_is_const_unit(function, a)) ||
       (hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_unit") &&
        (yyjson_is_null(yyjson_obj_get(instruction, "value")) ||
         (hako_physical_u32(yyjson_obj_get(instruction, "value"), &a) && hako_physical_value_available_at(function, a, current_block, current_index) && hako_physical_is_const_unit(function, a)))) ||
       (hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_bool") && hako_physical_u32(yyjson_obj_get(instruction, "value"), &a) && hako_physical_value_available_at(function, a, current_block, current_index)) ||
       (hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_i64") && hako_physical_u32(yyjson_obj_get(instruction, "value"), &a) && hako_physical_value_available_at(function, a, current_block, current_index)) ||
       ((hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_map") ||
         hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_text")) &&
        hako_physical_u32(yyjson_obj_get(instruction, "value"), &a) &&
        hako_physical_value_available_at(function, a, current_block, current_index)) ||
       (hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_handle") && hako_physical_u32(yyjson_obj_get(instruction, "value"), &a) && hako_physical_value_available_at(function, a, current_block, current_index)) ||
       ((hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_nullable_handle") ||
         (hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_borrowed_nullable_handle") ||
         hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_mixed_handle"))) && hako_physical_u32(yyjson_obj_get(instruction, "value"), &a) && hako_physical_value_available_at(function, a, current_block, current_index)));
  }
  if (!strcmp(op, "branch")) return hako_physical_exact_keys(instruction, branch, 6) && hako_physical_u32(yyjson_obj_get(instruction, "condition"), &a) && hako_physical_value_available_at(function, a, current_block, current_index) && hako_physical_u32(yyjson_obj_get(instruction, "then"), &b) && hako_physical_has_block(function, b) && hako_physical_u32(yyjson_obj_get(instruction, "else"), &c) && hako_physical_has_block(function, c) && hako_physical_edge_args(yyjson_obj_get(instruction, "then_args")) && hako_physical_values_available_at(function, yyjson_obj_get(yyjson_obj_get(instruction, "then_args"), "values"), current_block, current_index) && hako_physical_edge_args(yyjson_obj_get(instruction, "else_args")) && hako_physical_values_available_at(function, yyjson_obj_get(yyjson_obj_get(instruction, "else_args"), "values"), current_block, current_index);
  if (!strcmp(op, "jump")) return hako_physical_exact_keys(instruction, jump, 3) && hako_physical_u32(yyjson_obj_get(instruction, "target"), &a) && hako_physical_has_block(function, a) && hako_physical_edge_args(yyjson_obj_get(instruction, "args")) && hako_physical_values_available_at(function, yyjson_obj_get(yyjson_obj_get(instruction, "args"), "values"), current_block, current_index);
  return 0;
}

#include "hako_llvmc_ffi_published_lifecycle_owned_layout.inc"
