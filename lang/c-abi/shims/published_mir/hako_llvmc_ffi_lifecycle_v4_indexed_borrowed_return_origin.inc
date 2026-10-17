/* The existing nullable-forward law is Null or the SAME fixed logical formal.
 * Physical corroboration adds no Home or incoming-call permission. The
 * dedicated Call/Normal/anchor consumers remain independently mandatory. */
static int lv4_borrowed_return_origin(hako_lv4_function_index* fn, yyjson_val* returned) {
  static const char* const null_keys[] = {"kind", "producer", "copies"};
  static const char* const formal_keys[] = {"kind", "formal_ordinal", "copies"};
  yyjson_val* origin = yyjson_obj_get(returned, "result_origin");
  uint32_t ordinal, returned_value, value;
  yyjson_val* params = yyjson_obj_get(fn->function, "params");
  if (!lv4_is(fn->function, "role", "ordinary_borrowed_nullable_handle") ||
      !hako_physical_u32(yyjson_obj_get(fn->function, "result_formal_ordinal"), &ordinal) ||
      ordinal >= yyjson_arr_size(params) ||
      !hako_physical_u32(yyjson_obj_get(returned, "value"), &returned_value)) return 0;
  yyjson_val* formal = yyjson_arr_get(params, ordinal);
  if (!lv4_is(formal, "representation", "borrowed_kind_payload_v1") ||
      !hako_physical_u32(yyjson_obj_get(formal, "value"), &value)) return 0;
  if (lv4_is(origin, "kind", "null")) {
    yyjson_val* producer = lv4_result_coordinate(fn, yyjson_obj_get(origin, "producer"), NULL);
    if (!hako_physical_exact_keys(origin, null_keys, 3) ||
        !lv4_is(producer, "op", "const_null") ||
        !hako_physical_u32(yyjson_obj_get(producer, "dst"), &value)) return 0;
  } else {
    uint32_t original_ordinal;
    if (!hako_physical_exact_keys(origin, formal_keys, 3) ||
        !lv4_is(origin, "kind", "borrowed_formal") ||
        !hako_physical_u32(yyjson_obj_get(origin, "formal_ordinal"), &original_ordinal) ||
        original_ordinal != ordinal) return 0;
  }
  return lv4_result_copy_chain(fn, origin, value, returned_value);
}
