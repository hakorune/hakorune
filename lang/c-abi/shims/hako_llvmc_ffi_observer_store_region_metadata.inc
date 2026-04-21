      struct ArrayTextObserverStoreRegionRouteMetadata {
        int matched;
        long long begin_block;
        long long begin_to_header_block;
        long long header_block;
        long long observer_block;
        long long then_store_block;
        long long latch_block;
        long long exit_block;
        long long array_root_value;
        long long loop_bound_const;
        long long needle_reg;
        long long needle_len;
        long long suffix_reg;
        long long suffix_len;
      };

      auto int array_text_observer_region_read_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto int array_text_observer_region_string_array_eq2(
          yyjson_val* vals,
          const char* first,
          const char* second) {
        yyjson_val* v0 = NULL;
        yyjson_val* v1 = NULL;
        const char* s0 = NULL;
        const char* s1 = NULL;
        if (!(vals && yyjson_is_arr(vals) && yyjson_arr_size(vals) == 2)) return 0;
        v0 = yyjson_arr_get(vals, 0);
        v1 = yyjson_arr_get(vals, 1);
        if (!(v0 && v1 && yyjson_is_str(v0) && yyjson_is_str(v1))) return 0;
        s0 = yyjson_get_str(v0);
        s1 = yyjson_get_str(v1);
        return s0 && s1 && !strcmp(s0, first) && !strcmp(s1, second);
      }

      auto int array_text_observer_store_region_mapping_valid(
          yyjson_val* route,
          yyjson_val* contract) {
        yyjson_val* mapping = contract ? yyjson_obj_get(contract, "region_mapping") : NULL;
        long long route_block = 0;
        long long route_instruction_index = 0;
        long long route_array_value = 0;
        long long observer_arg0 = 0;
        long long observer_arg0_byte_len = 0;
        long long begin_block = 0;
        long long begin_to_header_block = 0;
        long long header_block = 0;
        long long observer_block = 0;
        long long observer_instruction_index = 0;
        long long then_store_block = 0;
        long long store_instruction_index = 0;
        long long latch_block = 0;
        long long exit_block = 0;
        long long array_root_value = 0;
        long long loop_index_initial_const = 0;
        long long loop_bound_const = 0;
        long long suffix_value = 0;
        long long suffix_byte_len = 0;
        if (!(mapping && yyjson_is_obj(mapping))) return 0;
        return array_text_observer_region_read_i64_field(route, "block", &route_block) &&
               array_text_observer_region_read_i64_field(
                   route,
                   "observer_instruction_index",
                   &route_instruction_index) &&
               array_text_observer_region_read_i64_field(
                   route,
                   "array_value",
                   &route_array_value) &&
               array_text_observer_region_read_i64_field(
                   route,
                   "observer_arg0",
                   &observer_arg0) &&
               array_text_observer_region_read_i64_field(
                   route,
                   "observer_arg0_byte_len",
                   &observer_arg0_byte_len) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "begin_block",
                   &begin_block) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "begin_to_header_block",
                   &begin_to_header_block) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "header_block",
                   &header_block) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "observer_block",
                   &observer_block) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "observer_instruction_index",
                   &observer_instruction_index) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "then_store_block",
                   &then_store_block) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "store_instruction_index",
                   &store_instruction_index) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "latch_block",
                   &latch_block) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "exit_block",
                   &exit_block) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "array_root_value",
                   &array_root_value) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "loop_index_initial_const",
                   &loop_index_initial_const) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "loop_bound_const",
                   &loop_bound_const) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "suffix_value",
                   &suffix_value) &&
               array_text_observer_region_read_i64_field(
                   mapping,
                   "suffix_byte_len",
                   &suffix_byte_len) &&
               begin_to_header_block == header_block &&
               observer_block == route_block &&
               observer_instruction_index == route_instruction_index &&
               store_instruction_index >= 0 &&
               array_root_value == route_array_value &&
               loop_index_initial_const == 0 &&
               loop_bound_const > 0 &&
               observer_arg0_byte_len >= 0 &&
               suffix_byte_len >= 0;
      }

      auto int array_text_observer_store_executor_contract_valid(yyjson_val* route) {
        yyjson_val* contract = route ? yyjson_obj_get(route, "executor_contract") : NULL;
        yyjson_val* effects = contract ? yyjson_obj_get(contract, "effects") : NULL;
        yyjson_val* capabilities =
            contract ? yyjson_obj_get(contract, "consumer_capabilities") : NULL;
        const char* observer_kind = route ? read_str(route, "observer_kind") : NULL;
        const char* observer_arg0_repr = route ? read_str(route, "observer_arg0_repr") : NULL;
        const char* consumer_shape = route ? read_str(route, "consumer_shape") : NULL;
        const char* route_publication_boundary =
            route ? read_str(route, "publication_boundary") : NULL;
        const char* result_repr = route ? read_str(route, "result_repr") : NULL;
        const char* execution_mode = contract ? read_str(contract, "execution_mode") : NULL;
        const char* proof_region = contract ? read_str(contract, "proof_region") : NULL;
        const char* publication_boundary =
            contract ? read_str(contract, "publication_boundary") : NULL;
        const char* carrier = contract ? read_str(contract, "carrier") : NULL;
        const char* materialization_policy =
            contract ? read_str(contract, "materialization_policy") : NULL;
        return route && contract && yyjson_is_obj(contract) &&
               observer_kind && !strcmp(observer_kind, "indexof") &&
               observer_arg0_repr && !strcmp(observer_arg0_repr, "const_utf8") &&
               consumer_shape && !strcmp(consumer_shape, "found_predicate") &&
               route_publication_boundary && !strcmp(route_publication_boundary, "none") &&
               result_repr && !strcmp(result_repr, "scalar_i64") &&
               !hako_llvmc_read_bool_key(route, "keep_get_live") &&
               !hako_llvmc_read_bool_key(route, "observer_arg0_keep_live") &&
               execution_mode && !strcmp(execution_mode, "single_region_executor") &&
               proof_region && !strcmp(proof_region, "loop_backedge_single_body") &&
               publication_boundary && !strcmp(publication_boundary, "none") &&
               carrier && !strcmp(carrier, "array_lane_text_cell") &&
               array_text_observer_region_string_array_eq2(
                   effects,
                   "observe.indexof",
                   "store.cell") &&
               array_text_observer_region_string_array_eq2(
                   capabilities,
                   "compare_only",
                   "sink_store") &&
               materialization_policy &&
                   !strcmp(materialization_policy, "text_resident_or_stringlike_slot") &&
               array_text_observer_store_region_mapping_valid(route, contract);
      }

      auto void array_text_observer_store_region_mapping_fill(
          yyjson_val* route,
          struct ArrayTextObserverStoreRegionRouteMetadata* out) {
        yyjson_val* contract = route ? yyjson_obj_get(route, "executor_contract") : NULL;
        yyjson_val* mapping = contract ? yyjson_obj_get(contract, "region_mapping") : NULL;
        long long tmp = 0;
        if (!out || !mapping) return;
        if (array_text_observer_region_read_i64_field(mapping, "begin_block", &tmp)) {
          out->begin_block = tmp;
        }
        if (array_text_observer_region_read_i64_field(mapping, "begin_to_header_block", &tmp)) {
          out->begin_to_header_block = tmp;
        }
        if (array_text_observer_region_read_i64_field(mapping, "header_block", &tmp)) {
          out->header_block = tmp;
        }
        if (array_text_observer_region_read_i64_field(mapping, "observer_block", &tmp)) {
          out->observer_block = tmp;
        }
        if (array_text_observer_region_read_i64_field(mapping, "then_store_block", &tmp)) {
          out->then_store_block = tmp;
        }
        if (array_text_observer_region_read_i64_field(mapping, "latch_block", &tmp)) {
          out->latch_block = tmp;
        }
        if (array_text_observer_region_read_i64_field(mapping, "exit_block", &tmp)) {
          out->exit_block = tmp;
        }
        if (array_text_observer_region_read_i64_field(mapping, "array_root_value", &tmp)) {
          out->array_root_value = tmp;
        }
        if (array_text_observer_region_read_i64_field(mapping, "loop_bound_const", &tmp)) {
          out->loop_bound_const = tmp;
        }
        if (array_text_observer_region_read_i64_field(route, "observer_arg0", &tmp)) {
          out->needle_reg = tmp;
        }
        if (array_text_observer_region_read_i64_field(route, "observer_arg0_byte_len", &tmp)) {
          out->needle_len = tmp;
        }
        if (array_text_observer_region_read_i64_field(mapping, "suffix_value", &tmp)) {
          out->suffix_reg = tmp;
        }
        if (array_text_observer_region_read_i64_field(mapping, "suffix_byte_len", &tmp)) {
          out->suffix_len = tmp;
        }
      }

      auto int array_text_observer_has_store_region_route(void) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "array_text_observer_routes");
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          if (array_text_observer_store_executor_contract_valid(route)) return 1;
        }
        return 0;
      }

      auto int match_array_text_observer_store_region_metadata_any(
          struct ArrayTextObserverStoreRegionRouteMetadata* out) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "array_text_observer_routes");
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          if (!array_text_observer_store_executor_contract_valid(route)) continue;
          if (out) {
            out->matched = 1;
            array_text_observer_store_region_mapping_fill(route, out);
          }
          return 1;
        }
        return 0;
      }
