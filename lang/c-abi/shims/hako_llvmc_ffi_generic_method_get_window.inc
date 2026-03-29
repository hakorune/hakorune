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
        size_t copy_iis[8];
        int copy_n;
        size_t len_ii;
        long long len_dst;
      };
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
      auto int inst_uses_reg(yyjson_val* ins, long long reg) {
        if (!ins || !reg) return 0;
        const char* op = read_str(ins, "op");
        if (!op) return 0;
        if (strcmp(op, "copy") == 0) return read_int(ins, "src") == reg;
        if (strcmp(op, "binop") == 0) {
          return read_int(ins, "lhs") == reg || read_int(ins, "rhs") == reg;
        }
        if (strcmp(op, "compare") == 0) {
          return read_int(ins, "lhs") == reg || read_int(ins, "left") == reg ||
                 read_int(ins, "rhs") == reg || read_int(ins, "right") == reg;
        }
        if (strcmp(op, "select") == 0) {
          return read_int(ins, "cond") == reg ||
                 read_int(ins, "then_val") == reg ||
                 read_int(ins, "else_val") == reg;
        }
        if (strcmp(op, "branch") == 0) return read_int(ins, "cond") == reg;
        if (strcmp(op, "ret") == 0) return read_int(ins, "value") == reg;
        if (strcmp(op, "mir_call") == 0) {
          yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
          yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
          if (cal && read_int(cal, "receiver") == reg) return 1;
          yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
          if (args && yyjson_is_arr(args)) {
            size_t n = yyjson_arr_size(args);
            for (size_t i = 0; i < n; i++) {
              yyjson_val* a = yyjson_arr_get(args, i);
              if (a && (long long)yyjson_get_sint(a) == reg) return 1;
            }
          }
        }
        return 0;
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
        if (!((bname && !strcmp(bname, "ArrayBox")) ||
              runtime_array_get ||
              runtime_array_string ||
              (bname && !strcmp(bname, "RuntimeDataBox") &&
               (recv_org == ORG_ARRAY_BIRTH || recv_org == ORG_ARRAY_STRING_BIRTH)))) {
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
        if (!(runtime_array_string ||
              (bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_STRING_BIRTH))) {
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
          for (size_t kk = jj + 1; kk < ilen; kk++) {
            yyjson_val* later = yyjson_arr_get(insts, kk);
            if (!(inst_uses_reg(later, cur_dst) || inst_uses_reg(later, carried))) continue;
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
            out->len_ii = jj;
            out->len_dst = read_int(n_len, "dst");
          }
        }
      done:
        if (extra && extra_cap) {
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld copy_n=%d len_ii=%zu len_dst=%lld keep_get_live=%d producer_kind=ArrayGet boundary_kind=Store post_store_use=LenObserver known_len=-1 direct_set_only=0",
                   ii, cur_dst, recv, idx,
                   out ? out->copy_n : 0,
                   out ? out->len_ii : 0,
                   out ? out->len_dst : 0,
                   out ? out->keep_get_live : 0);
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
