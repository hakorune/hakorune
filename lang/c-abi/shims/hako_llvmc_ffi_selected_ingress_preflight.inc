/* Strict selected C ingress admission.  Read the parsed instruction arrays;
 * never select a route from raw JSON text or from nested metadata payloads. */

static int hako_llvmc_ingress_profile_is_strict(
    enum HakoLlvmcIngressProfile profile) {
  return profile == HAKO_LLVMC_INGRESS_BOUNDARY_PURE_FIRST ||
      profile == HAKO_LLVMC_INGRESS_STATIC_V2;
}

static int hako_llvmc_reject_selected_legacy_call_op(
    enum HakoLlvmcIngressProfile profile,
    yyjson_val* root,
    char** err_out) {
  yyjson_val* functions;
  size_t function_index;

  if (!hako_llvmc_ingress_profile_is_strict(profile)) return 0;
  functions = root ? yyjson_obj_get(root, "functions") : NULL;
  if (!functions || !yyjson_is_arr(functions)) return 0;

  for (function_index = 0;
       function_index < yyjson_arr_size(functions);
       function_index++) {
    yyjson_val* function = yyjson_arr_get(functions, function_index);
    yyjson_val* blocks = function && yyjson_is_obj(function)
        ? yyjson_obj_get(function, "blocks") : NULL;
    size_t block_index;
    if (!blocks || !yyjson_is_arr(blocks)) continue;
    for (block_index = 0;
         block_index < yyjson_arr_size(blocks);
         block_index++) {
      yyjson_val* block = yyjson_arr_get(blocks, block_index);
      yyjson_val* instructions = block && yyjson_is_obj(block)
          ? yyjson_obj_get(block, "instructions") : NULL;
      size_t instruction_index;
      if (!instructions || !yyjson_is_arr(instructions)) continue;
      for (instruction_index = 0;
           instruction_index < yyjson_arr_size(instructions);
           instruction_index++) {
        yyjson_val* instruction = yyjson_arr_get(instructions, instruction_index);
        yyjson_val* op = instruction && yyjson_is_obj(instruction)
            ? yyjson_obj_get(instruction, "op") : NULL;
        const char* op_name = op ? yyjson_get_str(op) : NULL;
        if (op_name && strcmp(op_name, "call") == 0) {
          hako_set_last_error("UNSUPPORTED");
          return set_err_owned(
              err_out,
              "[freeze:contract][pure-first/legacy-op-call]");
        }
      }
    }
  }
  return 0;
}
