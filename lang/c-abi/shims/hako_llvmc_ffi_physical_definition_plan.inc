/* Borrowed definition intent only. Emitted-state and body eligibility stay
 * in the walkers. Both memberships are allowed for one physical symbol. */
struct PhysicalDefinitionPlan {
  const char* leaf[256];
  size_t leaf_count;
  const char* same_module[1024];
  size_t same_module_count;
  int read_result;
  const char* same_module_overflow; /* Borrowed from the same document. */
};

auto int physical_definition_plan_contains(
    const struct PhysicalDefinitionPlan* plan, int leaf, const char* name) {
  const char* const* symbols = leaf ? plan->leaf : plan->same_module;
  size_t count = leaf ? plan->leaf_count : plan->same_module_count;
  if (!name) return 0;
  for (size_t i = 0; i < count; i++)
    if (!strcmp(symbols[i], name)) return 1;
  return 0;
}

auto int physical_definition_plan_add(
    struct PhysicalDefinitionPlan* plan, int leaf, const char* name) {
  if (!name) return 0;
  if (physical_definition_plan_contains(plan, leaf, name)) return 1;
  const char** symbols = leaf ? plan->leaf : plan->same_module;
  size_t* count = leaf ? &plan->leaf_count : &plan->same_module_count;
  size_t capacity = leaf ? ARR_LEN(plan->leaf) : ARR_LEN(plan->same_module);
  if (*count >= capacity) {
    if (!leaf && !plan->same_module_overflow) plan->same_module_overflow = name;
    return 0;
  }
  symbols[(*count)++] = name;
  return 1;
}

auto int read_physical_definition_plan(
    struct PhysicalDefinitionPlan* plan, yyjson_val* fn) {
  yyjson_val* metadata = fn ? yyjson_obj_get(fn, "metadata") : NULL;
  yyjson_val* rows =
      metadata ? yyjson_obj_get(metadata, "same_module_function_definitions") : NULL;
  int planned = 0;
  plan->read_result = 0;
  plan->same_module_overflow = NULL;
  if (!rows || yyjson_is_null(rows)) return 0;
  if (!yyjson_is_arr(rows)) return plan->read_result = -1;
  for (size_t i = 0; i < yyjson_arr_size(rows); i++) {
    yyjson_val* row = yyjson_arr_get(rows, i);
    const char* target_symbol = row ? read_str(row, "target_symbol") : NULL;
    const char* definition_kind = row ? read_str(row, "definition_kind") : NULL;
    if (!(target_symbol && definition_kind)) return plan->read_result = -1;
    if (!llvm_quoted_symbol_name_is_safe(target_symbol)) return plan->read_result = -1;
    if (!strcmp(definition_kind, "leaf_i64_function")) {
      if (!physical_definition_plan_add(plan, 1, target_symbol)) return plan->read_result = -1;
      planned++;
      continue;
    }
    if (!strcmp(definition_kind, "same_module_function")) {
      if (!physical_definition_plan_add(plan, 0, target_symbol)) return plan->read_result = -1;
      planned++;
      continue;
    }
    return plan->read_result = -1;
  }
  return plan->read_result = planned;
}
