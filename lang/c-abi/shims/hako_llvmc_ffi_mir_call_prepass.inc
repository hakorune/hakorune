      auto void mark_array_string_indexof_need_flags(
          yyjson_val* blocks,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long a0,
          long long dst,
          int scan_org,
          struct GenericPureNeedFlags* needs) {
        struct ArrayStringIndexofSelectWindowMatch indexof_select_match;
        struct ArrayStringIndexofBranchWindowMatch indexof_branch_match;
        struct ArrayStringIndexofCrossBlockSelectWindowMatch indexof_cross_block_select_match;
        struct ArrayStringIndexofInterleavedBranchWindowMatch indexof_interleaved_branch_match;
        struct ArrayStringIndexofInterleavedSelectWindowMatch indexof_interleaved_select_match;

        if (!needs) return;
        if (analyze_array_string_indexof_select_window_candidate(
                ii, ilen, insts, bname, mname, recv, a0, dst, scan_org, 1,
                &indexof_select_match, NULL, NULL, 0)) {
          needs->arr_string_indexof = 1;
        } else if (analyze_array_string_indexof_branch_window_candidate(
                ii, ilen, insts, bname, mname, recv, a0, dst, scan_org, 1,
                &indexof_branch_match, NULL, NULL, 0)) {
          needs->arr_string_indexof = 1;
          if (branch_targets_use_reg(blocks, insts, indexof_branch_match.branch_ii, dst)) {
            if (branch_targets_use_reg_only_as_const_suffix_store(
                    blocks,
                    insts,
                    indexof_branch_match.branch_ii,
                    dst,
                    recv,
                    a0)) {
              needs->arr_kernel_slot_concat_by_index = 1;
            } else {
              needs->arr_get = 1;
              needs->arr_slot_load = 1;
            }
          }
        } else if (analyze_array_string_indexof_cross_block_select_window_candidate(
                blocks, ii, ilen, insts, bname, mname, recv, a0, dst, scan_org, 1,
                &indexof_cross_block_select_match, NULL, NULL, 0)) {
          needs->arr_string_indexof = 1;
        } else if (analyze_array_string_indexof_interleaved_branch_window_candidate(
                blocks, ii, ilen, insts, bname, mname, recv, a0, dst, scan_org, 1,
                &indexof_interleaved_branch_match, NULL, NULL, 0)) {
          needs->arr_string_indexof = 1;
        } else if (analyze_array_string_indexof_interleaved_select_window_candidate(
                blocks, ii, ilen, insts, bname, mname, recv, a0, dst, scan_org, 1,
                &indexof_interleaved_select_match, NULL, NULL, 0)) {
          needs->arr_string_indexof = 1;
        } else {
          needs->arr_get = 1;
          needs->arr_slot_load = 1;
        }
        if (dst) set_scan_origin(dst, ORG_STRING);
      }

      auto void scan_mir_call_need_flags(
          yyjson_val* blocks,
          long long bid,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          yyjson_val* ins,
          struct GenericPureNeedFlags* needs) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        const char* ctype = cal ? read_str(cal, "type") : NULL;
        const char* bname = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
        const char* mname = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;

        if (!needs) return;
        if (ctype && !strcmp(ctype, "Constructor")) {
          long long dst = read_int(ins, "dst");
          enum MirCallNeedKind need_kind =
              classify_mir_call_constructor_need_kind(bname);
          apply_mir_call_need_kind(needs, need_kind);
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH) {
            if (dst) set_scan_origin(dst, ORG_MAP_BIRTH);
          }
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH) {
            if (dst) set_scan_origin(dst, ORG_ARRAY_BIRTH);
          }
          return;
        }

        if (ctype && !strcmp(ctype, "Method")) {
          long long recv = cal ? read_int(cal, "receiver") : 0;
          yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
          long long a0 = 0;
          long long a1 = 0;
          struct GenericMethodRouteState route;
          enum MirCallReceiverFamilyKind recv_family;
          enum MirCallNeedKind need_kind;
          if (args && yyjson_is_arr(args)) {
            if (yyjson_arr_size(args) >= 1) a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
            if (yyjson_arr_size(args) >= 2) a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
          }
          route = classify_generic_method_route(bid, ii, bname, mname, recv);
          recv_family = classify_mir_call_receiver_family(route.recv_org);
          need_kind = classify_mir_call_method_need_kind(bname, mname, recv_family);
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_NONE &&
              mname &&
              route.runtime_string) {
            if (!strcmp(mname, "indexOf")) {
              need_kind = HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF;
            } else if (!strcmp(mname, "substring")) {
              need_kind = HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING;
            }
          }
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_STRING_INDEXOF) {
            mark_array_string_indexof_need_flags(
                blocks, ii, ilen, insts, bname, mname, recv, a0,
                read_int(ins, "dst"), route.recv_org, needs);
            return;
          }
          apply_mir_call_need_kind(needs, need_kind);
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH &&
              route.recv_org == ORG_ARRAY_BIRTH &&
              a0 && get_scan_origin(a0) == ORG_STRING) {
            promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
          }
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET &&
              route.recv_org == ORG_ARRAY_BIRTH &&
              a1 && get_scan_origin(a1) == ORG_STRING) {
            promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
          }
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING) {
            if (read_int(ins, "dst")) set_scan_origin(read_int(ins, "dst"), ORG_STRING);
          }
          return;
        }

        if (ctype && !strcmp(ctype, "Global")) {
          apply_mir_call_need_kind(needs, classify_mir_call_global_need_kind(mname));
          return;
        }

        if (ctype && !strcmp(ctype, "Extern") && mname) {
          enum MirCallNeedKind need_kind =
              classify_mir_call_extern_need_kind(mname);
          apply_mir_call_need_kind(needs, need_kind);
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT &&
              read_int(ins, "dst")) {
            set_scan_origin(read_int(ins, "dst"), ORG_STRING);
          }
        }
      }
