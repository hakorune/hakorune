      struct ArrayRmwWindowRouteMetadata {
        int matched;
        int invalid;
        size_t skip_iis[16];
        int skip_n;
        long long result_reg;
      };
      struct ArrayStringLenWindowRouteMetadata {
        int matched;
        int invalid;
        int keep_get_live;
        int source_only_insert_mid;
        size_t skip_iis[16];
        int skip_n;
        size_t len_ii;
        long long len_dst;
      };
      struct ArrayStringLoopCarryLenStoreMatch {
        int matched;
        size_t skip_iis[32];
        int skip_n;
        long long result_len_dst;
        long long middle_reg;
        long long middle_len;
      };
      struct ArrayTextResidenceSessionRouteMetadata {
        int matched;
        int executor_contract_matched;
        size_t skip_iis[32];
        int skip_n;
        long long begin_block;
        long long begin_to_header_block;
        long long header_block;
        long long body_block;
        long long exit_block;
        long long update_block;
        size_t update_instruction_index;
        long long end_block;
        size_t route_instruction_index;
        long long array_root_value;
        long long loop_bound_const;
        long long row_modulus_const;
        long long region_result_dst;
        long long result_len_dst;
        long long middle_reg;
        long long middle_len;
      };
      struct StringDirectSetWindowRouteMetadata {
        int matched;
        int invalid;
        size_t skip_iis[32];
        int skip_n;
        size_t second_ii;
        size_t concat_ii;
        long long source_reg;
        long long prefix_dst;
        long long suffix_dst;
        long long middle_reg;
        long long split_reg;
        long long result_reg;
        long long subrange_start;
        long long subrange_end;
      };
      auto void array_string_loopcarry_len_store_skip(
          struct ArrayStringLoopCarryLenStoreMatch* out,
          size_t ii) {
        if (!out || ii == (size_t)-1) return;
        for (int i = 0; i < out->skip_n; i++) {
          if (out->skip_iis[i] == ii) return;
        }
        if (out->skip_n < (int)(sizeof(out->skip_iis) / sizeof(out->skip_iis[0]))) {
          out->skip_iis[out->skip_n++] = ii;
        }
      }
      auto void array_rmw_window_route_skip(
          struct ArrayRmwWindowRouteMetadata* out,
          size_t ii) {
        if (!out || ii == (size_t)-1) return;
        for (int i = 0; i < out->skip_n; i++) {
          if (out->skip_iis[i] == ii) return;
        }
        if (out->skip_n < (int)(sizeof(out->skip_iis) / sizeof(out->skip_iis[0]))) {
          out->skip_iis[out->skip_n++] = ii;
        }
      }
      auto void array_string_len_window_route_skip(
          struct ArrayStringLenWindowRouteMetadata* out,
          size_t ii) {
        if (!out || ii == (size_t)-1) return;
        for (int i = 0; i < out->skip_n; i++) {
          if (out->skip_iis[i] == ii) return;
        }
        if (out->skip_n < (int)(sizeof(out->skip_iis) / sizeof(out->skip_iis[0]))) {
          out->skip_iis[out->skip_n++] = ii;
        }
      }
      auto void string_direct_set_window_route_skip(
          struct StringDirectSetWindowRouteMetadata* out,
          size_t ii) {
        if (!out || ii == (size_t)-1) return;
        for (int i = 0; i < out->skip_n; i++) {
          if (out->skip_iis[i] == ii) return;
        }
        if (out->skip_n < (int)(sizeof(out->skip_iis) / sizeof(out->skip_iis[0]))) {
          out->skip_iis[out->skip_n++] = ii;
        }
      }
      auto void array_text_residence_session_route_skip(
          struct ArrayTextResidenceSessionRouteMetadata* out,
          size_t ii) {
        if (!out || ii == (size_t)-1) return;
        for (int i = 0; i < out->skip_n; i++) {
          if (out->skip_iis[i] == ii) return;
        }
        if (out->skip_n < (int)(sizeof(out->skip_iis) / sizeof(out->skip_iis[0]))) {
          out->skip_iis[out->skip_n++] = ii;
        }
      }
      auto int array_text_residence_string_array_eq2(
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
      auto int array_text_residence_string_array_eq3(
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
        if (!(v0 && v1 && v2 &&
              yyjson_is_str(v0) && yyjson_is_str(v1) && yyjson_is_str(v2))) {
          return 0;
        }
        s0 = yyjson_get_str(v0);
        s1 = yyjson_get_str(v1);
        s2 = yyjson_get_str(v2);
        return s0 && s1 && s2 &&
               !strcmp(s0, first) &&
               !strcmp(s1, second) &&
               !strcmp(s2, third);
      }
      auto int array_text_residence_read_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }
      auto int array_text_residence_region_mapping_valid(
          yyjson_val* route,
          yyjson_val* contract) {
        yyjson_val* mapping = contract ? yyjson_obj_get(contract, "region_mapping") : NULL;
        long long route_index_value = 0;
        long long array_root_value = 0;
        long long loop_index_phi_value = 0;
        long long loop_index_initial_value = 0;
        long long loop_index_initial_const = 0;
        long long loop_index_next_value = 0;
        long long loop_bound_value = 0;
        long long loop_bound_const = 0;
        long long accumulator_phi_value = 0;
        long long accumulator_initial_value = 0;
        long long accumulator_initial_const = 0;
        long long accumulator_next_value = 0;
        long long exit_accumulator_value = 0;
        long long row_index_value = 0;
        long long row_modulus_value = 0;
        long long row_modulus_const = 0;
        if (!(mapping && yyjson_is_obj(mapping))) return 0;
        return array_text_residence_read_i64_field(route, "index_value", &route_index_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "array_root_value",
                   &array_root_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "loop_index_phi_value",
                   &loop_index_phi_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "loop_index_initial_value",
                   &loop_index_initial_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "loop_index_initial_const",
                   &loop_index_initial_const) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "loop_index_next_value",
                   &loop_index_next_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "loop_bound_value",
                   &loop_bound_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "loop_bound_const",
                   &loop_bound_const) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "accumulator_phi_value",
                   &accumulator_phi_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "accumulator_initial_value",
                   &accumulator_initial_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "accumulator_initial_const",
                   &accumulator_initial_const) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "accumulator_next_value",
                   &accumulator_next_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "exit_accumulator_value",
                   &exit_accumulator_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "row_index_value",
                   &row_index_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "row_modulus_value",
                   &row_modulus_value) &&
               array_text_residence_read_i64_field(
                   mapping,
                   "row_modulus_const",
                   &row_modulus_const) &&
               loop_bound_const > 0 &&
               row_modulus_const > 0 &&
               array_root_value != 0 &&
               loop_index_initial_const == 0 &&
               accumulator_initial_const == 0 &&
               row_index_value == route_index_value &&
               exit_accumulator_value == accumulator_phi_value &&
               loop_index_phi_value != 0 &&
               loop_index_initial_value != 0 &&
               loop_index_next_value != 0 &&
               loop_bound_value != 0 &&
               accumulator_initial_value != 0 &&
               accumulator_next_value != 0 &&
               row_modulus_value != 0;
      }
      auto int array_text_residence_executor_contract_valid(yyjson_val* route) {
        yyjson_val* contract = route ? yyjson_obj_get(route, "executor_contract") : NULL;
        yyjson_val* effects = contract ? yyjson_obj_get(contract, "effects") : NULL;
        yyjson_val* capabilities =
            contract ? yyjson_obj_get(contract, "consumer_capabilities") : NULL;
        const char* execution_mode = contract ? read_str(contract, "execution_mode") : NULL;
        const char* proof_region = contract ? read_str(contract, "proof_region") : NULL;
        const char* publication_boundary =
            contract ? read_str(contract, "publication_boundary") : NULL;
        const char* carrier = contract ? read_str(contract, "carrier") : NULL;
        const char* materialization_policy =
            contract ? read_str(contract, "materialization_policy") : NULL;
        return contract && yyjson_is_obj(contract) &&
               execution_mode && !strcmp(execution_mode, "single_region_executor") &&
               proof_region && !strcmp(proof_region, "loop_backedge_single_body") &&
               publication_boundary && !strcmp(publication_boundary, "none") &&
               carrier && !strcmp(carrier, "array_lane_text_cell") &&
               array_text_residence_string_array_eq2(
                   effects,
                   "store.cell",
                   "length_only_result_carry") &&
               array_text_residence_string_array_eq2(
                   capabilities,
                   "sink_store",
                   "length_only") &&
               materialization_policy &&
                   !strcmp(materialization_policy, "text_resident_or_stringlike_slot") &&
               array_text_residence_region_mapping_valid(route, contract);
      }
      auto void array_text_residence_region_mapping_fill(
          yyjson_val* route,
          struct ArrayTextResidenceSessionRouteMetadata* out) {
        yyjson_val* contract = route ? yyjson_obj_get(route, "executor_contract") : NULL;
        yyjson_val* mapping = contract ? yyjson_obj_get(contract, "region_mapping") : NULL;
        long long tmp = 0;
        if (!out || !mapping) return;
        if (array_text_residence_read_i64_field(mapping, "array_root_value", &tmp)) {
          out->array_root_value = tmp;
        }
        if (array_text_residence_read_i64_field(mapping, "loop_bound_const", &tmp)) {
          out->loop_bound_const = tmp;
        }
        if (array_text_residence_read_i64_field(mapping, "row_modulus_const", &tmp)) {
          out->row_modulus_const = tmp;
        }
        if (array_text_residence_read_i64_field(mapping, "exit_accumulator_value", &tmp)) {
          out->region_result_dst = tmp;
        }
      }
      auto void array_text_residence_session_to_loopcarry_update(
          const struct ArrayTextResidenceSessionRouteMetadata* session,
          struct ArrayStringLoopCarryLenStoreMatch* out) {
        if (!session || !out) return;
        memset(out, 0, sizeof(*out));
        out->matched = 1;
        out->result_len_dst = session->result_len_dst;
        out->middle_reg = session->middle_reg;
        out->middle_len = session->middle_len;
        for (int i = 0; i < session->skip_n; i++) {
          array_string_loopcarry_len_store_skip(out, session->skip_iis[i]);
        }
      }
      auto int match_array_text_residence_session_route_metadata(
          long long bid,
          size_t ii,
          struct ArrayTextResidenceSessionRouteMetadata* out) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "array_text_residence_sessions");
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* begin_block_val = route ? yyjson_obj_get(route, "begin_block") : NULL;
          yyjson_val* begin_to_header_block_val = route ? yyjson_obj_get(route, "begin_to_header_block") : NULL;
          yyjson_val* header_block_val = route ? yyjson_obj_get(route, "header_block") : NULL;
          yyjson_val* body_block_val = route ? yyjson_obj_get(route, "body_block") : NULL;
          yyjson_val* exit_block_val = route ? yyjson_obj_get(route, "exit_block") : NULL;
          yyjson_val* update_block_val = route ? yyjson_obj_get(route, "update_block") : NULL;
          yyjson_val* update_index_val = route ? yyjson_obj_get(route, "update_instruction_index") : NULL;
          yyjson_val* end_block_val = route ? yyjson_obj_get(route, "end_block") : NULL;
          yyjson_val* route_index_val = route ? yyjson_obj_get(route, "route_instruction_index") : NULL;
          yyjson_val* result_len_val = route ? yyjson_obj_get(route, "result_len_value") : NULL;
          yyjson_val* middle_val = route ? yyjson_obj_get(route, "middle_value") : NULL;
          yyjson_val* middle_len_val = route ? yyjson_obj_get(route, "middle_length") : NULL;
          yyjson_val* skip_vals = route ? yyjson_obj_get(route, "skip_instruction_indices") : NULL;
          const char* begin_placement = route ? read_str(route, "begin_placement") : NULL;
          const char* update_placement = route ? read_str(route, "update_placement") : NULL;
          const char* end_placement = route ? read_str(route, "end_placement") : NULL;
          const char* scope = route ? read_str(route, "scope") : NULL;
          const char* proof = route ? read_str(route, "proof") : NULL;
          const char* consumer_capability = route ? read_str(route, "consumer_capability") : NULL;
          const char* publication_boundary = route ? read_str(route, "publication_boundary") : NULL;
          long long begin_block = 0;
          long long begin_to_header_block = 0;
          long long header_block = 0;
          long long body_block = 0;
          long long exit_block = 0;
          long long update_block = 0;
          size_t update_index = 0;
          long long end_block = 0;
          size_t route_index = 0;
          if (!(update_block_val && update_index_val &&
                yyjson_is_num(update_block_val) && yyjson_is_num(update_index_val))) {
            continue;
          }
          if ((long long)yyjson_get_sint(update_block_val) != bid ||
              (size_t)yyjson_get_sint(update_index_val) != ii) {
            continue;
          }
          if (!(begin_block_val && begin_to_header_block_val && header_block_val &&
                body_block_val && exit_block_val && end_block_val && route_index_val &&
                result_len_val && middle_val && middle_len_val && skip_vals &&
                yyjson_is_num(begin_block_val) &&
                yyjson_is_num(begin_to_header_block_val) &&
                yyjson_is_num(header_block_val) &&
                yyjson_is_num(body_block_val) &&
                yyjson_is_num(exit_block_val) &&
                yyjson_is_num(end_block_val) &&
                yyjson_is_num(route_index_val) &&
                yyjson_is_num(result_len_val) &&
                yyjson_is_num(middle_val) &&
                yyjson_is_num(middle_len_val) &&
                yyjson_is_arr(skip_vals) &&
                begin_placement && !strcmp(begin_placement, "before_preheader_jump") &&
                update_placement && !strcmp(update_placement, "route_instruction") &&
                end_placement && !strcmp(end_placement, "exit_block_entry") &&
                scope && !strcmp(scope, "loop_backedge_single_body") &&
                proof && !strcmp(proof, "loopcarry_len_store_only") &&
                consumer_capability && !strcmp(consumer_capability, "slot_text_len_store_session") &&
                publication_boundary && !strcmp(publication_boundary, "none") &&
                array_text_residence_executor_contract_valid(route))) {
            return 0;
          }
          begin_block = (long long)yyjson_get_sint(begin_block_val);
          begin_to_header_block = (long long)yyjson_get_sint(begin_to_header_block_val);
          header_block = (long long)yyjson_get_sint(header_block_val);
          body_block = (long long)yyjson_get_sint(body_block_val);
          exit_block = (long long)yyjson_get_sint(exit_block_val);
          update_block = (long long)yyjson_get_sint(update_block_val);
          update_index = (size_t)yyjson_get_sint(update_index_val);
          end_block = (long long)yyjson_get_sint(end_block_val);
          route_index = (size_t)yyjson_get_sint(route_index_val);
          if (!(begin_to_header_block == header_block &&
                body_block == update_block &&
                end_block == exit_block &&
                route_index == update_index)) {
            return 0;
          }
          if (out) {
            out->matched = 1;
            out->executor_contract_matched = 1;
            out->begin_block = begin_block;
            out->begin_to_header_block = begin_to_header_block;
            out->header_block = header_block;
            out->body_block = body_block;
            out->exit_block = exit_block;
            out->update_block = update_block;
            out->update_instruction_index = update_index;
            out->end_block = end_block;
            out->route_instruction_index = route_index;
            array_text_residence_region_mapping_fill(route, out);
            out->result_len_dst = (long long)yyjson_get_sint(result_len_val);
            out->middle_reg = (long long)yyjson_get_sint(middle_val);
            out->middle_len = (long long)yyjson_get_sint(middle_len_val);
            for (size_t si = 0; si < yyjson_arr_size(skip_vals); si++) {
              yyjson_val* skip_val = yyjson_arr_get(skip_vals, si);
              if (skip_val && yyjson_is_num(skip_val)) {
                array_text_residence_session_route_skip(
                    out,
                    (size_t)yyjson_get_sint(skip_val));
              }
            }
          }
          if (hako_llvmc_route_trace_enabled()) {
            hako_llvmc_emit_route_trace(
                "array_text_residence_session",
                "hit",
                "mir_route_metadata",
                "");
          }
          return 1;
        }
        return 0;
      }
