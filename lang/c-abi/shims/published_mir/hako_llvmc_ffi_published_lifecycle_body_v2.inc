// V2 companion validation. JSON is a physical body carrier only; lifecycle
// authority is consumed exclusively from Rust-issued frame and site rows.
static int hako_llvmc_published_lifecycle_body_v2_json_matches(
    const char* json_in,
    const hako_llvmc_published_lifecycle_body_site_v1* site) {
  yyjson_read_err error;
  yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &error);
  yyjson_val* functions;
  size_t i;
  int matched = 0;
  if (!doc) return 0;
  functions = yyjson_obj_get(yyjson_doc_get_root(doc), "functions");
  if (!(functions && yyjson_is_arr(functions))) goto done;
  for (i = 0; i < yyjson_arr_size(functions); i++) {
    yyjson_val* function = yyjson_arr_get(functions, i);
    yyjson_val* name = function ? yyjson_obj_get(function, "name") : NULL;
    yyjson_val* blocks = function ? yyjson_obj_get(function, "blocks") : NULL;
    size_t b;
    if (!(name && yyjson_is_str(name) && strcmp(yyjson_get_str(name), site->function_name) == 0 && blocks && yyjson_is_arr(blocks))) continue;
    for (b = 0; b < yyjson_arr_size(blocks); b++) {
      yyjson_val* block = yyjson_arr_get(blocks, b);
      yyjson_val* id = block ? yyjson_obj_get(block, "id") : NULL;
      yyjson_val* instructions = block ? yyjson_obj_get(block, "instructions") : NULL;
      yyjson_val* instruction;
      yyjson_val* op;
      yyjson_val* kind;
      yyjson_val* fault_frame;
      yyjson_val* normal;
      yyjson_val* fault;
      if (!(id && yyjson_is_uint(id) && yyjson_get_uint(id) == site->block_id &&
            instructions && yyjson_is_arr(instructions) &&
            site->instruction_index < yyjson_arr_size(instructions))) continue;
      instruction = yyjson_arr_get(instructions, site->instruction_index);
      op = instruction ? yyjson_obj_get(instruction, "op") : NULL;
      kind = instruction ? yyjson_obj_get(instruction, "kind") : NULL;
      fault_frame = instruction ? yyjson_obj_get(instruction, "fault_frame") : NULL;
      normal = instruction ? yyjson_obj_get(instruction, "normal") : NULL;
      fault = instruction ? yyjson_obj_get(instruction, "fault") : NULL;
      if (op && kind && fault_frame && normal && fault && yyjson_is_str(op) &&
          yyjson_is_uint(kind) && yyjson_is_uint(fault_frame) && yyjson_is_uint(normal) && yyjson_is_uint(fault) &&
          strcmp(yyjson_get_str(op), "published_lifecycle_invoke") == 0 &&
          yyjson_get_uint(kind) == 2u && yyjson_get_uint(fault_frame) == site->fault_frame &&
          yyjson_get_uint(normal) == site->normal_landing && yyjson_get_uint(fault) == site->fault_landing) matched++;
    }
  }
done:
  yyjson_doc_free(doc);
  return matched == 1;
}

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
    if (!site->function_name || site->normal_result == UINT32_MAX ||
        site->fault_frame == UINT32_MAX || site->normal_landing == UINT32_MAX ||
        site->fault_landing == UINT32_MAX || site->object_id == UINT32_MAX)
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
    if (!hako_llvmc_published_lifecycle_body_v2_json_matches(json_in, site))
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle/body-json-coordinate]");
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
