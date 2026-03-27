      auto int analyze_array_string_indexof_cross_block_select_window_candidate(
          yyjson_val* blocks,
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
          struct ArrayStringIndexofCrossBlockSelectWindowMatch* out,
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
        if (!(runtime_array_string ||
              (bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_STRING_BIRTH))) {
          reason = "not_array_string_get";
          goto done;
        }
        if (ii + 4 >= ilen) {
          reason = "lookahead_short";
          goto done;
        }

        size_t jj = ii + 1;
        long long carried_get = cur_dst;
        if (!carried_get) {
          reason = "missing_get_dst";
          goto done;
        }
        while (jj < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, jj);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_get)) break;
          if (out && out->get_copy_n < (int)(sizeof(out->get_copy_iis) / sizeof(out->get_copy_iis[0]))) {
            out->get_copy_iis[out->get_copy_n++] = jj;
          }
          carried_get = read_int(c, "dst");
          jj++;
        }
        if (jj + 3 >= ilen) {
          reason = "copy_chain_short";
          goto done;
        }

        yyjson_val* n_needle = yyjson_arr_get(insts, jj);
        const char* op_needle = n_needle ? read_str(n_needle, "op") : NULL;
        long long needle_reg = 0;
        const char* needle_const = NULL;
        if (!(op_needle && strcmp(op_needle, "const") == 0)) {
          reason = "next_noncopy_not_const_string";
          goto done;
        }
        needle_reg = read_int(n_needle, "dst");
        needle_const = get_str_const(needle_reg);
        if (!needle_const) needle_const = read_string_const_from_inst(n_needle);
        if (!needle_const) {
          reason = "needle_not_string_const";
          goto done;
        }

        size_t kk = jj + 1;
        long long carried_recv = carried_get;
        while (kk < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, kk);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_recv)) break;
          if (out && out->recv_copy_n < (int)(sizeof(out->recv_copy_iis) / sizeof(out->recv_copy_iis[0]))) {
            out->recv_copy_iis[out->recv_copy_n++] = kk;
          }
          carried_recv = read_int(c, "dst");
          kk++;
        }
        if (kk + 1 >= ilen) {
          reason = "receiver_copy_chain_short";
          goto done;
        }

        yyjson_val* n_indexof = yyjson_arr_get(insts, kk);
        const char* op_indexof = n_indexof ? read_str(n_indexof, "op") : NULL;
        if (!(op_indexof && strcmp(op_indexof, "mir_call") == 0)) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        yyjson_val* mc_indexof = yyjson_obj_get(n_indexof, "mir_call");
        yyjson_val* cal_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "callee") : NULL;
        yyjson_val* args_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "args") : NULL;
        const char* ctype_indexof = cal_indexof ? read_str(cal_indexof, "type") : NULL;
        const char* bname_indexof = cal_indexof ? (read_str(cal_indexof, "box_name") ? read_str(cal_indexof, "box_name") : read_str(cal_indexof, "box_type")) : NULL;
        const char* mname_indexof = cal_indexof ? (read_str(cal_indexof, "method") ? read_str(cal_indexof, "method") : read_str(cal_indexof, "name")) : NULL;
        long long recv_indexof = cal_indexof ? read_int(cal_indexof, "receiver") : 0;
        long long arg_indexof = 0;
        if (args_indexof && yyjson_is_arr(args_indexof) && yyjson_arr_size(args_indexof) >= 1) {
          arg_indexof = (long long)yyjson_get_sint(yyjson_arr_get(args_indexof, 0));
        }
        if (!(ctype_indexof && !strcmp(ctype_indexof, "Method") &&
              mname_indexof && !strcmp(mname_indexof, "indexOf"))) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        if (recv_indexof != carried_recv) {
          reason = "indexof_recv_mismatch";
          goto done;
        }
        if (arg_indexof != needle_reg) {
          reason = "indexof_needle_mismatch";
          goto done;
        }
        if (!((bname_indexof && !strcmp(bname_indexof, "RuntimeDataBox")) ||
              (bname_indexof && !strcmp(bname_indexof, "StringBox")))) {
          reason = "indexof_not_string_method";
          goto done;
        }

        size_t ll = kk + 1;
        long long carried_indexof = read_int(n_indexof, "dst");
        if (!carried_indexof) {
          reason = "indexof_missing_dst";
          goto done;
        }
        while (ll < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, ll);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_indexof)) break;
          if (out && out->indexof_copy_n < (int)(sizeof(out->indexof_copy_iis) / sizeof(out->indexof_copy_iis[0]))) {
            out->indexof_copy_iis[out->indexof_copy_n++] = ll;
          }
          carried_indexof = read_int(c, "dst");
          ll++;
        }
        if (ll + 0 >= ilen) {
          reason = "indexof_copy_chain_short";
          goto done;
        }
        {
          yyjson_val* n_jump = yyjson_arr_get(insts, ll);
          const char* op_jump = n_jump ? read_str(n_jump, "op") : NULL;
          long long succ_bid = 0;
          yyjson_val* succ_block = NULL;
          yyjson_val* succ_insts = NULL;
          size_t slen = 0;
          size_t mm = 0;
          size_t ss = 0;
          long long carried_succ = carried_indexof;
          yyjson_val* n_zero = NULL;
          yyjson_val* n_compare = NULL;
          yyjson_val* n_then = NULL;
          yyjson_val* n_else = NULL;
          yyjson_val* n_select = NULL;
          const char* op_compare = NULL;
          const char* op_select = NULL;
          long long zero_reg = 0;
          long long zero_val = 0;
          long long compare_dst = 0;
          long long then_reg = 0;
          long long else_reg = 0;
          long long then_val = 0;
          long long else_val = 0;

          if (!(op_jump && strcmp(op_jump, "jump") == 0)) {
            reason = "next_after_indexof_not_jump";
            goto done;
          }
          succ_bid = read_int(n_jump, "target");
          if (!succ_bid) {
            reason = "jump_target_missing";
            goto done;
          }
          for (mm = ll + 1; mm < ilen; mm++) {
            yyjson_val* later = yyjson_arr_get(insts, mm);
            if (inst_uses_reg(later, cur_dst) || inst_uses_reg(later, carried_get) ||
                inst_uses_reg(later, carried_recv) || inst_uses_reg(later, carried_indexof)) {
              reason = "post_jump_uses_consumed_value";
              goto done;
            }
          }
          succ_block = find_block_by_id(blocks, succ_bid);
          if (!succ_block) {
            reason = "successor_block_missing";
            goto done;
          }
          succ_insts = yyjson_obj_get(succ_block, "instructions");
          if (!(succ_insts && yyjson_is_arr(succ_insts))) {
            reason = "successor_insts_missing";
            goto done;
          }
          slen = yyjson_arr_size(succ_insts);
          while (ss < slen) {
            yyjson_val* c = yyjson_arr_get(succ_insts, ss);
            const char* cop = c ? read_str(c, "op") : NULL;
            if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_succ)) break;
            carried_succ = read_int(c, "dst");
            ss++;
          }
          if (ss + 4 >= slen) {
            reason = "successor_shape_short";
            goto done;
          }
          n_zero = yyjson_arr_get(succ_insts, ss);
          n_compare = yyjson_arr_get(succ_insts, ss + 1);
          n_then = yyjson_arr_get(succ_insts, ss + 2);
          n_else = yyjson_arr_get(succ_insts, ss + 3);
          n_select = yyjson_arr_get(succ_insts, ss + 4);
          if (!(n_zero && read_str(n_zero, "op") && !strcmp(read_str(n_zero, "op"), "const"))) {
            reason = "successor_zero_missing";
            goto done;
          }
          zero_reg = read_int(n_zero, "dst");
          if (!(has_const(zero_reg, &zero_val) || read_i64_const_from_inst(n_zero, &zero_val)) || zero_val != 0) {
            reason = "successor_zero_not_zero";
            goto done;
          }
          if (!(n_compare && read_str(n_compare, "op") && !strcmp(read_str(n_compare, "op"), "compare"))) {
            reason = "successor_compare_missing";
            goto done;
          }
          op_compare = read_str(n_compare, "operation");
          if (!op_compare) op_compare = read_str(n_compare, "cmp");
          if (!(op_compare && (!strcmp(op_compare, ">=") || !strcmp(op_compare, "Ge") || !strcmp(op_compare, "ge")))) {
            reason = "successor_compare_not_ge_zero";
            goto done;
          }
          {
            long long lhs = read_int(n_compare, "lhs");
            if (!lhs) lhs = read_int(n_compare, "left");
            long long rhs = read_int(n_compare, "rhs");
            if (!rhs) rhs = read_int(n_compare, "right");
            if (!(lhs == carried_succ && rhs == zero_reg)) {
              reason = "successor_compare_not_indexof_ge_zero";
              goto done;
            }
          }
          compare_dst = read_int(n_compare, "dst");
          if (!(n_then && read_str(n_then, "op") && !strcmp(read_str(n_then, "op"), "const"))) {
            reason = "successor_then_not_const";
            goto done;
          }
          then_reg = read_int(n_then, "dst");
          if (!(has_const(then_reg, &then_val) || read_i64_const_from_inst(n_then, &then_val))) {
            reason = "successor_then_not_i64_const";
            goto done;
          }
          if (!(n_else && read_str(n_else, "op") && !strcmp(read_str(n_else, "op"), "const"))) {
            reason = "successor_else_not_const";
            goto done;
          }
          else_reg = read_int(n_else, "dst");
          if (!(has_const(else_reg, &else_val) || read_i64_const_from_inst(n_else, &else_val))) {
            reason = "successor_else_not_i64_const";
            goto done;
          }
          op_select = n_select ? read_str(n_select, "op") : NULL;
          if (!(op_select && !strcmp(op_select, "select"))) {
            reason = "successor_select_missing";
            goto done;
          }
          if (!(read_int(n_select, "cond") == compare_dst &&
                read_int(n_select, "then_val") == then_reg &&
                read_int(n_select, "else_val") == else_reg)) {
            reason = "successor_select_not_compare_terminal";
            goto done;
          }
          if (out) {
            out->matched = 1;
            out->needle_const_ii = jj;
            out->needle_reg = needle_reg;
            out->indexof_ii = kk;
            out->indexof_dst = read_int(n_indexof, "dst");
            out->jump_ii = ll;
            out->succ_bid = succ_bid;
          }
        }
      done:
        if (extra && extra_cap) {
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld get_copy_n=%d recv_copy_n=%d indexof_copy_n=%d indexof_ii=%zu jump_ii=%zu succ_bid=%lld",
                   ii, cur_dst, recv, idx,
                   out ? out->get_copy_n : 0,
                   out ? out->recv_copy_n : 0,
                   out ? out->indexof_copy_n : 0,
                   out ? out->indexof_ii : 0,
                   out ? out->jump_ii : 0,
                   out ? out->succ_bid : 0);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
      auto int analyze_array_string_indexof_interleaved_branch_window_candidate(
          yyjson_val* blocks,
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
          struct ArrayStringIndexofInterleavedBranchWindowMatch* out,
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
        if (!(runtime_array_string ||
              (bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_STRING_BIRTH))) {
          reason = "not_array_string_get";
          goto done;
        }
        if (ii + 8 >= ilen) {
          reason = "lookahead_short";
          goto done;
        }

        size_t jj = ii + 1;
        long long carried_get = cur_dst;
        if (!carried_get) {
          reason = "missing_get_dst";
          goto done;
        }
        while (jj < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, jj);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_get)) break;
          if (out && out->get_copy_n < (int)(sizeof(out->get_copy_iis) / sizeof(out->get_copy_iis[0]))) {
            out->get_copy_iis[out->get_copy_n++] = jj;
          }
          carried_get = read_int(c, "dst");
          jj++;
        }
        if (jj + 7 >= ilen) {
          reason = "copy_chain_short";
          goto done;
        }

        yyjson_val* n_needle = yyjson_arr_get(insts, jj);
        const char* op_needle = n_needle ? read_str(n_needle, "op") : NULL;
        long long needle_reg = 0;
        const char* needle_const = NULL;
        if (!(op_needle && strcmp(op_needle, "const") == 0)) {
          reason = "next_noncopy_not_const_string";
          goto done;
        }
        needle_reg = read_int(n_needle, "dst");
        needle_const = get_str_const(needle_reg);
        if (!needle_const) needle_const = read_string_const_from_inst(n_needle);
        if (!needle_const) {
          reason = "needle_not_string_const";
          goto done;
        }

        size_t kk = jj + 1;
        long long carried_recv = carried_get;
        while (kk < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, kk);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_recv)) break;
          if (out && out->recv_copy_n < (int)(sizeof(out->recv_copy_iis) / sizeof(out->recv_copy_iis[0]))) {
            out->recv_copy_iis[out->recv_copy_n++] = kk;
          }
          carried_recv = read_int(c, "dst");
          kk++;
        }
        if (kk + 6 >= ilen) {
          reason = "receiver_copy_chain_short";
          goto done;
        }

        yyjson_val* n_indexof = yyjson_arr_get(insts, kk);
        const char* op_indexof = n_indexof ? read_str(n_indexof, "op") : NULL;
        if (!(op_indexof && strcmp(op_indexof, "mir_call") == 0)) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        yyjson_val* mc_indexof = yyjson_obj_get(n_indexof, "mir_call");
        yyjson_val* cal_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "callee") : NULL;
        yyjson_val* args_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "args") : NULL;
        const char* ctype_indexof = cal_indexof ? read_str(cal_indexof, "type") : NULL;
        const char* bname_indexof = cal_indexof ? (read_str(cal_indexof, "box_name") ? read_str(cal_indexof, "box_name") : read_str(cal_indexof, "box_type")) : NULL;
        const char* mname_indexof = cal_indexof ? (read_str(cal_indexof, "method") ? read_str(cal_indexof, "method") : read_str(cal_indexof, "name")) : NULL;
        long long recv_indexof = cal_indexof ? read_int(cal_indexof, "receiver") : 0;
        long long arg_indexof = 0;
        if (args_indexof && yyjson_is_arr(args_indexof) && yyjson_arr_size(args_indexof) >= 1) {
          arg_indexof = (long long)yyjson_get_sint(yyjson_arr_get(args_indexof, 0));
        }
        if (!(ctype_indexof && !strcmp(ctype_indexof, "Method") &&
              mname_indexof && !strcmp(mname_indexof, "indexOf"))) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        if (recv_indexof != carried_recv) {
          reason = "indexof_recv_mismatch";
          goto done;
        }
        if (arg_indexof != needle_reg) {
          reason = "indexof_needle_mismatch";
          goto done;
        }
        if (!((bname_indexof && !strcmp(bname_indexof, "RuntimeDataBox")) ||
              (bname_indexof && !strcmp(bname_indexof, "StringBox")))) {
          reason = "indexof_not_string_method";
          goto done;
        }

        size_t ll = kk + 1;
        long long carried_indexof = read_int(n_indexof, "dst");
        if (!carried_indexof) {
          reason = "indexof_missing_dst";
          goto done;
        }
        while (ll < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, ll);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_indexof)) break;
          if (out && out->indexof_copy_n < (int)(sizeof(out->indexof_copy_iis) / sizeof(out->indexof_copy_iis[0]))) {
            out->indexof_copy_iis[out->indexof_copy_n++] = ll;
          }
          carried_indexof = read_int(c, "dst");
          ll++;
        }
        if (ll + 5 >= ilen) {
          reason = "indexof_copy_chain_short";
          goto done;
        }

        yyjson_val* n_guard_copy = yyjson_arr_get(insts, ll);
        const char* op_guard_copy = n_guard_copy ? read_str(n_guard_copy, "op") : NULL;
        long long guard_carried = 0;
        if (!(op_guard_copy && !strcmp(op_guard_copy, "copy"))) {
          reason = "guard_copy_missing";
          goto done;
        }
        guard_carried = read_int(n_guard_copy, "dst");
        if (!guard_carried) {
          reason = "guard_copy_dst_missing";
          goto done;
        }
        if (!read_int(n_guard_copy, "src")) {
          reason = "guard_copy_src_missing";
          goto done;
        }

        yyjson_val* n_guard_mod_const = yyjson_arr_get(insts, ll + 1);
        yyjson_val* n_guard_mod = yyjson_arr_get(insts, ll + 2);
        yyjson_val* n_guard_zero = yyjson_arr_get(insts, ll + 3);
        yyjson_val* n_guard_compare = yyjson_arr_get(insts, ll + 4);
        yyjson_val* n_guard_branch = yyjson_arr_get(insts, ll + 5);
        long long mod_const_reg = 0;
        long long mod_const_val = 0;
        long long zero_reg = 0;
        long long zero_val = 0;
        long long guard_mod_dst = 0;
        long long guard_compare_dst = 0;
        long long consumer_bid = 0;
        yyjson_val* consumer_block = NULL;
        yyjson_val* consumer_insts = NULL;
        size_t slen = 0;
        size_t ss = 0;
        long long carried_consumer = carried_indexof;

        if (!(n_guard_mod_const && read_str(n_guard_mod_const, "op") && !strcmp(read_str(n_guard_mod_const, "op"), "const"))) {
          reason = "guard_mod_const_missing";
          goto done;
        }
        mod_const_reg = read_int(n_guard_mod_const, "dst");
        if (!(has_const(mod_const_reg, &mod_const_val) || read_i64_const_from_inst(n_guard_mod_const, &mod_const_val)) || mod_const_val != 16) {
          reason = "guard_mod_const_not_16";
          goto done;
        }
        if (!(n_guard_mod && read_str(n_guard_mod, "op") && !strcmp(read_str(n_guard_mod, "op"), "binop"))) {
          reason = "guard_mod_missing";
          goto done;
        }
        {
          const char* guard_mod_op = read_str(n_guard_mod, "operation");
          if (!guard_mod_op) guard_mod_op = read_str(n_guard_mod, "op_kind");
          if (!(guard_mod_op && (!strcmp(guard_mod_op, "%") || !strcmp(guard_mod_op, "Mod")))) {
            reason = "guard_mod_not_percent";
            goto done;
          }
          if (!((read_int(n_guard_mod, "lhs") == guard_carried && read_int(n_guard_mod, "rhs") == mod_const_reg) ||
                (read_int(n_guard_mod, "rhs") == guard_carried && read_int(n_guard_mod, "lhs") == mod_const_reg))) {
            reason = "guard_mod_not_copy_percent_16";
            goto done;
          }
        }
        guard_mod_dst = read_int(n_guard_mod, "dst");
        if (!(n_guard_zero && read_str(n_guard_zero, "op") && !strcmp(read_str(n_guard_zero, "op"), "const"))) {
          reason = "guard_zero_missing";
          goto done;
        }
        zero_reg = read_int(n_guard_zero, "dst");
        if (!(has_const(zero_reg, &zero_val) || read_i64_const_from_inst(n_guard_zero, &zero_val)) || zero_val != 0) {
          reason = "guard_zero_not_zero";
          goto done;
        }
        if (!(n_guard_compare && read_str(n_guard_compare, "op") && !strcmp(read_str(n_guard_compare, "op"), "compare"))) {
          reason = "guard_compare_missing";
          goto done;
        }
        {
          const char* guard_compare_op = read_str(n_guard_compare, "operation");
          if (!guard_compare_op) guard_compare_op = read_str(n_guard_compare, "cmp");
          if (!(guard_compare_op && (!strcmp(guard_compare_op, "==") || !strcmp(guard_compare_op, "Eq") || !strcmp(guard_compare_op, "eq")))) {
            reason = "guard_compare_not_eq_zero";
            goto done;
          }
          if (!((read_int(n_guard_compare, "lhs") == guard_mod_dst && read_int(n_guard_compare, "rhs") == zero_reg) ||
                (read_int(n_guard_compare, "right") == zero_reg && read_int(n_guard_compare, "left") == guard_mod_dst))) {
            reason = "guard_compare_not_mod_eq_zero";
            goto done;
          }
        }
        guard_compare_dst = read_int(n_guard_compare, "dst");
        if (!(n_guard_branch && read_str(n_guard_branch, "op") && !strcmp(read_str(n_guard_branch, "op"), "branch"))) {
          reason = "guard_branch_missing";
          goto done;
        }
        if (read_int(n_guard_branch, "cond") != guard_compare_dst) {
          reason = "guard_branch_cond_mismatch";
          goto done;
        }
        consumer_bid = read_int(n_guard_branch, "then");
        if (!consumer_bid) {
          reason = "consumer_then_missing";
          goto done;
        }
        consumer_block = find_block_by_id(blocks, consumer_bid);
        if (!consumer_block) {
          reason = "consumer_block_missing";
          goto done;
        }
        consumer_insts = yyjson_obj_get(consumer_block, "instructions");
        if (!(consumer_insts && yyjson_is_arr(consumer_insts))) {
          reason = "consumer_insts_missing";
          goto done;
        }
        slen = yyjson_arr_size(consumer_insts);
        while (ss < slen) {
          yyjson_val* c = yyjson_arr_get(consumer_insts, ss);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_consumer)) break;
          carried_consumer = read_int(c, "dst");
          ss++;
        }
        if (ss + 2 >= slen) {
          reason = "consumer_shape_short";
          goto done;
        }
        {
          yyjson_val* n_consumer_zero = yyjson_arr_get(consumer_insts, ss);
          yyjson_val* n_consumer_compare = yyjson_arr_get(consumer_insts, ss + 1);
          yyjson_val* n_consumer_branch = yyjson_arr_get(consumer_insts, ss + 2);
          long long consumer_zero_reg = 0;
          long long consumer_zero_val = 0;
          if (!(n_consumer_zero && read_str(n_consumer_zero, "op") && !strcmp(read_str(n_consumer_zero, "op"), "const"))) {
            reason = "consumer_zero_missing";
            goto done;
          }
          consumer_zero_reg = read_int(n_consumer_zero, "dst");
          if (!(has_const(consumer_zero_reg, &consumer_zero_val) || read_i64_const_from_inst(n_consumer_zero, &consumer_zero_val)) || consumer_zero_val != 0) {
            reason = "consumer_zero_not_zero";
            goto done;
          }
          if (!(n_consumer_compare && read_str(n_consumer_compare, "op") && !strcmp(read_str(n_consumer_compare, "op"), "compare"))) {
            reason = "consumer_compare_missing";
            goto done;
          }
          {
            const char* consumer_compare_op = read_str(n_consumer_compare, "operation");
            if (!consumer_compare_op) consumer_compare_op = read_str(n_consumer_compare, "cmp");
            if (!(consumer_compare_op && (!strcmp(consumer_compare_op, ">=") || !strcmp(consumer_compare_op, "Ge") || !strcmp(consumer_compare_op, "ge")))) {
              reason = "consumer_compare_not_ge_zero";
              goto done;
            }
            if (!((read_int(n_consumer_compare, "lhs") == carried_consumer && read_int(n_consumer_compare, "rhs") == consumer_zero_reg) ||
                  (read_int(n_consumer_compare, "left") == carried_consumer && read_int(n_consumer_compare, "right") == consumer_zero_reg))) {
              reason = "consumer_compare_not_indexof_ge_zero";
              goto done;
            }
          }
          if (!(n_consumer_branch && read_str(n_consumer_branch, "op") && !strcmp(read_str(n_consumer_branch, "op"), "branch"))) {
            reason = "consumer_branch_missing";
            goto done;
          }
          if (read_int(n_consumer_branch, "cond") != read_int(n_consumer_compare, "dst")) {
            reason = "consumer_branch_cond_mismatch";
            goto done;
          }
        }

        if (out) {
          out->matched = 1;
          out->needle_const_ii = jj;
          out->needle_reg = needle_reg;
          out->indexof_ii = kk;
          out->indexof_dst = read_int(n_indexof, "dst");
          out->guard_compare_ii = ll + 4;
          out->guard_branch_ii = ll + 5;
          out->consumer_bid = consumer_bid;
        }
      done:
        if (extra && extra_cap) {
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld get_copy_n=%d recv_copy_n=%d indexof_copy_n=%d indexof_ii=%zu guard_branch_ii=%zu consumer_bid=%lld",
                   ii, cur_dst, recv, idx,
                   out ? out->get_copy_n : 0,
                   out ? out->recv_copy_n : 0,
                   out ? out->indexof_copy_n : 0,
                   out ? out->indexof_ii : 0,
                   out ? out->guard_branch_ii : 0,
                   out ? out->consumer_bid : 0);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
      auto int analyze_array_string_indexof_interleaved_select_window_candidate(
          yyjson_val* blocks,
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
          struct ArrayStringIndexofInterleavedSelectWindowMatch* out,
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
        if (!(runtime_array_string ||
              (bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_STRING_BIRTH))) {
          reason = "not_array_string_get";
          goto done;
        }
        if (ii + 8 >= ilen) {
          reason = "lookahead_short";
          goto done;
        }

        size_t jj = ii + 1;
        long long carried_get = cur_dst;
        if (!carried_get) {
          reason = "missing_get_dst";
          goto done;
        }
        while (jj < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, jj);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_get)) break;
          if (out && out->get_copy_n < (int)(sizeof(out->get_copy_iis) / sizeof(out->get_copy_iis[0]))) {
            out->get_copy_iis[out->get_copy_n++] = jj;
          }
          carried_get = read_int(c, "dst");
          jj++;
        }
        if (jj + 7 >= ilen) {
          reason = "copy_chain_short";
          goto done;
        }

        yyjson_val* n_needle = yyjson_arr_get(insts, jj);
        const char* op_needle = n_needle ? read_str(n_needle, "op") : NULL;
        long long needle_reg = 0;
        const char* needle_const = NULL;
        if (!(op_needle && strcmp(op_needle, "const") == 0)) {
          reason = "next_noncopy_not_const_string";
          goto done;
        }
        needle_reg = read_int(n_needle, "dst");
        needle_const = get_str_const(needle_reg);
        if (!needle_const) needle_const = read_string_const_from_inst(n_needle);
        if (!needle_const) {
          reason = "needle_not_string_const";
          goto done;
        }

        size_t kk = jj + 1;
        long long carried_recv = carried_get;
        while (kk < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, kk);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_recv)) break;
          if (out && out->recv_copy_n < (int)(sizeof(out->recv_copy_iis) / sizeof(out->recv_copy_iis[0]))) {
            out->recv_copy_iis[out->recv_copy_n++] = kk;
          }
          carried_recv = read_int(c, "dst");
          kk++;
        }
        if (kk + 6 >= ilen) {
          reason = "receiver_copy_chain_short";
          goto done;
        }

        yyjson_val* n_indexof = yyjson_arr_get(insts, kk);
        const char* op_indexof = n_indexof ? read_str(n_indexof, "op") : NULL;
        if (!(op_indexof && strcmp(op_indexof, "mir_call") == 0)) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        yyjson_val* mc_indexof = yyjson_obj_get(n_indexof, "mir_call");
        yyjson_val* cal_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "callee") : NULL;
        yyjson_val* args_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "args") : NULL;
        const char* ctype_indexof = cal_indexof ? read_str(cal_indexof, "type") : NULL;
        const char* bname_indexof = cal_indexof ? (read_str(cal_indexof, "box_name") ? read_str(cal_indexof, "box_name") : read_str(cal_indexof, "box_type")) : NULL;
        const char* mname_indexof = cal_indexof ? (read_str(cal_indexof, "method") ? read_str(cal_indexof, "method") : read_str(cal_indexof, "name")) : NULL;
        long long recv_indexof = cal_indexof ? read_int(cal_indexof, "receiver") : 0;
        long long arg_indexof = 0;
        if (args_indexof && yyjson_is_arr(args_indexof) && yyjson_arr_size(args_indexof) >= 1) {
          arg_indexof = (long long)yyjson_get_sint(yyjson_arr_get(args_indexof, 0));
        }
        if (!(ctype_indexof && !strcmp(ctype_indexof, "Method") &&
              mname_indexof && !strcmp(mname_indexof, "indexOf"))) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        if (recv_indexof != carried_recv) {
          reason = "indexof_recv_mismatch";
          goto done;
        }
        if (arg_indexof != needle_reg) {
          reason = "indexof_needle_mismatch";
          goto done;
        }
        if (!((bname_indexof && !strcmp(bname_indexof, "RuntimeDataBox")) ||
              (bname_indexof && !strcmp(bname_indexof, "StringBox")))) {
          reason = "indexof_not_string_method";
          goto done;
        }

        size_t ll = kk + 1;
        long long carried_indexof = read_int(n_indexof, "dst");
        if (!carried_indexof) {
          reason = "indexof_missing_dst";
          goto done;
        }
        while (ll < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, ll);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_indexof)) break;
          if (out && out->indexof_copy_n < (int)(sizeof(out->indexof_copy_iis) / sizeof(out->indexof_copy_iis[0]))) {
            out->indexof_copy_iis[out->indexof_copy_n++] = ll;
          }
          carried_indexof = read_int(c, "dst");
          ll++;
        }
        if (ll + 5 >= ilen) {
          reason = "indexof_copy_chain_short";
          goto done;
        }

        yyjson_val* n_guard_copy = yyjson_arr_get(insts, ll);
        const char* op_guard_copy = n_guard_copy ? read_str(n_guard_copy, "op") : NULL;
        long long guard_carried = 0;
        if (!(op_guard_copy && !strcmp(op_guard_copy, "copy"))) {
          reason = "guard_copy_missing";
          goto done;
        }
        guard_carried = read_int(n_guard_copy, "dst");
        if (!guard_carried) {
          reason = "guard_copy_dst_missing";
          goto done;
        }
        if (!read_int(n_guard_copy, "src")) {
          reason = "guard_copy_src_missing";
          goto done;
        }

        yyjson_val* n_guard_mod_const = yyjson_arr_get(insts, ll + 1);
        yyjson_val* n_guard_mod = yyjson_arr_get(insts, ll + 2);
        yyjson_val* n_guard_zero = yyjson_arr_get(insts, ll + 3);
        yyjson_val* n_guard_compare = yyjson_arr_get(insts, ll + 4);
        yyjson_val* n_guard_branch = yyjson_arr_get(insts, ll + 5);
        long long mod_const_reg = 0;
        long long mod_const_val = 0;
        long long zero_reg = 0;
        long long zero_val = 0;
        long long guard_mod_dst = 0;
        long long guard_compare_dst = 0;
        long long bridge_bid = 0;
        yyjson_val* bridge_block = NULL;
        yyjson_val* bridge_insts = NULL;
        yyjson_val* consumer_block = NULL;
        yyjson_val* consumer_insts = NULL;
        long long consumer_bid = 0;
        size_t slen = 0;
        size_t ss = 0;
        long long carried_consumer = carried_indexof;

        if (!(n_guard_mod_const && read_str(n_guard_mod_const, "op") && !strcmp(read_str(n_guard_mod_const, "op"), "const"))) {
          reason = "guard_mod_const_missing";
          goto done;
        }
        mod_const_reg = read_int(n_guard_mod_const, "dst");
        if (!(has_const(mod_const_reg, &mod_const_val) || read_i64_const_from_inst(n_guard_mod_const, &mod_const_val)) || mod_const_val != 16) {
          reason = "guard_mod_const_not_16";
          goto done;
        }
        if (!(n_guard_mod && read_str(n_guard_mod, "op") && !strcmp(read_str(n_guard_mod, "op"), "binop"))) {
          reason = "guard_mod_missing";
          goto done;
        }
        {
          const char* guard_mod_op = read_str(n_guard_mod, "operation");
          if (!guard_mod_op) guard_mod_op = read_str(n_guard_mod, "op_kind");
          if (!(guard_mod_op && (!strcmp(guard_mod_op, "%") || !strcmp(guard_mod_op, "Mod")))) {
            reason = "guard_mod_not_percent";
            goto done;
          }
          if (!((read_int(n_guard_mod, "lhs") == guard_carried && read_int(n_guard_mod, "rhs") == mod_const_reg) ||
                (read_int(n_guard_mod, "rhs") == guard_carried && read_int(n_guard_mod, "lhs") == mod_const_reg))) {
            reason = "guard_mod_not_copy_percent_16";
            goto done;
          }
        }
        guard_mod_dst = read_int(n_guard_mod, "dst");
        if (!(n_guard_zero && read_str(n_guard_zero, "op") && !strcmp(read_str(n_guard_zero, "op"), "const"))) {
          reason = "guard_zero_missing";
          goto done;
        }
        zero_reg = read_int(n_guard_zero, "dst");
        if (!(has_const(zero_reg, &zero_val) || read_i64_const_from_inst(n_guard_zero, &zero_val)) || zero_val != 0) {
          reason = "guard_zero_not_zero";
          goto done;
        }
        if (!(n_guard_compare && read_str(n_guard_compare, "op") && !strcmp(read_str(n_guard_compare, "op"), "compare"))) {
          reason = "guard_compare_missing";
          goto done;
        }
        {
          const char* guard_compare_op = read_str(n_guard_compare, "operation");
          if (!guard_compare_op) guard_compare_op = read_str(n_guard_compare, "cmp");
          if (!(guard_compare_op && (!strcmp(guard_compare_op, "==") || !strcmp(guard_compare_op, "Eq") || !strcmp(guard_compare_op, "eq")))) {
            reason = "guard_compare_not_eq_zero";
            goto done;
          }
          if (!((read_int(n_guard_compare, "lhs") == guard_mod_dst && read_int(n_guard_compare, "rhs") == zero_reg) ||
                (read_int(n_guard_compare, "right") == zero_reg && read_int(n_guard_compare, "left") == guard_mod_dst))) {
            reason = "guard_compare_not_mod_eq_zero";
            goto done;
          }
        }
        guard_compare_dst = read_int(n_guard_compare, "dst");
        if (!(n_guard_branch && read_str(n_guard_branch, "op") && !strcmp(read_str(n_guard_branch, "op"), "branch"))) {
          reason = "guard_branch_missing";
          goto done;
        }
        if (read_int(n_guard_branch, "cond") != guard_compare_dst) {
          reason = "guard_branch_cond_mismatch";
          goto done;
        }
        bridge_bid = read_int(n_guard_branch, "else");
        if (!bridge_bid) {
          reason = "guard_else_missing";
          goto done;
        }
        bridge_block = find_block_by_id(blocks, bridge_bid);
        if (!bridge_block) {
          reason = "bridge_block_missing";
          goto done;
        }
        bridge_insts = yyjson_obj_get(bridge_block, "instructions");
        if (!(bridge_insts && yyjson_is_arr(bridge_insts) && yyjson_arr_size(bridge_insts) == 1)) {
          reason = "bridge_shape_not_single_jump";
          goto done;
        }
        {
          yyjson_val* n_bridge_jump = yyjson_arr_get(bridge_insts, 0);
          if (!(n_bridge_jump && read_str(n_bridge_jump, "op") && !strcmp(read_str(n_bridge_jump, "op"), "jump"))) {
            reason = "bridge_not_jump";
            goto done;
          }
          consumer_bid = read_int(n_bridge_jump, "target");
        }
        if (!consumer_bid) {
          reason = "bridge_target_missing";
          goto done;
        }
        consumer_block = find_block_by_id(blocks, consumer_bid);
        if (!consumer_block) {
          reason = "consumer_block_missing";
          goto done;
        }
        consumer_insts = yyjson_obj_get(consumer_block, "instructions");
        if (!(consumer_insts && yyjson_is_arr(consumer_insts))) {
          reason = "consumer_insts_missing";
          goto done;
        }
        slen = yyjson_arr_size(consumer_insts);
        while (ss < slen) {
          yyjson_val* c = yyjson_arr_get(consumer_insts, ss);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_consumer)) break;
          carried_consumer = read_int(c, "dst");
          ss++;
        }
        if (ss + 5 >= slen) {
          reason = "consumer_shape_short";
          goto done;
        }
        {
          yyjson_val* n_consumer_zero = yyjson_arr_get(consumer_insts, ss);
          yyjson_val* n_consumer_compare = yyjson_arr_get(consumer_insts, ss + 1);
          yyjson_val* n_base_copy = yyjson_arr_get(consumer_insts, ss + 2);
          yyjson_val* n_add_one = yyjson_arr_get(consumer_insts, ss + 3);
          yyjson_val* n_add = yyjson_arr_get(consumer_insts, ss + 4);
          yyjson_val* n_select = yyjson_arr_get(consumer_insts, ss + 5);
          long long consumer_zero_reg = 0;
          long long consumer_zero_val = 0;
          long long base_reg = 0;
          long long add_one_reg = 0;
          long long add_one_val = 0;
          if (!(n_consumer_zero && read_str(n_consumer_zero, "op") && !strcmp(read_str(n_consumer_zero, "op"), "const"))) {
            reason = "consumer_zero_missing";
            goto done;
          }
          consumer_zero_reg = read_int(n_consumer_zero, "dst");
          if (!(has_const(consumer_zero_reg, &consumer_zero_val) || read_i64_const_from_inst(n_consumer_zero, &consumer_zero_val)) || consumer_zero_val != 0) {
            reason = "consumer_zero_not_zero";
            goto done;
          }
          if (!(n_consumer_compare && read_str(n_consumer_compare, "op") && !strcmp(read_str(n_consumer_compare, "op"), "compare"))) {
            reason = "consumer_compare_missing";
            goto done;
          }
          {
            const char* consumer_compare_op = read_str(n_consumer_compare, "operation");
            if (!consumer_compare_op) consumer_compare_op = read_str(n_consumer_compare, "cmp");
            if (!(consumer_compare_op && (!strcmp(consumer_compare_op, ">=") || !strcmp(consumer_compare_op, "Ge") || !strcmp(consumer_compare_op, "ge")))) {
              reason = "consumer_compare_not_ge_zero";
              goto done;
            }
            if (!((read_int(n_consumer_compare, "lhs") == carried_consumer && read_int(n_consumer_compare, "rhs") == consumer_zero_reg) ||
                  (read_int(n_consumer_compare, "left") == carried_consumer && read_int(n_consumer_compare, "right") == consumer_zero_reg))) {
              reason = "consumer_compare_not_indexof_ge_zero";
              goto done;
            }
          }
          if (!(n_base_copy && read_str(n_base_copy, "op") && !strcmp(read_str(n_base_copy, "op"), "copy"))) {
            reason = "consumer_base_copy_missing";
            goto done;
          }
          base_reg = read_int(n_base_copy, "dst");
          if (!(n_add_one && read_str(n_add_one, "op") && !strcmp(read_str(n_add_one, "op"), "const"))) {
            reason = "consumer_add_one_const_missing";
            goto done;
          }
          add_one_reg = read_int(n_add_one, "dst");
          if (!(has_const(add_one_reg, &add_one_val) || read_i64_const_from_inst(n_add_one, &add_one_val)) || add_one_val != 1) {
            reason = "consumer_add_one_not_1";
            goto done;
          }
          if (!(n_add && read_str(n_add, "op") && !strcmp(read_str(n_add, "op"), "binop"))) {
            reason = "consumer_add_missing";
            goto done;
          }
          {
            const char* add_op = read_str(n_add, "operation");
            if (!add_op) add_op = read_str(n_add, "op_kind");
            if (!(add_op && (!strcmp(add_op, "+") || !strcmp(add_op, "Add")))) {
              reason = "consumer_add_not_plus";
              goto done;
            }
            if (!((read_int(n_add, "lhs") == base_reg && read_int(n_add, "rhs") == add_one_reg) ||
                  (read_int(n_add, "rhs") == base_reg && read_int(n_add, "lhs") == add_one_reg))) {
              reason = "consumer_add_not_base_plus_1";
              goto done;
            }
          }
          if (!(n_select && read_str(n_select, "op") && !strcmp(read_str(n_select, "op"), "select"))) {
            reason = "consumer_select_missing";
            goto done;
          }
          if (!(read_int(n_select, "cond") == read_int(n_consumer_compare, "dst") &&
                read_int(n_select, "then_val") == read_int(n_add, "dst") &&
                read_int(n_select, "else_val") == base_reg)) {
            reason = "consumer_select_not_compare_terminal";
            goto done;
          }
        }

        if (out) {
          out->matched = 1;
          out->needle_const_ii = jj;
          out->needle_reg = needle_reg;
          out->indexof_ii = kk;
          out->indexof_dst = read_int(n_indexof, "dst");
          out->guard_compare_ii = ll + 4;
          out->guard_branch_ii = ll + 5;
          out->bridge_bid = bridge_bid;
          out->consumer_bid = consumer_bid;
        }
      done:
        if (extra && extra_cap) {
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld get_copy_n=%d recv_copy_n=%d indexof_copy_n=%d indexof_ii=%zu guard_branch_ii=%zu bridge_bid=%lld consumer_bid=%lld",
                   ii, cur_dst, recv, idx,
                   out ? out->get_copy_n : 0,
                   out ? out->recv_copy_n : 0,
                   out ? out->indexof_copy_n : 0,
                   out ? out->indexof_ii : 0,
                   out ? out->guard_branch_ii : 0,
                   out ? out->bridge_bid : 0,
                   out ? out->consumer_bid : 0);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
