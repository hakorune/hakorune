      struct StringDeadTextRegionPlanMetadata {
        int matched;
        long long loop_header;
        long long loop_body;
        long long loop_exit;
        long long text_phi_value;
        long long text_initial_value;
        long long loop_index_phi_value;
        long long loop_index_initial_value;
        long long loop_index_initial_const;
        long long loop_index_next_value;
        long long loop_bound_value;
        long long loop_bound_const;
        long long base_len_value;
        long long base_len_const;
        long long inserted_text_value;
        const char* inserted_text;
        long long inserted_len_const;
        long long accumulator_phi_value;
        long long accumulator_initial_value;
        long long accumulator_initial_const;
        long long accumulator_next_value;
        long long accumulator_delta_value;
        long long accumulator_delta_const;
        long long exit_accumulator_value;
        long long final_len_value;
        long long final_return_value;
        long long substring_left_value;
        long long substring_right_value;
        long long concat_result_value;
        long long rotation_start_value;
        long long rotation_end_value;
        long long split_value;
        const char* publication_boundary;
        int final_text_content_observed;
        int backend_consumer_enabled;
      };

      auto int string_dead_text_region_read_i64(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto int string_dead_text_region_read_bool(
          yyjson_val* obj,
          const char* key,
          int* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_bool(val))) return 0;
        if (out) *out = yyjson_get_bool(val) ? 1 : 0;
        return 1;
      }

      auto int string_dead_text_region_plan_valid(yyjson_val* plan) {
        const char* route_id = plan ? read_str(plan, "route_id") : NULL;
        const char* inserted_text = plan ? read_str(plan, "inserted_text") : NULL;
        const char* publication_boundary =
            plan ? read_str(plan, "publication_boundary") : NULL;
        long long loop_header = 0;
        long long loop_body = 0;
        long long loop_exit = 0;
        long long text_phi_value = 0;
        long long text_initial_value = 0;
        long long loop_index_phi_value = 0;
        long long loop_index_initial_value = 0;
        long long loop_index_initial_const = 0;
        long long loop_index_next_value = 0;
        long long loop_bound_value = 0;
        long long loop_bound_const = 0;
        long long base_len_value = 0;
        long long base_len_const = 0;
        long long inserted_text_value = 0;
        long long inserted_len_const = 0;
        long long accumulator_phi_value = 0;
        long long accumulator_initial_value = 0;
        long long accumulator_initial_const = 0;
        long long accumulator_next_value = 0;
        long long accumulator_delta_value = 0;
        long long accumulator_delta_const = 0;
        long long exit_accumulator_value = 0;
        long long final_len_value = 0;
        long long final_return_value = 0;
        long long substring_left_value = 0;
        long long substring_right_value = 0;
        long long concat_result_value = 0;
        long long rotation_start_value = 0;
        long long rotation_end_value = 0;
        long long split_value = 0;
        int final_text_content_observed = 1;
        int mir_json_export_only = 0;
        int backend_consumer_enabled = 0;
        long long expected_delta = 0;
        long long expected_final_return = 0;
        return plan &&
               route_id &&
               !strcmp(route_id, "string.dead_text_region.plan") &&
               string_dead_text_region_read_i64(plan, "loop_header", &loop_header) &&
               string_dead_text_region_read_i64(plan, "loop_body", &loop_body) &&
               string_dead_text_region_read_i64(plan, "loop_exit", &loop_exit) &&
               string_dead_text_region_read_i64(plan, "text_phi_value", &text_phi_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "text_initial_value",
                   &text_initial_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "loop_index_phi_value",
                   &loop_index_phi_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "loop_index_initial_value",
                   &loop_index_initial_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "loop_index_initial_const",
                   &loop_index_initial_const) &&
               loop_index_initial_const == 0 &&
               string_dead_text_region_read_i64(
                   plan,
                   "loop_index_next_value",
                   &loop_index_next_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "loop_bound_value",
                   &loop_bound_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "loop_bound_const",
                   &loop_bound_const) &&
               loop_bound_const >= 0 &&
               string_dead_text_region_read_i64(plan, "base_len_value", &base_len_value) &&
               string_dead_text_region_read_i64(plan, "base_len_const", &base_len_const) &&
               base_len_const > 0 &&
               string_dead_text_region_read_i64(
                   plan,
                   "inserted_text_value",
                   &inserted_text_value) &&
               inserted_text &&
               string_dead_text_region_read_i64(
                   plan,
                   "inserted_len_const",
                   &inserted_len_const) &&
               inserted_len_const >= 0 &&
               (long long)strlen(inserted_text) == inserted_len_const &&
               string_dead_text_region_read_i64(
                   plan,
                   "accumulator_phi_value",
                   &accumulator_phi_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "accumulator_initial_value",
                   &accumulator_initial_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "accumulator_initial_const",
                   &accumulator_initial_const) &&
               accumulator_initial_const == 0 &&
               string_dead_text_region_read_i64(
                   plan,
                   "accumulator_next_value",
                   &accumulator_next_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "accumulator_delta_value",
                   &accumulator_delta_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "accumulator_delta_const",
                   &accumulator_delta_const) &&
               string_dead_text_region_read_i64(
                   plan,
                   "exit_accumulator_value",
                   &exit_accumulator_value) &&
               string_dead_text_region_read_i64(plan, "final_len_value", &final_len_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "final_return_value",
                   &final_return_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "substring_left_value",
                   &substring_left_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "substring_right_value",
                   &substring_right_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "concat_result_value",
                   &concat_result_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "rotation_start_value",
                   &rotation_start_value) &&
               string_dead_text_region_read_i64(
                   plan,
                   "rotation_end_value",
                   &rotation_end_value) &&
               string_dead_text_region_read_i64(plan, "split_value", &split_value) &&
               publication_boundary &&
               !strcmp(publication_boundary, "none") &&
               string_dead_text_region_read_bool(
                   plan,
                   "final_text_content_observed",
                   &final_text_content_observed) &&
               !final_text_content_observed &&
               string_dead_text_region_read_bool(
                   plan,
                   "mir_json_export_only",
                   &mir_json_export_only) &&
               mir_json_export_only &&
               string_dead_text_region_read_bool(
                   plan,
                   "backend_consumer_enabled",
                   &backend_consumer_enabled) &&
               (expected_delta = base_len_const + inserted_len_const) &&
               accumulator_delta_const == expected_delta &&
               (expected_final_return =
                    accumulator_initial_const +
                    loop_bound_const * accumulator_delta_const +
                    base_len_const) >= 0 &&
               final_return_value == expected_final_return;
      }

      auto void string_dead_text_region_plan_fill(
          yyjson_val* plan,
          struct StringDeadTextRegionPlanMetadata* out) {
        long long tmp = 0;
        int flag = 0;
        if (!plan || !out) return;
        memset(out, 0, sizeof(*out));
        out->matched = 1;
        if (string_dead_text_region_read_i64(plan, "loop_header", &tmp)) out->loop_header = tmp;
        if (string_dead_text_region_read_i64(plan, "loop_body", &tmp)) out->loop_body = tmp;
        if (string_dead_text_region_read_i64(plan, "loop_exit", &tmp)) out->loop_exit = tmp;
        if (string_dead_text_region_read_i64(plan, "text_phi_value", &tmp)) {
          out->text_phi_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "text_initial_value", &tmp)) {
          out->text_initial_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "loop_index_phi_value", &tmp)) {
          out->loop_index_phi_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "loop_index_initial_value", &tmp)) {
          out->loop_index_initial_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "loop_index_initial_const", &tmp)) {
          out->loop_index_initial_const = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "loop_index_next_value", &tmp)) {
          out->loop_index_next_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "loop_bound_value", &tmp)) {
          out->loop_bound_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "loop_bound_const", &tmp)) {
          out->loop_bound_const = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "base_len_value", &tmp)) {
          out->base_len_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "base_len_const", &tmp)) {
          out->base_len_const = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "inserted_text_value", &tmp)) {
          out->inserted_text_value = tmp;
        }
        out->inserted_text = read_str(plan, "inserted_text");
        if (string_dead_text_region_read_i64(plan, "inserted_len_const", &tmp)) {
          out->inserted_len_const = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "accumulator_phi_value", &tmp)) {
          out->accumulator_phi_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "accumulator_initial_value", &tmp)) {
          out->accumulator_initial_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "accumulator_initial_const", &tmp)) {
          out->accumulator_initial_const = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "accumulator_next_value", &tmp)) {
          out->accumulator_next_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "accumulator_delta_value", &tmp)) {
          out->accumulator_delta_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "accumulator_delta_const", &tmp)) {
          out->accumulator_delta_const = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "exit_accumulator_value", &tmp)) {
          out->exit_accumulator_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "final_len_value", &tmp)) {
          out->final_len_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "final_return_value", &tmp)) {
          out->final_return_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "substring_left_value", &tmp)) {
          out->substring_left_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "substring_right_value", &tmp)) {
          out->substring_right_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "concat_result_value", &tmp)) {
          out->concat_result_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "rotation_start_value", &tmp)) {
          out->rotation_start_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "rotation_end_value", &tmp)) {
          out->rotation_end_value = tmp;
        }
        if (string_dead_text_region_read_i64(plan, "split_value", &tmp)) {
          out->split_value = tmp;
        }
        out->publication_boundary = read_str(plan, "publication_boundary");
        if (string_dead_text_region_read_bool(plan, "final_text_content_observed", &flag)) {
          out->final_text_content_observed = flag;
        }
        if (string_dead_text_region_read_bool(plan, "backend_consumer_enabled", &flag)) {
          out->backend_consumer_enabled = flag;
        }
      }

      auto int match_string_dead_text_region_plan_by_header_metadata(
          long long loop_header,
          struct StringDeadTextRegionPlanMetadata* out) {
        yyjson_val* routes =
            fn_metadata ? yyjson_obj_get(fn_metadata, "string_dead_text_region_plans") : NULL;
        size_t idx = 0;
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (idx = 0; idx < yyjson_arr_size(routes); idx++) {
          yyjson_val* plan = yyjson_arr_get(routes, idx);
          long long plan_header = 0;
          if (!string_dead_text_region_plan_valid(plan)) continue;
          if (!string_dead_text_region_read_i64(plan, "loop_header", &plan_header)) continue;
          if (plan_header != loop_header) continue;
          if (out) string_dead_text_region_plan_fill(plan, out);
          return 1;
        }
        return 0;
      }

      auto int string_dead_text_region_has_plan(void) {
        yyjson_val* routes =
            fn_metadata ? yyjson_obj_get(fn_metadata, "string_dead_text_region_plans") : NULL;
        size_t idx = 0;
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (idx = 0; idx < yyjson_arr_size(routes); idx++) {
          yyjson_val* plan = yyjson_arr_get(routes, idx);
          if (string_dead_text_region_plan_valid(plan)) return 1;
        }
        return 0;
      }
