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
        definition->receiver_formal != UINT32_MAX || definition->object_id != UINT32_MAX ||
        definition->flags != 1u)
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-root-definition]");
    return_count = 0u;
    for (j = 0; j < frame->control_count; j++) {
      const hako_llvmc_published_lifecycle_control_v2* control = &frame->controls[j];
      if (control->function_name && strcmp(control->function_name, definition->function_name) == 0 &&
          control->kind == HAKO_LLVMC_LIFECYCLE_CONTROL_KIND_RETURN_V2) {
        if (control->mode != expected_mode ||
            (expected_mode == 0u && control->operand != UINT32_MAX) ||
            (expected_mode == 1u && control->operand == UINT32_MAX))
          return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-root-return]");
        return_count++;
      }
    }
    if (return_count != 1u)
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-root-return-count]");
  }
  if (root_count != 1u)
    return set_err_owned(err_out, "[freeze:contract][published-lifecycle/v2-root-definition-count]");
  return 0;
}
