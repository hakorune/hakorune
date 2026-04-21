      struct ArrayTextCombinedRegionRouteMetadata {
        int matched;
        long long begin_block;
        long long header_block;
        long long edit_block;
        long long observer_begin_block;
        long long observer_header_block;
        long long observer_block;
        long long observer_store_block;
        long long observer_latch_block;
        long long observer_exit_block;
        long long latch_block;
        long long exit_block;
        long long array_value;
        long long accumulator_phi_value;
        long long loop_bound_const;
        long long row_modulus_const;
        long long observer_period_const;
        long long observer_bound_const;
        long long middle_reg;
        long long middle_len;
        long long needle_reg;
        long long needle_len;
        long long suffix_reg;
        long long suffix_len;
      };

      auto int array_text_combined_region_read_i64(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto int array_text_combined_region_string_array_eq3(
          yyjson_val* vals,
          const char* first,
          const char* second,
          const char* third) {
        yyjson_val* v0 = NULL;
        yyjson_val* v1 = NULL;
        yyjson_val* v2 = NULL;
        const char* s0 = NULL;
        const char* s1 = NULL;
        const char* s2 = NULL;
        if (!(vals && yyjson_is_arr(vals) && yyjson_arr_size(vals) == 3)) return 0;
        v0 = yyjson_arr_get(vals, 0);
        v1 = yyjson_arr_get(vals, 1);
        v2 = yyjson_arr_get(vals, 2);
        if (!(v0 && v1 && v2 && yyjson_is_str(v0) && yyjson_is_str(v1) && yyjson_is_str(v2))) {
          return 0;
        }
        s0 = yyjson_get_str(v0);
        s1 = yyjson_get_str(v1);
        s2 = yyjson_get_str(v2);
        return s0 && s1 && s2 && !strcmp(s0, first) && !strcmp(s1, second) &&
               !strcmp(s2, third);
      }

      auto int array_text_combined_region_string_array_eq4(
          yyjson_val* vals,
          const char* first,
          const char* second,
          const char* third,
          const char* fourth) {
        yyjson_val* v0 = NULL;
        yyjson_val* v1 = NULL;
        yyjson_val* v2 = NULL;
        yyjson_val* v3 = NULL;
        const char* s0 = NULL;
        const char* s1 = NULL;
        const char* s2 = NULL;
        const char* s3 = NULL;
        if (!(vals && yyjson_is_arr(vals) && yyjson_arr_size(vals) == 4)) return 0;
        v0 = yyjson_arr_get(vals, 0);
        v1 = yyjson_arr_get(vals, 1);
        v2 = yyjson_arr_get(vals, 2);
        v3 = yyjson_arr_get(vals, 3);
        if (!(v0 && v1 && v2 && v3 && yyjson_is_str(v0) && yyjson_is_str(v1) &&
              yyjson_is_str(v2) && yyjson_is_str(v3))) {
          return 0;
        }
        s0 = yyjson_get_str(v0);
        s1 = yyjson_get_str(v1);
        s2 = yyjson_get_str(v2);
        s3 = yyjson_get_str(v3);
        return s0 && s1 && s2 && s3 && !strcmp(s0, first) && !strcmp(s1, second) &&
               !strcmp(s2, third) && !strcmp(s3, fourth);
      }

      auto int array_text_combined_region_route_valid(yyjson_val* route) {
        const char* execution_mode = route ? read_str(route, "execution_mode") : NULL;
        const char* proof_region = route ? read_str(route, "proof_region") : NULL;
        const char* proof = route ? read_str(route, "proof") : NULL;
        const char* publication_boundary =
            route ? read_str(route, "publication_boundary") : NULL;
        const char* carrier = route ? read_str(route, "carrier") : NULL;
        const char* materialization_policy =
            route ? read_str(route, "materialization_policy") : NULL;
        yyjson_val* effects = route ? yyjson_obj_get(route, "effects") : NULL;
        yyjson_val* capabilities =
            route ? yyjson_obj_get(route, "consumer_capabilities") : NULL;
        long long outer_index_phi_value = 0;
        long long accumulator_phi_value = 0;
        long long loop_bound_const = 0;
        long long row_modulus_const = 0;
        long long observer_period_const = 0;
        long long observer_bound_const = 0;
        long long edit_middle_len = 0;
        long long needle_len = 0;
        long long suffix_len = 0;
        return route && execution_mode && !strcmp(execution_mode, "single_region_executor") &&
               proof_region && !strcmp(proof_region, "outer_loop_with_periodic_observer_store") &&
               proof && !strcmp(proof, "outer_lenhalf_edit_with_periodic_observer_store") &&
               publication_boundary && !strcmp(publication_boundary, "none") &&
               carrier && !strcmp(carrier, "array_lane_text_cell") &&
               materialization_policy &&
                   !strcmp(materialization_policy, "text_resident_or_stringlike_slot") &&
               array_text_combined_region_string_array_eq4(
                   effects,
                   "store.cell(lenhalf_insert_mid_const)",
                   "observe.indexof",
                   "store.cell(const_suffix_append)",
                   "scalar_accumulator(+1)") &&
               array_text_combined_region_string_array_eq3(
                   capabilities,
                   "sink_store",
                   "compare_only",
                   "length_only_result_carry") &&
               array_text_combined_region_read_i64(
                   route,
                   "outer_index_phi_value",
                   &outer_index_phi_value) &&
               array_text_combined_region_read_i64(
                   route,
                   "accumulator_phi_value",
                   &accumulator_phi_value) &&
               outer_index_phi_value != accumulator_phi_value &&
               array_text_combined_region_read_i64(route, "loop_bound_const", &loop_bound_const) &&
               array_text_combined_region_read_i64(
                   route,
                   "row_modulus_const",
                   &row_modulus_const) &&
               array_text_combined_region_read_i64(
                   route,
                   "observer_period_const",
                   &observer_period_const) &&
               array_text_combined_region_read_i64(
                   route,
                   "observer_bound_const",
                   &observer_bound_const) &&
               array_text_combined_region_read_i64(
                   route,
                   "edit_middle_byte_len",
                   &edit_middle_len) &&
               array_text_combined_region_read_i64(
                   route,
                   "observer_needle_byte_len",
                   &needle_len) &&
               array_text_combined_region_read_i64(
                   route,
                   "observer_suffix_byte_len",
                   &suffix_len) &&
               loop_bound_const > 0 && row_modulus_const > 0 && observer_period_const > 0 &&
               observer_bound_const >= 0 && edit_middle_len >= 0 && needle_len >= 0 &&
               suffix_len >= 0;
      }

      auto void array_text_combined_region_route_fill(
          yyjson_val* route,
          struct ArrayTextCombinedRegionRouteMetadata* out) {
        long long tmp = 0;
        if (!route || !out) return;
        memset(out, 0, sizeof(*out));
        out->matched = 1;
        if (array_text_combined_region_read_i64(route, "begin_block", &tmp)) out->begin_block = tmp;
        if (array_text_combined_region_read_i64(route, "header_block", &tmp)) out->header_block = tmp;
        if (array_text_combined_region_read_i64(route, "edit_block", &tmp)) out->edit_block = tmp;
        if (array_text_combined_region_read_i64(route, "observer_begin_block", &tmp)) out->observer_begin_block = tmp;
        if (array_text_combined_region_read_i64(route, "observer_header_block", &tmp)) out->observer_header_block = tmp;
        if (array_text_combined_region_read_i64(route, "observer_block", &tmp)) out->observer_block = tmp;
        if (array_text_combined_region_read_i64(route, "observer_store_block", &tmp)) out->observer_store_block = tmp;
        if (array_text_combined_region_read_i64(route, "observer_latch_block", &tmp)) out->observer_latch_block = tmp;
        if (array_text_combined_region_read_i64(route, "observer_exit_block", &tmp)) out->observer_exit_block = tmp;
        if (array_text_combined_region_read_i64(route, "latch_block", &tmp)) out->latch_block = tmp;
        if (array_text_combined_region_read_i64(route, "exit_block", &tmp)) out->exit_block = tmp;
        if (array_text_combined_region_read_i64(route, "array_value", &tmp)) out->array_value = tmp;
        if (array_text_combined_region_read_i64(route, "accumulator_phi_value", &tmp)) out->accumulator_phi_value = tmp;
        if (array_text_combined_region_read_i64(route, "loop_bound_const", &tmp)) out->loop_bound_const = tmp;
        if (array_text_combined_region_read_i64(route, "row_modulus_const", &tmp)) out->row_modulus_const = tmp;
        if (array_text_combined_region_read_i64(route, "observer_period_const", &tmp)) out->observer_period_const = tmp;
        if (array_text_combined_region_read_i64(route, "observer_bound_const", &tmp)) out->observer_bound_const = tmp;
        if (array_text_combined_region_read_i64(route, "edit_middle_value", &tmp)) out->middle_reg = tmp;
        if (array_text_combined_region_read_i64(route, "edit_middle_byte_len", &tmp)) out->middle_len = tmp;
        if (array_text_combined_region_read_i64(route, "observer_needle_value", &tmp)) out->needle_reg = tmp;
        if (array_text_combined_region_read_i64(route, "observer_needle_byte_len", &tmp)) out->needle_len = tmp;
        if (array_text_combined_region_read_i64(route, "observer_suffix_value", &tmp)) out->suffix_reg = tmp;
        if (array_text_combined_region_read_i64(route, "observer_suffix_byte_len", &tmp)) out->suffix_len = tmp;
      }

      auto int array_text_combined_region_has_route(void) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "array_text_combined_regions");
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          if (array_text_combined_region_route_valid(route)) return 1;
        }
        return 0;
      }

      auto int match_array_text_combined_region_metadata_any(
          struct ArrayTextCombinedRegionRouteMetadata* out) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "array_text_combined_regions");
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          if (!array_text_combined_region_route_valid(route)) continue;
          if (out) array_text_combined_region_route_fill(route, out);
          return 1;
        }
        return 0;
      }
