// Until the source-backed physical body handoff exists, json_in only proves
// that the companion was selected. Frame and site rows remain the sole checks.
static int hako_llvmc_published_lifecycle_body_v2_validate(
    const char* json_in,
    const hako_llvmc_published_lifecycle_frame_v2* frame,
    const hako_llvmc_published_lifecycle_body_site_v1* sites,
    size_t site_count,
    const char* obj_out,
    char** err_out) {
  size_t i, j, matches;
  if (!json_in || !json_in[0])
    return set_err_owned(err_out, "[freeze:contract][published-lifecycle/body-json]");
  if (hako_llvmc_published_lifecycle_v2_validate(frame, obj_out, err_out) != 0)
    return -1;
  if (!sites || site_count == 0u)
    return set_err_owned(err_out, "[freeze:contract][published-lifecycle/body-site-rows]");
  for (i = 0; i < site_count; i++) {
    const hako_llvmc_published_lifecycle_body_site_v1* site = &sites[i];
    if (!site->function_name || site->normal_result == HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2 ||
        site->fault_frame == HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2 || site->normal_landing == HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2 ||
        site->fault_landing == HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2 || site->object_id == HAKO_LLVMC_PUBLISHED_LIFECYCLE_ABSENT_U32_V2)
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/body-site-invalid]");
    matches = 0u;
    for (j = 0; j < frame->operation_count; j++) {
      const hako_llvmc_published_lifecycle_operation_v2* op = &frame->operations[j];
      if (op->kind == 2u && op->function_name && strcmp(op->function_name, site->function_name) == 0 &&
          op->block_id == site->block_id && op->instruction_index == site->instruction_index &&
          op->fault_frame == site->fault_frame && op->normal_landing == site->normal_landing &&
          op->fault_landing == site->fault_landing && op->object_id == site->object_id)
        matches++;
    }
    if (matches != 1u)
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/body-site-mismatch]");
  }
  for (j = 0; j < frame->operation_count; j++) {
    const hako_llvmc_published_lifecycle_operation_v2* op = &frame->operations[j];
    if (op->kind != 2u) continue;
    matches = 0u;
    for (i = 0; i < site_count; i++) {
      const hako_llvmc_published_lifecycle_body_site_v1* site = &sites[i];
      if (site->function_name && op->function_name && strcmp(site->function_name, op->function_name) == 0 &&
          site->block_id == op->block_id && site->instruction_index == op->instruction_index)
        matches++;
    }
    if (matches != 1u)
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/body-site-residual]");
  }
  return 0;
}
