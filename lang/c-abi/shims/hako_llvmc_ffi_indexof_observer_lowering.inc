      auto void append_i64_arg_ref(char* out, size_t out_cap, long long reg, int first) {
        long long cv = 0;
        long long base = resolve_copy_source_reg(reg);
        char tmp[64];
        size_t used = strlen(out);
        if (used >= out_cap) return;
        if (has_const(base, &cv)) snprintf(tmp, sizeof(tmp), "i64 %lld", cv);
        else snprintf(tmp, sizeof(tmp), "i64 %%r%lld", base);
        snprintf(out + used, out_cap - used, "%s%s", first ? "" : ", ", tmp);
      }
      auto void append_array_string_indexof_method_receiver_args(
          struct DeferredArrayStringIndexofSelect* deferred_array_string_indexof,
          long long recv,
          char* out,
          size_t out_cap) {
        if (deferred_array_string_indexof) {
          append_i64_arg_ref(out, out_cap, deferred_array_string_indexof->recv, 1);
          append_i64_arg_ref(out, out_cap, deferred_array_string_indexof->idx, 0);
          append_i64_arg_ref(out, out_cap, deferred_array_string_indexof->needle_reg, 0);
        } else if (recv) {
          append_i64_arg_ref(out, out_cap, recv, 1);
        }
      }
      auto int try_defer_array_string_indexof_get_window(
          FILE* f,
          yyjson_val* blocks,
          long long bid,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long dst,
          int recv_org,
          int runtime_array_string,
          unsigned char* skip_inst) {
        struct ArrayStringIndexofSelectWindowMatch string_indexof_select_match;
        struct ArrayStringIndexofBranchWindowMatch string_indexof_branch_match;
        struct ArrayStringIndexofCrossBlockSelectWindowMatch string_indexof_cross_block_select_match;
        struct ArrayStringIndexofInterleavedBranchWindowMatch string_indexof_interleaved_branch_match;
        struct ArrayStringIndexofInterleavedSelectWindowMatch string_indexof_interleaved_select_match;
        trace_array_string_indexof_select_window_candidate(ii, ilen, insts, bname, mname, recv, idx, dst, recv_org, runtime_array_string);
        trace_array_string_indexof_branch_window_candidate(ii, ilen, insts, bname, mname, recv, idx, dst, recv_org, runtime_array_string);
        trace_array_string_indexof_cross_block_select_window_candidate(blocks, ii, ilen, insts, bname, mname, recv, idx, dst, recv_org, runtime_array_string);
        trace_array_string_indexof_interleaved_branch_window_candidate(blocks, ii, ilen, insts, bname, mname, recv, idx, dst, recv_org, runtime_array_string);
        trace_array_string_indexof_interleaved_select_window_candidate(blocks, ii, ilen, insts, bname, mname, recv, idx, dst, recv_org, runtime_array_string);
        if (analyze_array_string_indexof_select_window_candidate(
                ii, ilen, insts, bname, mname, recv, idx, dst, recv_org,
                runtime_array_string, &string_indexof_select_match, NULL, NULL, 0)) {
          remember_deferred_array_string_indexof_select(
              bid,
              string_indexof_select_match.indexof_ii,
              recv,
              idx,
              string_indexof_select_match.needle_reg);
          for (int si = 0; si < string_indexof_select_match.get_copy_n; si++) skip_inst[string_indexof_select_match.get_copy_iis[si]] = 1;
          for (int si = 0; si < string_indexof_select_match.recv_copy_n; si++) skip_inst[string_indexof_select_match.recv_copy_iis[si]] = 1;
          return 1;
        }
        if (analyze_array_string_indexof_branch_window_candidate(
                ii, ilen, insts, bname, mname, recv, idx, dst, recv_org,
                runtime_array_string, &string_indexof_branch_match, NULL, NULL, 0)) {
          int branch_uses_get =
              branch_targets_use_reg(blocks, insts, string_indexof_branch_match.branch_ii, dst);
          int branch_uses_get_only_as_slot_const_suffix_store =
              branch_uses_get &&
              branch_targets_use_reg_only_as_const_suffix_store(
                  blocks,
                  insts,
                  string_indexof_branch_match.branch_ii,
                  dst,
                  recv,
                  idx);
          string_indexof_branch_match.keep_get_live =
              branch_uses_get && !branch_uses_get_only_as_slot_const_suffix_store;
          if (string_indexof_branch_match.keep_get_live) {
            char keep_ab[128];
            keep_ab[0] = '\0';
            append_i64_arg_ref(keep_ab, sizeof(keep_ab), recv, 1);
            append_i64_arg_ref(keep_ab, sizeof(keep_ab), idx, 0);
            if (dst) {
              hako_llvmc_emit_array_slot_load(
                  f, dst, keep_ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_REF);
            } else {
              hako_llvmc_emit_array_slot_load(
                  f, dst, keep_ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_REF);
            }
          } else if (branch_uses_get_only_as_slot_const_suffix_store) {
            remember_array_string_get_source(dst, recv, idx);
            if (dst) {
              set_type(dst, T_I64);
              set_origin(dst, ORG_STRING);
              set_scan_origin(dst, ORG_STRING);
            }
          }
          remember_deferred_array_string_indexof_select(
              bid,
              string_indexof_branch_match.indexof_ii,
              recv,
              idx,
              string_indexof_branch_match.needle_reg);
          if (!string_indexof_branch_match.keep_get_live) {
            for (int si = 0; si < string_indexof_branch_match.get_copy_n; si++) skip_inst[string_indexof_branch_match.get_copy_iis[si]] = 1;
            for (int si = 0; si < string_indexof_branch_match.recv_copy_n; si++) skip_inst[string_indexof_branch_match.recv_copy_iis[si]] = 1;
          }
          return 1;
        }
        if (analyze_array_string_indexof_cross_block_select_window_candidate(
                blocks, ii, ilen, insts, bname, mname, recv, idx, dst, recv_org,
                runtime_array_string, &string_indexof_cross_block_select_match, NULL, NULL, 0)) {
          remember_deferred_array_string_indexof_select(
              bid,
              string_indexof_cross_block_select_match.indexof_ii,
              recv,
              idx,
              string_indexof_cross_block_select_match.needle_reg);
          for (int si = 0; si < string_indexof_cross_block_select_match.get_copy_n; si++) skip_inst[string_indexof_cross_block_select_match.get_copy_iis[si]] = 1;
          for (int si = 0; si < string_indexof_cross_block_select_match.recv_copy_n; si++) skip_inst[string_indexof_cross_block_select_match.recv_copy_iis[si]] = 1;
          return 1;
        }
        if (analyze_array_string_indexof_interleaved_branch_window_candidate(
                blocks, ii, ilen, insts, bname, mname, recv, idx, dst, recv_org,
                runtime_array_string, &string_indexof_interleaved_branch_match, NULL, NULL, 0)) {
          remember_deferred_array_string_indexof_select(
              bid,
              string_indexof_interleaved_branch_match.indexof_ii,
              recv,
              idx,
              string_indexof_interleaved_branch_match.needle_reg);
          for (int si = 0; si < string_indexof_interleaved_branch_match.get_copy_n; si++) skip_inst[string_indexof_interleaved_branch_match.get_copy_iis[si]] = 1;
          for (int si = 0; si < string_indexof_interleaved_branch_match.recv_copy_n; si++) skip_inst[string_indexof_interleaved_branch_match.recv_copy_iis[si]] = 1;
          return 1;
        }
        if (analyze_array_string_indexof_interleaved_select_window_candidate(
                blocks, ii, ilen, insts, bname, mname, recv, idx, dst, recv_org,
                runtime_array_string, &string_indexof_interleaved_select_match, NULL, NULL, 0)) {
          remember_deferred_array_string_indexof_select(
              bid,
              string_indexof_interleaved_select_match.indexof_ii,
              recv,
              idx,
              string_indexof_interleaved_select_match.needle_reg);
          for (int si = 0; si < string_indexof_interleaved_select_match.get_copy_n; si++) skip_inst[string_indexof_interleaved_select_match.get_copy_iis[si]] = 1;
          for (int si = 0; si < string_indexof_interleaved_select_match.recv_copy_n; si++) skip_inst[string_indexof_interleaved_select_match.recv_copy_iis[si]] = 1;
          return 1;
        }
        return 0;
      }
      auto int emit_deferred_array_string_indexof_call(
          FILE* f,
          struct DeferredArrayStringIndexofSelect* deferred_array_string_indexof,
          long long dst,
          const char* args) {
        if (!deferred_array_string_indexof) return 0;
        hako_llvmc_emit_array_string_indexof(f, dst, args);
        return 1;
      }
