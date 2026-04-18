      enum GenericMethodSubstringRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_INSERT_MID_WINDOW = 0,
        HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_DIRECT_KERNEL_PLAN = 1,
        HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_STRING_SUBSTRING_CALL = 2,
      };

      auto enum GenericMethodSubstringRouteKind classify_generic_method_substring_route(
          int matched_insert_window,
          int matched_direct_kernel_plan) {
        if (matched_insert_window) {
          return HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_INSERT_MID_WINDOW;
        }
        if (matched_direct_kernel_plan) {
          return HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_DIRECT_KERNEL_PLAN;
        }
        return HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_STRING_SUBSTRING_CALL;
      }

      auto int emit_string_substring_concat_call(
          FILE* f,
          size_t ii,
          long long recv,
          long long dst,
          long long concat_first,
          long long concat_second,
          long long concat_third,
          int concat_term_count,
          long long a0,
          long long a1,
          const char* route_reason) {
        char concat_ab[256];
        char route_extra[256];

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
              recv,
              dst,
              concat_term_count,
              a0,
              a1);
          hako_llvmc_emit_route_trace(
              "string_substring_route",
              concat_term_count == 3 ? "substring_concat3_hhhii" : "substring_concat_hhii",
              route_reason,
              route_extra);
        }
        return 1;
      }

      auto int emit_piecewise_subrange_call(
          FILE* f,
          size_t ii,
          long long recv,
          long long dst,
          long long source_h,
          long long middle_reg,
          long long split,
          long long start,
          long long end,
          const char* route_reason) {
        const char* middle_const = get_str_const(middle_reg);
        char ab[256];
        char route_extra[256];

        if (!middle_const) {
          return 0;
        }
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), source_h, 1);
        strncat(ab, ", ptr getelementptr inbounds (", sizeof(ab) - strlen(ab) - 1);
        {
          char tail[128];
          snprintf(
              tail,
              sizeof(tail),
              "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
              strlen(middle_const) + 1,
              middle_reg);
          strncat(ab, tail, sizeof(ab) - strlen(ab) - 1);
        }
        append_i64_arg_ref(ab, sizeof(ab), split, 0);
        if (start) append_i64_arg_ref(ab, sizeof(ab), start, 0);
        if (end) append_i64_arg_ref(ab, sizeof(ab), end, 0);
        if (dst) {
          fprintf(
              f,
              "  %%r%lld = call i64 @\"nyash.string.piecewise_subrange_hsiii\"(%s)\n",
              dst,
              ab);
          set_type(dst, T_I64);
          set_origin(dst, ORG_STRING);
          set_scan_origin(dst, ORG_STRING);
          remember_string_substring_call(dst, recv, start, end);
        } else {
          fprintf(f, "  call i64 @\"nyash.string.piecewise_subrange_hsiii\"(%s)\n", ab);
        }
        if (hako_llvmc_route_trace_enabled()) {
          snprintf(
              route_extra,
              sizeof(route_extra),
              "ii=%zu recv=%lld dst=%lld source=%lld middle=%lld split=%lld start=%lld end=%lld",
              ii,
              recv,
              dst,
              source_h,
              middle_reg,
              split,
              start,
              end);
          hako_llvmc_emit_route_trace(
              "string_substring_route",
              "piecewise_subrange_hsiii",
              route_reason,
              route_extra);
        }
        return 1;
      }

      auto int emit_const_suffix_piecewise_subrange_call(
          FILE* f,
          size_t ii,
          long long recv,
          long long dst,
          long long source_h,
          long long middle_reg,
          long long split_len,
          long long start,
          long long end,
          const char* route_reason) {
        const char* middle_const = get_str_const(middle_reg);
        long long start_const = 0;
        long long end_const = 0;
        char ab[256];
        char route_extra[256];

        if (!(dst && source_h && middle_reg && middle_const)) return 0;
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), source_h, 1);
        strncat(ab, ", ptr getelementptr inbounds (", sizeof(ab) - strlen(ab) - 1);
        {
          char tail[128];
          snprintf(
              tail,
              sizeof(tail),
              "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
              strlen(middle_const) + 1,
              middle_reg);
          strncat(ab, tail, sizeof(ab) - strlen(ab) - 1);
        }
        {
          char tail[64];
          snprintf(tail, sizeof(tail), ", i64 %lld", split_len);
          strncat(ab, tail, sizeof(ab) - strlen(ab) - 1);
        }
        if (start) append_i64_arg_ref(ab, sizeof(ab), start, 0);
        if (end) append_i64_arg_ref(ab, sizeof(ab), end, 0);
        fprintf(
            f,
            "  %%r%lld = call i64 @\"nyash.string.piecewise_subrange_hsiii\"(%s)\n",
            dst,
            ab);
        set_type(dst, T_I64);
        set_origin(dst, ORG_STRING);
        set_scan_origin(dst, ORG_STRING);
        remember_string_substring_call(dst, recv, start, end);
        if (has_const(resolve_copy_source_reg(start), &start_const) &&
            has_const(resolve_copy_source_reg(end), &end_const) &&
            end_const >= start_const) {
          put_known_string_len(dst, end_const - start_const);
        }
        if (hako_llvmc_route_trace_enabled()) {
          snprintf(
              route_extra,
              sizeof(route_extra),
              "ii=%zu recv=%lld dst=%lld source=%lld middle=%lld split_len=%lld start=%lld end=%lld",
              ii,
              recv,
              dst,
              source_h,
              middle_reg,
              split_len,
              start,
              end);
          hako_llvmc_emit_route_trace(
              "string_substring_route",
              "piecewise_subrange_hsiii",
              route_reason,
              route_extra);
        }
        return 1;
      }

      auto int match_piecewise_slot_hop_substring_consumer(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long value,
          size_t* use_ii_out,
          long long* next_dst_out,
          long long* next_start_out,
          long long* next_end_out,
          size_t* copy_iis_out,
          int* copy_n_out) {
        size_t use_ii = 0;
        long long final_reg = 0;
        yyjson_val* use_ins = NULL;
        yyjson_val* use_mc = NULL;
        yyjson_val* use_cal = NULL;
        yyjson_val* use_args = NULL;
        yyjson_val* start_val = NULL;
        yyjson_val* end_val = NULL;
        const char* use_op = NULL;
        const char* use_ctype = NULL;
        const char* use_mname = NULL;
        long long next_dst = 0;

        if (!(value && hako_llvmc_string_kernel_plan_is_slot_text_carrier(value))) {
          return 0;
        }
        if (!follow_single_use_copy_chain_to_use(
                ii + 1, ilen, insts, value, &use_ii, &final_reg, copy_iis_out, copy_n_out)) {
          return 0;
        }
        use_ins = yyjson_arr_get(insts, use_ii);
        use_op = use_ins ? read_str(use_ins, "op") : NULL;
        if (!(use_op && !strcmp(use_op, "mir_call"))) {
          return 0;
        }
        next_dst = read_int(use_ins, "dst");
        if (!next_dst || hako_llvmc_string_kernel_plan_is_slot_text_carrier(next_dst)) {
          return 0;
        }
        use_mc = yyjson_obj_get(use_ins, "mir_call");
        use_cal = use_mc ? yyjson_obj_get(use_mc, "callee") : NULL;
        use_args = use_mc ? yyjson_obj_get(use_mc, "args") : NULL;
        use_ctype = use_cal ? read_str(use_cal, "type") : NULL;
        use_mname = use_cal
                        ? (read_str(use_cal, "method") ? read_str(use_cal, "method")
                                                        : read_str(use_cal, "name"))
                        : NULL;
        if (use_ctype && !strcmp(use_ctype, "Method") && use_mname &&
            !strcmp(use_mname, "substring") && use_args && yyjson_is_arr(use_args) &&
            yyjson_arr_size(use_args) == 2 &&
            resolve_copy_source_reg(read_int(use_cal, "receiver")) == final_reg) {
          start_val = yyjson_arr_get(use_args, 0);
          end_val = yyjson_arr_get(use_args, 1);
        } else if (use_ctype && !strcmp(use_ctype, "Extern") && use_mname &&
                   !strcmp(use_mname, "nyash.string.substring_hii") && use_args &&
                   yyjson_is_arr(use_args) && yyjson_arr_size(use_args) == 3 &&
                   resolve_copy_source_reg((long long)yyjson_get_sint(yyjson_arr_get(use_args, 0))) ==
                       final_reg) {
          start_val = yyjson_arr_get(use_args, 1);
          end_val = yyjson_arr_get(use_args, 2);
        } else {
          return 0;
        }
        if (!(start_val && end_val && yyjson_is_num(start_val) && yyjson_is_num(end_val))) {
          return 0;
        }
        if (use_ii_out) *use_ii_out = use_ii;
        if (next_dst_out) *next_dst_out = next_dst;
        if (next_start_out) *next_start_out = (long long)yyjson_get_sint(start_val);
        if (next_end_out) *next_end_out = (long long)yyjson_get_sint(end_val);
        return 1;
      }

      auto int emit_piecewise_slot_hop_publish_call(
          FILE* f,
          size_t ii,
          long long intermediate_value,
          long long final_dst,
          long long source_h,
          long long middle_reg,
          long long split,
          long long start,
          long long end,
          long long hop_start,
          long long hop_end,
          const char* route_reason) {
        const char* middle_const = get_str_const(middle_reg);
        char ab[256];
        char hop_ab[128];
        char route_extra[256];

        if (!(final_dst && source_h && middle_reg && middle_const)) {
          return 0;
        }

        fprintf(
            f,
            "  %%slothop.slotsave.%lld.%zu = call ptr @llvm.stacksave()\n",
            final_dst,
            ii);
        fprintf(
            f,
            "  %%slothop.slot.%lld.%zu = alloca { i8, ptr, i64, i64 }, align 8\n",
            final_dst,
            ii);
        fprintf(
            f,
            "  store { i8, ptr, i64, i64 } zeroinitializer, ptr %%slothop.slot.%lld.%zu, align 8\n",
            final_dst,
            ii);

        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), source_h, 1);
        strncat(ab, ", ptr getelementptr inbounds (", sizeof(ab) - strlen(ab) - 1);
        {
          char tail[128];
          snprintf(
              tail,
              sizeof(tail),
              "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
              strlen(middle_const) + 1,
              middle_reg);
          strncat(ab, tail, sizeof(ab) - strlen(ab) - 1);
        }
        append_i64_arg_ref(ab, sizeof(ab), split, 0);
        append_i64_arg_ref(ab, sizeof(ab), start, 0);
        append_i64_arg_ref(ab, sizeof(ab), end, 0);
        fprintf(
            f,
            "  call i64 @\"nyash.string.kernel_slot_piecewise_subrange_hsiii\"(ptr %%slothop.slot.%lld.%zu, %s)\n",
            final_dst,
            ii,
            ab);

        hop_ab[0] = '\0';
        append_i64_arg_ref(hop_ab, sizeof(hop_ab), hop_start, 1);
        append_i64_arg_ref(hop_ab, sizeof(hop_ab), hop_end, 0);
        fprintf(
            f,
            "  call i64 @\"nyash.string.kernel_slot_substring_hii_in_place\"(ptr %%slothop.slot.%lld.%zu, %s)\n",
            final_dst,
            ii,
            hop_ab);
        fprintf(
            f,
            "  %%r%lld = call i64 @\"nyash.string.kernel_slot_publish_h\"(ptr %%slothop.slot.%lld.%zu)\n",
            final_dst,
            final_dst,
            ii);
        fprintf(
            f,
            "  call void @llvm.stackrestore(ptr %%slothop.slotsave.%lld.%zu)\n",
            final_dst,
            ii);
        set_type(final_dst, T_I64);
        set_origin(final_dst, ORG_STRING);
        set_scan_origin(final_dst, ORG_STRING);
        if (hako_llvmc_route_trace_enabled()) {
          snprintf(
              route_extra,
              sizeof(route_extra),
              "ii=%zu mid_value=%lld final_dst=%lld source=%lld middle=%lld split=%lld start=%lld end=%lld hop_start=%lld hop_end=%lld",
              ii,
              intermediate_value,
              final_dst,
              source_h,
              middle_reg,
              split,
              start,
              end,
              hop_start,
              hop_end);
          hako_llvmc_emit_route_trace(
              "string_substring_route",
              "piecewise_slot_hop_publish",
              route_reason,
              route_extra);
        }
        return 1;
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
        long long piecewise_source = 0;
        long long piecewise_middle = 0;
        long long piecewise_split = 0;
        long long const_suffix_source = 0;
        long long const_suffix_middle = 0;
        long long const_suffix_split_len = 0;
        long long concat_first = 0;
        long long concat_second = 0;
        long long concat_third = 0;
        int concat_term_count = 0;
        int matched_direct_kernel_plan = 0;
        size_t piecewise_use_ii = 0;
        size_t piecewise_copy_iis[8];
        int piecewise_copy_n = 0;
        long long piecewise_next_dst = 0;
        long long piecewise_next_start = 0;
        long long piecewise_next_end = 0;
        if (classify_generic_method_substring_route(
                try_emit_string_insert_mid_window_call(
                    f, ii, ilen, insts, recv, dst, a0, a1, skip_inst),
                0) ==
            HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_INSERT_MID_WINDOW) {
          return 1;
        }
        if (resolved_recv &&
            find_deferred_const_suffix_kernel_slot(
                resolved_recv, &const_suffix_source, &const_suffix_middle) &&
            known_string_value_len(const_suffix_source, &const_suffix_split_len) &&
            emit_const_suffix_piecewise_subrange_call(
                f,
                ii,
                resolved_recv,
                dst,
                const_suffix_source,
                const_suffix_middle,
                const_suffix_split_len,
                a0,
                a1,
                "const_suffix_deferred_producer")) {
          return 1;
        }
        if (resolved_recv &&
            find_deferred_piecewise_subrange(
                resolved_recv, &piecewise_source, &piecewise_middle, &piecewise_split)) {
          if (dst &&
              match_piecewise_slot_hop_substring_consumer(
                  ii,
                  ilen,
                  insts,
                  dst,
                  &piecewise_use_ii,
                  &piecewise_next_dst,
                  &piecewise_next_start,
                  &piecewise_next_end,
                  piecewise_copy_iis,
                  &piecewise_copy_n) &&
              emit_piecewise_slot_hop_publish_call(
                  f,
                  ii,
                  dst,
                  piecewise_next_dst,
                  piecewise_source,
                  piecewise_middle,
                  piecewise_split,
                  a0,
                  a1,
                  piecewise_next_start,
                  piecewise_next_end,
                  "slot_text_boundary")) {
            skip_inst[piecewise_use_ii] = 1;
            for (int ci = 0; ci < piecewise_copy_n && ci < 8; ci++) {
              skip_inst[piecewise_copy_iis[ci]] = 1;
            }
            return 1;
          }
          if (emit_piecewise_subrange_call(
                  f,
                  ii,
                  resolved_recv,
                  dst,
                  piecewise_source,
                  piecewise_middle,
                  piecewise_split,
                  a0,
                  a1,
                  "publication_boundary")) {
            return 1;
          }
        }
        if (resolved_recv &&
            hako_llvmc_string_corridor_read_concat_triplet_plan_values(
                resolved_recv,
                "direct_kernel_entry",
                &concat_first,
                &concat_second,
                &concat_third)) {
          concat_first = resolve_copy_source_reg(concat_first);
          concat_second = resolve_copy_source_reg(concat_second);
          concat_third = resolve_copy_source_reg(concat_third);
          if (concat_first && concat_second && concat_third) {
            matched_direct_kernel_plan = 1;
          }
        }
        if (classify_generic_method_substring_route(0, matched_direct_kernel_plan) ==
            HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_DIRECT_KERNEL_PLAN) {
          return emit_string_substring_concat_call(
              f,
              ii,
              resolved_recv,
              dst,
              concat_first,
              concat_second,
              concat_third,
              3,
              a0,
              a1,
              "direct_kernel_plan_proof");
        }
        if (resolved_recv &&
            resolve_string_concat_chain_terms(
                resolved_recv,
                &concat_first,
                &concat_second,
                &concat_third,
                &concat_term_count)) {
          return emit_string_substring_concat_call(
              f,
              ii,
              resolved_recv,
              dst,
              concat_first,
              concat_second,
              concat_third,
              concat_term_count,
              a0,
              a1,
              "concat_chain");
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
