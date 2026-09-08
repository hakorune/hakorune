/* One borrowed document-root lookup for layout readers and Named selection. */
static yyjson_val* typed_object_plans_array(yyjson_val* root) {
  yyjson_val* plans = root ? yyjson_obj_get(root, "typed_object_plans") : NULL;
  return (plans && yyjson_is_arr(plans)) ? plans : NULL;
}

static yyjson_val* typed_object_plan_at_index(yyjson_val* root, int plan_index) {
  yyjson_val* plans = typed_object_plans_array(root);
  if (!plans || plan_index < 0) return NULL;
  return yyjson_arr_get(plans, (size_t)plan_index);
}

static size_t typed_object_plan_count(yyjson_val* root) {
  yyjson_val* plans = typed_object_plans_array(root);
  return plans ? yyjson_arr_size(plans) : 0;
}

static int find_typed_object_plan_index(yyjson_val* root, const char* box_name, int* out_plan_index) {
  yyjson_val* plans = typed_object_plans_array(root);
  size_t count = typed_object_plan_count(root);
  if (!(box_name && plans)) return 0;
  for (size_t i = 0; i < count; i++) {
    yyjson_val* plan = yyjson_arr_get(plans, i);
    const char* plan_box = plan ? yyjson_get_str(yyjson_obj_get(plan, "box_name")) : NULL;
    if (plan_box && strcmp(plan_box, box_name) == 0) {
      if (out_plan_index) *out_plan_index = (int)i;
      return 1;
    }
  }
  return 0;
}

static long long typed_object_plan_type_id(yyjson_val* root, int plan_index) {
  yyjson_val* plan = typed_object_plan_at_index(root, plan_index);
  return plan ? (long long)yyjson_get_sint(yyjson_obj_get(plan, "type_id")) : 0;
}

static long long typed_object_plan_field_count(yyjson_val* root, int plan_index) {
  yyjson_val* plan = typed_object_plan_at_index(root, plan_index);
  return plan ? (long long)yyjson_get_sint(yyjson_obj_get(plan, "field_count")) : 0;
}
