/* Call-local parser for hako.published-lifecycle-physical-program.v1.
 * It is deliberately preartifact: no generic reader, LLVM API or V2 row. */
static int hako_llvmc_published_lifecycle_physical_v1_opcode(yyjson_val* instruction);
static int hako_llvmc_validate_published_lifecycle_physical_v1_doc(
    yyjson_val* root, char** err_out) {
  yyjson_val *schema, *functions, *layouts, *fault_abi;
  size_t i;
  schema = root ? yyjson_obj_get(root, "schema") : NULL;
  functions = root ? yyjson_obj_get(root, "functions") : NULL;
  layouts = root ? yyjson_obj_get(root, "layouts") : NULL;
  fault_abi = root ? yyjson_obj_get(root, "fault_abi_version") : NULL;
  if (!(schema && yyjson_is_str(schema) &&
        strcmp(yyjson_get_str(schema), "hako.published-lifecycle-physical-program.v1") == 0))
    return set_err_owned(err_out, "[freeze:contract][published-lifecycle-physical-parser/schema]");
  if (!(functions && yyjson_is_arr(functions) && yyjson_arr_size(functions) >= 2u))
    return set_err_owned(err_out, "[freeze:contract][published-lifecycle-physical-parser/functions]");
  if (!(layouts && yyjson_is_arr(layouts) && yyjson_arr_size(layouts) > 0u &&
        fault_abi && yyjson_is_uint(fault_abi) && yyjson_get_uint(fault_abi) == 1u))
    return set_err_owned(err_out, "[freeze:contract][published-lifecycle-physical-parser/abi]");
  for (i = 0; i < yyjson_arr_size(functions); i++) {
    yyjson_val *fn = yyjson_arr_get(functions, i);
    yyjson_val *role = fn ? yyjson_obj_get(fn, "role") : NULL;
    yyjson_val *params = fn ? yyjson_obj_get(fn, "params") : NULL;
    yyjson_val *blocks = fn ? yyjson_obj_get(fn, "blocks") : NULL;
    if (!(role && yyjson_is_str(role) && params && yyjson_is_arr(params) &&
          blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) > 0u))
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle-physical-parser/function]");
    if ((i == 0u && strcmp(yyjson_get_str(role), "root_i64") != 0) ||
        (i > 0u && strcmp(yyjson_get_str(role), "birth_unit") != 0))
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle-physical-parser/function-role]");
    for (size_t bi = 0; bi < yyjson_arr_size(blocks); bi++) {
      yyjson_val *block = yyjson_arr_get(blocks, bi);
      yyjson_val *instructions = block ? yyjson_obj_get(block, "instructions") : NULL;
      yyjson_val *terminator = block ? yyjson_obj_get(block, "terminator") : NULL;
      yyjson_val *edges = block ? yyjson_obj_get(block, "edges") : NULL;
      if (!(instructions && yyjson_is_arr(instructions) && terminator && yyjson_is_obj(terminator) &&
            edges && yyjson_is_arr(edges)))
        return set_err_owned(err_out, "[freeze:contract][published-lifecycle-physical-parser/block]");
      for (size_t ii = 0; ii < yyjson_arr_size(instructions); ii++) {
        yyjson_val *row = yyjson_arr_get(instructions, ii);
        yyjson_val *index = row ? yyjson_obj_get(row, "index") : NULL;
        yyjson_val *instruction = row ? yyjson_obj_get(row, "instruction") : NULL;
        if (!(index && yyjson_is_uint(index) && yyjson_get_uint(index) == ii &&
              instruction && yyjson_is_obj(instruction) &&
              hako_llvmc_published_lifecycle_physical_v1_opcode(instruction)))
          return set_err_owned(err_out, "[freeze:contract][published-lifecycle-physical-parser/instruction-index]");
      }
      { yyjson_val *index = yyjson_obj_get(terminator, "index");
        yyjson_val *instruction = yyjson_obj_get(terminator, "instruction");
        if (!(index && yyjson_is_uint(index) && yyjson_get_uint(index) == yyjson_arr_size(instructions) &&
              instruction && yyjson_is_obj(instruction) &&
              hako_llvmc_published_lifecycle_physical_v1_opcode(instruction)))
          return set_err_owned(err_out, "[freeze:contract][published-lifecycle-physical-parser/terminator-index]");
      }
      for (size_t ei = 0; ei < yyjson_arr_size(edges); ei++) {
        yyjson_val *edge = yyjson_arr_get(edges, ei);
        yyjson_val *target = edge ? yyjson_obj_get(edge, "target") : NULL;
        if (!(target && yyjson_is_uint(target)))
          return set_err_owned(err_out, "[freeze:contract][published-lifecycle-physical-parser/edge-target]");
      }
    }
  }
  for (i = 0; i < yyjson_arr_size(layouts); i++) {
    yyjson_val *layout = yyjson_arr_get(layouts, i);
    yyjson_val *count = layout ? yyjson_obj_get(layout, "field_count") : NULL;
    yyjson_val *fields = layout ? yyjson_obj_get(layout, "fields") : NULL;
    if (!(count && yyjson_is_uint(count) && fields && yyjson_is_arr(fields) &&
          yyjson_arr_size(fields) == yyjson_get_uint(count)))
      return set_err_owned(err_out, "[freeze:contract][published-lifecycle-physical-parser/layout]");
  }
  return 0;
}

static int hako_llvmc_published_lifecycle_physical_v1_opcode(yyjson_val* instruction) {
  yyjson_val* op = yyjson_obj_get(instruction, "op");
  const char* name = (op && yyjson_is_str(op)) ? yyjson_get_str(op) : NULL;
  static const char* const allowed[] = {
    "const_i64", "const_string", "const_unit", "add", "copy", "phi",
    "object_field_get", "invoke", "invoke_normal_result", "return_fault",
    "fault_frame_enter", "branch", "jump", "return", "birth_call"
  };
  size_t i;
  if (!name) return 0;
  for (i = 0; i < sizeof(allowed) / sizeof(allowed[0]); i++)
    if (strcmp(name, allowed[i]) == 0) return 1;
  return 0;
}
