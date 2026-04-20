      struct ArrayRmwWindowMatch {
        int matched;
        size_t copy_iis[8];
        int copy_n;
        size_t const_ii;
        size_t binop_ii;
        size_t set_ii;
        long long input_reg;
        long long const_reg;
        long long result_reg;
      };
      struct ArrayStringLenWindowMatch {
        int matched;
        int keep_get_live;
        int source_only_insert_mid;
        size_t copy_iis[8];
        int copy_n;
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
      auto int match_array_text_loopcarry_len_store_route_plan(
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
                "mir_route_plan",
                "");
          }
          return 1;
        }
        return 0;
      }
      auto int inst_is_array_string_len_safe_reuse(yyjson_val* ins, long long reg) {
        if (!ins || !reg) return 0;
        const char* op = read_str(ins, "op");
        if (!(op && strcmp(op, "mir_call") == 0)) return 0;
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        const char* ctype = cal ? read_str(cal, "type") : NULL;
        const char* bname = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
        const char* mname = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
        long long recv = cal ? read_int(cal, "receiver") : 0;
        if (!(ctype && !strcmp(ctype, "Method"))) return 0;
        if (recv != reg) return 0;
        if (!(bname && !strcmp(bname, "RuntimeDataBox"))) return 0;
        return mname && !strcmp(mname, "substring");
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
      auto int inst_is_array_string_len_insert_mid_source_reuse(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long source_reg,
          size_t* second_ii_out) {
        yyjson_val* first_ins = NULL;
        yyjson_val* first_mc = NULL;
        yyjson_val* first_cal = NULL;
        yyjson_val* first_args = NULL;
        yyjson_val* second_ins = NULL;
        yyjson_val* second_mc = NULL;
        yyjson_val* second_cal = NULL;
        yyjson_val* second_args = NULL;
        yyjson_val* pair_ins = NULL;
        yyjson_val* final_ins = NULL;
        const char* first_op = NULL;
        const char* first_ctype = NULL;
        const char* first_mname = NULL;
        const char* second_op = NULL;
        const char* second_ctype = NULL;
        const char* second_mname = NULL;
        const char* pair_op = NULL;
        const char* pair_kind = NULL;
        const char* final_op = NULL;
        const char* final_kind = NULL;
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
        size_t final_use_ii = 0;
        size_t prefix_copy_iis[8];
        size_t suffix_copy_iis[8];
        int prefix_copy_n = 0;
        int suffix_copy_n = 0;
        long long prefix_final = 0;
        long long suffix_final = 0;
        long long pair_lhs = 0;
        long long pair_rhs = 0;
        long long pair_dst = 0;
        long long final_lhs = 0;
        long long final_rhs = 0;
        long long result_reg = 0;
        long long middle_reg = 0;

        source_reg = resolve_copy_source_reg(source_reg);
        if (!(source_reg && ii + 1 < ilen)) return 0;
        first_ins = yyjson_arr_get(insts, ii);
        first_op = first_ins ? read_str(first_ins, "op") : NULL;
        if (!(first_op && !strcmp(first_op, "mir_call"))) return 0;
        first_mc = yyjson_obj_get(first_ins, "mir_call");
        first_cal = first_mc ? yyjson_obj_get(first_mc, "callee") : NULL;
        first_args = first_mc ? yyjson_obj_get(first_mc, "args") : NULL;
        if (!(first_cal && first_args && yyjson_is_arr(first_args) && yyjson_arr_size(first_args) == 2)) return 0;
        first_ctype = read_str(first_cal, "type");
        first_mname = read_str(first_cal, "method");
        if (!first_mname) first_mname = read_str(first_cal, "name");
        if (!(first_ctype && !strcmp(first_ctype, "Method") &&
              first_mname && !strcmp(first_mname, "substring"))) return 0;
        first_recv = resolve_copy_source_reg(read_int(first_cal, "receiver"));
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
        if (!(second_cal && second_args && yyjson_is_arr(second_args) && yyjson_arr_size(second_args) == 2)) return 0;
        second_ctype = read_str(second_cal, "type");
        second_mname = read_str(second_cal, "method");
        if (!second_mname) second_mname = read_str(second_cal, "name");
        if (!(second_ctype && !strcmp(second_ctype, "Method") &&
              second_mname && !strcmp(second_mname, "substring"))) return 0;
        second_recv = resolve_copy_source_reg(read_int(second_cal, "receiver"));
        second_start = (long long)yyjson_get_sint(yyjson_arr_get(second_args, 0));
        second_dst = read_int(second_ins, "dst");
        if (!(second_recv == source_reg &&
              resolve_copy_source_reg(second_start) == resolve_copy_source_reg(first_end))) {
          return 0;
        }

        if (!follow_single_use_copy_chain_to_use(
                ii + 2, ilen, insts, first_dst, &prefix_use_ii, &prefix_final,
                prefix_copy_iis, &prefix_copy_n)) return 0;
        if (!follow_single_use_copy_chain_to_use(
                ii + 2, ilen, insts, second_dst, &suffix_use_ii, &suffix_final,
                suffix_copy_iis, &suffix_copy_n)) return 0;

        pair_ins = yyjson_arr_get(insts, prefix_use_ii);
        pair_op = pair_ins ? read_str(pair_ins, "op") : NULL;
        pair_kind = pair_ins ? (read_str(pair_ins, "op_kind") ? read_str(pair_ins, "op_kind")
                                                               : read_str(pair_ins, "operation"))
                             : NULL;
        if (!(pair_op && !strcmp(pair_op, "binop") && pair_kind &&
              (!strcmp(pair_kind, "Add") || !strcmp(pair_kind, "+")))) return 0;
        pair_lhs = read_int(pair_ins, "lhs");
        pair_rhs = read_int(pair_ins, "rhs");
        pair_dst = read_int(pair_ins, "dst");
        if (pair_lhs == prefix_final) middle_reg = resolve_copy_chain_source_reg(insts, prefix_use_ii + 1, pair_rhs);
        else if (pair_rhs == prefix_final) middle_reg = resolve_copy_chain_source_reg(insts, prefix_use_ii + 1, pair_lhs);
        else return 0;
        if (!get_str_const(middle_reg)) return 0;

        if (!find_single_use_inst(prefix_use_ii + 1, ilen, insts, pair_dst, &final_use_ii)) return 0;
        if (final_use_ii != suffix_use_ii) return 0;
        final_ins = yyjson_arr_get(insts, final_use_ii);
        final_op = final_ins ? read_str(final_ins, "op") : NULL;
        final_kind = final_ins ? (read_str(final_ins, "op_kind") ? read_str(final_ins, "op_kind")
                                                                 : read_str(final_ins, "operation"))
                               : NULL;
        if (!(final_op && !strcmp(final_op, "binop") && final_kind &&
              (!strcmp(final_kind, "Add") || !strcmp(final_kind, "+")))) return 0;
        final_lhs = read_int(final_ins, "lhs");
        final_rhs = read_int(final_ins, "rhs");
        result_reg = read_int(final_ins, "dst");
        if (!((final_lhs == pair_dst && final_rhs == suffix_final) ||
              (final_rhs == pair_dst && final_lhs == suffix_final))) return 0;
        if (!hako_llvmc_value_has_direct_set_consumer(result_reg)) return 0;
        if (second_ii_out) *second_ii_out = ii + 1;
        return 1;
      }
      auto int analyze_array_rmw_window_candidate(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long cur_dst,
          int recv_org,
          int runtime_array_get,
          int runtime_array_string,
          struct ArrayRmwWindowMatch* out,
          const char** reason_out,
          char* extra,
          size_t extra_cap) {
        const char* reason = "shape_match";
        if (out) memset(out, 0, sizeof(*out));
        if (extra && extra_cap) extra[0] = '\0';
        if (!mname || strcmp(mname, "get") != 0) {
          reason = "not_get";
          goto done;
        }
        if (!hako_llvmc_window_array_read_enabled(
                bname, recv_org, runtime_array_get, runtime_array_string)) {
          reason = "not_array_get";
          goto done;
        }
        if (ii + 3 >= ilen) {
          reason = "lookahead_short";
          goto done;
        }

        size_t jj = ii + 1;
        long long carried = cur_dst;
        if (!carried) {
          reason = "missing_get_dst";
          goto done;
        }
        if (out) out->input_reg = cur_dst;
        while (jj < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, jj);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried)) break;
          if (out && out->copy_n < (int)(sizeof(out->copy_iis) / sizeof(out->copy_iis[0]))) {
            out->copy_iis[out->copy_n++] = jj;
          }
          carried = read_int(c, "dst");
          jj++;
        }

        if (jj + 2 >= ilen) {
          reason = "copy_chain_short";
          goto done;
        }

        yyjson_val* n_const = yyjson_arr_get(insts, jj);
        yyjson_val* n_bin = yyjson_arr_get(insts, jj + 1);
        yyjson_val* n_set = yyjson_arr_get(insts, jj + 2);
        const char* op_const = n_const ? read_str(n_const, "op") : NULL;
        const char* op_bin = n_bin ? read_str(n_bin, "op") : NULL;
        const char* op_set = n_set ? read_str(n_set, "op") : NULL;
        long long const_reg = 0, const_val = 0;
        long long bin_lhs = 0, bin_rhs = 0;
        const char* add_kind = NULL;

        if (!(op_const && strcmp(op_const, "const") == 0)) {
          reason = "next_noncopy_not_const";
          goto done;
        }
        const_reg = read_int(n_const, "dst");
        if (!has_const(const_reg, &const_val) || const_val != 1) {
          reason = "const_not_1";
          goto done;
        }
        if (!(op_bin && strcmp(op_bin, "binop") == 0)) {
          reason = "next_after_const_not_binop";
          goto done;
        }
        add_kind = read_str(n_bin, "op_kind");
        if (!add_kind) add_kind = read_str(n_bin, "operation");
        if (!(add_kind && (!strcmp(add_kind, "Add") || !strcmp(add_kind, "+")))) {
          reason = "binop_not_add";
          goto done;
        }
        bin_lhs = read_int(n_bin, "lhs");
        bin_rhs = read_int(n_bin, "rhs");
        if (!((bin_lhs == carried && bin_rhs == const_reg) ||
              (bin_rhs == carried && bin_lhs == const_reg))) {
          reason = "binop_not_copy_transparent_get_plus_1";
          goto done;
        }
        if (!(op_set && strcmp(op_set, "mir_call") == 0)) {
          reason = "next_after_binop_not_set";
          goto done;
        }
        {
          yyjson_val* mc = yyjson_obj_get(n_set, "mir_call");
          yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
          yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
          const char* ctype = cal ? read_str(cal, "type") : NULL;
          const char* bname2 = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
          const char* mname2 = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
          long long recv2 = cal ? read_int(cal, "receiver") : 0;
          long long idx2 = 0;
          long long value2 = 0;
          if (args && yyjson_is_arr(args)) {
            if (yyjson_arr_size(args) >= 1) idx2 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
            if (yyjson_arr_size(args) >= 2) value2 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
          }
          if (!(ctype && !strcmp(ctype, "Method") && mname2 && !strcmp(mname2, "set") &&
                ((bname2 && !strcmp(bname2, "ArrayBox")) ||
                 (bname2 && !strcmp(bname2, "RuntimeDataBox") &&
                  (recv_org == ORG_ARRAY_BIRTH || recv_org == ORG_ARRAY_STRING_BIRTH))))) {
            reason = "set_not_array_method";
            goto done;
          }
          if (recv && recv2 && recv != recv2) {
            reason = "set_recv_mismatch";
            goto done;
          }
          if (idx && idx2 && idx != idx2) {
            reason = "set_idx_mismatch";
            goto done;
          }
          if (value2 != read_int(n_bin, "dst")) {
            reason = "set_value_not_binop_dst";
            goto done;
          }
        }

        if (out) {
          for (size_t kk = jj + 3; kk < ilen; kk++) {
            yyjson_val* later = yyjson_arr_get(insts, kk);
            if (inst_uses_reg(later, cur_dst) || inst_uses_reg(later, carried)) {
              reason = "post_set_uses_consumed_get_value";
              goto done;
            }
          }
          out->matched = 1;
          out->const_ii = jj;
          out->const_reg = const_reg;
          out->binop_ii = jj + 1;
          out->result_reg = read_int(n_bin, "dst");
          out->set_ii = jj + 2;
        }

      done:
        if (extra && extra_cap) {
          long long result_reg = out && out->matched ? out->result_reg : 0;
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld copy_n=%d carried=%lld const_ii=%zu binop_ii=%zu set_ii=%zu result=%lld",
                   ii, cur_dst, recv, idx,
                   out ? out->copy_n : 0,
                   carried,
                   out ? out->const_ii : 0,
                   out ? out->binop_ii : 0,
                   out ? out->set_ii : 0,
                   result_reg);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
      auto int analyze_array_string_len_window_candidate(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long cur_dst,
          int recv_org,
          int runtime_array_string,
          struct ArrayStringLenWindowMatch* out,
          const char** reason_out,
          char* extra,
          size_t extra_cap) {
        const char* reason = "shape_match";
        if (out) memset(out, 0, sizeof(*out));
        if (extra && extra_cap) extra[0] = '\0';
        if (!mname || strcmp(mname, "get") != 0) {
          reason = "not_get";
          goto done;
        }
        if (!hako_llvmc_window_array_text_read_enabled(
                bname, recv_org, runtime_array_string)) {
          reason = "not_array_string_get";
          goto done;
        }
        if (ii + 2 >= ilen) {
          reason = "lookahead_short";
          goto done;
        }
        size_t jj = ii + 1;
        long long carried = cur_dst;
        if (!carried) {
          reason = "missing_get_dst";
          goto done;
        }
        while (jj < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, jj);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried)) break;
          if (out && out->copy_n < (int)(sizeof(out->copy_iis) / sizeof(out->copy_iis[0]))) {
            out->copy_iis[out->copy_n++] = jj;
          }
          carried = read_int(c, "dst");
          jj++;
        }
        if (jj >= ilen) {
          reason = "copy_chain_short";
          goto done;
        }
        yyjson_val* n_len = yyjson_arr_get(insts, jj);
        const char* op_len = n_len ? read_str(n_len, "op") : NULL;
        if (!(op_len && strcmp(op_len, "mir_call") == 0)) {
          reason = "next_noncopy_not_len";
          goto done;
        }
        {
          yyjson_val* mc = yyjson_obj_get(n_len, "mir_call");
          yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
          const char* ctype = cal ? read_str(cal, "type") : NULL;
          const char* bname2 = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
          const char* mname2 = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
          long long recv2 = cal ? read_int(cal, "receiver") : 0;
          if (!(ctype && !strcmp(ctype, "Method") &&
                mname2 && (!strcmp(mname2, "len") || !strcmp(mname2, "length") || !strcmp(mname2, "size")) &&
                bname2 && !strcmp(bname2, "RuntimeDataBox"))) {
            reason = "next_noncopy_not_len";
            goto done;
          }
          if (recv2 && carried && recv2 != carried) {
            reason = "len_recv_mismatch";
            goto done;
          }
          int saw_source_only_insert_mid = 0;
          size_t source_only_second_ii = (size_t)-1;
          for (size_t kk = jj + 1; kk < ilen; kk++) {
            yyjson_val* later = yyjson_arr_get(insts, kk);
            size_t matched_second_ii = 0;
            struct ArrayStringPiecewiseDirectSetSourceReuseMatch piecewise_reuse;
            if (kk == source_only_second_ii) continue;
            if (!(inst_uses_reg(later, cur_dst) || inst_uses_reg(later, carried))) continue;
            if (inst_is_array_string_len_insert_mid_source_reuse(
                    kk, ilen, insts, cur_dst, &matched_second_ii) ||
                inst_is_array_string_len_insert_mid_source_reuse(
                    kk, ilen, insts, carried, &matched_second_ii)) {
              saw_source_only_insert_mid = 1;
              source_only_second_ii = matched_second_ii;
              continue;
            }
            if (match_array_string_piecewise_concat3_direct_set_source_reuse(
                    kk, ilen, insts, cur_dst, &piecewise_reuse) ||
                match_array_string_piecewise_concat3_direct_set_source_reuse(
                    kk, ilen, insts, carried, &piecewise_reuse)) {
              saw_source_only_insert_mid = 1;
              source_only_second_ii = piecewise_reuse.second_ii;
              continue;
            }
            if (inst_is_array_string_len_safe_reuse(later, cur_dst) ||
                inst_is_array_string_len_safe_reuse(later, carried)) {
              if (out) out->keep_get_live = 1;
              continue;
            }
            if (inst_uses_reg(later, cur_dst) || inst_uses_reg(later, carried)) {
              reason = "post_len_uses_consumed_get_value";
              goto done;
            }
          }
          if (out) {
            out->matched = 1;
            out->source_only_insert_mid =
                saw_source_only_insert_mid && !out->keep_get_live;
            out->len_ii = jj;
            out->len_dst = read_int(n_len, "dst");
          }
        }
      done:
        if (extra && extra_cap) {
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld copy_n=%d len_ii=%zu len_dst=%lld keep_get_live=%d source_only_insert_mid=%d producer_kind=ArrayGet boundary_kind=Store post_store_use=LenObserver known_len=-1 direct_set_only=0",
                   ii, cur_dst, recv, idx,
                   out ? out->copy_n : 0,
                   out ? out->len_ii : 0,
                   out ? out->len_dst : 0,
                   out ? out->keep_get_live : 0,
                   out ? out->source_only_insert_mid : 0);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
      auto void trace_array_rmw_window_candidate(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long cur_dst,
          int recv_org,
          int runtime_array_get,
          int runtime_array_string) {
        if (!hako_llvmc_route_trace_enabled() || array_rmw_trace_budget <= 0) return;
        array_rmw_trace_budget--;
        struct ArrayRmwWindowMatch match;
        const char* reason = NULL;
        char extra[384];
        int matched = analyze_array_rmw_window_candidate(
            ii, ilen, insts, bname, mname, recv, idx, cur_dst, recv_org,
            runtime_array_get, runtime_array_string, &match, &reason, extra, sizeof(extra));
        hako_llvmc_emit_route_trace("array_rmw_window", matched ? "hit" : "miss", reason ? reason : "unknown", extra);
      }
      auto void trace_array_string_len_window_candidate(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long cur_dst,
          int recv_org,
          int runtime_array_string) {
        if (!hako_llvmc_route_trace_enabled() || array_string_len_trace_budget <= 0) return;
        array_string_len_trace_budget--;
        struct ArrayStringLenWindowMatch match;
        const char* reason = NULL;
        char extra[256];
        int matched = analyze_array_string_len_window_candidate(
            ii, ilen, insts, bname, mname, recv, idx, cur_dst, recv_org,
            runtime_array_string, &match, &reason, extra, sizeof(extra));
        hako_llvmc_emit_route_trace("array_string_len_window", matched ? "hit" : "miss", reason ? reason : "unknown", extra);
      }
