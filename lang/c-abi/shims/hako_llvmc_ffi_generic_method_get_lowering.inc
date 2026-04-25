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
        struct ArrayRmwWindowRouteMetadata rmw_route_match;
        struct ArrayStringLenWindowRouteMetadata string_len_route_match;
        struct ArrayTextEditLenHalfRouteMetadata edit_lenhalf_match;
        struct ArrayTextResidenceSessionRouteMetadata residence_session_match;
        struct ArrayStringLoopCarryLenStoreMatch loopcarry_match;
        char loopcarry_ab[360];
        memset(&string_len_route_match, 0, sizeof(string_len_route_match));
        if (match_array_rmw_window_route_metadata(bid, ii, &rmw_route_match)) {
          if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
          fprintf(f, "  %%r%lld = call i64 @\"nyash.array.rmw_add1_hi\"(%s)\n", rmw_route_match.result_reg, ab);
          set_type(rmw_route_match.result_reg, T_I64);
          for (int si = 0; si < rmw_route_match.skip_n; si++) skip_inst[rmw_route_match.skip_iis[si]] = 1;
          return 1;
        }
        if (rmw_route_match.invalid) {
          return -1;
        }
        int loopcarry_len_store_hit = 0;
        int residence_session_hit = 0;
        if (hako_llvmc_window_array_text_read_enabled(
                bname, recv_org, runtime_array_string)) {
          residence_session_hit =
              match_array_text_residence_session_route_metadata(
                  bid, ii, &residence_session_match);
          if (residence_session_hit) {
            array_text_residence_session_to_loopcarry_update(
                &residence_session_match,
                &loopcarry_match);
            loopcarry_len_store_hit = 1;
          } else {
            loopcarry_len_store_hit =
                match_array_text_loopcarry_len_store_route_metadata(
                    bid, ii, &loopcarry_match);
          }
        }
        if (loopcarry_len_store_hit) {
          loopcarry_ab[0] = '\0';
          append_i64_arg_ref(loopcarry_ab, sizeof(loopcarry_ab), recv, 1);
          if (a0) append_i64_arg_ref(loopcarry_ab, sizeof(loopcarry_ab), a0, 0);
          strncat(
              loopcarry_ab,
              ", ptr getelementptr inbounds (",
              sizeof(loopcarry_ab) - strlen(loopcarry_ab) - 1);
          {
            char tail[128];
            snprintf(
                tail,
                sizeof(tail),
                "[%lld x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
                loopcarry_match.middle_len + 1,
                loopcarry_match.middle_reg);
            strncat(
                loopcarry_ab,
                tail,
                sizeof(loopcarry_ab) - strlen(loopcarry_ab) - 1);
          }
          {
            char tail[64];
            snprintf(tail, sizeof(tail), ", i64 %lld", loopcarry_match.middle_len);
            strncat(
                loopcarry_ab,
                tail,
                sizeof(loopcarry_ab) - strlen(loopcarry_ab) - 1);
          }
          fprintf(
              f,
              "  %%r%lld = call i64 @\"nyash.array.string_insert_mid_subrange_len_store_hisi\"(%s)\n",
              loopcarry_match.result_len_dst,
              loopcarry_ab);
          set_type(loopcarry_match.result_len_dst, T_I64);
          for (int si = 0; si < loopcarry_match.skip_n; si++) {
            skip_inst[loopcarry_match.skip_iis[si]] = 1;
          }
          return 1;
        }
        if (hako_llvmc_window_array_text_read_enabled(
                bname, recv_org, runtime_array_string) &&
            match_array_text_edit_lenhalf_route_metadata(bid, ii, &edit_lenhalf_match)) {
          char edit_ab[320];
          edit_ab[0] = '\0';
          append_i64_arg_ref(edit_ab, sizeof(edit_ab), recv, 1);
          if (a0) append_i64_arg_ref(edit_ab, sizeof(edit_ab), a0, 0);
          strncat(
              edit_ab,
              ", ptr getelementptr inbounds (",
              sizeof(edit_ab) - strlen(edit_ab) - 1);
          {
            char tail[128];
            snprintf(
                tail,
                sizeof(tail),
                "[%lld x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
                edit_lenhalf_match.middle_len + 1,
                edit_lenhalf_match.middle_reg);
            strncat(edit_ab, tail, sizeof(edit_ab) - strlen(edit_ab) - 1);
          }
          {
            char tail[64];
            snprintf(tail, sizeof(tail), ", i64 %lld", edit_lenhalf_match.middle_len);
            strncat(edit_ab, tail, sizeof(edit_ab) - strlen(edit_ab) - 1);
          }
          fprintf(
              f,
              "  %%r%lld = call i64 @\"nyash.array.string_insert_mid_lenhalf_store_hisi\"(%s)\n",
              dst,
              edit_ab);
          set_type(dst, T_I64);
          if (dst && runtime_array_string) {
            set_origin(dst, ORG_STRING);
            set_scan_origin(dst, ORG_STRING);
          }
          for (int si = 0; si < edit_lenhalf_match.skip_n; si++) {
            skip_inst[edit_lenhalf_match.skip_iis[si]] = 1;
          }
          if (hako_llvmc_route_trace_enabled()) {
            char edit_extra[256];
            snprintf(
                edit_extra,
                sizeof(edit_extra),
                "ii=%zu dst=%lld recv=%lld idx=%lld middle=%lld middle_len=%lld skip_n=%d proof=array_get_lenhalf_insert_mid_same_slot",
                ii,
                dst,
                recv,
                a0,
                edit_lenhalf_match.middle_reg,
                edit_lenhalf_match.middle_len,
                edit_lenhalf_match.skip_n);
            hako_llvmc_emit_route_trace(
                "array_text_edit_lenhalf",
                "hit",
                "mir_route_metadata",
                edit_extra);
          }
          return 1;
        }
        if (hako_llvmc_window_array_text_read_enabled(
                bname, recv_org, runtime_array_string) &&
            match_array_string_len_window_route_metadata(
                bid, ii, &string_len_route_match)) {
          if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
          if (string_len_route_match.source_only_insert_mid) {
            if (dst) {
              remember_array_string_get_source(dst, recv, a0);
              set_type(dst, T_I64);
              set_origin(dst, ORG_STRING);
              set_scan_origin(dst, ORG_STRING);
            }
            hako_llvmc_emit_array_string_len(f, string_len_route_match.len_dst, ab);
            if (dst) remember_string_length_call(string_len_route_match.len_dst, dst);
            for (int si = 0; si < string_len_route_match.skip_n; si++) {
              skip_inst[string_len_route_match.skip_iis[si]] = 1;
            }
            return 1;
          }
          if (string_len_route_match.keep_get_live) {
            hako_llvmc_emit_array_slot_load(
                f, dst, ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_REF);
            if (dst) {
              remember_array_string_get_source(dst, recv, a0);
            }
          }
          hako_llvmc_emit_array_string_len(f, string_len_route_match.len_dst, ab);
          if (string_len_route_match.keep_get_live && dst) {
            remember_string_length_call(string_len_route_match.len_dst, dst);
          }
          for (int si = 0; si < string_len_route_match.skip_n; si++) {
            skip_inst[string_len_route_match.skip_iis[si]] = 1;
          }
          return 1;
        }
        if (string_len_route_match.invalid) {
          return -1;
        }
        if (try_defer_array_string_indexof_get_window(
                f, blocks, bid, ii, ilen, insts, bname, mname, recv, a0, dst, recv_org,
                runtime_array_string, skip_inst)) {
          return 1;
        }
        if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
        int fallback_handled =
            emit_generic_method_get_fallback_by_policy(
                f, bid, ii, bname, dst, recv, a0, plan, ab);
        if (fallback_handled < 0) {
          return -1;
        }
        if (!fallback_handled) {
          return -1;
        }
        return 1;
      }
