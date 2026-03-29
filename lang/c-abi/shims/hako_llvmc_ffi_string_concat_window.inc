      auto int find_single_use_inst(
          size_t start_ii,
          size_t ilen,
          yyjson_val* insts,
          long long reg,
          size_t* use_ii_out) {
        int use_count = 0;
        size_t use_ii = 0;
        if (!reg) return 0;
        for (size_t jj = start_ii; jj < ilen; jj++) {
          yyjson_val* later = yyjson_arr_get(insts, jj);
          if (!inst_uses_reg(later, reg)) continue;
          use_count++;
          use_ii = jj;
          if (use_count > 1) return 0;
        }
        if (use_count != 1) return 0;
        if (use_ii_out) *use_ii_out = use_ii;
        return 1;
      }

      auto long long resolve_copy_chain_source_reg(
          yyjson_val* insts,
          size_t search_end_ii,
          long long reg) {
        long long cursor = reg;
        int depth = 0;
        if (!cursor) return 0;
        while (depth < 8) {
          int advanced = 0;
          for (size_t jj = search_end_ii; jj-- > 0;) {
            yyjson_val* ins = yyjson_arr_get(insts, jj);
            const char* op = ins ? read_str(ins, "op") : NULL;
            if (read_int(ins, "dst") != cursor) continue;
            if (!(op && !strcmp(op, "copy"))) return cursor;
            cursor = read_int(ins, "src");
            advanced = 1;
            break;
          }
          if (!advanced) break;
          depth++;
        }
        return cursor;
      }

      auto int find_def_inst_index(
          size_t search_end_ii,
          yyjson_val* insts,
          long long reg,
          size_t* def_ii_out) {
        if (!reg) return 0;
        for (size_t jj = search_end_ii; jj-- > 0;) {
          yyjson_val* ins = yyjson_arr_get(insts, jj);
          if (read_int(ins, "dst") != reg) continue;
          if (def_ii_out) *def_ii_out = jj;
          return 1;
        }
        return 0;
      }

      auto void trace_string_concat_lowering_gate(
          const char* stage,
          const char* result,
          const char* reason,
          size_t ii,
          size_t ilen,
          long long reg,
          size_t use_ii,
          long long consumer_reg,
          long long first_reg,
          long long second_reg,
          long long third_reg,
          const char* producer_kind,
          const char* boundary_kind,
          const char* post_store_use,
          long long known_len,
          int direct_set_only) {
        char extra[320];
        if (!hako_llvmc_route_trace_enabled()) return;
        snprintf(
            extra,
            sizeof(extra),
            "ii=%zu ilen=%zu reg=%lld use_ii=%zu consumer=%lld first=%lld second=%lld third=%lld producer_kind=%s boundary_kind=%s post_store_use=%s known_len=%lld direct_set_only=%d",
            ii,
            ilen,
            reg,
            use_ii,
            consumer_reg,
            first_reg,
            second_reg,
            third_reg,
            producer_kind ? producer_kind : "unknown",
            boundary_kind ? boundary_kind : "unknown",
            post_store_use ? post_store_use : "unknown",
            known_len,
            direct_set_only);
        hako_llvmc_emit_route_trace(stage, result, reason, extra);
      }

      auto int has_direct_array_set_consumer(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long reg) {
        size_t use_ii = 0;
        yyjson_val* use_ins = NULL;
        yyjson_val* mc = NULL;
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        const char* op = NULL;
        const char* ctype = NULL;
        const char* mname = NULL;
        long long consumer_reg = 0;
        int matched = 0;
        const char* producer_kind = string_concat_producer_kind_for_reg(reg);
        const char* reason = "no_single_use";
        if (!find_single_use_inst(ii + 1, ilen, insts, reg, &use_ii)) {
          trace_string_concat_lowering_gate(
              "string_direct_array_set_consumer",
              "miss",
              reason,
              ii,
              ilen,
              reg,
              0,
              0,
              0,
              0,
              0,
              producer_kind,
              string_boundary_kind_store_name(),
              string_post_store_use_name(HAKO_LLVMC_STRING_POST_STORE_USE_NONE),
              -1,
              1);
          return 0;
        }
        use_ins = yyjson_arr_get(insts, use_ii);
        op = use_ins ? read_str(use_ins, "op") : NULL;
        if (!(op && !strcmp(op, "mir_call"))) {
          reason = "use_not_mir_call";
          goto done;
        }
        mc = yyjson_obj_get(use_ins, "mir_call");
        cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        args = mc ? yyjson_obj_get(mc, "args") : NULL;
        ctype = cal ? read_str(cal, "type") : NULL;
        mname = cal ? (read_str(cal, "method") ? read_str(cal, "method")
                                               : read_str(cal, "name"))
                    : NULL;
        if (!(ctype && !strcmp(ctype, "Method") && mname && !strcmp(mname, "set"))) {
          reason = "use_not_array_set";
          goto done;
        }
        if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) >= 2)) {
          reason = "set_args_shape";
          goto done;
        }
        consumer_reg = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
        matched = consumer_reg == reg;
        reason = matched ? "array_set_consumer" : "set_rhs_mismatch";
      done:
        trace_string_concat_lowering_gate(
            "string_direct_array_set_consumer",
            matched ? "hit" : "miss",
            reason,
            ii,
            ilen,
            reg,
            use_ii,
            consumer_reg,
            0,
            0,
            0,
            producer_kind,
            string_boundary_kind_store_name(),
            string_post_store_use_name(HAKO_LLVMC_STRING_POST_STORE_USE_NONE),
            -1,
            1);
        return matched;
      }

      auto int follow_single_use_copy_chain_to_use(
          size_t start_ii,
          size_t ilen,
          yyjson_val* insts,
          long long seed_reg,
          size_t* use_ii_out,
          long long* final_reg_out,
          size_t* copy_iis_out,
          int* copy_n_out) {
        long long cursor = seed_reg;
        int copy_n = 0;
        if (!cursor) return 0;
        while (1) {
          size_t use_ii = 0;
          yyjson_val* use_ins = NULL;
          const char* op = NULL;
          if (!find_single_use_inst(start_ii, ilen, insts, cursor, &use_ii)) return 0;
          use_ins = yyjson_arr_get(insts, use_ii);
          op = use_ins ? read_str(use_ins, "op") : NULL;
          if (op && !strcmp(op, "copy") && read_int(use_ins, "src") == cursor) {
            if (copy_iis_out && copy_n < 8) copy_iis_out[copy_n] = use_ii;
            copy_n++;
            cursor = read_int(use_ins, "dst");
            start_ii = use_ii + 1;
            continue;
          }
          if (use_ii_out) *use_ii_out = use_ii;
          if (final_reg_out) *final_reg_out = cursor;
          if (copy_n_out) *copy_n_out = copy_n;
          return 1;
        }
      }
