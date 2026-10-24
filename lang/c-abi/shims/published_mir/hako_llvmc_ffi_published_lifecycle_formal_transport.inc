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
/* Finite source classes are input membership, never an exclusive read view.
 * Canonical tag order is Null, I64, Bool, Text. Zero also denotes invalid. */
static unsigned hako_physical_finite_domain(yyjson_val* param) {
  yyjson_val* domain = yyjson_obj_get(param, "finite_domain");
  unsigned mask = 0, previous = 0;
  if (!yyjson_is_arr(domain) || !yyjson_arr_size(domain) || yyjson_arr_size(domain) > 4) return 0;
  for (size_t i = 0; i < yyjson_arr_size(domain); i++) {
    yyjson_val* name = yyjson_arr_get(domain, i);
    unsigned tag = hako_physical_string(name, "null") ? 0 :
        hako_physical_string(name, "i64") ? 1 : hako_physical_string(name, "bool") ? 2 :
        hako_physical_string(name, "text") ? 4 : 5;
    if (tag == 5 || (i && tag <= previous)) return 0;
    mask |= 1u << tag; previous = tag;
  }
  return mask;
}
static unsigned hako_physical_input_domain(yyjson_val* param) {
  if (yyjson_is_true(yyjson_obj_get(param, "text_view"))) return 1u << 4;
  if (yyjson_is_true(yyjson_obj_get(param, "integer_view"))) return 1u << 1;
  if (yyjson_is_true(yyjson_obj_get(param, "bool_view"))) return 1u << 2;
  return hako_physical_finite_domain(param);
}

static int hako_physical_params(yyjson_val* fn, yyjson_val* layouts) {
  static const char* const keys[] = {"value", "representation"};
  static const char* const view_keys[] = {"value", "representation", "object_view"};
  static const char* const text_keys[] = {"value", "representation", "object_view", "text_view"};
  static const char* const integer_keys[] = {"value", "representation", "object_view", "integer_view"};
  static const char* const finite_keys[] = {"value", "representation", "object_view", "finite_domain"};
  static const char* const bool_keys[] = {"value", "representation", "object_view", "bool_view"};
  yyjson_val* params = yyjson_obj_get(fn, "params");
  if (!yyjson_is_arr(params)) return 0;
  for (size_t i = 0; i < yyjson_arr_size(params); i++) {
    yyjson_val* param = yyjson_arr_get(params, i);
    if (hako_physical_string(yyjson_obj_get(param, "representation"), "borrowed_kind_payload_v1")) {
      /* Mandatory null means unconstrained opaque; uint names the exact
         class/null domain, including a declared but ignored formal. */
      uint32_t object;
      yyjson_val* view = yyjson_obj_get(param, "object_view");
      yyjson_val* text = yyjson_obj_get(param, "text_view");
      yyjson_val* integer = yyjson_obj_get(param, "integer_view");
      yyjson_val* boolean = yyjson_obj_get(param, "bool_view");
      yyjson_val* finite = yyjson_obj_get(param, "finite_domain");
      yyjson_val* scalar_view = text ? text : integer ? integer : boolean;
      if (((!!text + !!integer + !!boolean + !!finite) > 1) ||
          (scalar_view && (!yyjson_is_true(scalar_view) || !yyjson_is_null(view) ||
              !(hako_physical_ordinary_function(fn) ||
                (text && hako_physical_string(yyjson_obj_get(fn, "role"), "birth_unit")))))) return 0;
      if (finite && (!yyjson_is_null(view) || !hako_physical_ordinary_function(fn) ||
          !hako_physical_finite_domain(param))) return 0;
      if (!(finite ? hako_physical_exact_keys(param, finite_keys, 4) : text ? hako_physical_exact_keys(param, text_keys, 4) :
          integer ? hako_physical_exact_keys(param, integer_keys, 4) :
          boolean ? hako_physical_exact_keys(param, bool_keys, 4) :
          hako_physical_exact_keys(param, view_keys, 3)) ||
          (!yyjson_is_null(view) && (!hako_physical_u32(view, &object) ||
          !hako_physical_layout_has_object(layouts, object)))) return 0;
    } else if (!hako_physical_exact_keys(param, keys, 2)) return 0;
    if (!hako_physical_u32(yyjson_obj_get(param, "value"), NULL)) return 0;
    if (hako_physical_ordinary_function(fn)) {
      /* Checked-map and tagged input lanes are independent of result kind.
         Receiver/layout, actual provenance and each input use remain checked
         by their existing owners. */
      if (!hako_physical_string(yyjson_obj_get(param, "representation"), "i64") &&
          !hako_physical_string(yyjson_obj_get(param, "representation"), "map") &&
          !hako_physical_string(yyjson_obj_get(param, "representation"), "borrowed_kind_payload_v1")) return 0;
    } else if (!(hako_physical_string(yyjson_obj_get(fn, "role"), "birth_unit") &&
        hako_physical_string(yyjson_obj_get(param, "representation"), "borrowed_kind_payload_v1") &&
        yyjson_is_true(yyjson_obj_get(param, "text_view"))) &&
        !hako_physical_string(yyjson_obj_get(param, "representation"), "kind_payload_v1") &&
        /* A birth formal sealed as object-field-provided spells the
           nullable lane — same pair transport, the kind==0 branch is
           admitted beside the scalar kinds. */
        !hako_physical_string(yyjson_obj_get(param, "representation"), "nullable_kind_payload_v1")) return 0;
  }
  return 1;
}
