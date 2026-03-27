      auto void append_i64_arg_ref(char* out, size_t out_cap, long long reg, int first) {
        long long cv = 0;
        char tmp[64];
        size_t used = strlen(out);
        if (used >= out_cap) return;
        if (has_const(reg, &cv)) snprintf(tmp, sizeof(tmp), "i64 %lld", cv);
        else snprintf(tmp, sizeof(tmp), "i64 %%r%lld", reg);
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
          remember_deferred_array_string_indexof_select(
              bid,
              string_indexof_branch_match.indexof_ii,
              recv,
              idx,
              string_indexof_branch_match.needle_reg);
          for (int si = 0; si < string_indexof_branch_match.get_copy_n; si++) skip_inst[string_indexof_branch_match.get_copy_iis[si]] = 1;
          for (int si = 0; si < string_indexof_branch_match.recv_copy_n; si++) skip_inst[string_indexof_branch_match.recv_copy_iis[si]] = 1;
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
        if (dst) {
          fprintf(f, "  %%r%lld = call i64 @\"nyash.array.string_indexof_hih\"(%s)\n", dst, args);
          set_type(dst, T_I64);
        } else {
          fprintf(f, "  call i64 @\"nyash.array.string_indexof_hih\"(%s)\n", args);
        }
        return 1;
      }
