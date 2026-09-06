// V2 validation consumes Rust-issued row arrays without reselecting profile.
static int hako_llvmc_published_lifecycle_v2_validate(
    const hako_llvmc_published_lifecycle_frame_v2* frame,
    const char* obj_out,
    char** err_out) {
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
  return 0;
}
