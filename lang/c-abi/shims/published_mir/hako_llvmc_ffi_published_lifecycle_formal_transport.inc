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
static int hako_physical_layout_has_object(yyjson_val* layouts, uint32_t object_id) {
  size_t i;
  int count = 0;
  for (i = 0; layouts && i < yyjson_arr_size(layouts); i++) {
    uint32_t id;
    yyjson_val* layout = yyjson_arr_get(layouts, i);
    if (hako_physical_u32(layout ? yyjson_obj_get(layout, "object_id") : NULL, &id) && id == object_id)
      count++;
  }
  return count == 1;
}
static int hako_physical_params(yyjson_val* fn, yyjson_val* layouts) {
  static const char* const keys[] = {"value", "representation"};
  static const char* const view_keys[] = {"value", "representation", "object_view"};
  yyjson_val* params = yyjson_obj_get(fn, "params");
  if (!yyjson_is_arr(params)) return 0;
  for (size_t i = 0; i < yyjson_arr_size(params); i++) {
    yyjson_val* param = yyjson_arr_get(params, i);
    if (hako_physical_string(yyjson_obj_get(param, "representation"), "borrowed_kind_payload_v1")) {
      /* Mandatory null means unconstrained opaque; uint names the exact
         class/null domain, including a declared but ignored formal. */
      uint32_t object;
      yyjson_val* view = yyjson_obj_get(param, "object_view");
      if (!hako_physical_exact_keys(param, view_keys, 3) ||
          (!yyjson_is_null(view) && (!hako_physical_u32(view, &object) ||
          !hako_physical_layout_has_object(layouts, object)))) return 0;
    } else if (!hako_physical_exact_keys(param, keys, 2)) return 0;
    if (!hako_physical_u32(yyjson_obj_get(param, "value"), NULL)) return 0;
    if (hako_physical_string(yyjson_obj_get(fn, "role"), "ordinary_i64") ||
        hako_physical_string(yyjson_obj_get(fn, "role"), "ordinary_map") ||
        hako_physical_string(yyjson_obj_get(fn, "role"), "ordinary_nullable_handle")) {
      /* "map" marks a borrowed checked-map storage pointer (MapBox formal). */
      if (!hako_physical_string(yyjson_obj_get(param, "representation"), "i64") &&
          !hako_physical_string(yyjson_obj_get(param, "representation"), "map")) {
        /* The selected internal carrier uses the same pair transport. Static
           I64 has no receiver; nullable-handle keeps its instance boundary.
           The ordinary prologue revalidates the exact pair either way. */
        if ((!hako_physical_string(yyjson_obj_get(fn, "role"), "ordinary_i64") &&
             !hako_physical_string(yyjson_obj_get(fn, "role"), "ordinary_nullable_handle")) ||
            (!yyjson_is_uint(yyjson_obj_get(fn, "receiver")) &&
             !(hako_physical_string(yyjson_obj_get(fn, "role"), "ordinary_i64") &&
               yyjson_is_null(yyjson_obj_get(fn, "receiver")) &&
               yyjson_is_null(yyjson_obj_get(fn, "receiver_object")))) ||
            !hako_physical_string(yyjson_obj_get(param, "representation"), "borrowed_kind_payload_v1")) return 0;
      }
    } else if (!hako_physical_string(yyjson_obj_get(param, "representation"), "kind_payload_v1") &&
        /* A birth formal sealed as object-field-provided spells the
           nullable lane — same pair transport, the kind==0 branch is
           admitted beside the scalar kinds. */
        !hako_physical_string(yyjson_obj_get(param, "representation"), "nullable_kind_payload_v1")) return 0;
  }
  return 1;
}
