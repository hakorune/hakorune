      #ifndef HAKO_LLVMC_STRING_CANDIDATE_PLAN_READERS_INC
      #define HAKO_LLVMC_STRING_CANDIDATE_PLAN_READERS_INC

      auto yyjson_val* hako_llvmc_string_corridor_value_candidates(
          long long value) {
        char key[32];
        yyjson_val* candidates = NULL;
        if (!(fn_metadata && value)) return NULL;
        value = resolve_copy_source_reg(value);
        snprintf(key, sizeof(key), "%lld", value);
        candidates = yyjson_obj_get(fn_metadata, "string_corridor_candidates");
        if (!(candidates && yyjson_is_obj(candidates))) return NULL;
        candidates = yyjson_obj_get(candidates, key);
        if (!(candidates && yyjson_is_arr(candidates))) return NULL;
        return candidates;
      }

      auto yyjson_val* hako_llvmc_string_corridor_candidate_plan_by_kind(
          long long value,
          const char* kind) {
        yyjson_val* candidates = hako_llvmc_string_corridor_value_candidates(value);
        size_t count = candidates ? yyjson_arr_size(candidates) : 0;
        if (!(candidates && kind)) return NULL;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* candidate = yyjson_arr_get(candidates, i);
          const char* candidate_kind =
              candidate ? yyjson_get_str(yyjson_obj_get(candidate, "kind")) : NULL;
          yyjson_val* plan = candidate ? yyjson_obj_get(candidate, "plan") : NULL;
          if (candidate_kind && !strcmp(candidate_kind, kind) && plan &&
              yyjson_is_obj(plan)) {
            return plan;
          }
        }
        return NULL;
      }

      auto int hako_llvmc_string_corridor_read_plan_window(
          long long value,
          const char* kind,
          long long* start,
          long long* end) {
        yyjson_val* plan =
            hako_llvmc_string_corridor_candidate_plan_by_kind(value, kind);
        yyjson_val* start_val = plan ? yyjson_obj_get(plan, "start") : NULL;
        yyjson_val* end_val = plan ? yyjson_obj_get(plan, "end") : NULL;
        if (!(start_val && end_val && yyjson_is_num(start_val) && yyjson_is_num(end_val))) {
          return 0;
        }
        if (start) *start = (long long)yyjson_get_sint(start_val);
        if (end) *end = (long long)yyjson_get_sint(end_val);
        return 1;
      }

      auto int hako_llvmc_string_corridor_read_plan_source_window(
          long long value,
          const char* kind,
          long long* source_root,
          long long* start,
          long long* end) {
        yyjson_val* plan =
            hako_llvmc_string_corridor_candidate_plan_by_kind(value, kind);
        yyjson_val* source_val = plan ? yyjson_obj_get(plan, "source_root") : NULL;
        yyjson_val* start_val = plan ? yyjson_obj_get(plan, "start") : NULL;
        yyjson_val* end_val = plan ? yyjson_obj_get(plan, "end") : NULL;
        if (!(source_val && start_val && end_val && yyjson_is_num(source_val) &&
              yyjson_is_num(start_val) && yyjson_is_num(end_val))) {
          return 0;
        }
        if (source_root) *source_root = (long long)yyjson_get_sint(source_val);
        if (start) *start = (long long)yyjson_get_sint(start_val);
        if (end) *end = (long long)yyjson_get_sint(end_val);
        return 1;
      }

      auto int hako_llvmc_string_corridor_read_concat_triplet_plan_values(
          long long value,
          const char* kind,
          long long* left_value,
          long long* middle,
          long long* right_value) {
        yyjson_val* plan =
            hako_llvmc_string_corridor_candidate_plan_by_kind(value, kind);
        yyjson_val* proof = plan ? yyjson_obj_get(plan, "proof") : NULL;
        yyjson_val* proof_kind = proof ? yyjson_obj_get(proof, "kind") : NULL;
        yyjson_val* left_val = proof ? yyjson_obj_get(proof, "left_value") : NULL;
        yyjson_val* middle_val = proof ? yyjson_obj_get(proof, "middle") : NULL;
        yyjson_val* right_val = proof ? yyjson_obj_get(proof, "right_value") : NULL;
        if (!(proof && yyjson_is_obj(proof) && proof_kind && yyjson_is_str(proof_kind) &&
              !strcmp(yyjson_get_str(proof_kind), "concat_triplet") && left_val &&
              middle_val && right_val && yyjson_is_num(left_val) &&
              yyjson_is_num(middle_val) && yyjson_is_num(right_val))) {
          return 0;
        }
        if (left_value) *left_value = (long long)yyjson_get_sint(left_val);
        if (middle) *middle = (long long)yyjson_get_sint(middle_val);
        if (right_value) *right_value = (long long)yyjson_get_sint(right_val);
        return 1;
      }

      auto int hako_llvmc_string_corridor_read_any_plan_source_window(
          long long value,
          long long* source_root,
          long long* start,
          long long* end) {
        return hako_llvmc_string_corridor_read_plan_source_window(
                   value,
                   "direct_kernel_entry",
                   source_root,
                   start,
                   end) ||
               hako_llvmc_string_corridor_read_plan_source_window(
                   value,
                   "publication_sink",
                   source_root,
                   start,
                   end) ||
               hako_llvmc_string_corridor_read_plan_source_window(
                   value,
                   "materialization_sink",
                   source_root,
                   start,
                   end) ||
               hako_llvmc_string_corridor_read_plan_source_window(
                   value,
                   "borrowed_corridor_fusion",
                   source_root,
                   start,
                   end);
      }

      #endif
