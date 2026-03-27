      struct GenericPureNeedFlags {
        int map_birth;
        int map_set;
        int map_size;
        int map_get;
        int map_has;
        int arr_birth;
        int arr_push;
        int arr_len;
        int arr_set;
        int arr_get;
        int arr_has;
        int arr_string_indexof;
        int string_const_box;
        int string_len;
        int string_substring;
        int string_indexof;
        int string_concat;
        int printf;
      };

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
            needs->arr_get = 1;
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
          if (bname && !strcmp(bname, "MapBox")) {
            needs->map_birth = 1;
            if (dst) set_scan_origin(dst, ORG_MAP_BIRTH);
          }
          if (bname && !strcmp(bname, "ArrayBox")) {
            needs->arr_birth = 1;
            if (dst) set_scan_origin(dst, ORG_ARRAY_BIRTH);
          }
          return;
        }

        if (ctype && !strcmp(ctype, "Method")) {
          long long recv = cal ? read_int(cal, "receiver") : 0;
          yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
          long long a0 = 0;
          long long a1 = 0;
          int scan_org = recv ? get_scan_origin(recv) : ORG_NONE;
          if (args && yyjson_is_arr(args)) {
            if (yyjson_arr_size(args) >= 1) a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
            if (yyjson_arr_size(args) >= 2) a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
          }

          if (bname && !strcmp(bname, "MapBox")) {
            if (!mname) return;
            if (!strcmp(mname, "set")) needs->map_set = 1;
            else if (!strcmp(mname, "size") || !strcmp(mname, "len")) needs->map_size = 1;
            else if (!strcmp(mname, "get")) needs->map_get = 1;
            else if (!strcmp(mname, "has")) needs->map_has = 1;
            return;
          }

          if (bname && !strcmp(bname, "ArrayBox")) {
            if (!mname) return;
            if (!strcmp(mname, "push")) needs->arr_push = 1;
            else if (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size")) needs->arr_len = 1;
            else if (!strcmp(mname, "set")) needs->arr_set = 1;
            else if (!strcmp(mname, "get")) needs->arr_get = 1;
            return;
          }

          if (!(bname && !strcmp(bname, "RuntimeDataBox"))) return;

          if (scan_org == ORG_ARRAY_BIRTH && mname && !strcmp(mname, "get")) needs->arr_get = 1;
          if (scan_org == ORG_ARRAY_BIRTH && mname && !strcmp(mname, "push")) {
            needs->arr_push = 1;
            if (a0 && get_scan_origin(a0) == ORG_STRING) {
              promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
            }
          }
          if (scan_org == ORG_ARRAY_BIRTH && mname &&
              (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size"))) {
            needs->arr_len = 1;
          }
          if (scan_org == ORG_ARRAY_BIRTH && mname && !strcmp(mname, "has")) needs->arr_has = 1;
          if (scan_org == ORG_ARRAY_BIRTH && mname && !strcmp(mname, "set")) {
            needs->arr_set = 1;
            if (a1 && get_scan_origin(a1) == ORG_STRING) {
              promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
            }
          }
          if (scan_org == ORG_ARRAY_STRING_BIRTH && mname && !strcmp(mname, "get")) {
            mark_array_string_indexof_need_flags(
                blocks, ii, ilen, insts, bname, mname, recv, a0, read_int(ins, "dst"),
                scan_org, needs);
          }
          if (scan_org == ORG_ARRAY_STRING_BIRTH && mname && !strcmp(mname, "push")) needs->arr_push = 1;
          if (scan_org == ORG_ARRAY_STRING_BIRTH && mname && !strcmp(mname, "set")) needs->arr_set = 1;
          if (scan_org == ORG_ARRAY_STRING_BIRTH && mname &&
              (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size"))) {
            needs->arr_len = 1;
          }
          if (scan_org == ORG_MAP_BIRTH && mname && !strcmp(mname, "get")) needs->map_get = 1;
          if (scan_org == ORG_MAP_BIRTH && mname &&
              (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size"))) {
            needs->map_size = 1;
          }
          if (scan_org == ORG_MAP_BIRTH && mname && !strcmp(mname, "has")) needs->map_has = 1;
          if (scan_org == ORG_STRING && mname &&
              (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size"))) {
            needs->string_len = 1;
          }
          if (scan_org == ORG_STRING && mname && !strcmp(mname, "substring")) {
            needs->string_substring = 1;
            if (read_int(ins, "dst")) set_scan_origin(read_int(ins, "dst"), ORG_STRING);
          }
          if (scan_org == ORG_STRING && mname && !strcmp(mname, "indexOf")) needs->string_indexof = 1;
          return;
        }

        if (ctype && !strcmp(ctype, "Global") && mname && !strcmp(mname, "print")) {
          needs->printf = 1;
          return;
        }

        if (ctype && !strcmp(ctype, "Extern") && mname) {
          if (!strcmp(mname, "nyash.string.concat_hh") ||
              !strcmp(mname, "nyash.string.concat3_hhh")) {
            needs->string_concat = 1;
            if (read_int(ins, "dst")) set_scan_origin(read_int(ins, "dst"), ORG_STRING);
          }
        }
      }
