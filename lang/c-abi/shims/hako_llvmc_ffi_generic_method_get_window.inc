      #include "hako_llvmc_ffi_generic_method_get_window_metadata.inc"
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
      auto int match_string_direct_set_window_route_metadata(
          long long bid,
          size_t ii,
          struct StringDirectSetWindowRouteMetadata* out) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "string_direct_set_window_routes");
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* block_val = route ? yyjson_obj_get(route, "block") : NULL;
          yyjson_val* index_val = route ? yyjson_obj_get(route, "instruction_index") : NULL;
          yyjson_val* second_val = route ? yyjson_obj_get(route, "second_instruction_index") : NULL;
          yyjson_val* concat_val = route ? yyjson_obj_get(route, "concat_instruction_index") : NULL;
          yyjson_val* source_val = route ? yyjson_obj_get(route, "source_value") : NULL;
          yyjson_val* prefix_val = route ? yyjson_obj_get(route, "prefix_value") : NULL;
          yyjson_val* suffix_val = route ? yyjson_obj_get(route, "suffix_value") : NULL;
          yyjson_val* middle_val = route ? yyjson_obj_get(route, "middle_value") : NULL;
          yyjson_val* split_val = route ? yyjson_obj_get(route, "split_value") : NULL;
          yyjson_val* result_val = route ? yyjson_obj_get(route, "result_value") : NULL;
          yyjson_val* start_val = route ? yyjson_obj_get(route, "subrange_start") : NULL;
          yyjson_val* end_val = route ? yyjson_obj_get(route, "subrange_end") : NULL;
          yyjson_val* skip_vals = route ? yyjson_obj_get(route, "skip_instruction_indices") : NULL;
          yyjson_val* effects = route ? yyjson_obj_get(route, "effects") : NULL;
          const char* route_id = route ? read_str(route, "route_id") : NULL;
          const char* proof = route ? read_str(route, "proof") : NULL;
          const char* consumer = route ? read_str(route, "consumer") : NULL;
          if (!(block_val && index_val &&
                yyjson_is_num(block_val) && yyjson_is_num(index_val))) {
            continue;
          }
          if ((long long)yyjson_get_sint(block_val) != bid ||
              (size_t)yyjson_get_sint(index_val) != ii) {
            continue;
          }
          if (!(route_id && !strcmp(route_id, "string.direct_set_source_window") &&
                proof && !strcmp(proof, "piecewise_concat3_direct_set_source_window") &&
                consumer && !strcmp(consumer, "direct_set") &&
                second_val && yyjson_is_num(second_val) &&
                concat_val && yyjson_is_num(concat_val) &&
                source_val && yyjson_is_num(source_val) &&
                prefix_val && yyjson_is_num(prefix_val) &&
                suffix_val && yyjson_is_num(suffix_val) &&
                middle_val && yyjson_is_num(middle_val) &&
                split_val && yyjson_is_num(split_val) &&
                result_val && yyjson_is_num(result_val) &&
                start_val && yyjson_is_num(start_val) &&
                end_val && yyjson_is_num(end_val) &&
                skip_vals && yyjson_is_arr(skip_vals) &&
                array_text_residence_string_array_eq3(
                    effects, "observe.substring", "defer.piecewise",
                    "direct.set.consumer"))) {
            if (hako_llvmc_route_trace_enabled()) {
              hako_llvmc_emit_route_trace(
                  "string_direct_set_source_window",
                  "miss",
                  "malformed_mir_route_metadata",
                  "");
            }
            if (out) out->invalid = 1;
            return 0;
          }
          if (out) {
            out->matched = 1;
            out->second_ii = (size_t)yyjson_get_sint(second_val);
            out->concat_ii = (size_t)yyjson_get_sint(concat_val);
            out->source_reg = (long long)yyjson_get_sint(source_val);
            out->prefix_dst = (long long)yyjson_get_sint(prefix_val);
            out->suffix_dst = (long long)yyjson_get_sint(suffix_val);
            out->middle_reg = (long long)yyjson_get_sint(middle_val);
            out->split_reg = (long long)yyjson_get_sint(split_val);
            out->result_reg = (long long)yyjson_get_sint(result_val);
            out->subrange_start = (long long)yyjson_get_sint(start_val);
            out->subrange_end = (long long)yyjson_get_sint(end_val);
            for (size_t si = 0; si < yyjson_arr_size(skip_vals); si++) {
              yyjson_val* skip_val = yyjson_arr_get(skip_vals, si);
              if (!(skip_val && yyjson_is_num(skip_val))) {
                if (hako_llvmc_route_trace_enabled()) {
                  hako_llvmc_emit_route_trace(
                      "string_direct_set_source_window",
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
                      "string_direct_set_source_window",
                      "miss",
                      "metadata_skips_current_instruction",
                      "");
                }
                out->invalid = 1;
                return 0;
              }
              string_direct_set_window_route_skip(out, skip_ii);
            }
          }
          if (hako_llvmc_route_trace_enabled()) {
            char extra[256];
            snprintf(
                extra,
                sizeof(extra),
                "ii=%zu source=%lld middle=%lld split=%lld result=%lld concat_ii=%zu skip_n=%d",
                ii,
                out ? out->source_reg : (long long)yyjson_get_sint(source_val),
                out ? out->middle_reg : (long long)yyjson_get_sint(middle_val),
                out ? out->split_reg : (long long)yyjson_get_sint(split_val),
                out ? out->result_reg : (long long)yyjson_get_sint(result_val),
                out ? out->concat_ii : (size_t)yyjson_get_sint(concat_val),
                out ? out->skip_n : 0);
            hako_llvmc_emit_route_trace(
                "string_direct_set_source_window",
                "hit",
                "mir_route_metadata",
                extra);
          }
          return 1;
        }
        return 0;
      }
