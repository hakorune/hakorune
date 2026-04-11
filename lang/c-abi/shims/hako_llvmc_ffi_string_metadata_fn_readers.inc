static int hako_llvmc_string_corridor_read_plan_source_window_fn(
    yyjson_val* fn,
    long long value,
    const char* kind,
    long long* source_root,
    long long* start,
    long long* end) {
  yyjson_val* metadata;
  yyjson_val* candidates;
  yyjson_val* arr;
  size_t i;
  char key[32];

  if (!(fn && kind && value > 0)) return 0;

  metadata = yyjson_obj_get(fn, "metadata");
  candidates = metadata ? yyjson_obj_get(metadata, "string_corridor_candidates") : NULL;
  if (!(candidates && yyjson_is_obj(candidates))) return 0;

  snprintf(key, sizeof(key), "%lld", value);
  arr = yyjson_obj_get(candidates, key);
  if (!(arr && yyjson_is_arr(arr))) return 0;

  for (i = 0; i < yyjson_arr_size(arr); i++) {
    yyjson_val* candidate = yyjson_arr_get(arr, i);
    yyjson_val* candidate_kind = candidate ? yyjson_obj_get(candidate, "kind") : NULL;
    yyjson_val* plan = candidate ? yyjson_obj_get(candidate, "plan") : NULL;
    yyjson_val* source_val = plan ? yyjson_obj_get(plan, "source_root") : NULL;
    yyjson_val* start_val = plan ? yyjson_obj_get(plan, "start") : NULL;
    yyjson_val* end_val = plan ? yyjson_obj_get(plan, "end") : NULL;

    if (!(candidate_kind && yyjson_is_str(candidate_kind) &&
          strcmp(yyjson_get_str(candidate_kind), kind) == 0)) {
      continue;
    }
    if (!(plan && yyjson_is_obj(plan) && source_val && start_val && end_val &&
          yyjson_is_num(source_val) && yyjson_is_num(start_val) &&
          yyjson_is_num(end_val))) {
      return 0;
    }
    if (source_root) *source_root = (long long)yyjson_get_sint(source_val);
    if (start) *start = (long long)yyjson_get_sint(start_val);
    if (end) *end = (long long)yyjson_get_sint(end_val);
    return 1;
  }

  return 0;
}

static int hako_llvmc_string_corridor_read_relation_details_fn(
    yyjson_val* fn,
    long long value,
    const char* kind,
    long long* base_value,
    long long* witness_value,
    const char** window_contract) {
  yyjson_val* metadata;
  yyjson_val* relations;
  yyjson_val* arr;
  size_t i;
  char key[32];

  if (!(fn && kind && value > 0)) return 0;

  metadata = yyjson_obj_get(fn, "metadata");
  relations = metadata ? yyjson_obj_get(metadata, "string_corridor_relations") : NULL;
  if (!(relations && yyjson_is_obj(relations))) return 0;

  snprintf(key, sizeof(key), "%lld", value);
  arr = yyjson_obj_get(relations, key);
  if (!(arr && yyjson_is_arr(arr))) return 0;

  for (i = 0; i < yyjson_arr_size(arr); i++) {
    yyjson_val* relation = yyjson_arr_get(arr, i);
    yyjson_val* kind_val = relation ? yyjson_obj_get(relation, "kind") : NULL;
    yyjson_val* base_val = relation ? yyjson_obj_get(relation, "base_value") : NULL;
    yyjson_val* witness_val = relation ? yyjson_obj_get(relation, "witness_value") : NULL;
    yyjson_val* window_val = relation ? yyjson_obj_get(relation, "window_contract") : NULL;

    if (!(kind_val && yyjson_is_str(kind_val) &&
          strcmp(yyjson_get_str(kind_val), kind) == 0)) {
      continue;
    }
    if (base_value) {
      *base_value =
          (base_val && yyjson_is_num(base_val))
              ? (long long)yyjson_get_sint(base_val)
              : 0;
    }
    if (witness_value) {
      *witness_value =
          (witness_val && yyjson_is_num(witness_val))
              ? (long long)yyjson_get_sint(witness_val)
              : 0;
    }
    if (window_contract) {
      *window_contract =
          (window_val && yyjson_is_str(window_val)) ? yyjson_get_str(window_val)
                                                    : NULL;
    }
    return 1;
  }

  return 0;
}

static yyjson_val* hako_llvmc_string_kernel_plan_by_corridor_root_fn(
    yyjson_val* fn,
    long long corridor_root) {
  yyjson_val* metadata;
  yyjson_val* plans;
  char key[32];

  if (!(fn && corridor_root > 0)) return NULL;
  metadata = yyjson_obj_get(fn, "metadata");
  plans = metadata ? yyjson_obj_get(metadata, "string_kernel_plans") : NULL;
  if (!(plans && yyjson_is_obj(plans))) return NULL;

  snprintf(key, sizeof(key), "%lld", corridor_root);
  plans = yyjson_obj_get(plans, key);
  if (!(plans && yyjson_is_obj(plans))) return NULL;
  return plans;
}

static int hako_llvmc_string_kernel_plan_read_concat_triplet_len_fn(
    yyjson_val* fn,
    long long corridor_root,
    long long* source_root,
    long long* middle_value,
    long long* middle_known_length) {
  yyjson_val* plan = hako_llvmc_string_kernel_plan_by_corridor_root_fn(fn, corridor_root);
  yyjson_val* parts = plan ? yyjson_obj_get(plan, "parts") : NULL;
  yyjson_val* legality = plan ? yyjson_obj_get(plan, "legality") : NULL;
  yyjson_val* first_part = parts ? yyjson_arr_get(parts, 0) : NULL;
  yyjson_val* middle_part = parts ? yyjson_arr_get(parts, 1) : NULL;
  yyjson_val* last_part = parts ? yyjson_arr_get(parts, 2) : NULL;
  yyjson_val* source_root_val = plan ? yyjson_obj_get(plan, "source_root") : NULL;
  yyjson_val* family_val = plan ? yyjson_obj_get(plan, "family") : NULL;
  yyjson_val* consumer_val = plan ? yyjson_obj_get(plan, "consumer") : NULL;
  yyjson_val* retained_form_val = plan ? yyjson_obj_get(plan, "retained_form") : NULL;
  yyjson_val* byte_exact_val = legality ? yyjson_obj_get(legality, "byte_exact") : NULL;
  yyjson_val* first_kind = first_part ? yyjson_obj_get(first_part, "kind") : NULL;
  yyjson_val* middle_kind = middle_part ? yyjson_obj_get(middle_part, "kind") : NULL;
  yyjson_val* last_kind = last_part ? yyjson_obj_get(last_part, "kind") : NULL;
  yyjson_val* first_source = first_part ? yyjson_obj_get(first_part, "source") : NULL;
  yyjson_val* last_source = last_part ? yyjson_obj_get(last_part, "source") : NULL;
  yyjson_val* middle_value_val = middle_part ? yyjson_obj_get(middle_part, "value") : NULL;
  yyjson_val* middle_len_val =
      middle_part ? yyjson_obj_get(middle_part, "known_length") : NULL;

  if (!(plan && parts && yyjson_is_arr(parts) && yyjson_arr_size(parts) == 3 &&
        source_root_val && yyjson_is_num(source_root_val) &&
        family_val && yyjson_is_str(family_val) &&
        consumer_val && yyjson_is_str(consumer_val) &&
        retained_form_val && yyjson_is_str(retained_form_val) &&
        byte_exact_val && yyjson_is_bool(byte_exact_val) &&
        yyjson_get_bool(byte_exact_val) &&
        first_kind && yyjson_is_str(first_kind) &&
        middle_kind && yyjson_is_str(middle_kind) &&
        last_kind && yyjson_is_str(last_kind) &&
        !strcmp(yyjson_get_str(family_val), "concat_triplet_window") &&
        !strcmp(yyjson_get_str(consumer_val), "direct_kernel_entry") &&
        !strcmp(yyjson_get_str(retained_form_val), "borrowed_text") &&
        !strcmp(yyjson_get_str(first_kind), "slice") &&
        !strcmp(yyjson_get_str(middle_kind), "const") &&
        !strcmp(yyjson_get_str(last_kind), "slice") &&
        first_source && yyjson_is_num(first_source) &&
        last_source && yyjson_is_num(last_source) &&
        middle_value_val && yyjson_is_num(middle_value_val) &&
        middle_len_val && yyjson_is_num(middle_len_val))) {
    return 0;
  }

  if ((long long)yyjson_get_sint(first_source) !=
      (long long)yyjson_get_sint(source_root_val)) {
    return 0;
  }
  if ((long long)yyjson_get_sint(last_source) !=
      (long long)yyjson_get_sint(source_root_val)) {
    return 0;
  }

  if (source_root) *source_root = (long long)yyjson_get_sint(source_root_val);
  if (middle_value) *middle_value = (long long)yyjson_get_sint(middle_value_val);
  if (middle_known_length) {
    *middle_known_length = (long long)yyjson_get_sint(middle_len_val);
  }
  return 1;
}

static int hako_llvmc_string_kernel_plan_read_concat_triplet_loop_fn(
    yyjson_val* fn,
    long long corridor_root,
    long long* source_root,
    const char** seed_literal,
    long long* seed_length,
    const char** middle_literal,
    long long* middle_known_length,
    long long* loop_bound,
    long long* split_length) {
  yyjson_val* plan = hako_llvmc_string_kernel_plan_by_corridor_root_fn(fn, corridor_root);
  yyjson_val* parts = plan ? yyjson_obj_get(plan, "parts") : NULL;
  yyjson_val* legality = plan ? yyjson_obj_get(plan, "legality") : NULL;
  yyjson_val* loop_payload = plan ? yyjson_obj_get(plan, "loop_payload") : NULL;
  yyjson_val* first_part = parts ? yyjson_arr_get(parts, 0) : NULL;
  yyjson_val* middle_part = parts ? yyjson_arr_get(parts, 1) : NULL;
  yyjson_val* last_part = parts ? yyjson_arr_get(parts, 2) : NULL;
  yyjson_val* source_root_val = plan ? yyjson_obj_get(plan, "source_root") : NULL;
  yyjson_val* family_val = plan ? yyjson_obj_get(plan, "family") : NULL;
  yyjson_val* consumer_val = plan ? yyjson_obj_get(plan, "consumer") : NULL;
  yyjson_val* retained_form_val = plan ? yyjson_obj_get(plan, "retained_form") : NULL;
  yyjson_val* byte_exact_val = legality ? yyjson_obj_get(legality, "byte_exact") : NULL;
  yyjson_val* first_kind = first_part ? yyjson_obj_get(first_part, "kind") : NULL;
  yyjson_val* middle_kind = middle_part ? yyjson_obj_get(middle_part, "kind") : NULL;
  yyjson_val* last_kind = last_part ? yyjson_obj_get(last_part, "kind") : NULL;
  yyjson_val* first_source = first_part ? yyjson_obj_get(first_part, "source") : NULL;
  yyjson_val* last_source = last_part ? yyjson_obj_get(last_part, "source") : NULL;
  yyjson_val* middle_literal_val =
      middle_part ? yyjson_obj_get(middle_part, "literal") : NULL;
  yyjson_val* middle_len_val =
      middle_part ? yyjson_obj_get(middle_part, "known_length") : NULL;
  yyjson_val* seed_literal_val =
      loop_payload ? yyjson_obj_get(loop_payload, "seed_literal") : NULL;
  yyjson_val* seed_length_val =
      loop_payload ? yyjson_obj_get(loop_payload, "seed_length") : NULL;
  yyjson_val* loop_bound_val =
      loop_payload ? yyjson_obj_get(loop_payload, "loop_bound") : NULL;
  yyjson_val* split_length_val =
      loop_payload ? yyjson_obj_get(loop_payload, "split_length") : NULL;

  if (!(plan && parts && yyjson_is_arr(parts) && yyjson_arr_size(parts) == 3 &&
        loop_payload && yyjson_is_obj(loop_payload) && source_root_val &&
        yyjson_is_num(source_root_val) && family_val && yyjson_is_str(family_val) &&
        consumer_val && yyjson_is_str(consumer_val) && retained_form_val &&
        yyjson_is_str(retained_form_val) && byte_exact_val &&
        yyjson_is_bool(byte_exact_val) && yyjson_get_bool(byte_exact_val) &&
        first_kind && yyjson_is_str(first_kind) && middle_kind &&
        yyjson_is_str(middle_kind) && last_kind && yyjson_is_str(last_kind) &&
        !strcmp(yyjson_get_str(family_val), "concat_triplet_window") &&
        !strcmp(yyjson_get_str(consumer_val), "direct_kernel_entry") &&
        !strcmp(yyjson_get_str(retained_form_val), "borrowed_text") &&
        !strcmp(yyjson_get_str(first_kind), "slice") &&
        !strcmp(yyjson_get_str(middle_kind), "const") &&
        !strcmp(yyjson_get_str(last_kind), "slice") && first_source &&
        yyjson_is_num(first_source) && last_source && yyjson_is_num(last_source) &&
        middle_literal_val && yyjson_is_str(middle_literal_val) && middle_len_val &&
        yyjson_is_num(middle_len_val) && seed_literal_val &&
        yyjson_is_str(seed_literal_val) && seed_length_val &&
        yyjson_is_num(seed_length_val) && loop_bound_val &&
        yyjson_is_num(loop_bound_val) && split_length_val &&
        yyjson_is_num(split_length_val))) {
    return 0;
  }

  if ((long long)yyjson_get_sint(first_source) !=
      (long long)yyjson_get_sint(source_root_val)) {
    return 0;
  }
  if ((long long)yyjson_get_sint(last_source) !=
      (long long)yyjson_get_sint(source_root_val)) {
    return 0;
  }

  if (source_root) *source_root = (long long)yyjson_get_sint(source_root_val);
  if (seed_literal) *seed_literal = yyjson_get_str(seed_literal_val);
  if (seed_length) *seed_length = (long long)yyjson_get_sint(seed_length_val);
  if (middle_literal) *middle_literal = yyjson_get_str(middle_literal_val);
  if (middle_known_length) {
    *middle_known_length = (long long)yyjson_get_sint(middle_len_val);
  }
  if (loop_bound) *loop_bound = (long long)yyjson_get_sint(loop_bound_val);
  if (split_length) *split_length = (long long)yyjson_get_sint(split_length_val);
  return 1;
}
