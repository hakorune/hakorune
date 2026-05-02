      #include "hako_llvmc_ffi_string_concat_match_state.inc"


      auto int try_emit_string_insert_mid_window_call(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long recv,
          long long dst,
          long long start,
          long long end,
          unsigned char* skip_inst) {
        yyjson_val* second_ins = NULL;
        yyjson_val* second_mc = NULL;
        yyjson_val* second_cal = NULL;
        yyjson_val* second_args = NULL;
        const char* second_name = NULL;
        long long second_dst = 0;
        long long second_recv = 0;
        long long second_start = 0;
        long long second_end = 0;
        size_t prefix_use_ii = 0;
        size_t suffix_use_ii = 0;
        size_t final_use_ii = 0;
        size_t prefix_copy_iis[8];
        size_t suffix_copy_iis[8];
        int prefix_copy_n = 0;
        int suffix_copy_n = 0;
        long long prefix_final = 0;
        long long suffix_final = 0;
        yyjson_val* pair_ins = NULL;
        yyjson_val* final_ins = NULL;
        const char* pair_op = NULL;
        const char* pair_kind = NULL;
        const char* final_op = NULL;
        const char* final_kind = NULL;
        long long pair_lhs = 0;
        long long pair_rhs = 0;
        long long pair_dst = 0;
        long long final_lhs = 0;
        long long final_rhs = 0;
        long long result_reg = 0;
        long long middle_reg = 0;
        long long plan_src_prefix = 0;
        long long plan_src_suffix = 0;
        long long plan_prefix_start = 0;
        long long plan_prefix_end = 0;
        long long plan_suffix_start = 0;
        long long plan_suffix_end = 0;
        long long zero = 0;
        int direct_set = 0;
        int used_plan_window = 0;
        const char* middle_const = NULL;
        const char* reason = "shape_match";
        char extra[256];
        int matched = 0;
        if (!(recv && dst && ii + 1 < ilen)) {
          reason = "short_or_missing_recv";
          goto done;
        }
        if (!(has_const(start, &zero) && zero == 0)) {
          reason = "start_not_zero";
          goto done;
        }
        second_ins = yyjson_arr_get(insts, ii + 1);
        if (!(second_ins && read_str(second_ins, "op") && !strcmp(read_str(second_ins, "op"), "mir_call"))) {
          reason = "next_not_substring";
          goto done;
        }
        second_mc = yyjson_obj_get(second_ins, "mir_call");
        second_cal = second_mc ? yyjson_obj_get(second_mc, "callee") : NULL;
        second_args = second_mc ? yyjson_obj_get(second_mc, "args") : NULL;
        if (!(second_cal && second_args && yyjson_is_arr(second_args) && yyjson_arr_size(second_args) == 2)) {
          reason = "next_args_shape";
          goto done;
        }
        if (!(read_str(second_cal, "type") && !strcmp(read_str(second_cal, "type"), "Method"))) {
          reason = "next_not_method";
          goto done;
        }
        second_name = read_str(second_cal, "method");
        if (!second_name) second_name = read_str(second_cal, "name");
        if (!(second_name && !strcmp(second_name, "substring"))) {
          reason = "next_not_substring";
          goto done;
        }
        second_recv = read_int(second_cal, "receiver");
        second_start = (long long)yyjson_get_sint(yyjson_arr_get(second_args, 0));
        second_end = (long long)yyjson_get_sint(yyjson_arr_get(second_args, 1));
        second_dst = read_int(second_ins, "dst");
        if (!(second_recv == recv && second_start == end && second_end)) {
          reason = "second_substring_shape";
          goto done;
        }

        if (!follow_single_use_copy_chain_to_use(
                ii + 2, ilen, insts, dst, &prefix_use_ii, &prefix_final,
                prefix_copy_iis, &prefix_copy_n)) {
          reason = "prefix_chain";
          goto done;
        }
        if (!follow_single_use_copy_chain_to_use(
                ii + 2, ilen, insts, second_dst, &suffix_use_ii, &suffix_final,
                suffix_copy_iis, &suffix_copy_n)) {
          reason = "suffix_chain";
          goto done;
        }

        pair_ins = yyjson_arr_get(insts, prefix_use_ii);
        pair_op = pair_ins ? read_str(pair_ins, "op") : NULL;
        pair_kind = pair_ins ? (read_str(pair_ins, "op_kind") ? read_str(pair_ins, "op_kind")
                                                             : read_str(pair_ins, "operation"))
                             : NULL;
        if (!(pair_op && !strcmp(pair_op, "binop") && pair_kind &&
              (!strcmp(pair_kind, "Add") || !strcmp(pair_kind, "+")))) {
          reason = "pair_not_add";
          goto done;
        }
        pair_lhs = read_int(pair_ins, "lhs");
        pair_rhs = read_int(pair_ins, "rhs");
        pair_dst = read_int(pair_ins, "dst");
        if (pair_lhs == prefix_final) middle_reg = resolve_copy_chain_source_reg(insts, prefix_use_ii + 1, pair_rhs);
        else if (pair_rhs == prefix_final) middle_reg = resolve_copy_chain_source_reg(insts, prefix_use_ii + 1, pair_lhs);
        else {
          reason = "pair_prefix_mismatch";
          goto done;
        }
        middle_const = get_str_const(middle_reg);
        if (!middle_const) {
          reason = "middle_not_const";
          goto done;
        }

        if (!find_single_use_inst(prefix_use_ii + 1, ilen, insts, pair_dst, &final_use_ii)) {
          reason = "pair_use_not_single";
          goto done;
        }
        if (final_use_ii != suffix_use_ii) {
          reason = "final_use_mismatch";
          goto done;
        }
        final_ins = yyjson_arr_get(insts, final_use_ii);
        final_op = final_ins ? read_str(final_ins, "op") : NULL;
        final_kind = final_ins ? (read_str(final_ins, "op_kind") ? read_str(final_ins, "op_kind")
                                                                 : read_str(final_ins, "operation"))
                               : NULL;
        if (!(final_op && !strcmp(final_op, "binop") && final_kind &&
              (!strcmp(final_kind, "Add") || !strcmp(final_kind, "+")))) {
          reason = "final_not_add";
          goto done;
        }
        final_lhs = read_int(final_ins, "lhs");
        final_rhs = read_int(final_ins, "rhs");
        result_reg = read_int(final_ins, "dst");
        if (!((final_lhs == pair_dst && final_rhs == suffix_final) ||
              (final_rhs == pair_dst && final_lhs == suffix_final))) {
          reason = "final_suffix_mismatch";
          goto done;
        }
        direct_set = hako_llvmc_value_has_direct_set_consumer(result_reg);
        if (!direct_set) {
          reason = "set_consumer_missing";
          goto done;
        }
        if (hako_llvmc_string_corridor_read_any_plan_source_window(
                result_reg,
                &plan_src_prefix,
                &plan_prefix_start,
                &plan_prefix_end) &&
            plan_src_prefix &&
            plan_src_prefix == recv &&
            has_const(start, &zero) &&
            zero == 0 &&
            plan_prefix_start &&
            plan_prefix_start == start &&
            plan_prefix_end &&
            plan_prefix_end == end) {
          used_plan_window = 1;
          reason = "plan_window_match";
        } else if (hako_llvmc_string_corridor_read_any_plan_source_window(
                       dst,
                       &plan_src_prefix,
                       &plan_prefix_start,
                       &plan_prefix_end) &&
                   hako_llvmc_string_corridor_read_any_plan_source_window(
                       second_dst,
                       &plan_src_suffix,
                       &plan_suffix_start,
                       &plan_suffix_end) &&
                   plan_src_prefix &&
                   plan_src_prefix == plan_src_suffix &&
                   has_const(plan_prefix_start, &zero) &&
                   zero == 0 &&
                   plan_prefix_end &&
                   plan_prefix_end == plan_suffix_start) {
          used_plan_window = 1;
          reason = "plan_window_match";
        }

        remember_deferred_insert_mid_kernel_slot(result_reg, recv, middle_reg, end);
        fprintf(f, "  %%r%lld = add i64 0, 0\n", result_reg);
        finalize_string_concat_result(result_reg);
        skip_inst[ii + 1] = 1;
        for (int ci = 0; ci < prefix_copy_n && ci < 8; ci++) skip_inst[prefix_copy_iis[ci]] = 1;
        for (int ci = 0; ci < suffix_copy_n && ci < 8; ci++) skip_inst[suffix_copy_iis[ci]] = 1;
        skip_inst[prefix_use_ii] = 1;
        skip_inst[final_use_ii] = 1;
        matched = 1;
      done:
        if (hako_llvmc_route_trace_enabled()) {
          snprintf(
              extra,
              sizeof(extra),
              "ii=%zu recv=%lld dst=%lld start=%lld end=%lld second_dst=%lld result=%lld pair_use=%zu final_use=%zu prefix_final=%lld suffix_final=%lld direct_set=%d plan=%d",
              ii,
              recv,
              dst,
              start,
              end,
              second_dst,
              result_reg,
              prefix_use_ii,
              final_use_ii,
              prefix_final,
              suffix_final,
              direct_set,
              used_plan_window);
          hako_llvmc_emit_route_trace(
              "string_insert_mid_window",
              matched ? "hit" : "miss",
              reason,
              extra);
        }
        return matched;
      }

      auto int should_skip_string_const_box_for_concat_suffix(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long reg) {
        if (should_skip_string_const_box_for_insert_mid_window(ii, ilen, insts, reg)) return 1;
        size_t use_ii = 0;
        yyjson_val* use_ins = NULL;
        const char* op = NULL;
        const char* kind = NULL;
        long long lhs = 0;
        long long rhs = 0;
        long long concat_dst = 0;
        if (!get_str_const(reg)) return 0;
        if (!find_single_use_inst(ii + 1, ilen, insts, reg, &use_ii)) return 0;
        use_ins = yyjson_arr_get(insts, use_ii);
        op = use_ins ? read_str(use_ins, "op") : NULL;
        if (!(op && !strcmp(op, "binop"))) return 0;
        kind = read_str(use_ins, "op_kind");
        if (!kind) kind = read_str(use_ins, "operation");
        if (!(kind && (!strcmp(kind, "Add") || !strcmp(kind, "+")))) return 0;
        lhs = read_int(use_ins, "lhs");
        rhs = read_int(use_ins, "rhs");
        if (rhs != reg) return 0;
        concat_dst = read_int(use_ins, "dst");
        return hako_llvmc_value_has_direct_set_consumer(concat_dst);
      }

      auto int should_skip_string_const_box_for_insert_mid_window(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long reg) {
        size_t pair_use_ii = 0;
        size_t final_use_ii = 0;
        size_t prefix_def_ii = 0;
        size_t suffix_def_ii = 0;
        size_t len_def_ii = 0;
        int middle_copy_n = 0;
        yyjson_val* pair_ins = NULL;
        yyjson_val* final_ins = NULL;
        yyjson_val* prefix_def = NULL;
        yyjson_val* suffix_def = NULL;
        yyjson_val* len_def = NULL;
        yyjson_val* prefix_mc = NULL;
        yyjson_val* suffix_mc = NULL;
        yyjson_val* len_mc = NULL;
        yyjson_val* prefix_cal = NULL;
        yyjson_val* suffix_cal = NULL;
        yyjson_val* len_cal = NULL;
        yyjson_val* prefix_args = NULL;
        yyjson_val* suffix_args = NULL;
        yyjson_val* len_args = NULL;
        const char* pair_op = NULL;
        const char* pair_kind = NULL;
        const char* final_op = NULL;
        const char* final_kind = NULL;
        const char* prefix_name = NULL;
        const char* suffix_name = NULL;
        const char* len_name = NULL;
        long long middle_final = 0;
        long long prefix_final = 0;
        long long suffix_final = 0;
        long long pair_lhs = 0;
        long long pair_rhs = 0;
        long long pair_dst = 0;
        long long final_lhs = 0;
        long long final_rhs = 0;
        long long result_reg = 0;
        long long recv = 0;
        long long prefix_recv = 0;
        long long suffix_recv = 0;
        long long len_recv = 0;
        long long prefix_start = 0;
        long long prefix_end = 0;
        long long suffix_start = 0;
        long long suffix_end = 0;
        long long zero = 0;

        if (!get_str_const(reg)) return 0;
        if (!follow_single_use_copy_chain_to_use(
                ii + 1, ilen, insts, reg, &pair_use_ii, &middle_final, NULL, &middle_copy_n)) {
          return 0;
        }

        pair_ins = yyjson_arr_get(insts, pair_use_ii);
        pair_op = pair_ins ? read_str(pair_ins, "op") : NULL;
        pair_kind = pair_ins ? (read_str(pair_ins, "op_kind") ? read_str(pair_ins, "op_kind")
                                                              : read_str(pair_ins, "operation"))
                             : NULL;
        if (!(pair_op && !strcmp(pair_op, "binop") && pair_kind &&
              (!strcmp(pair_kind, "Add") || !strcmp(pair_kind, "+")))) {
          return 0;
        }
        pair_lhs = read_int(pair_ins, "lhs");
        pair_rhs = read_int(pair_ins, "rhs");
        pair_dst = read_int(pair_ins, "dst");
        if (pair_lhs == middle_final) {
          prefix_final = resolve_copy_chain_source_reg(insts, pair_use_ii + 1, pair_rhs);
        } else if (pair_rhs == middle_final) {
          prefix_final = resolve_copy_chain_source_reg(insts, pair_use_ii + 1, pair_lhs);
        } else {
          return 0;
        }

        if (!find_single_use_inst(pair_use_ii + 1, ilen, insts, pair_dst, &final_use_ii)) return 0;
        final_ins = yyjson_arr_get(insts, final_use_ii);
        final_op = final_ins ? read_str(final_ins, "op") : NULL;
        final_kind = final_ins ? (read_str(final_ins, "op_kind") ? read_str(final_ins, "op_kind")
                                                                 : read_str(final_ins, "operation"))
                               : NULL;
        if (!(final_op && !strcmp(final_op, "binop") && final_kind &&
              (!strcmp(final_kind, "Add") || !strcmp(final_kind, "+")))) {
          return 0;
        }
        final_lhs = read_int(final_ins, "lhs");
        final_rhs = read_int(final_ins, "rhs");
        result_reg = read_int(final_ins, "dst");
        if (final_lhs == pair_dst) {
          suffix_final = resolve_copy_chain_source_reg(insts, final_use_ii + 1, final_rhs);
        } else if (final_rhs == pair_dst) {
          suffix_final = resolve_copy_chain_source_reg(insts, final_use_ii + 1, final_lhs);
        } else {
          return 0;
        }
        if (!hako_llvmc_value_has_direct_set_consumer(result_reg)) return 0;

        prefix_final = resolve_copy_chain_source_reg(insts, pair_use_ii + 1, prefix_final);
        suffix_final = resolve_copy_chain_source_reg(insts, final_use_ii + 1, suffix_final);
        if (!find_def_inst_index(pair_use_ii + 1, insts, prefix_final, &prefix_def_ii)) return 0;
        if (!find_def_inst_index(final_use_ii + 1, insts, suffix_final, &suffix_def_ii)) return 0;
        prefix_def = yyjson_arr_get(insts, prefix_def_ii);
        suffix_def = yyjson_arr_get(insts, suffix_def_ii);
        if (!(prefix_def && suffix_def)) return 0;
        if (!(read_str(prefix_def, "op") && !strcmp(read_str(prefix_def, "op"), "mir_call"))) return 0;
        if (!(read_str(suffix_def, "op") && !strcmp(read_str(suffix_def, "op"), "mir_call"))) return 0;
        prefix_mc = yyjson_obj_get(prefix_def, "mir_call");
        suffix_mc = yyjson_obj_get(suffix_def, "mir_call");
        prefix_cal = prefix_mc ? yyjson_obj_get(prefix_mc, "callee") : NULL;
        suffix_cal = suffix_mc ? yyjson_obj_get(suffix_mc, "callee") : NULL;
        prefix_args = prefix_mc ? yyjson_obj_get(prefix_mc, "args") : NULL;
        suffix_args = suffix_mc ? yyjson_obj_get(suffix_mc, "args") : NULL;
        if (!(prefix_cal && suffix_cal && prefix_args && suffix_args &&
              yyjson_is_arr(prefix_args) && yyjson_arr_size(prefix_args) == 2 &&
              yyjson_is_arr(suffix_args) && yyjson_arr_size(suffix_args) == 2)) {
          return 0;
        }
        prefix_name = read_str(prefix_cal, "method");
        if (!prefix_name) prefix_name = read_str(prefix_cal, "name");
        suffix_name = read_str(suffix_cal, "method");
        if (!suffix_name) suffix_name = read_str(suffix_cal, "name");
        if (!(prefix_name && !strcmp(prefix_name, "substring"))) return 0;
        if (!(suffix_name && !strcmp(suffix_name, "substring"))) return 0;
        recv = 0;
        prefix_recv = resolve_copy_chain_source_reg(insts, prefix_def_ii + 1, read_int(prefix_cal, "receiver"));
        suffix_recv = resolve_copy_chain_source_reg(insts, suffix_def_ii + 1, read_int(suffix_cal, "receiver"));
        if (!(prefix_recv && prefix_recv == suffix_recv)) return 0;
        recv = prefix_recv;
        prefix_start = resolve_copy_chain_source_reg(
            insts, prefix_def_ii + 1, (long long)yyjson_get_sint(yyjson_arr_get(prefix_args, 0)));
        prefix_end = resolve_copy_chain_source_reg(
            insts, prefix_def_ii + 1, (long long)yyjson_get_sint(yyjson_arr_get(prefix_args, 1)));
        suffix_start = resolve_copy_chain_source_reg(
            insts, suffix_def_ii + 1, (long long)yyjson_get_sint(yyjson_arr_get(suffix_args, 0)));
        suffix_end = resolve_copy_chain_source_reg(
            insts, suffix_def_ii + 1, (long long)yyjson_get_sint(yyjson_arr_get(suffix_args, 1)));
        if (!(has_const(prefix_start, &zero) && zero == 0)) return 0;
        if (!(prefix_end && prefix_end == suffix_start)) return 0;
        suffix_end = resolve_copy_chain_source_reg(insts, suffix_def_ii + 1, suffix_end);
        if (!find_def_inst_index(suffix_def_ii + 1, insts, suffix_end, &len_def_ii)) return 0;
        len_def = yyjson_arr_get(insts, len_def_ii);
        if (!(len_def && read_str(len_def, "op") && !strcmp(read_str(len_def, "op"), "mir_call"))) return 0;
        len_mc = yyjson_obj_get(len_def, "mir_call");
        len_cal = len_mc ? yyjson_obj_get(len_mc, "callee") : NULL;
        len_args = len_mc ? yyjson_obj_get(len_mc, "args") : NULL;
        if (!(len_cal && len_args && yyjson_is_arr(len_args) && yyjson_arr_size(len_args) == 0)) return 0;
        len_name = read_str(len_cal, "method");
        if (!len_name) len_name = read_str(len_cal, "name");
        if (!(len_name && !strcmp(len_name, "length"))) return 0;
        len_recv = resolve_copy_chain_source_reg(insts, len_def_ii + 1, read_int(len_cal, "receiver"));
        return len_recv == recv;
      }

      auto int should_defer_string_concat_pair(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst) {
        size_t use_ii = 0;
        long long final_reg = 0;
        if (!dst) return 0;
        if (!follow_single_use_copy_chain_to_use(
                ii + 1, ilen, insts, dst, &use_ii, &final_reg, NULL, NULL)) {
          return 0;
        }
        return is_string_concat_chain_consumer_inst(
                   yyjson_arr_get(insts, use_ii), insts, ii + 1, use_ii, final_reg) ||
                is_string_concat_len_consumer_inst(
                    yyjson_arr_get(insts, use_ii), final_reg) ||
                is_string_concat_substring_consumer_inst(
                    yyjson_arr_get(insts, use_ii), final_reg);
      }
