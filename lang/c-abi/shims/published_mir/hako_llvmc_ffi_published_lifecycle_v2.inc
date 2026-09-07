// V2 validation consumes Rust-issued row arrays without reselecting profile.
static int hako_llvmc_published_lifecycle_v2_validate(
    const hako_llvmc_published_lifecycle_frame_v2* frame,
    const char* obj_out,
    char** err_out) {
  size_t i, j, root_count = 0u, return_count;
  if (!frame || !obj_out || !obj_out[0])
    return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-null-boundary]");
  if (frame->abi_revision != HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABI_REVISION_V2)
    return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-abi-revision]");
  if (frame->storage_profile != HAKO_LLVMC_OBJECT_STORAGE_SAFE_MUTEX_V1 &&
      frame->storage_profile != HAKO_LLVMC_OBJECT_STORAGE_SINGLE_THREAD_EXACT_V1)
    return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-storage-profile]");
#define REQUIRE_ROWS(ptr, count, name) \
  do { if (!(ptr) || (count) == 0u) return set_err_owned(err_out, \
    "[freeze:contract][published-lifecycle/v2-" name "-rows]"); } while (0)
  REQUIRE_ROWS(frame->definitions, frame->definition_count, "definition");
  REQUIRE_ROWS(frame->formals, frame->formal_count, "formal");
  REQUIRE_ROWS(frame->operations, frame->operation_count, "operation");
  REQUIRE_ROWS(frame->operands, frame->operand_count, "operand");
  REQUIRE_ROWS(frame->controls, frame->control_count, "control");
  REQUIRE_ROWS(frame->layouts, frame->layout_count, "layout");
  REQUIRE_ROWS(frame->fields, frame->field_count, "field");
#undef REQUIRE_ROWS
#define ABSENT(value) ((value) == HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2)
#define PRESENT(value) (!ABSENT(value))
  for (i = 0; i < frame->formal_count; i++) {
    const hako_llvmc_published_lifecycle_formal_v2* formal = &frame->formals[i];
    const hako_llvmc_published_lifecycle_definition_v2* definition;
    if (formal->definition_index >= frame->definition_count ||
        ABSENT(formal->physical_ordinal) || ABSENT(formal->value_id) ||
        formal->wire_revision != 2u)
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-formal-range]");
    definition = &frame->definitions[formal->definition_index];
    if (definition->role != HAKO_LLVMC_LIFECYCLE_DEFINITION_ROLE_BIRTH_UNIT_V2)
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-formal-definition]");
    if (formal->input_kind == 1u) {
      if (!ABSENT(formal->source_ordinal) || formal->physical_ordinal != 0u)
        return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-formal-receiver]");
    } else if (formal->input_kind == 2u) {
      if (ABSENT(formal->source_ordinal) || formal->physical_ordinal == 0u)
        return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-formal-parameter]");
    } else {
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-formal-kind]");
    }
  }
  for (i = 0; i < frame->operation_count; i++) {
    const hako_llvmc_published_lifecycle_operation_v2* op = &frame->operations[i];
    int invoke_lanes = PRESENT(op->fault_frame) && PRESENT(op->normal_landing) && PRESENT(op->fault_landing);
    int bare_lanes = ABSENT(op->fault_frame) && ABSENT(op->normal_landing) && ABSENT(op->fault_landing);
    if (!op->function_name || (!invoke_lanes && !bare_lanes))
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-operation-lanes]");
    switch (op->kind) {
      case 1u:
        if (op->definition_index >= frame->definition_count || ABSENT(op->receiver) ||
            PRESENT(op->object_id) || PRESENT(op->field_ordinal) || PRESENT(op->base) ||
            op->operand_count == 0u)
          return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-operation-call]");
        break;
      case 2u:
        if (!invoke_lanes || PRESENT(op->definition_index) || ABSENT(op->object_id) ||
            PRESENT(op->field_ordinal) || PRESENT(op->base) || PRESENT(op->value) ||
            PRESENT(op->receiver) || op->operand_count != 0u)
          return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-operation-newbox]");
        break;
      case 3u:
        if (!invoke_lanes || PRESENT(op->definition_index) || ABSENT(op->object_id) ||
            ABSENT(op->field_ordinal) || ABSENT(op->base) || ABSENT(op->value) ||
            ABSENT(op->receiver) || op->operand_count != 2u)
          return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-operation-field-set]");
        break;
      case 4u:
      case 5u:
        if (!invoke_lanes || PRESENT(op->definition_index) || ABSENT(op->object_id) ||
            PRESENT(op->field_ordinal) || PRESENT(op->base) || ABSENT(op->value) ||
            PRESENT(op->receiver) || op->operand_count != 1u)
          return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-operation-cleanup]");
        break;
      case 6u:
        if (!bare_lanes || PRESENT(op->definition_index) || ABSENT(op->object_id) ||
            ABSENT(op->field_ordinal) || ABSENT(op->base) || ABSENT(op->value) ||
            ABSENT(op->receiver) || op->operand_count != 1u)
          return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-operation-field-get]");
        break;
      default:
        return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-operation-kind]");
    }
  }
  for (i = 0; i < frame->control_count; i++) {
    const hako_llvmc_published_lifecycle_control_v2* control = &frame->controls[i];
    if (!control->function_name)
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-control-name]");
    if (control->kind == 1u) {
      if (ABSENT(control->operand) || ABSENT(control->origin_block) || control->mode != 0u)
        return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-control-normal]");
    } else if (control->kind == 2u) {
      if (ABSENT(control->operand) || PRESENT(control->origin_block) || control->mode != 0u)
        return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-control-fault]");
    } else if (control->kind == 3u) {
      if (ABSENT(control->operand) || PRESENT(control->origin_block) ||
          (control->mode != 1u && control->mode != 2u))
        return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-control-frame]");
    } else if (control->kind == HAKO_LLVMC_LIFECYCLE_CONTROL_KIND_RETURN_V2) {
      if (PRESENT(control->origin_block) ||
          (control->mode == 0u && PRESENT(control->operand)) ||
          (control->mode == 1u && ABSENT(control->operand)) ||
          (control->mode != 0u && control->mode != 1u))
        return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-control-return]");
    } else {
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-control-kind]");
    }
  }
  for (i = 0; i < frame->definition_count; i++) {
    const hako_llvmc_published_lifecycle_definition_v2* definition = &frame->definitions[i];
    unsigned expected_result, expected_mode;
    if (definition->role == HAKO_LLVMC_LIFECYCLE_DEFINITION_ROLE_BIRTH_UNIT_V2) {
      if (definition->result_kind != HAKO_LLVMC_LIFECYCLE_RESULT_KIND_UNIT_V2)
        return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-birth-result-kind]");
      continue;
    }
    if (definition->role == HAKO_LLVMC_LIFECYCLE_DEFINITION_ROLE_ROOT_I64_V2) {
      expected_result = HAKO_LLVMC_LIFECYCLE_RESULT_KIND_I64_V2;
      expected_mode = 1u;
    } else if (definition->role == HAKO_LLVMC_LIFECYCLE_DEFINITION_ROLE_ROOT_UNIT_V2) {
      expected_result = HAKO_LLVMC_LIFECYCLE_RESULT_KIND_UNIT_V2;
      expected_mode = 0u;
    } else {
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-definition-role]");
    }
    if (++root_count != 1u || !definition->function_name || !definition->target_symbol ||
        definition->result_kind != expected_result || definition->source_arity != 0u ||
        definition->receiver_formal != HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2 || definition->object_id != HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2 ||
        definition->flags != 1u)
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-root-definition]");
    return_count = 0u;
    for (j = 0; j < frame->control_count; j++) {
      const hako_llvmc_published_lifecycle_control_v2* control = &frame->controls[j];
      if (control->function_name && strcmp(control->function_name, definition->function_name) == 0 &&
          control->kind == HAKO_LLVMC_LIFECYCLE_CONTROL_KIND_RETURN_V2) {
        if (control->mode != expected_mode ||
            (expected_mode == 0u && control->operand != HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2) ||
            (expected_mode == 1u && control->operand == HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2))
          return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-root-return]");
        return_count++;
      }
    }
    if (return_count != 1u)
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-root-return-count]");
  }
  if (root_count != 1u)
    return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-root-definition-count]");
#undef PRESENT
#undef ABSENT
  return 0;
}
