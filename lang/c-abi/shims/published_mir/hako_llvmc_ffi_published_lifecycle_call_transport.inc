/* Exact Birth/ordinary Call transport; included after value availability helpers. */
static int hako_physical_call_args(yyjson_val* function, yyjson_val* args,
    uint32_t block, uint32_t index) {
  static const char* const keys[] = {"kind", "value"};
  if (!yyjson_is_arr(args)) return 0;
  for (size_t i = 0; i < yyjson_arr_size(args); i++) {
    yyjson_val* arg = yyjson_arr_get(args, i);
    uint32_t kind, value;
    if (!hako_physical_exact_keys(arg, keys, 2) ||
        !hako_physical_u32(yyjson_obj_get(arg, "kind"), &kind) || (kind != 1 && kind != 2) ||
        !hako_physical_u32(yyjson_obj_get(arg, "value"), &value) ||
        !hako_physical_value_available_at(function, value, block, index)) return 0;
  }
  return 1;
}

static int hako_physical_validate_call(yyjson_val* function, yyjson_val* functions, yyjson_val* call, uint32_t block, uint32_t index) {
  static const char* const keys[] = {"target", "receiver", "args", "dst"};
  uint32_t target, receiver;
  yyjson_val* callee;
  if (!hako_physical_exact_keys(call, keys, 4) ||
      !hako_physical_u32(yyjson_obj_get(call, "target"), &target) ||
      !hako_physical_u32(yyjson_obj_get(call, "receiver"), &receiver) ||
      !hako_physical_call_args(function, yyjson_obj_get(call, "args"), block, index) ||
      !hako_physical_value_available_at(function, receiver, block, index) || target == 0 || target >= yyjson_arr_size(functions))
    return 0;
  callee = yyjson_arr_get(functions, target);
  return hako_physical_string(callee ? yyjson_obj_get(callee, "role") : NULL, "birth_unit") &&
      yyjson_arr_size(yyjson_obj_get(callee, "params")) == yyjson_arr_size(yyjson_obj_get(call, "args")) &&
      yyjson_is_null(yyjson_obj_get(call, "dst"));
}

static int hako_physical_validate_ordinary_call(yyjson_val* function, yyjson_val* functions,
    yyjson_val* call, uint32_t block, uint32_t index, const char* want_role) {
  static const char* const static_keys[] = {"target", "args", "dst"};
  static const char* const instance_keys[] = {"target", "receiver", "args", "dst"};
  static const char* const arg_keys[] = {"kind", "value"};
  uint32_t target;
  yyjson_val* callee;
  yyjson_val* args;
  yyjson_val* receiver = yyjson_obj_get(call, "receiver");
  int has_receiver = receiver && !yyjson_is_null(receiver);
  if ((!has_receiver && !hako_physical_exact_keys(call, static_keys, 3)) ||
      (has_receiver && !hako_physical_exact_keys(call, instance_keys, 4)) ||
      !hako_physical_u32(yyjson_obj_get(call, "target"), &target) ||
      target == 0 || target >= yyjson_arr_size(functions) ||
      !yyjson_is_null(yyjson_obj_get(call, "dst")) ||
      !(args = yyjson_obj_get(call, "args")) || !yyjson_is_arr(args)) return 0;
  callee = yyjson_arr_get(functions, target);
  if (!hako_physical_string(callee ? yyjson_obj_get(callee, "role") : NULL, want_role) ||
      (yyjson_is_uint(yyjson_obj_get(callee, "receiver")) != has_receiver) ||
      (has_receiver && (!hako_physical_u32(receiver, NULL) ||
          !hako_physical_value_available_at(function, yyjson_get_uint(receiver), block, index))) ||
      yyjson_arr_size(yyjson_obj_get(callee, "params")) != yyjson_arr_size(args)) return 0;
  for (size_t i = 0; i < yyjson_arr_size(args); i++) {
    yyjson_val* arg = yyjson_arr_get(args, i);
    yyjson_val* param = yyjson_arr_get(yyjson_obj_get(callee, "params"), i);
    uint32_t value;
    const char* arg_kind;
    /* The corroborated pair rule: a caller-owned checked-map actual spells
       "map" and is admitted only against a "map" formal — the storage
       pointer rides the edge borrowed. Every other pair stays i64; a map
       claim on one side alone is ABI drift, never a silent scalar fill. */
    if (!hako_physical_exact_keys(arg, arg_keys, 2) ||
        !hako_physical_u32(yyjson_obj_get(arg, "value"), &value) ||
        !hako_physical_value_available_at(function, value, block, index)) return 0;
    arg_kind = yyjson_get_str(yyjson_obj_get(arg, "kind"));
    if (hako_physical_string(yyjson_obj_get(param, "representation"), "borrowed_kind_payload_v1")) {
      uint32_t kind;
      if ((!arg_kind || strcmp(arg_kind, "tagged")) &&
          (!hako_physical_u32(yyjson_obj_get(arg, "kind"), &kind) || kind < 1 || kind > 3)) return 0;
    } else if (!arg_kind) { return 0;
    } else if (!strcmp(arg_kind, "map")) {
      if (!hako_physical_string(yyjson_obj_get(param, "representation"), "map")) return 0;
    } else if (strcmp(arg_kind, "i64") ||
        !hako_physical_string(yyjson_obj_get(param, "representation"), "i64")) {
      return 0;
    }
  }
  return 1;
}

