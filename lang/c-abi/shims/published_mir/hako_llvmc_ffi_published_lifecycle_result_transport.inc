/* Structural result/role correspondence shared by schema, indexed flow and
 * emission. This names existing transport only: it grants no source proof,
 * object identity, ownership or new result admission. Unknown results refuse. */
static const char* hako_physical_ordinary_result_role(yyjson_val* result) {
  if (hako_physical_string(result, "unit")) return "ordinary_unit";
  if (hako_physical_string(result, "bool")) return "ordinary_bool";
  if (hako_physical_string(result, "map")) return "ordinary_map";
  if (hako_physical_string(result, "text")) return "ordinary_text";
  if (hako_physical_string(result, "handle")) return "ordinary_handle";
  if (hako_physical_string(result, "nullable_handle")) return "ordinary_nullable_handle";
  if (hako_physical_string(result, "i64")) return "ordinary_i64";
  if (hako_physical_string(result, "borrowed_nullable_handle")) return "ordinary_borrowed_nullable_handle";
  if (hako_physical_string(result, "mixed_handle")) return "ordinary_mixed_handle";
  return NULL;
}

/* Input transport does not depend on the result lane. */
static int hako_physical_ordinary_function(yyjson_val* function) {
  yyjson_val* role = yyjson_obj_get(function, "role");
  return hako_physical_string(role, "ordinary_unit") ||
      hako_physical_string(role, "ordinary_bool") ||
      hako_physical_string(role, "ordinary_i64") ||
      hako_physical_string(role, "ordinary_map") ||
      hako_physical_string(role, "ordinary_text") ||
      hako_physical_string(role, "ordinary_handle") ||
      hako_physical_string(role, "ordinary_nullable_handle") ||
      hako_physical_string(role, "ordinary_borrowed_nullable_handle") ||
      hako_physical_string(role, "ordinary_mixed_handle");
}

static int hako_physical_object_result_role(yyjson_val* function) {
  return hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_handle") ||
      hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_nullable_handle") ||
      hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_borrowed_nullable_handle") ||
      hako_physical_string(yyjson_obj_get(function, "role"), "ordinary_mixed_handle");
}
static int hako_physical_object_result_call(yyjson_val* operation) {
  return hako_physical_string(yyjson_obj_get(operation, "kind"), "ordinary_call") &&
      (hako_physical_string(yyjson_obj_get(operation, "result"), "handle") ||
       hako_physical_string(yyjson_obj_get(operation, "result"), "nullable_handle") ||
       hako_physical_string(yyjson_obj_get(operation, "result"), "borrowed_nullable_handle") ||
       hako_physical_string(yyjson_obj_get(operation, "result"), "mixed_handle"));
}
