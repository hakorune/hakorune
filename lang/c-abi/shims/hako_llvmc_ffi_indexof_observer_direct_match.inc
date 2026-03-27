      auto int analyze_array_string_indexof_select_window_candidate(
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
          struct ArrayStringIndexofSelectWindowMatch* out,
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
        if (ii + 6 >= ilen) {
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
        if (jj + 5 >= ilen) {
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
        if (kk + 4 >= ilen) {
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
        if (ll + 4 >= ilen) {
          reason = "indexof_copy_chain_short";
          goto done;
        }

        yyjson_val* n_zero = yyjson_arr_get(insts, ll);
        yyjson_val* n_compare = yyjson_arr_get(insts, ll + 1);
        yyjson_val* n_then = yyjson_arr_get(insts, ll + 2);
        yyjson_val* n_else = yyjson_arr_get(insts, ll + 3);
        yyjson_val* n_select = yyjson_arr_get(insts, ll + 4);
        const char* op_zero = n_zero ? read_str(n_zero, "op") : NULL;
        const char* op_compare = n_compare ? read_str(n_compare, "op") : NULL;
        const char* op_then = n_then ? read_str(n_then, "op") : NULL;
        const char* op_else = n_else ? read_str(n_else, "op") : NULL;
        const char* op_select = n_select ? read_str(n_select, "op") : NULL;
        long long zero_reg = 0;
        long long zero_val = 0;
        long long then_reg = 0;
        long long else_reg = 0;
        long long then_val = 0;
        long long else_val = 0;
        long long compare_dst = 0;
        const char* compare_op = NULL;
        if (!(op_zero && strcmp(op_zero, "const") == 0)) {
          reason = "next_after_indexof_not_zero_const";
          goto done;
        }
        zero_reg = read_int(n_zero, "dst");
        if (!(has_const(zero_reg, &zero_val) || read_i64_const_from_inst(n_zero, &zero_val)) || zero_val != 0) {
          reason = "compare_const_not_zero";
          goto done;
        }
        if (!(op_compare && strcmp(op_compare, "compare") == 0)) {
          reason = "next_after_zero_not_compare";
          goto done;
        }
        compare_op = read_str(n_compare, "operation");
        if (!compare_op) compare_op = read_str(n_compare, "cmp");
        if (!(compare_op && (!strcmp(compare_op, ">=") || !strcmp(compare_op, "Ge") || !strcmp(compare_op, "ge")))) {
          reason = "compare_not_ge_zero";
          goto done;
        }
        {
          long long lhs = read_int(n_compare, "lhs");
          if (!lhs) lhs = read_int(n_compare, "left");
          long long rhs = read_int(n_compare, "rhs");
          if (!rhs) rhs = read_int(n_compare, "right");
          if (!(lhs == carried_indexof && rhs == zero_reg)) {
            reason = "compare_not_indexof_ge_zero";
            goto done;
          }
        }
        compare_dst = read_int(n_compare, "dst");
        if (!(op_then && strcmp(op_then, "const") == 0)) {
          reason = "then_not_const";
          goto done;
        }
        then_reg = read_int(n_then, "dst");
        if (!(has_const(then_reg, &then_val) || read_i64_const_from_inst(n_then, &then_val))) {
          reason = "then_not_i64_const";
          goto done;
        }
        if (!(op_else && strcmp(op_else, "const") == 0)) {
          reason = "else_not_const";
          goto done;
        }
        else_reg = read_int(n_else, "dst");
        if (!(has_const(else_reg, &else_val) || read_i64_const_from_inst(n_else, &else_val))) {
          reason = "else_not_i64_const";
          goto done;
        }
        if (!(op_select && strcmp(op_select, "select") == 0)) {
          reason = "next_after_else_not_select";
          goto done;
        }
        if (!(read_int(n_select, "cond") == compare_dst &&
              read_int(n_select, "then_val") == then_reg &&
              read_int(n_select, "else_val") == else_reg)) {
          reason = "select_not_compare_terminal";
          goto done;
        }

        if (out) {
          for (size_t mm = ll + 5; mm < ilen; mm++) {
            yyjson_val* later = yyjson_arr_get(insts, mm);
            if (inst_uses_reg(later, cur_dst) || inst_uses_reg(later, carried_get) || inst_uses_reg(later, carried_recv)) {
              reason = "post_select_uses_consumed_get_value";
              goto done;
            }
          }
          out->matched = 1;
          out->needle_const_ii = jj;
          out->needle_reg = needle_reg;
          out->indexof_ii = kk;
          out->indexof_dst = read_int(n_indexof, "dst");
          out->zero_const_ii = ll;
          out->compare_ii = ll + 1;
          out->then_const_ii = ll + 2;
          out->else_const_ii = ll + 3;
          out->select_ii = ll + 4;
        }
      done:
        if (extra && extra_cap) {
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld get_copy_n=%d recv_copy_n=%d indexof_copy_n=%d indexof_ii=%zu compare_ii=%zu select_ii=%zu",
                   ii, cur_dst, recv, idx,
                   out ? out->get_copy_n : 0,
                   out ? out->recv_copy_n : 0,
                   out ? out->indexof_copy_n : 0,
                   out ? out->indexof_ii : 0,
                   out ? out->compare_ii : 0,
                   out ? out->select_ii : 0);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
      auto int analyze_array_string_indexof_branch_window_candidate(
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
          struct ArrayStringIndexofBranchWindowMatch* out,
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
        if (ii + 5 >= ilen) {
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
        if (jj + 4 >= ilen) {
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
        if (kk + 3 >= ilen) {
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
        if (ll + 2 >= ilen) {
          reason = "indexof_copy_chain_short";
          goto done;
        }

        yyjson_val* n_zero = yyjson_arr_get(insts, ll);
        yyjson_val* n_compare = yyjson_arr_get(insts, ll + 1);
        yyjson_val* n_branch = yyjson_arr_get(insts, ll + 2);
        const char* op_zero = n_zero ? read_str(n_zero, "op") : NULL;
        const char* op_compare = n_compare ? read_str(n_compare, "op") : NULL;
        const char* op_branch = n_branch ? read_str(n_branch, "op") : NULL;
        long long zero_reg = 0;
        long long zero_val = 0;
        long long compare_dst = 0;
        const char* compare_op = NULL;
        if (!(op_zero && strcmp(op_zero, "const") == 0)) {
          reason = "next_after_indexof_not_zero_const";
          goto done;
        }
        zero_reg = read_int(n_zero, "dst");
        if (!(has_const(zero_reg, &zero_val) || read_i64_const_from_inst(n_zero, &zero_val)) || zero_val != 0) {
          reason = "compare_const_not_zero";
          goto done;
        }
        if (!(op_compare && strcmp(op_compare, "compare") == 0)) {
          reason = "next_after_zero_not_compare";
          goto done;
        }
        compare_op = read_str(n_compare, "operation");
        if (!compare_op) compare_op = read_str(n_compare, "cmp");
        if (!(compare_op && (!strcmp(compare_op, ">=") || !strcmp(compare_op, "Ge") || !strcmp(compare_op, "ge")))) {
          reason = "compare_not_ge_zero";
          goto done;
        }
        {
          long long lhs = read_int(n_compare, "lhs");
          if (!lhs) lhs = read_int(n_compare, "left");
          long long rhs = read_int(n_compare, "rhs");
          if (!rhs) rhs = read_int(n_compare, "right");
          if (!(lhs == carried_indexof && rhs == zero_reg)) {
            reason = "compare_not_indexof_ge_zero";
            goto done;
          }
        }
        compare_dst = read_int(n_compare, "dst");
        if (!(op_branch && strcmp(op_branch, "branch") == 0)) {
          reason = "next_after_compare_not_branch";
          goto done;
        }
        if (read_int(n_branch, "cond") != compare_dst) {
          reason = "branch_not_compare_terminal";
          goto done;
        }

        if (out) {
          out->matched = 1;
          out->needle_const_ii = jj;
          out->needle_reg = needle_reg;
          out->indexof_ii = kk;
          out->indexof_dst = read_int(n_indexof, "dst");
          out->zero_const_ii = ll;
          out->compare_ii = ll + 1;
          out->branch_ii = ll + 2;
        }
      done:
        if (extra && extra_cap) {
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld get_copy_n=%d recv_copy_n=%d indexof_copy_n=%d indexof_ii=%zu compare_ii=%zu branch_ii=%zu",
                   ii, cur_dst, recv, idx,
                   out ? out->get_copy_n : 0,
                   out ? out->recv_copy_n : 0,
                   out ? out->indexof_copy_n : 0,
                   out ? out->indexof_ii : 0,
                   out ? out->compare_ii : 0,
                   out ? out->branch_ii : 0);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
