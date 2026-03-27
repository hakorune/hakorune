      int array_string_indexof_select_trace_budget = 8;
      int array_string_indexof_branch_trace_budget = 8;
      int array_string_indexof_cross_block_select_trace_budget = 8;
      int array_string_indexof_interleaved_branch_trace_budget = 8;
      int array_string_indexof_interleaved_select_trace_budget = 8;
      auto void trace_array_string_indexof_select_window_candidate(
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
        if (!hako_llvmc_route_trace_enabled() || array_string_indexof_select_trace_budget <= 0) return;
        array_string_indexof_select_trace_budget--;
        struct ArrayStringIndexofSelectWindowMatch match;
        const char* reason = NULL;
        char extra[320];
        int matched = analyze_array_string_indexof_select_window_candidate(
            ii, ilen, insts, bname, mname, recv, idx, cur_dst, recv_org,
            runtime_array_string, &match, &reason, extra, sizeof(extra));
        hako_llvmc_emit_route_trace("array_string_indexof_select_window", matched ? "hit" : "miss", reason ? reason : "unknown", extra);
      }
      auto void trace_array_string_indexof_branch_window_candidate(
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
        if (!hako_llvmc_route_trace_enabled() || array_string_indexof_branch_trace_budget <= 0) return;
        array_string_indexof_branch_trace_budget--;
        struct ArrayStringIndexofBranchWindowMatch match;
        const char* reason = NULL;
        char extra[320];
        int matched = analyze_array_string_indexof_branch_window_candidate(
            ii, ilen, insts, bname, mname, recv, idx, cur_dst, recv_org,
            runtime_array_string, &match, &reason, extra, sizeof(extra));
        hako_llvmc_emit_route_trace("array_string_indexof_branch_window", matched ? "hit" : "miss", reason ? reason : "unknown", extra);
      }
      auto void trace_array_string_indexof_cross_block_select_window_candidate(
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
          int runtime_array_string) {
        if (!hako_llvmc_route_trace_enabled() || array_string_indexof_cross_block_select_trace_budget <= 0) return;
        array_string_indexof_cross_block_select_trace_budget--;
        struct ArrayStringIndexofCrossBlockSelectWindowMatch match;
        const char* reason = NULL;
        char extra[352];
        int matched = analyze_array_string_indexof_cross_block_select_window_candidate(
            blocks, ii, ilen, insts, bname, mname, recv, idx, cur_dst, recv_org,
            runtime_array_string, &match, &reason, extra, sizeof(extra));
        hako_llvmc_emit_route_trace("array_string_indexof_cross_block_select_window", matched ? "hit" : "miss", reason ? reason : "unknown", extra);
      }
      auto void trace_array_string_indexof_interleaved_branch_window_candidate(
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
          int runtime_array_string) {
        if (!hako_llvmc_route_trace_enabled() || array_string_indexof_interleaved_branch_trace_budget <= 0) return;
        array_string_indexof_interleaved_branch_trace_budget--;
        struct ArrayStringIndexofInterleavedBranchWindowMatch match;
        const char* reason = NULL;
        char extra[352];
        int matched = analyze_array_string_indexof_interleaved_branch_window_candidate(
            blocks, ii, ilen, insts, bname, mname, recv, idx, cur_dst, recv_org,
            runtime_array_string, &match, &reason, extra, sizeof(extra));
        hako_llvmc_emit_route_trace("array_string_indexof_interleaved_branch_window", matched ? "hit" : "miss", reason ? reason : "unknown", extra);
      }
      auto void trace_array_string_indexof_interleaved_select_window_candidate(
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
          int runtime_array_string) {
        if (!hako_llvmc_route_trace_enabled() || array_string_indexof_interleaved_select_trace_budget <= 0) return;
        array_string_indexof_interleaved_select_trace_budget--;
        struct ArrayStringIndexofInterleavedSelectWindowMatch match;
        const char* reason = NULL;
        char extra[384];
        int matched = analyze_array_string_indexof_interleaved_select_window_candidate(
            blocks, ii, ilen, insts, bname, mname, recv, idx, cur_dst, recv_org,
            runtime_array_string, &match, &reason, extra, sizeof(extra));
        hako_llvmc_emit_route_trace("array_string_indexof_interleaved_select_window", matched ? "hit" : "miss", reason ? reason : "unknown", extra);
      }
