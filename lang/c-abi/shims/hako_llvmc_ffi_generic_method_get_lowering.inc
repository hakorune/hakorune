      auto int emit_generic_method_get_by_window_or_policy(
          FILE* f,
          yyjson_val* blocks,
          long long bid,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long dst,
          long long recv,
          long long a0,
          int recv_org,
          int runtime_array_get,
          int runtime_array_string,
          const struct GenericMethodEmitPlan* plan,
          char* ab,
          size_t ab_cap,
          unsigned char* skip_inst) {
        struct ArrayRmwWindowMatch rmw_match;
        const char* rmw_reason = NULL;
        char rmw_extra[384];
        struct ArrayStringLenWindowMatch string_len_match;
        const char* string_len_reason = NULL;
        char string_len_extra[256];
        trace_array_rmw_window_candidate(ii, ilen, insts, bname, mname, recv, a0, dst, recv_org, runtime_array_get, runtime_array_string);
        trace_array_string_len_window_candidate(ii, ilen, insts, bname, mname, recv, a0, dst, recv_org, runtime_array_string);
        if (analyze_array_rmw_window_candidate(
                ii, ilen, insts, bname, mname, recv, a0, dst, recv_org,
                runtime_array_get, runtime_array_string, &rmw_match, &rmw_reason,
                rmw_extra, sizeof(rmw_extra))) {
          if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
          fprintf(f, "  %%r%lld = call i64 @\"nyash.array.rmw_add1_hi\"(%s)\n", rmw_match.result_reg, ab);
          set_type(rmw_match.result_reg, T_I64);
          for (int si = 0; si < rmw_match.copy_n; si++) skip_inst[rmw_match.copy_iis[si]] = 1;
          skip_inst[rmw_match.const_ii] = 1;
          skip_inst[rmw_match.binop_ii] = 1;
          skip_inst[rmw_match.set_ii] = 1;
          return 1;
        }
        if (analyze_array_string_len_window_candidate(
                ii, ilen, insts, bname, mname, recv, a0, dst, recv_org,
                runtime_array_string, &string_len_match, &string_len_reason,
                string_len_extra, sizeof(string_len_extra))) {
          if (string_len_match.source_only_insert_mid) {
            if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
            if (dst && runtime_array_string) {
              remember_array_string_get_source(dst, recv, a0);
              set_type(dst, T_I64);
              set_origin(dst, ORG_STRING);
              set_scan_origin(dst, ORG_STRING);
            }
            hako_llvmc_emit_array_string_len(f, string_len_match.len_dst, ab);
            if (dst) remember_string_length_call(string_len_match.len_dst, dst);
            for (int si = 0; si < string_len_match.copy_n; si++) skip_inst[string_len_match.copy_iis[si]] = 1;
            skip_inst[string_len_match.len_ii] = 1;
            return 1;
          }
          if (string_len_match.keep_get_live) {
            if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
            if (dst) {
              hako_llvmc_emit_array_slot_load(
                  f, dst, ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_REF);
            } else {
              hako_llvmc_emit_array_slot_load(
                  f, dst, ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_REF);
            }
            if (dst && runtime_array_string) {
              remember_array_string_get_source(dst, recv, a0);
            }
            hako_llvmc_emit_array_string_len(f, string_len_match.len_dst, ab);
            if (dst) remember_string_length_call(string_len_match.len_dst, dst);
            skip_inst[string_len_match.len_ii] = 1;
            return 1;
          }
          if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
          hako_llvmc_emit_array_string_len(f, string_len_match.len_dst, ab);
          for (int si = 0; si < string_len_match.copy_n; si++) skip_inst[string_len_match.copy_iis[si]] = 1;
          skip_inst[string_len_match.len_ii] = 1;
          return 1;
        }
        if (try_defer_array_string_indexof_get_window(
                f, blocks, bid, ii, ilen, insts, bname, mname, recv, a0, dst, recv_org,
                runtime_array_string, skip_inst)) {
          return 1;
        }
        if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
        if (!emit_generic_method_get_fallback_by_policy(
                f, bname, dst, plan, ab)) {
          return -1;
        }
        return 1;
      }
