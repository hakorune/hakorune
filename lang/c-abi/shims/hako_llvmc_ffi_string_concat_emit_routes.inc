      auto int try_emit_string_insert_mid_const_call(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          long long lhs,
          long long rhs) {
        long long prefix = 0;
        long long middle = 0;
        long long src_prefix = 0;
        long long src_suffix = 0;
        long long prefix_start = 0;
        long long prefix_end = 0;
        long long suffix_start = 0;
        long long suffix_end = 0;
        long long len_src = 0;
        long long zero = 0;
        long long recv = 0;
        long long plan_middle = 0;
        int used_plan_window = 0;
        int direct_set = 0;
        int shared_receiver = 0;
        const char* reason = "shape_match";
        char extra[256];
        const char* middle_const = NULL;
        if (!(dst && lhs && rhs)) return 0;
        direct_set = hako_llvmc_string_kernel_plan_direct_set_consumer(dst);
        shared_receiver = hako_llvmc_string_kernel_plan_shared_receiver(dst);
        if (!(direct_set || shared_receiver)) return 0;
        lhs = resolve_copy_chain_source_reg(insts, ii + 1, lhs);
        rhs = resolve_copy_chain_source_reg(insts, ii + 1, rhs);
        if (!find_string_concat_pair(lhs, &prefix, &middle)) return 0;
        prefix = resolve_copy_chain_source_reg(insts, ii + 1, prefix);
        middle = resolve_copy_chain_source_reg(insts, ii + 1, middle);
        middle_const = get_str_const(middle);
        if (!middle_const) return 0;
        if (hako_llvmc_string_kernel_plan_read_insert_mid_window_values(
                dst,
                &src_prefix,
                &plan_middle,
                &prefix_end) &&
            resolve_copy_chain_source_reg(insts, ii + 1, plan_middle) == middle) {
          recv = src_prefix;
          used_plan_window = 1;
          reason = "kernel_plan_window_match";
        }

        if (!used_plan_window) {
          if (!find_string_substring_call(prefix, &src_prefix, &prefix_start, &prefix_end))
            return 0;
          if (!find_string_substring_call(rhs, &src_suffix, &suffix_start, &suffix_end))
            return 0;
          src_prefix = resolve_copy_chain_source_reg(insts, ii + 1, src_prefix);
          src_suffix = resolve_copy_chain_source_reg(insts, ii + 1, src_suffix);
          prefix_start = resolve_copy_chain_source_reg(insts, ii + 1, prefix_start);
          prefix_end = resolve_copy_chain_source_reg(insts, ii + 1, prefix_end);
          suffix_start = resolve_copy_chain_source_reg(insts, ii + 1, suffix_start);
          suffix_end = resolve_copy_chain_source_reg(insts, ii + 1, suffix_end);
          if (!(src_prefix && src_prefix == src_suffix)) return 0;
          if (!(has_const(prefix_start, &zero) && zero == 0)) return 0;
          if (!(prefix_end && prefix_end == suffix_start)) return 0;
          if (!find_string_length_call(suffix_end, &len_src)) return 0;
          len_src = resolve_copy_chain_source_reg(insts, ii + 1, len_src);
          if (len_src != src_prefix) return 0;
          recv = src_prefix;
        }

        remember_deferred_insert_mid_kernel_slot(dst, src_prefix, middle, prefix_end);
        if (shared_receiver) {
          remember_deferred_piecewise_subrange(dst, src_prefix, middle, prefix_end);
          reason = used_plan_window ? "kernel_plan_window_shared_match"
                                    : "shape_shared_match";
        }
        fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
        finalize_string_concat_result(dst);
        if (hako_llvmc_route_trace_enabled()) {
          snprintf(
              extra,
              sizeof(extra),
              "ii=%zu dst=%lld lhs=%lld rhs=%lld recv=%lld prefix=%lld middle=%lld suffix=%lld plan=%d shared_receiver=%d",
              ii,
              dst,
              lhs,
              rhs,
              recv,
              prefix,
              middle,
              rhs,
              used_plan_window,
              shared_receiver);
          hako_llvmc_emit_route_trace(
              "string_insert_mid_window",
              "hit",
              reason,
              extra);
        }
        return 1;
      }

      auto int try_emit_substring_concat3_piecewise_direct_set_call(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          yyjson_val* args) {
        long long prefix = 0;
        long long middle = 0;
        long long suffix = 0;
        long long start = 0;
        long long end = 0;
        long long source_prefix = 0;
        long long source_suffix = 0;
        long long prefix_start = 0;
        long long prefix_end = 0;
        long long suffix_start = 0;
        long long suffix_end = 0;
        long long len_src = 0;
        long long zero = 0;
        int used_plan_window = 0;
        const char* middle_const = NULL;
        const char* reason = "shape_match";
        char extra[256];

        if (!(dst && args && yyjson_is_arr(args) && yyjson_arr_size(args) == 5)) return 0;
        if (!hako_llvmc_string_kernel_plan_direct_set_consumer(dst)) return 0;

        prefix = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
        middle = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
        suffix = (long long)yyjson_get_sint(yyjson_arr_get(args, 2));
        start = (long long)yyjson_get_sint(yyjson_arr_get(args, 3));
        end = (long long)yyjson_get_sint(yyjson_arr_get(args, 4));

        prefix = resolve_copy_chain_source_reg(insts, ii + 1, prefix);
        middle = resolve_copy_chain_source_reg(insts, ii + 1, middle);
        suffix = resolve_copy_chain_source_reg(insts, ii + 1, suffix);
        middle_const = get_str_const(middle);
        if (!(prefix && middle && suffix && middle_const)) return 0;

        if (hako_llvmc_string_corridor_read_any_plan_source_window(
                prefix,
                &source_prefix,
                &prefix_start,
                &prefix_end) &&
            hako_llvmc_string_corridor_read_any_plan_source_window(
                suffix,
                &source_suffix,
                &suffix_start,
                &suffix_end) &&
            source_prefix &&
            source_prefix == source_suffix &&
            has_const(prefix_start, &zero) &&
            zero == 0 &&
            prefix_end &&
            prefix_end == suffix_start) {
          used_plan_window = 1;
          reason = "plan_window_match";
        }

        if (!used_plan_window) {
          if (!find_string_substring_call(prefix, &source_prefix, &prefix_start, &prefix_end))
            return 0;
          if (!find_string_substring_call(suffix, &source_suffix, &suffix_start, &suffix_end))
            return 0;
          source_prefix = resolve_copy_chain_source_reg(insts, ii + 1, source_prefix);
          source_suffix = resolve_copy_chain_source_reg(insts, ii + 1, source_suffix);
          prefix_start = resolve_copy_chain_source_reg(insts, ii + 1, prefix_start);
          prefix_end = resolve_copy_chain_source_reg(insts, ii + 1, prefix_end);
          suffix_start = resolve_copy_chain_source_reg(insts, ii + 1, suffix_start);
          suffix_end = resolve_copy_chain_source_reg(insts, ii + 1, suffix_end);
          source_prefix = resolve_copy_source_reg(source_prefix);
          source_suffix = resolve_copy_source_reg(source_suffix);
          prefix_start = resolve_copy_source_reg(prefix_start);
          prefix_end = resolve_copy_source_reg(prefix_end);
          suffix_start = resolve_copy_source_reg(suffix_start);
          suffix_end = resolve_copy_source_reg(suffix_end);
          if (!(source_prefix && source_prefix == source_suffix)) return 0;
          if (!(has_const(prefix_start, &zero) && zero == 0)) return 0;
          if (!(prefix_end && prefix_end == suffix_start)) return 0;
          if (!find_string_length_call(suffix_end, &len_src)) return 0;
          len_src = resolve_copy_chain_source_reg(insts, ii + 1, len_src);
          if (len_src != source_prefix) return 0;
        }

        remember_deferred_piecewise_subrange(dst, source_prefix, middle, prefix_end);
        remember_string_substring_call(dst, dst, start, end);
        fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
        finalize_string_concat_result(dst);
        if (hako_llvmc_route_trace_enabled()) {
          snprintf(
              extra,
              sizeof(extra),
              "ii=%zu dst=%lld source=%lld prefix=%lld middle=%lld suffix=%lld split=%lld start=%lld end=%lld plan=%d",
              ii,
              dst,
              source_prefix,
              prefix,
              middle,
              suffix,
              prefix_end,
              start,
              end,
              used_plan_window);
          hako_llvmc_emit_route_trace(
              "substring_concat3_piecewise_direct_set",
              "hit",
              reason,
              extra);
        }
        return 1;
      }

      auto int try_emit_string_concat3_chain_call(
          FILE* f,
          long long dst,
          long long lhs,
          long long rhs) {
        long long a = 0;
        long long b = 0;
        char ab[192];
        ab[0] = '\0';
        if (find_string_concat_pair(lhs, &a, &b)) {
          append_string_concat_arg_ref(ab, sizeof(ab), a, 1);
          append_string_concat_arg_ref(ab, sizeof(ab), b, 0);
          append_string_concat_arg_ref(ab, sizeof(ab), rhs, 0);
          fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat3_hhh\"(%s)\n", dst, ab);
          finalize_string_concat_result(dst);
          remember_string_concat_triple(dst, a, b, rhs);
          if (hako_llvmc_route_trace_enabled()) {
            char extra[256];
            snprintf(
                extra,
                sizeof(extra),
                "dst=%lld lhs=%lld rhs=%lld producer_kind=Concat3 boundary_kind=Store post_store_use=None known_len=-1 direct_set_only=0",
                dst,
                lhs,
                rhs);
            hako_llvmc_emit_route_trace("string_concat3_window", "hit", "pair_lhs_match", extra);
          }
          return 1;
        }
        if (find_string_concat_pair(rhs, &a, &b)) {
          append_string_concat_arg_ref(ab, sizeof(ab), lhs, 1);
          append_string_concat_arg_ref(ab, sizeof(ab), a, 0);
          append_string_concat_arg_ref(ab, sizeof(ab), b, 0);
          fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat3_hhh\"(%s)\n", dst, ab);
          finalize_string_concat_result(dst);
          remember_string_concat_triple(dst, lhs, a, b);
          if (hako_llvmc_route_trace_enabled()) {
            char extra[256];
            snprintf(
                extra,
                sizeof(extra),
                "dst=%lld lhs=%lld rhs=%lld producer_kind=Concat3 boundary_kind=Store post_store_use=None known_len=-1 direct_set_only=0",
                dst,
                lhs,
                rhs);
            hako_llvmc_emit_route_trace("string_concat3_window", "hit", "pair_rhs_match", extra);
          }
          return 1;
        }
        return 0;
      }

      auto int emit_supported_string_extern_mir_call(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          enum MirCallStringExternSurfaceKind kind,
          long long dst,
          yyjson_val* args,
          unsigned char* skip_inst) {
        char ab[192];
        size_t argc = 0;
        if (!(args && yyjson_is_arr(args))) return -1;
        argc = yyjson_arr_size(args);
        ab[0] = '\0';

        if (kind == HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_CONCAT_HH) {
          if (argc != 2) return -1;
          append_string_concat_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 0)), 1);
          append_string_concat_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 1)), 0);
          if (dst) {
            fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat_hh\"(%s)\n", dst, ab);
            finalize_string_concat_result(dst);
            remember_string_concat_pair(
                dst,
                (long long)yyjson_get_sint(yyjson_arr_get(args, 0)),
                (long long)yyjson_get_sint(yyjson_arr_get(args, 1)));
          } else {
            fprintf(f, "  call i64 @\"nyash.string.concat_hh\"(%s)\n", ab);
          }
          return 1;
        }

        if (kind == HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_CONCAT3_HHH) {
          if (argc != 3) return -1;
          append_string_concat_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 0)), 1);
          append_string_concat_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 1)), 0);
          append_string_concat_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 2)), 0);
          if (dst) {
            fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat3_hhh\"(%s)\n", dst, ab);
            finalize_string_concat_result(dst);
            remember_string_concat_triple(
                dst,
                (long long)yyjson_get_sint(yyjson_arr_get(args, 0)),
                (long long)yyjson_get_sint(yyjson_arr_get(args, 1)),
                (long long)yyjson_get_sint(yyjson_arr_get(args, 2)));
          } else {
            fprintf(f, "  call i64 @\"nyash.string.concat3_hhh\"(%s)\n", ab);
          }
          return 1;
        }

        if (kind == HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_INSERT_HSI) {
          yyjson_val* use_ins = NULL;
          yyjson_val* use_mc = NULL;
          yyjson_val* use_cal = NULL;
          yyjson_val* use_args = NULL;
          yyjson_val* use_start_val = NULL;
          yyjson_val* use_end_val = NULL;
          const char* use_op = NULL;
          const char* use_ctype = NULL;
          const char* use_mname = NULL;
          const char* reason = "emit_insert_hsi";
          size_t use_ii = 0;
          long long plan_source = 0;
          long long plan_start = 0;
          long long plan_end = 0;
          long long final_reg = 0;
          long long source = 0;
          long long middle = 0;
          long long split = 0;
          int shared_receiver = 0;
          const char* middle_const = NULL;
          if (argc != 3) return -1;
          source = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
          middle = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
          split = (long long)yyjson_get_sint(yyjson_arr_get(args, 2));
          source = resolve_copy_chain_source_reg(insts, ii + 1, source);
          middle = resolve_copy_chain_source_reg(insts, ii + 1, middle);
          split = resolve_copy_chain_source_reg(insts, ii + 1, split);
          middle_const = get_str_const(middle);
          if (!middle_const) return -1;
          int direct_set = dst ? hako_llvmc_string_kernel_plan_direct_set_consumer(dst) : 0;
          shared_receiver = hako_llvmc_string_kernel_plan_shared_receiver(dst);
          if (dst && (direct_set || shared_receiver)) {
            remember_deferred_insert_mid_kernel_slot(dst, source, middle, split);
            if (shared_receiver) {
              remember_deferred_piecewise_subrange(dst, source, middle, split);
            }
            fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
            finalize_string_concat_result(dst);
            reason = shared_receiver
                         ? "publication_boundary_deferred_insert_mid_kernel_slot_shared"
                         : "publication_boundary_deferred_insert_mid_kernel_slot";
            if (hako_llvmc_route_trace_enabled()) {
              char extra[224];
              snprintf(
                  extra,
                  sizeof(extra),
                  "ii=%zu dst=%lld source=%lld middle=%lld split=%lld shared_receiver=%d",
                  ii,
                  dst,
                  source,
                  middle,
                  split,
                  shared_receiver);
              hako_llvmc_emit_route_trace(
                  "mir_call_string_extern",
                  "hit",
                  reason,
                  extra);
            }
            return 1;
          }
          if (dst &&
              follow_single_use_copy_chain_to_use(
                  ii + 1, ilen, insts, dst, &use_ii, &final_reg, NULL, NULL)) {
            use_ins = yyjson_arr_get(insts, use_ii);
            use_op = use_ins ? read_str(use_ins, "op") : NULL;
            use_mc = use_ins ? yyjson_obj_get(use_ins, "mir_call") : NULL;
            use_cal = use_mc ? yyjson_obj_get(use_mc, "callee") : NULL;
            use_args = use_mc ? yyjson_obj_get(use_mc, "args") : NULL;
            use_start_val = use_args ? yyjson_arr_get(use_args, 1) : NULL;
            use_end_val = use_args ? yyjson_arr_get(use_args, 2) : NULL;
            use_ctype = use_cal ? read_str(use_cal, "type") : NULL;
            use_mname = use_cal ? (read_str(use_cal, "method") ? read_str(use_cal, "method")
                                                                : read_str(use_cal, "name"))
                                : NULL;
            if (use_op && !strcmp(use_op, "mir_call") && use_ctype &&
                !strcmp(use_ctype, "Extern") && use_mname &&
                !strcmp(use_mname, "nyash.string.substring_hii") && use_args &&
                yyjson_is_arr(use_args) && yyjson_arr_size(use_args) == 3 &&
                (long long)yyjson_get_sint(yyjson_arr_get(use_args, 0)) == final_reg &&
                hako_llvmc_string_kernel_plan_read_publication_boundary_window(
                    read_int(use_ins, "dst"),
                    "publish_now_not_required_before_first_external_boundary",
                    &plan_source,
                    &plan_start,
                    &plan_end) &&
                use_start_val && yyjson_is_num(use_start_val) && use_end_val &&
                yyjson_is_num(use_end_val) &&
                plan_source == dst &&
                plan_start == (long long)yyjson_get_sint(use_start_val) &&
                plan_end == (long long)yyjson_get_sint(use_end_val)) {
              remember_deferred_piecewise_subrange(dst, source, middle, split);
              fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
              finalize_string_concat_result(dst);
              reason = "publication_boundary_deferred_piecewise";
              if (hako_llvmc_route_trace_enabled()) {
                char extra[224];
                snprintf(
                    extra,
                    sizeof(extra),
                    "ii=%zu dst=%lld source=%lld middle=%lld split=%lld use_ii=%zu final_reg=%lld",
                    ii,
                    dst,
                    source,
                    middle,
                    split,
                    use_ii,
                    final_reg);
                hako_llvmc_emit_route_trace(
                    "mir_call_string_extern",
                    "hit",
                    reason,
                    extra);
              }
              return 1;
            }
          }
          append_i64_arg_ref(ab, sizeof(ab), source, 1);
          strncat(ab, ", ptr getelementptr inbounds (", sizeof(ab) - strlen(ab) - 1);
          {
            char tail[128];
            snprintf(
                tail,
                sizeof(tail),
                "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
                strlen(middle_const) + 1,
                middle);
            strncat(ab, tail, sizeof(ab) - strlen(ab) - 1);
          }
          append_i64_arg_ref(ab, sizeof(ab), split, 0);
          if (dst) {
            fprintf(f, "  %%r%lld = call i64 @\"nyash.string.insert_hsi\"(%s)\n", dst, ab);
            finalize_string_concat_result(dst);
          } else {
            fprintf(f, "  call i64 @\"nyash.string.insert_hsi\"(%s)\n", ab);
          }
          if (hako_llvmc_route_trace_enabled()) {
            char extra[192];
            snprintf(
                extra,
                sizeof(extra),
                "argc=%zu source=%lld middle=%lld split=%lld",
                argc,
                source,
                middle,
                split);
            hako_llvmc_emit_route_trace(
                "mir_call_string_extern",
                "hit",
                reason,
                extra);
          }
          return 1;
        }

        if (kind == HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_SUBSTRING_HII) {
          long long recv_reg = 0;
          long long start_reg = 0;
          long long end_reg = 0;
          long long piecewise_source = 0;
          long long piecewise_middle = 0;
          long long piecewise_split = 0;
          long long nested_piecewise_root = 0;
          long long nested_start = 0;
          long long nested_end = 0;
          long long nested_source = 0;
          long long nested_middle = 0;
          long long nested_split = 0;
          size_t piecewise_use_ii = 0;
          size_t piecewise_copy_iis[8];
          int piecewise_copy_n = 0;
          long long piecewise_next_dst = 0;
          long long piecewise_next_start = 0;
          long long piecewise_next_end = 0;
          int direct_set = 0;
          int shared_receiver = 0;
          const char* middle_const = NULL;
          if (argc != 3) return -1;
          recv_reg = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
          start_reg = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
          end_reg = (long long)yyjson_get_sint(yyjson_arr_get(args, 2));
          recv_reg = resolve_copy_chain_source_reg(insts, ii + 1, recv_reg);
          start_reg = resolve_copy_chain_source_reg(insts, ii + 1, start_reg);
          end_reg = resolve_copy_chain_source_reg(insts, ii + 1, end_reg);
          if (find_deferred_piecewise_subrange(
                  recv_reg, &piecewise_source, &piecewise_middle, &piecewise_split)) {
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
                    start_reg,
                    end_reg,
                    piecewise_next_start,
                    piecewise_next_end,
                    "slot_text_boundary")) {
              skip_inst[piecewise_use_ii] = 1;
              for (int ci = 0; ci < piecewise_copy_n && ci < 8; ci++) {
                skip_inst[piecewise_copy_iis[ci]] = 1;
              }
              return 1;
            }
            middle_const = get_str_const(piecewise_middle);
            if (middle_const) {
              direct_set = dst ? hako_llvmc_string_kernel_plan_direct_set_consumer(dst) : 0;
              shared_receiver = hako_llvmc_string_kernel_plan_shared_receiver(dst);
              if (dst && (direct_set || shared_receiver)) {
                remember_string_substring_call(dst, recv_reg, start_reg, end_reg);
                fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
                finalize_string_concat_result(dst);
                if (hako_llvmc_route_trace_enabled()) {
                  char extra[224];
                  snprintf(
                      extra,
                      sizeof(extra),
                      "ii=%zu dst=%lld recv=%lld source=%lld middle=%lld split=%lld start=%lld end=%lld",
                      ii,
                      dst,
                      recv_reg,
                      piecewise_source,
                      piecewise_middle,
                      piecewise_split,
                      start_reg,
                      end_reg);
                  hako_llvmc_emit_route_trace(
                      "mir_call_string_extern",
                      "hit",
                      shared_receiver ? "publication_boundary_deferred_piecewise_shared"
                                      : "publication_boundary_deferred_piecewise",
                      extra);
                }
                return 1;
              }
              ab[0] = '\0';
              append_i64_arg_ref(ab, sizeof(ab), piecewise_source, 1);
              strncat(ab, ", ptr getelementptr inbounds (", sizeof(ab) - strlen(ab) - 1);
              {
                char tail[128];
                snprintf(
                    tail,
                    sizeof(tail),
                    "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
                    strlen(middle_const) + 1,
                    piecewise_middle);
                strncat(ab, tail, sizeof(ab) - strlen(ab) - 1);
              }
              append_i64_arg_ref(ab, sizeof(ab), piecewise_split, 0);
              append_i64_arg_ref(ab, sizeof(ab), start_reg, 0);
              append_i64_arg_ref(ab, sizeof(ab), end_reg, 0);
              if (dst) {
                fprintf(
                    f,
                    "  %%r%lld = call i64 @\"nyash.string.piecewise_subrange_hsiii\"(%s)\n",
                    dst,
                    ab);
                set_type(dst, T_I64);
                set_origin(dst, ORG_STRING);
                set_scan_origin(dst, ORG_STRING);
                remember_string_substring_call(dst, recv_reg, start_reg, end_reg);
              } else {
                fprintf(f, "  call i64 @\"nyash.string.piecewise_subrange_hsiii\"(%s)\n", ab);
              }
              if (hako_llvmc_route_trace_enabled()) {
                char extra[224];
                snprintf(
                    extra,
                    sizeof(extra),
                    "recv=%lld source=%lld middle=%lld split=%lld start=%lld end=%lld",
                    recv_reg,
                    piecewise_source,
                    piecewise_middle,
                    piecewise_split,
                    start_reg,
                    end_reg);
                hako_llvmc_emit_route_trace(
                    "mir_call_string_extern",
                    "hit",
                    "piecewise_subrange_hsiii",
                    extra);
              }
              return 1;
            }
          }
          if (find_string_substring_call(
                  recv_reg, &nested_piecewise_root, &nested_start, &nested_end) &&
              find_deferred_piecewise_subrange(
                  nested_piecewise_root, &nested_source, &nested_middle, &nested_split)) {
            const char* nested_middle_const = get_str_const(nested_middle);
            if (nested_middle_const) {
              char base_start_operand[64];
              char start_operand[64];
              char end_operand[64];
              char ab_piecewise[256];
              char route_extra[256];
              long long composed_anchor = dst ? dst : recv_reg;
              long long nested_start_base =
                  resolve_copy_chain_source_reg(insts, ii + 1, nested_start);
              long long start_base =
                  resolve_copy_chain_source_reg(insts, ii + 1, start_reg);
              long long end_base =
                  resolve_copy_chain_source_reg(insts, ii + 1, end_reg);
              long long cv = 0;

              if (has_const(nested_start_base, &cv)) {
                snprintf(base_start_operand, sizeof(base_start_operand), "%lld", cv);
              } else {
                snprintf(base_start_operand, sizeof(base_start_operand), "%%r%lld", nested_start_base);
              }
              if (has_const(start_base, &cv)) {
                snprintf(start_operand, sizeof(start_operand), "%lld", cv);
              } else {
                snprintf(start_operand, sizeof(start_operand), "%%r%lld", start_base);
              }
              if (has_const(end_base, &cv)) {
                snprintf(end_operand, sizeof(end_operand), "%lld", cv);
              } else {
                snprintf(end_operand, sizeof(end_operand), "%%r%lld", end_base);
              }

              fprintf(
                  f,
                  "  %%piecewise.compose.start.%lld.%zu = add i64 %s, %s\n",
                  composed_anchor,
                  ii,
                  base_start_operand,
                  start_operand);
              fprintf(
                  f,
                  "  %%piecewise.compose.end.%lld.%zu = add i64 %s, %s\n",
                  composed_anchor,
                  ii,
                  base_start_operand,
                  end_operand);

              ab_piecewise[0] = '\0';
              append_i64_arg_ref(ab_piecewise, sizeof(ab_piecewise), nested_source, 1);
              strncat(
                  ab_piecewise,
                  ", ptr getelementptr inbounds (",
                  sizeof(ab_piecewise) - strlen(ab_piecewise) - 1);
              {
                char tail[128];
                snprintf(
                    tail,
                    sizeof(tail),
                    "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
                    strlen(nested_middle_const) + 1,
                    nested_middle);
                strncat(ab_piecewise, tail, sizeof(ab_piecewise) - strlen(ab_piecewise) - 1);
              }
              append_i64_arg_ref(ab_piecewise, sizeof(ab_piecewise), nested_split, 0);
              {
                char tail[128];
                snprintf(
                    tail,
                    sizeof(tail),
                    ", i64 %%piecewise.compose.start.%lld.%zu, i64 %%piecewise.compose.end.%lld.%zu",
                    composed_anchor,
                    ii,
                    composed_anchor,
                    ii);
                strncat(ab_piecewise, tail, sizeof(ab_piecewise) - strlen(ab_piecewise) - 1);
              }
              if (dst) {
                fprintf(
                    f,
                    "  %%r%lld = call i64 @\"nyash.string.piecewise_subrange_hsiii\"(%s)\n",
                    dst,
                    ab_piecewise);
                set_type(dst, T_I64);
                set_origin(dst, ORG_STRING);
                set_scan_origin(dst, ORG_STRING);
                remember_string_substring_call(dst, recv_reg, start_reg, end_reg);
              } else {
                fprintf(f, "  call i64 @\"nyash.string.piecewise_subrange_hsiii\"(%s)\n", ab_piecewise);
              }
              if (hako_llvmc_route_trace_enabled()) {
                snprintf(
                    route_extra,
                    sizeof(route_extra),
                    "recv=%lld root=%lld source=%lld middle=%lld split=%lld base_start=%lld start=%lld end=%lld",
                    recv_reg,
                    nested_piecewise_root,
                    nested_source,
                    nested_middle,
                    nested_split,
                    nested_start,
                    start_reg,
                    end_reg);
                hako_llvmc_emit_route_trace(
                    "mir_call_string_extern",
                    "hit",
                    "piecewise_subrange_hsiii_composed_window",
                    route_extra);
              }
              return 1;
            }
          }
          append_i64_arg_ref(
              ab, sizeof(ab), recv_reg, 1);
          append_i64_arg_ref(ab, sizeof(ab), start_reg, 0);
          append_i64_arg_ref(ab, sizeof(ab), end_reg, 0);
          if (dst) {
            fprintf(f, "  %%r%lld = call i64 @\"nyash.string.substring_hii\"(%s)\n", dst, ab);
            set_type(dst, T_I64);
            set_origin(dst, ORG_STRING);
            set_scan_origin(dst, ORG_STRING);
          } else {
            fprintf(f, "  call i64 @\"nyash.string.substring_hii\"(%s)\n", ab);
          }
          if (hako_llvmc_route_trace_enabled()) {
            char extra[192];
            snprintf(extra, sizeof(extra), "argc=%zu args=%s", argc, ab);
            hako_llvmc_emit_route_trace(
                "mir_call_string_extern",
                "hit",
                "substring_hii",
                extra);
          }
          return 1;
        }

        if (kind == HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_SUBSTRING_CONCAT_HHII) {
          if (argc != 4) return -1;
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 0)), 1);
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 1)), 0);
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 2)), 0);
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 3)), 0);
          if (dst) {
            fprintf(
                f,
                "  %%r%lld = call i64 @\"nyash.string.substring_concat_hhii\"(%s)\n",
                dst,
                ab);
            set_type(dst, T_I64);
            set_origin(dst, ORG_STRING);
            set_scan_origin(dst, ORG_STRING);
          } else {
            fprintf(f, "  call i64 @\"nyash.string.substring_concat_hhii\"(%s)\n", ab);
          }
          return 1;
        }

        if (kind == HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_SUBSTRING_CONCAT3_HHHII) {
          if (argc != 5) return -1;
          if (try_emit_substring_concat3_piecewise_direct_set_call(
                  f, ii, ilen, insts, dst, args)) {
            return 1;
          }
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 0)), 1);
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 1)), 0);
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 2)), 0);
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 3)), 0);
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 4)), 0);
          if (dst) {
            fprintf(
                f,
                "  %%r%lld = call i64 @\"nyash.string.substring_concat3_hhhii\"(%s)\n",
                dst,
                ab);
            set_type(dst, T_I64);
            set_origin(dst, ORG_STRING);
            set_scan_origin(dst, ORG_STRING);
          } else {
            fprintf(f, "  call i64 @\"nyash.string.substring_concat3_hhhii\"(%s)\n", ab);
          }
          return 1;
        }

        if (kind == HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_SUBSTRING_LEN_HII) {
          if (argc != 3) return -1;
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 0)), 1);
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 1)), 0);
          append_i64_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 2)), 0);
          if (dst) {
            fprintf(
                f,
                "  %%r%lld = call i64 @\"nyash.string.substring_len_hii\"(%s)\n",
                dst,
                ab);
            set_type(dst, T_I64);
          } else {
            fprintf(f, "  call i64 @\"nyash.string.substring_len_hii\"(%s)\n", ab);
          }
          if (hako_llvmc_route_trace_enabled()) {
            char extra[192];
            snprintf(extra, sizeof(extra), "argc=%zu args=%s", argc, ab);
            hako_llvmc_emit_route_trace(
                "mir_call_string_extern",
                "hit",
                "substring_len_hii",
                extra);
          }
          return 1;
        }

        if (hako_llvmc_route_trace_enabled()) {
          char extra[192];
          snprintf(extra, sizeof(extra), "kind=%d argc=%zu", (int)kind, argc);
          hako_llvmc_emit_route_trace(
              "mir_call_string_extern",
              "miss",
              "unsupported",
              extra);
        }
        return -1;
      }
