/* Exact logical formal transport; included before SSA availability helpers. */
/* Receiver and logical formals share one SSA namespace, never LLVM lane indices. */
static size_t hako_physical_formal_count(yyjson_val* fn) {
  return yyjson_arr_size(yyjson_obj_get(fn, "params")) +
      (yyjson_is_uint(yyjson_obj_get(fn, "receiver")) ? 1u : 0u);
}
static yyjson_val* hako_physical_formal_value(yyjson_val* fn, size_t index) {
  yyjson_val* receiver = yyjson_obj_get(fn, "receiver");
  if (yyjson_is_uint(receiver)) { if (!index) return receiver; index--; }
  return yyjson_obj_get(yyjson_arr_get(yyjson_obj_get(fn, "params"), index), "value");
}
static int hako_physical_params(yyjson_val* fn) {
  static const char* const keys[] = {"value", "representation"};
  yyjson_val* params = yyjson_obj_get(fn, "params");
  if (!yyjson_is_arr(params)) return 0;
  for (size_t i = 0; i < yyjson_arr_size(params); i++) {
    yyjson_val* param = yyjson_arr_get(params, i);
    if (!hako_physical_exact_keys(param, keys, 2) ||
        !hako_physical_u32(yyjson_obj_get(param, "value"), NULL)) return 0;
    if (hako_physical_string(yyjson_obj_get(fn, "role"), "ordinary_i64") ||
        hako_physical_string(yyjson_obj_get(fn, "role"), "ordinary_map") ||
        hako_physical_string(yyjson_obj_get(fn, "role"), "ordinary_nullable_handle")) {
      /* "map" marks a borrowed checked-map storage pointer (MapBox formal). */
      if (!hako_physical_string(yyjson_obj_get(param, "representation"), "i64") &&
          !hako_physical_string(yyjson_obj_get(param, "representation"), "map")) return 0;
    } else if (!hako_physical_string(yyjson_obj_get(param, "representation"), "kind_payload_v1")) return 0;
  }
  return 1;
}

