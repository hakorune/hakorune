/* Structural result/role correspondence shared by schema, indexed flow and
 * emission. This names existing transport only: it grants no source proof,
 * object identity, ownership or new result admission. Unknown results refuse. */
static const char* hako_physical_ordinary_result_role(yyjson_val* result) {
  if (hako_physical_string(result, "unit")) return "ordinary_unit";
  if (hako_physical_string(result, "bool")) return "ordinary_bool";
  if (hako_physical_string(result, "map")) return "ordinary_map";
  if (hako_physical_string(result, "text")) return "ordinary_text";
  if (hako_physical_string(result, "nullable_text")) return "ordinary_nullable_text";
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
      hako_physical_string(role, "ordinary_nullable_text") ||
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

/* Explicit receiver-slot support. Logical parameter ordinals exclude receiver.
 * Parsing this tuple grants neither nonnull nor a live borrowed result. */
typedef struct {
  uint32_t receiver_object, field, key_ordinal, payload_object;
} hako_physical_slot_support;

static int hako_physical_result_slot(yyjson_val* owner, hako_physical_slot_support* out) {
  static const char* const keys[] = {"kind", "field", "key_ordinal", "object_id"};
  static const char* const field_keys[] = {"object_id", "field_ordinal"};
  yyjson_val* support = yyjson_obj_get(owner, "result_support");
  yyjson_val* field = yyjson_obj_get(support, "field");
  hako_physical_slot_support row;
  if (yyjson_obj_get(owner, "result_formal_ordinal") ||
      !hako_physical_exact_keys(support, keys, 4) ||
      !hako_physical_string(yyjson_obj_get(support, "kind"), "receiver_field_slot") ||
      !hako_physical_exact_keys(field, field_keys, 2) ||
      !hako_physical_u32(yyjson_obj_get(field, "object_id"), &row.receiver_object) ||
      !hako_physical_u32(yyjson_obj_get(field, "field_ordinal"), &row.field) ||
      !hako_physical_u32(yyjson_obj_get(support, "key_ordinal"), &row.key_ordinal) ||
      !hako_physical_u32(yyjson_obj_get(support, "object_id"), &row.payload_object)) return 0;
  if (out) *out = row;
  return 1;
}

static int hako_physical_result_slot_equal(yyjson_val* a, yyjson_val* b) {
  hako_physical_slot_support x, y;
  return hako_physical_result_slot(a, &x) && hako_physical_result_slot(b, &y) &&
      x.receiver_object == y.receiver_object && x.field == y.field &&
      x.key_ordinal == y.key_ordinal && x.payload_object == y.payload_object;
}
