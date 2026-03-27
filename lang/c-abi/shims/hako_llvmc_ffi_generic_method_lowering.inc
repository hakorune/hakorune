      auto int try_emit_non_indexof_generic_method_call(
          FILE* f,
          yyjson_val* blocks,
          long long bid,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long dst,
          long long recv,
          long long a0,
          long long a1,
          int recv_org,
          int runtime_array_len,
          int runtime_array_get,
          int runtime_array_push,
          int runtime_array_has,
          int runtime_string,
          int runtime_array_string,
          int runtime_map_get,
          int runtime_map_size,
          int runtime_map_has,
          char* ab,
          size_t ab_cap,
          unsigned char* skip_inst) {
        if (mname && !strcmp(mname, "set")) {
          int array_set_plain_i64 = a1 && is_plain_i64_value(a1);
          if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
          if (a1) append_i64_arg_ref(ab, ab_cap, a1, 0);
          if (bname && !strcmp(bname, "MapBox")) {
            fprintf(f, "  call i64 @\"nyash.map.slot_store_hhh\"(%s)\n", ab);
          } else if (bname && !strcmp(bname, "ArrayBox")) {
            if (array_set_plain_i64) fprintf(f, "  call i64 @\"nyash.array.slot_store_hii\"(%s)\n", ab);
            else fprintf(f, "  call i64 @\"nyash.array.set_hih\"(%s)\n", ab);
          } else if (runtime_array_string || runtime_array_push || runtime_array_get || runtime_array_len || runtime_array_has) {
            if (runtime_array_string) fprintf(f, "  call i64 @\"nyash.array.set_hih\"(%s)\n", ab);
            else if (array_set_plain_i64) fprintf(f, "  call i64 @\"nyash.array.slot_store_hii\"(%s)\n", ab);
            else fprintf(f, "  call i64 @\"nyash.array.set_hih\"(%s)\n", ab);
            if (a1 && get_origin(a1) == ORG_STRING) {
              promote_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
              promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
            }
            if (recv) {
              set_origin(recv, ORG_ARRAY_STRING_BIRTH);
              set_scan_origin(recv, ORG_ARRAY_STRING_BIRTH);
            }
          } else {
            return -1;
          }
          return 1;
        }
        if (mname && !strcmp(mname, "get")) {
          struct ArrayRmwWindowMatch rmw_match;
          const char* rmw_reason = NULL;
          char rmw_extra[384];
          struct ArrayStringLenWindowMatch string_len_match;
          const char* string_len_reason = NULL;
          char string_len_extra[256];
          trace_array_rmw_window_candidate(ii, ilen, insts, bname, mname, recv, a0, dst, recv_org, runtime_array_get, runtime_array_string);
          trace_array_string_len_window_candidate(ii, ilen, insts, bname, mname, recv, a0, dst, recv_org, runtime_array_string);
          if (analyze_array_rmw_window_candidate(
                  ii, ilen, insts, bname, mname, recv, a0, dst, recv_org,
                  runtime_array_get, runtime_array_string, &rmw_match, &rmw_reason,
                  rmw_extra, sizeof(rmw_extra))) {
            if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
            fprintf(f, "  %%r%lld = call i64 @\"nyash.array.rmw_add1_hi\"(%s)\n", rmw_match.result_reg, ab);
            set_type(rmw_match.result_reg, T_I64);
            for (int si = 0; si < rmw_match.copy_n; si++) skip_inst[rmw_match.copy_iis[si]] = 1;
            skip_inst[rmw_match.const_ii] = 1;
            skip_inst[rmw_match.binop_ii] = 1;
            skip_inst[rmw_match.set_ii] = 1;
            return 1;
          }
          if (analyze_array_string_len_window_candidate(
                  ii, ilen, insts, bname, mname, recv, a0, dst, recv_org,
                  runtime_array_string, &string_len_match, &string_len_reason,
                  string_len_extra, sizeof(string_len_extra))) {
            if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
            fprintf(f, "  %%r%lld = call i64 @\"nyash.array.string_len_hi\"(%s)\n", string_len_match.len_dst, ab);
            set_type(string_len_match.len_dst, T_I64);
            for (int si = 0; si < string_len_match.copy_n; si++) skip_inst[string_len_match.copy_iis[si]] = 1;
            skip_inst[string_len_match.len_ii] = 1;
            return 1;
          }
          if (try_defer_array_string_indexof_get_window(
                  blocks, bid, ii, ilen, insts, bname, mname, recv, a0, dst, recv_org,
                  runtime_array_string, skip_inst)) {
            return 1;
          }
          if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
          if ((bname && !strcmp(bname, "MapBox")) || runtime_map_get) {
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.map.slot_load_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
              set_origin(dst, ORG_MAP_GET);
            } else {
              fprintf(f, "  call i64 @\"nyash.map.slot_load_hh\"(%s)\n", ab);
            }
          } else if ((bname && !strcmp(bname, "ArrayBox")) || runtime_array_get || runtime_array_string) {
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.array.slot_load_hi\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
              if (runtime_array_string) {
                set_origin(dst, ORG_STRING);
                set_scan_origin(dst, ORG_STRING);
              }
            } else {
              fprintf(f, "  call i64 @\"nyash.array.slot_load_hi\"(%s)\n", ab);
            }
          } else {
            return -1;
          }
          return 1;
        }
        if (mname && (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size"))) {
          if ((bname && !strcmp(bname, "MapBox")) || runtime_map_size) {
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.map.entry_count_h\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.map.entry_count_h\"(%s)\n", ab);
            }
          } else if ((bname && !strcmp(bname, "ArrayBox")) || runtime_array_len || runtime_array_string) {
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.array.slot_len_h\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.array.slot_len_h\"(%s)\n", ab);
            }
          } else if (runtime_string) {
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.string.len_h\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.string.len_h\"(%s)\n", ab);
            }
          } else {
            return -1;
          }
          return 1;
        }
        if (mname && !strcmp(mname, "push")) {
          if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
          if (runtime_array_push) {
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.array.slot_append_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.array.slot_append_hh\"(%s)\n", ab);
            }
            if (a0 && get_origin(a0) == ORG_STRING) {
              promote_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
              promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
              if (recv) {
                set_origin(recv, ORG_ARRAY_STRING_BIRTH);
                set_scan_origin(recv, ORG_ARRAY_STRING_BIRTH);
              }
            }
          } else if (runtime_array_string) {
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.array.slot_append_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.array.slot_append_hh\"(%s)\n", ab);
            }
            if (recv) {
              set_origin(recv, ORG_ARRAY_STRING_BIRTH);
              set_scan_origin(recv, ORG_ARRAY_STRING_BIRTH);
            }
          } else if (bname && !strcmp(bname, "ArrayBox")) {
            fprintf(f, "  call i64 @\"nyash.array.slot_append_hh\"(%s)\n", ab);
          } else {
            return -1;
          }
          return 1;
        }
        if (mname && !strcmp(mname, "substring") && runtime_string) {
          if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
          if (a1) append_i64_arg_ref(ab, ab_cap, a1, 0);
          if (dst) {
            fprintf(f, "  %%r%lld = call i64 @\"nyash.string.substring_hii\"(%s)\n", dst, ab);
            set_type(dst, T_I64);
            set_origin(dst, ORG_STRING);
            set_scan_origin(dst, ORG_STRING);
          } else {
            fprintf(f, "  call i64 @\"nyash.string.substring_hii\"(%s)\n", ab);
          }
          return 1;
        }
        if (mname && !strcmp(mname, "has")) {
          if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
          if (runtime_array_has) {
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.array.has_hi\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.array.has_hi\"(%s)\n", ab);
            }
          } else if ((bname && !strcmp(bname, "MapBox")) || runtime_map_has) {
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.map.probe_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.map.probe_hh\"(%s)\n", ab);
            }
          } else {
            return -1;
          }
          return 1;
        }
        return 0;
      }
