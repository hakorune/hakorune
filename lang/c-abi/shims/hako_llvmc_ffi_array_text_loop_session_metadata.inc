      struct ArrayTextLoopSessionPlanMetadata {
        int matched;
        long long loop_header;
        long long loop_exit;
        long long array_value;
        long long index_value;
        long long len_call_count;
        int backend_session_lowering_allowed;
        int backend_consumer_enabled;
        int region_payload_present;
        long long region_array_root_value;
        long long region_loop_index_phi_value;
        long long region_loop_index_initial_value;
        long long region_loop_index_initial_const;
        long long region_loop_index_next_value;
        long long region_loop_bound_value;
        long long region_loop_bound_const;
        long long region_accumulator_phi_value;
        long long region_accumulator_initial_value;
        long long region_accumulator_initial_const;
        long long region_accumulator_next_value;
        long long region_exit_accumulator_value;
        long long region_row_index_value;
        long long region_row_modulus_value;
        long long region_row_modulus_const;
      };

      auto int array_text_loop_session_read_i64(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto int array_text_loop_session_read_bool(
          yyjson_val* obj,
          const char* key,
          int* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_bool(val))) return 0;
        if (out) *out = yyjson_get_bool(val) ? 1 : 0;
        return 1;
      }

      auto int array_text_loop_session_plan_valid(yyjson_val* plan) {
        const char* route_id = plan ? read_str(plan, "route_id") : NULL;
        long long loop_header = 0;
        long long loop_exit = 0;
        long long array_value = 0;
        long long index_value = 0;
        long long len_call_count = 0;
        int same_array_handle = 0;
        int read_only_region = 0;
        int no_mutation_region = 0;
        int no_drop_or_publication_boundary = 0;
        int index_domain_guarded = 0;
        int backend_session_lowering_allowed = 0;
        int mir_json_export_only = 0;
        int backend_consumer_enabled = 0;
        yyjson_val* first_reject_reason = plan ? yyjson_obj_get(plan, "first_reject_reason") : NULL;
        return plan && route_id && !strcmp(route_id, "array_text.loop_session.plan") &&
               array_text_loop_session_read_i64(plan, "loop_header", &loop_header) &&
               array_text_loop_session_read_i64(plan, "loop_exit", &loop_exit) &&
               array_text_loop_session_read_i64(plan, "array_value", &array_value) &&
               array_text_loop_session_read_i64(plan, "index_value", &index_value) &&
               array_text_loop_session_read_i64(plan, "len_call_count", &len_call_count) &&
               len_call_count > 0 &&
               array_text_loop_session_read_bool(plan, "same_array_handle", &same_array_handle) &&
               same_array_handle &&
               array_text_loop_session_read_bool(plan, "read_only_region", &read_only_region) &&
               read_only_region &&
               array_text_loop_session_read_bool(plan, "no_mutation_region", &no_mutation_region) &&
               no_mutation_region &&
               array_text_loop_session_read_bool(
                   plan,
                   "no_drop_or_publication_boundary",
                   &no_drop_or_publication_boundary) &&
               no_drop_or_publication_boundary &&
               array_text_loop_session_read_bool(
                   plan,
                   "index_domain_guarded",
                   &index_domain_guarded) &&
               index_domain_guarded &&
               array_text_loop_session_read_bool(
                   plan,
                   "backend_session_lowering_allowed",
                   &backend_session_lowering_allowed) &&
               backend_session_lowering_allowed &&
               first_reject_reason && yyjson_is_null(first_reject_reason) &&
               array_text_loop_session_read_bool(
                   plan,
                   "mir_json_export_only",
                   &mir_json_export_only) &&
               mir_json_export_only &&
               array_text_loop_session_read_bool(
                   plan,
                   "backend_consumer_enabled",
                   &backend_consumer_enabled);
      }

      auto int array_text_loop_session_region_payload_fill(
          yyjson_val* plan,
          struct ArrayTextLoopSessionPlanMetadata* out) {
        yyjson_val* payload = plan ? yyjson_obj_get(plan, "region_payload") : NULL;
        if (!(payload && yyjson_is_obj(payload) && out)) return 0;
        return array_text_loop_session_read_i64(
                   payload,
                   "array_root_value",
                   &out->region_array_root_value) &&
               array_text_loop_session_read_i64(
                   payload,
                   "loop_index_phi_value",
                   &out->region_loop_index_phi_value) &&
               array_text_loop_session_read_i64(
                   payload,
                   "loop_index_initial_value",
                   &out->region_loop_index_initial_value) &&
               array_text_loop_session_read_i64(
                   payload,
                   "loop_index_initial_const",
                   &out->region_loop_index_initial_const) &&
               array_text_loop_session_read_i64(
                   payload,
                   "loop_index_next_value",
                   &out->region_loop_index_next_value) &&
               array_text_loop_session_read_i64(
                   payload,
                   "loop_bound_value",
                   &out->region_loop_bound_value) &&
               array_text_loop_session_read_i64(
                   payload,
                   "loop_bound_const",
                   &out->region_loop_bound_const) &&
               array_text_loop_session_read_i64(
                   payload,
                   "accumulator_phi_value",
                   &out->region_accumulator_phi_value) &&
               array_text_loop_session_read_i64(
                   payload,
                   "accumulator_initial_value",
                   &out->region_accumulator_initial_value) &&
               array_text_loop_session_read_i64(
                   payload,
                   "accumulator_initial_const",
                   &out->region_accumulator_initial_const) &&
               array_text_loop_session_read_i64(
                   payload,
                   "accumulator_next_value",
                   &out->region_accumulator_next_value) &&
               array_text_loop_session_read_i64(
                   payload,
                   "exit_accumulator_value",
                   &out->region_exit_accumulator_value) &&
               array_text_loop_session_read_i64(
                   payload,
                   "row_index_value",
                   &out->region_row_index_value) &&
               array_text_loop_session_read_i64(
                   payload,
                   "row_modulus_value",
                   &out->region_row_modulus_value) &&
               array_text_loop_session_read_i64(
                   payload,
                   "row_modulus_const",
                   &out->region_row_modulus_const);
      }

      auto void array_text_loop_session_plan_fill(
          yyjson_val* plan,
          struct ArrayTextLoopSessionPlanMetadata* out) {
        long long tmp = 0;
        int flag = 0;
        if (!plan || !out) return;
        memset(out, 0, sizeof(*out));
        out->matched = 1;
        if (array_text_loop_session_read_i64(plan, "loop_header", &tmp)) out->loop_header = tmp;
        if (array_text_loop_session_read_i64(plan, "loop_exit", &tmp)) out->loop_exit = tmp;
        if (array_text_loop_session_read_i64(plan, "array_value", &tmp)) out->array_value = tmp;
        if (array_text_loop_session_read_i64(plan, "index_value", &tmp)) out->index_value = tmp;
        if (array_text_loop_session_read_i64(plan, "len_call_count", &tmp)) {
          out->len_call_count = tmp;
        }
        if (array_text_loop_session_read_bool(plan, "backend_session_lowering_allowed", &flag)) {
          out->backend_session_lowering_allowed = flag;
        }
        if (array_text_loop_session_read_bool(plan, "backend_consumer_enabled", &flag)) {
          out->backend_consumer_enabled = flag;
        }
        if (array_text_loop_session_region_payload_fill(plan, out)) {
          out->region_payload_present = 1;
        }
      }

      auto int match_array_text_loop_session_plan_by_header_metadata(
          long long loop_header,
          struct ArrayTextLoopSessionPlanMetadata* out) {
        yyjson_val* routes = fn_metadata ? yyjson_obj_get(fn_metadata, "array_text_loop_session_plans") : NULL;
        size_t idx = 0;
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (idx = 0; idx < yyjson_arr_size(routes); idx++) {
          yyjson_val* plan = yyjson_arr_get(routes, idx);
          long long plan_header = 0;
          if (!array_text_loop_session_plan_valid(plan)) continue;
          if (!array_text_loop_session_read_i64(plan, "loop_header", &plan_header)) continue;
          if (plan_header != loop_header) continue;
          if (out) array_text_loop_session_plan_fill(plan, out);
          return 1;
        }
        return 0;
      }
