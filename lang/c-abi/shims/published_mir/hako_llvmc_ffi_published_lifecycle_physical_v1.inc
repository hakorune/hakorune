/* Call-local parser for hako.published-lifecycle-physical-program.v1.
 * It is deliberately preartifact: no generic reader, LLVM API or V2 row. */
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
