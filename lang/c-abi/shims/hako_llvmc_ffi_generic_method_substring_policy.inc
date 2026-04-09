      enum GenericMethodSubstringRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_INSERT_MID_WINDOW = 0,
        HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_STRING_SUBSTRING_CALL = 1,
      };

      auto enum GenericMethodSubstringRouteKind classify_generic_method_substring_route(
          int matched_insert_window) {
        if (matched_insert_window) {
          return HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_INSERT_MID_WINDOW;
        }
        return HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_STRING_SUBSTRING_CALL;
      }

      auto int emit_generic_method_substring_by_policy(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long recv,
          long long dst,
          long long a0,
          long long a1,
          unsigned char* skip_inst,
          char* ab,
          size_t ab_cap) {
        long long resolved_recv = resolve_copy_source_reg(recv);
        long long concat_first = 0;
        long long concat_second = 0;
        long long concat_third = 0;
        int concat_term_count = 0;
        char route_extra[256];
        if (classify_generic_method_substring_route(
                try_emit_string_insert_mid_window_call(
                    f, ii, ilen, insts, recv, dst, a0, a1, skip_inst)) ==
            HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_INSERT_MID_WINDOW) {
          return 1;
        }
        if (resolved_recv &&
            resolve_string_concat_chain_terms(
                resolved_recv,
                &concat_first,
                &concat_second,
                &concat_third,
                &concat_term_count)) {
          char concat_ab[256];
          concat_ab[0] = '\0';
          append_i64_arg_ref(concat_ab, sizeof(concat_ab), concat_first, 1);
          append_i64_arg_ref(concat_ab, sizeof(concat_ab), concat_second, 0);
          if (concat_term_count == 3) {
            append_i64_arg_ref(concat_ab, sizeof(concat_ab), concat_third, 0);
          }
          if (a0) append_i64_arg_ref(concat_ab, sizeof(concat_ab), a0, 0);
          if (a1) append_i64_arg_ref(concat_ab, sizeof(concat_ab), a1, 0);
          if (dst) {
            fprintf(
                f,
                concat_term_count == 3
                    ? "  %%r%lld = call i64 @\"nyash.string.substring_concat3_hhhii\"(%s)\n"
                    : "  %%r%lld = call i64 @\"nyash.string.substring_concat_hhii\"(%s)\n",
                dst,
                concat_ab);
            set_type(dst, T_I64);
            set_origin(dst, ORG_STRING);
            set_scan_origin(dst, ORG_STRING);
            remember_string_substring_call(dst, recv, a0, a1);
          } else {
            fprintf(
                f,
                concat_term_count == 3
                    ? "  call i64 @\"nyash.string.substring_concat3_hhhii\"(%s)\n"
                    : "  call i64 @\"nyash.string.substring_concat_hhii\"(%s)\n",
                concat_ab);
          }
          if (hako_llvmc_route_trace_enabled()) {
            snprintf(
                route_extra,
                sizeof(route_extra),
                "ii=%zu recv=%lld dst=%lld terms=%d start=%lld end=%lld",
                ii,
                resolved_recv,
                dst,
                concat_term_count,
                a0,
                a1);
            hako_llvmc_emit_route_trace(
                "string_substring_route",
                concat_term_count == 3 ? "substring_concat3_hhhii" : "substring_concat_hhii",
                "concat_chain",
                route_extra);
          }
          return 1;
        }
        if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
        if (a1) append_i64_arg_ref(ab, ab_cap, a1, 0);
        if (dst) {
          fprintf(f, "  %%r%lld = call i64 @\"nyash.string.substring_hii\"(%s)\n", dst, ab);
          set_type(dst, T_I64);
          set_origin(dst, ORG_STRING);
          set_scan_origin(dst, ORG_STRING);
          remember_string_substring_call(dst, recv, a0, a1);
        } else {
          fprintf(f, "  call i64 @\"nyash.string.substring_hii\"(%s)\n", ab);
        }
        return 1;
      }
