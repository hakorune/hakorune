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
      struct ArrayStringPiecewiseDirectSetSourceReuseMatch {
        size_t second_ii;
        size_t concat_ii;
        size_t prefix_copy_iis[8];
        size_t suffix_copy_iis[8];
        int prefix_copy_n;
        int suffix_copy_n;
        long long source_reg;
        long long prefix_dst;
        long long suffix_dst;
        long long prefix_final;
        long long suffix_final;
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
      auto int match_array_text_residence_session_region_begin_metadata(
          long long bid,
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
          if (!(begin_block_val && yyjson_is_num(begin_block_val))) continue;
          if ((long long)yyjson_get_sint(begin_block_val) != bid) continue;
          if (!(begin_to_header_block_val && header_block_val && body_block_val &&
                exit_block_val && update_block_val && update_index_val &&
                end_block_val && route_index_val && result_len_val &&
                middle_val && middle_len_val &&
                yyjson_is_num(begin_to_header_block_val) &&
                yyjson_is_num(header_block_val) &&
                yyjson_is_num(body_block_val) &&
                yyjson_is_num(exit_block_val) &&
                yyjson_is_num(update_block_val) &&
                yyjson_is_num(update_index_val) &&
                yyjson_is_num(end_block_val) &&
                yyjson_is_num(route_index_val) &&
                yyjson_is_num(result_len_val) &&
                yyjson_is_num(middle_val) &&
                yyjson_is_num(middle_len_val) &&
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
          }
          if (hako_llvmc_route_trace_enabled()) {
            hako_llvmc_emit_route_trace(
                "array_text_residence_region_begin",
                "hit",
                "mir_region_mapping",
                "");
          }
          return 1;
        }
        return 0;
      }
      auto int match_array_text_loopcarry_len_store_route_metadata(
          long long bid,
          size_t ii,
          struct ArrayStringLoopCarryLenStoreMatch* out) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "array_text_loopcarry_len_store_routes");
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* block_val = route ? yyjson_obj_get(route, "block") : NULL;
          yyjson_val* index_val = route ? yyjson_obj_get(route, "instruction_index") : NULL;
          yyjson_val* proof_val = route ? yyjson_obj_get(route, "proof") : NULL;
          yyjson_val* result_len_val = route ? yyjson_obj_get(route, "result_len_value") : NULL;
          yyjson_val* middle_val = route ? yyjson_obj_get(route, "middle_value") : NULL;
          yyjson_val* middle_len_val = route ? yyjson_obj_get(route, "middle_length") : NULL;
          yyjson_val* skip_vals = route ? yyjson_obj_get(route, "skip_instruction_indices") : NULL;
          const char* proof = proof_val && yyjson_is_str(proof_val) ? yyjson_get_str(proof_val) : NULL;
          if (!(block_val && index_val && yyjson_is_num(block_val) && yyjson_is_num(index_val))) {
            continue;
          }
          if ((long long)yyjson_get_sint(block_val) != bid ||
              (size_t)yyjson_get_sint(index_val) != ii) {
            continue;
          }
          if (!(proof && !strcmp(proof, "insert_mid_subrange_trailing_len") &&
                result_len_val && yyjson_is_num(result_len_val) &&
                middle_val && yyjson_is_num(middle_val) &&
                middle_len_val && yyjson_is_num(middle_len_val) &&
                skip_vals && yyjson_is_arr(skip_vals))) {
            return 0;
          }
          if (out) {
            out->matched = 1;
            out->result_len_dst = (long long)yyjson_get_sint(result_len_val);
            out->middle_reg = (long long)yyjson_get_sint(middle_val);
            out->middle_len = (long long)yyjson_get_sint(middle_len_val);
            for (size_t si = 0; si < yyjson_arr_size(skip_vals); si++) {
              yyjson_val* skip_val = yyjson_arr_get(skip_vals, si);
              if (skip_val && yyjson_is_num(skip_val)) {
                array_string_loopcarry_len_store_skip(
                    out,
                    (size_t)yyjson_get_sint(skip_val));
              }
            }
          }
          if (hako_llvmc_route_trace_enabled()) {
            hako_llvmc_emit_route_trace(
                "array_string_loopcarry_len_store_window",
                "hit",
                "mir_route_metadata",
                "");
          }
          return 1;
        }
        return 0;
      }
      auto int match_array_rmw_window_route_metadata(
          long long bid,
          size_t ii,
          struct ArrayRmwWindowRouteMetadata* out) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "array_rmw_window_routes");
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* block_val = route ? yyjson_obj_get(route, "block") : NULL;
          yyjson_val* index_val = route ? yyjson_obj_get(route, "instruction_index") : NULL;
          yyjson_val* result_val = route ? yyjson_obj_get(route, "result_value") : NULL;
          yyjson_val* set_index_val = route ? yyjson_obj_get(route, "set_instruction_index") : NULL;
          yyjson_val* skip_vals = route ? yyjson_obj_get(route, "skip_instruction_indices") : NULL;
          yyjson_val* array_val = route ? yyjson_obj_get(route, "array_value") : NULL;
          yyjson_val* idx_val = route ? yyjson_obj_get(route, "index_value") : NULL;
          yyjson_val* input_val = route ? yyjson_obj_get(route, "input_value") : NULL;
          yyjson_val* const_val = route ? yyjson_obj_get(route, "const_value") : NULL;
          yyjson_val* effects = route ? yyjson_obj_get(route, "effects") : NULL;
          const char* route_id = route ? read_str(route, "route_id") : NULL;
          const char* proof = route ? read_str(route, "proof") : NULL;
          const char* emit_symbol = route ? read_str(route, "emit_symbol") : NULL;
          if (!(block_val && index_val &&
                yyjson_is_num(block_val) && yyjson_is_num(index_val))) {
            continue;
          }
          if ((long long)yyjson_get_sint(block_val) != bid ||
              (size_t)yyjson_get_sint(index_val) != ii) {
            continue;
          }
          if (!(route_id && !strcmp(route_id, "array.rmw_add1.window") &&
                proof && !strcmp(proof, "array_get_add1_set_same_slot") &&
                emit_symbol && !strcmp(emit_symbol, "nyash.array.rmw_add1_hi") &&
                result_val && yyjson_is_num(result_val) &&
                set_index_val && yyjson_is_num(set_index_val) &&
                array_val && yyjson_is_num(array_val) &&
                idx_val && yyjson_is_num(idx_val) &&
                input_val && yyjson_is_num(input_val) &&
                const_val && yyjson_is_num(const_val) &&
                skip_vals && yyjson_is_arr(skip_vals) &&
                array_text_residence_string_array_eq2(effects, "load.cell", "store.cell"))) {
            if (hako_llvmc_route_trace_enabled()) {
              hako_llvmc_emit_route_trace(
                  "array_rmw_window",
                  "miss",
                  "malformed_mir_route_metadata",
                  "");
            }
            if (out) out->invalid = 1;
            return 0;
          }
          if (out) {
            out->matched = 1;
            out->result_reg = (long long)yyjson_get_sint(result_val);
            for (size_t si = 0; si < yyjson_arr_size(skip_vals); si++) {
              yyjson_val* skip_val = yyjson_arr_get(skip_vals, si);
              if (!(skip_val && yyjson_is_num(skip_val))) {
                if (hako_llvmc_route_trace_enabled()) {
                  hako_llvmc_emit_route_trace(
                      "array_rmw_window",
                      "miss",
                      "malformed_mir_route_skip",
                      "");
                }
                out->invalid = 1;
                return 0;
              }
              size_t skip_ii = (size_t)yyjson_get_sint(skip_val);
              if (skip_ii == ii) {
                if (hako_llvmc_route_trace_enabled()) {
                  hako_llvmc_emit_route_trace(
                      "array_rmw_window",
                      "miss",
                      "metadata_skips_current_instruction",
                      "");
                }
                out->invalid = 1;
                return 0;
              }
              array_rmw_window_route_skip(out, skip_ii);
            }
          }
          if (hako_llvmc_route_trace_enabled()) {
            hako_llvmc_emit_route_trace(
                "array_rmw_window",
                "hit",
                "mir_route_metadata",
                "");
          }
          return 1;
        }
        return 0;
      }
      auto int match_array_string_len_window_route_metadata(
          long long bid,
          size_t ii,
          struct ArrayStringLenWindowRouteMetadata* out) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "array_string_len_window_routes");
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* block_val = route ? yyjson_obj_get(route, "block") : NULL;
          yyjson_val* index_val = route ? yyjson_obj_get(route, "instruction_index") : NULL;
          yyjson_val* array_val = route ? yyjson_obj_get(route, "array_value") : NULL;
          yyjson_val* idx_val = route ? yyjson_obj_get(route, "index_value") : NULL;
          yyjson_val* source_val = route ? yyjson_obj_get(route, "source_value") : NULL;
          yyjson_val* len_index_val = route ? yyjson_obj_get(route, "len_instruction_index") : NULL;
          yyjson_val* len_val = route ? yyjson_obj_get(route, "len_value") : NULL;
          yyjson_val* skip_vals = route ? yyjson_obj_get(route, "skip_instruction_indices") : NULL;
          yyjson_val* effects = route ? yyjson_obj_get(route, "effects") : NULL;
          yyjson_val* keep_val = route ? yyjson_obj_get(route, "keep_get_live") : NULL;
          yyjson_val* source_only_val = route ? yyjson_obj_get(route, "source_only_insert_mid") : NULL;
          const char* route_id = route ? read_str(route, "route_id") : NULL;
          const char* mode = route ? read_str(route, "mode") : NULL;
          const char* proof = route ? read_str(route, "proof") : NULL;
          const char* emit_symbol = route ? read_str(route, "emit_symbol") : NULL;
          int mode_len_only = mode && !strcmp(mode, "len_only");
          int mode_keep_get_live = mode && !strcmp(mode, "keep_get_live");
          int mode_source_only = mode && !strcmp(mode, "source_only_insert_mid");
          int keep_get_live = keep_val && yyjson_is_bool(keep_val) && yyjson_get_bool(keep_val);
          int source_only_insert_mid =
              source_only_val && yyjson_is_bool(source_only_val) &&
              yyjson_get_bool(source_only_val);
          const char* expected_proof = mode_source_only
              ? "array_get_len_source_only_direct_set"
              : (mode_keep_get_live
                    ? "array_get_len_keep_source_live"
                    : "array_get_len_no_later_source_use");
          int effects_ok = mode_source_only
              ? array_text_residence_string_array_eq3(
                    effects, "load.cell", "observe.len", "publish.source.ref")
              : (mode_keep_get_live
                    ? array_text_residence_string_array_eq3(
                          effects, "load.cell", "observe.len", "keep.source.live")
                    : array_text_residence_string_array_eq2(
                          effects, "load.cell", "observe.len"));
          if (!(block_val && index_val &&
                yyjson_is_num(block_val) && yyjson_is_num(index_val))) {
            continue;
          }
          if ((long long)yyjson_get_sint(block_val) != bid ||
              (size_t)yyjson_get_sint(index_val) != ii) {
            continue;
          }
          if (!(route_id && !strcmp(route_id, "array.string_len.window") &&
                (mode_len_only || mode_keep_get_live || mode_source_only) &&
                proof && !strcmp(proof, expected_proof) &&
                emit_symbol && !strcmp(emit_symbol, "nyash.array.string_len_hi") &&
                array_val && yyjson_is_num(array_val) &&
                idx_val && yyjson_is_num(idx_val) &&
                source_val && yyjson_is_num(source_val) &&
                len_index_val && yyjson_is_num(len_index_val) &&
                len_val && yyjson_is_num(len_val) &&
                skip_vals && yyjson_is_arr(skip_vals) &&
                keep_val && yyjson_is_bool(keep_val) &&
                source_only_val && yyjson_is_bool(source_only_val) &&
                keep_get_live == mode_keep_get_live &&
                source_only_insert_mid == mode_source_only &&
                effects_ok)) {
            if (hako_llvmc_route_trace_enabled()) {
              hako_llvmc_emit_route_trace(
                  "array_string_len_window",
                  "miss",
                  "malformed_mir_route_metadata",
                  "");
            }
            if (out) out->invalid = 1;
            return 0;
          }
          if (out) {
            out->matched = 1;
            out->keep_get_live = keep_get_live;
            out->source_only_insert_mid = source_only_insert_mid;
            out->len_ii = (size_t)yyjson_get_sint(len_index_val);
            out->len_dst = (long long)yyjson_get_sint(len_val);
            for (size_t si = 0; si < yyjson_arr_size(skip_vals); si++) {
              yyjson_val* skip_val = yyjson_arr_get(skip_vals, si);
              if (!(skip_val && yyjson_is_num(skip_val))) {
                if (hako_llvmc_route_trace_enabled()) {
                  hako_llvmc_emit_route_trace(
                      "array_string_len_window",
                      "miss",
                      "malformed_mir_route_skip",
                      "");
                }
                out->invalid = 1;
                return 0;
              }
              size_t skip_ii = (size_t)yyjson_get_sint(skip_val);
              if (skip_ii == ii) {
                if (hako_llvmc_route_trace_enabled()) {
                  hako_llvmc_emit_route_trace(
                      "array_string_len_window",
                      "miss",
                      "metadata_skips_current_instruction",
                      "");
                }
                out->invalid = 1;
                return 0;
              }
              array_string_len_window_route_skip(out, skip_ii);
            }
          }
          if (hako_llvmc_route_trace_enabled()) {
            char extra[256];
            snprintf(
                extra,
                sizeof(extra),
                "ii=%zu len_ii=%zu len_dst=%lld source_only_insert_mid=%d keep_get_live=%d skip_n=%d proof=%s",
                ii,
                out ? out->len_ii : (size_t)yyjson_get_sint(len_index_val),
                out ? out->len_dst : (long long)yyjson_get_sint(len_val),
                source_only_insert_mid,
                keep_get_live,
                out ? out->skip_n : 0,
                expected_proof);
            hako_llvmc_emit_route_trace(
                "array_string_len_window",
                "hit",
                "mir_route_metadata",
                extra);
          }
          return 1;
        }
        return 0;
      }
      auto int match_array_string_piecewise_concat3_direct_set_source_reuse(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long source_reg,
          struct ArrayStringPiecewiseDirectSetSourceReuseMatch* out) {
        yyjson_val* first_ins = NULL;
        yyjson_val* first_mc = NULL;
        yyjson_val* first_cal = NULL;
        yyjson_val* first_args = NULL;
        yyjson_val* second_ins = NULL;
        yyjson_val* second_mc = NULL;
        yyjson_val* second_cal = NULL;
        yyjson_val* second_args = NULL;
        yyjson_val* concat_ins = NULL;
        yyjson_val* concat_mc = NULL;
        yyjson_val* concat_cal = NULL;
        yyjson_val* concat_args = NULL;
        const char* first_op = NULL;
        const char* first_ctype = NULL;
        const char* first_mname = NULL;
        const char* second_op = NULL;
        const char* second_ctype = NULL;
        const char* second_mname = NULL;
        const char* concat_op = NULL;
        const char* concat_ctype = NULL;
        const char* concat_mname = NULL;
        long long first_recv = 0;
        long long first_start = 0;
        long long first_end = 0;
        long long first_dst = 0;
        long long second_recv = 0;
        long long second_start = 0;
        long long second_dst = 0;
        long long zero = 0;
        size_t prefix_use_ii = 0;
        size_t suffix_use_ii = 0;
        size_t prefix_copy_iis[8];
        size_t suffix_copy_iis[8];
        int prefix_copy_n = 0;
        int suffix_copy_n = 0;
        long long prefix_final = 0;
        long long suffix_final = 0;
        long long concat_dst = 0;
        long long concat_prefix = 0;
        long long concat_middle = 0;
        long long concat_suffix = 0;
        long long concat_start = 0;
        long long concat_end = 0;

        if (out) memset(out, 0, sizeof(*out));
        source_reg = resolve_copy_source_reg(
            resolve_copy_chain_source_reg(insts, ii + 1, source_reg));
        if (!(source_reg && ii + 1 < ilen)) return 0;

        first_ins = yyjson_arr_get(insts, ii);
        first_op = first_ins ? read_str(first_ins, "op") : NULL;
        if (!(first_op && !strcmp(first_op, "mir_call"))) return 0;
        first_mc = yyjson_obj_get(first_ins, "mir_call");
        first_cal = first_mc ? yyjson_obj_get(first_mc, "callee") : NULL;
        first_args = first_mc ? yyjson_obj_get(first_mc, "args") : NULL;
        if (!(first_cal && first_args && yyjson_is_arr(first_args) &&
              yyjson_arr_size(first_args) == 2)) return 0;
        first_ctype = read_str(first_cal, "type");
        first_mname = read_str(first_cal, "method");
        if (!first_mname) first_mname = read_str(first_cal, "name");
        if (!(first_ctype && !strcmp(first_ctype, "Method") &&
              first_mname && !strcmp(first_mname, "substring"))) return 0;
        first_recv = resolve_copy_source_reg(
            resolve_copy_chain_source_reg(
                insts, ii + 1, read_int(first_cal, "receiver")));
        first_start = (long long)yyjson_get_sint(yyjson_arr_get(first_args, 0));
        first_end = (long long)yyjson_get_sint(yyjson_arr_get(first_args, 1));
        first_dst = read_int(first_ins, "dst");
        if (first_recv != source_reg) return 0;
        if (!(has_const(first_start, &zero) && zero == 0)) return 0;

        second_ins = yyjson_arr_get(insts, ii + 1);
        second_op = second_ins ? read_str(second_ins, "op") : NULL;
        if (!(second_op && !strcmp(second_op, "mir_call"))) return 0;
        second_mc = yyjson_obj_get(second_ins, "mir_call");
        second_cal = second_mc ? yyjson_obj_get(second_mc, "callee") : NULL;
        second_args = second_mc ? yyjson_obj_get(second_mc, "args") : NULL;
        if (!(second_cal && second_args && yyjson_is_arr(second_args) &&
              yyjson_arr_size(second_args) == 2)) return 0;
        second_ctype = read_str(second_cal, "type");
        second_mname = read_str(second_cal, "method");
        if (!second_mname) second_mname = read_str(second_cal, "name");
        if (!(second_ctype && !strcmp(second_ctype, "Method") &&
              second_mname && !strcmp(second_mname, "substring"))) return 0;
        second_recv = resolve_copy_source_reg(
            resolve_copy_chain_source_reg(
                insts, ii + 2, read_int(second_cal, "receiver")));
        second_start = (long long)yyjson_get_sint(yyjson_arr_get(second_args, 0));
        second_dst = read_int(second_ins, "dst");
        if (!(second_recv == source_reg &&
              resolve_copy_source_reg(
                  resolve_copy_chain_source_reg(insts, ii + 2, second_start)) ==
                  resolve_copy_source_reg(
                      resolve_copy_chain_source_reg(insts, ii + 1, first_end)))) {
          return 0;
        }

        if (!follow_single_use_copy_chain_to_use(
                ii + 2, ilen, insts, first_dst, &prefix_use_ii, &prefix_final,
                prefix_copy_iis, &prefix_copy_n)) return 0;
        if (!follow_single_use_copy_chain_to_use(
                ii + 2, ilen, insts, second_dst, &suffix_use_ii, &suffix_final,
                suffix_copy_iis, &suffix_copy_n)) return 0;
        if (prefix_use_ii != suffix_use_ii) return 0;

        concat_ins = yyjson_arr_get(insts, prefix_use_ii);
        concat_op = concat_ins ? read_str(concat_ins, "op") : NULL;
        if (!(concat_op && !strcmp(concat_op, "mir_call"))) return 0;
        concat_mc = yyjson_obj_get(concat_ins, "mir_call");
        concat_cal = concat_mc ? yyjson_obj_get(concat_mc, "callee") : NULL;
        concat_args = concat_mc ? yyjson_obj_get(concat_mc, "args") : NULL;
        concat_ctype = concat_cal ? read_str(concat_cal, "type") : NULL;
        concat_mname = concat_cal ? (read_str(concat_cal, "method")
                                        ? read_str(concat_cal, "method")
                                        : read_str(concat_cal, "name"))
                                  : NULL;
        if (!(concat_ctype && !strcmp(concat_ctype, "Extern") &&
              concat_mname && !strcmp(concat_mname, "nyash.string.substring_concat3_hhhii") &&
              concat_args && yyjson_is_arr(concat_args) &&
              yyjson_arr_size(concat_args) == 5)) return 0;
        concat_dst = read_int(concat_ins, "dst");
        concat_prefix = resolve_copy_chain_source_reg(
            insts, prefix_use_ii + 1,
            (long long)yyjson_get_sint(yyjson_arr_get(concat_args, 0)));
        concat_middle = resolve_copy_chain_source_reg(
            insts, prefix_use_ii + 1,
            (long long)yyjson_get_sint(yyjson_arr_get(concat_args, 1)));
        concat_suffix = resolve_copy_chain_source_reg(
            insts, prefix_use_ii + 1,
            (long long)yyjson_get_sint(yyjson_arr_get(concat_args, 2)));
        concat_start = (long long)yyjson_get_sint(yyjson_arr_get(concat_args, 3));
        concat_end = (long long)yyjson_get_sint(yyjson_arr_get(concat_args, 4));
        if (!(concat_dst &&
              concat_prefix == prefix_final &&
              concat_suffix == suffix_final &&
              concat_middle &&
              get_str_const(concat_middle))) return 0;
        if (!hako_llvmc_value_has_direct_set_consumer(concat_dst)) return 0;

        if (out) {
          out->second_ii = ii + 1;
          out->concat_ii = prefix_use_ii;
          out->prefix_copy_n = prefix_copy_n;
          out->suffix_copy_n = suffix_copy_n;
          for (int ci = 0; ci < prefix_copy_n && ci < 8; ci++) {
            out->prefix_copy_iis[ci] = prefix_copy_iis[ci];
          }
          for (int ci = 0; ci < suffix_copy_n && ci < 8; ci++) {
            out->suffix_copy_iis[ci] = suffix_copy_iis[ci];
          }
          out->source_reg = source_reg;
          out->prefix_dst = first_dst;
          out->suffix_dst = second_dst;
          out->prefix_final = prefix_final;
          out->suffix_final = suffix_final;
          out->middle_reg = concat_middle;
          out->split_reg = first_end;
          out->result_reg = concat_dst;
          out->subrange_start = concat_start;
          out->subrange_end = concat_end;
        }
        return 1;
      }
