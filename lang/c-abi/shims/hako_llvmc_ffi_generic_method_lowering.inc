#include "hako_llvmc_ffi_generic_method_match.inc"
#include "hako_llvmc_ffi_generic_method_len_policy.inc"

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
          enum GenericMethodEmitKind emit_kind,
          const struct GenericMethodRouteState* route,
          char* ab,
          size_t ab_cap,
          unsigned char* skip_inst) {
        struct GenericMethodEmitPlan plan;
        if (!classify_generic_method_emit_plan(bname, mname, route, a0, a1, &plan)) return 0;
        int recv_org = plan.recv_org;
        int runtime_array_len = plan.runtime_array_len;
        int runtime_array_get = plan.runtime_array_get;
        int runtime_array_push = plan.runtime_array_push;
        int runtime_array_has = plan.runtime_array_has;
        int runtime_array_string = plan.runtime_array_string;
        int runtime_map_get = plan.runtime_map_get;
        int runtime_map_has = plan.runtime_map_has;

        switch (emit_kind) {
          case HAKO_LLVMC_GENERIC_METHOD_EMIT_BIRTH_ZERO:
            if (dst) {
              fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
              set_type(dst, T_I64);
            }
            return 1;

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_SET: {
            int array_set_plain_i64 = a1 && is_plain_i64_value(a1);
            int array_set_string_handle = a1 && get_origin(a1) == ORG_STRING;
            enum GenericMethodSetRouteKind set_route =
                classify_generic_method_set_route(
                    bname, &plan, array_set_plain_i64, array_set_string_handle);
            if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
            if (a1) append_i64_arg_ref(ab, ab_cap, a1, 0);
            switch (set_route) {
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY:
                fprintf(f, "  call i64 @\"nyash.map.slot_store_hhh\"(%s)\n", ab);
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_I64:
                fprintf(f, "  call i64 @\"nyash.array.slot_store_hii\"(%s)\n", ab);
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING:
                fprintf(f, "  call i64 @\"nyash.array.set_his\"(%s)\n", ab);
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_ANY:
                fprintf(f, "  call i64 @\"nyash.array.set_hih\"(%s)\n", ab);
                break;
              case HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE:
              default:
                return -1;
            }
            if (should_promote_array_string_birth_after_set(&plan, array_set_string_handle)) {
              promote_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
              promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
              if (recv) {
                set_origin(recv, ORG_ARRAY_STRING_BIRTH);
                set_scan_origin(recv, ORG_ARRAY_STRING_BIRTH);
              }
            }
            return 1;
          }

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_GET: {
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
              if (string_len_match.keep_get_live) {
                if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
                if (dst) {
                  fprintf(f, "  %%r%lld = call i64 @\"nyash.array.slot_load_hi\"(%s)\n", dst, ab);
                  set_type(dst, T_I64);
                  set_origin(dst, ORG_STRING);
                  set_scan_origin(dst, ORG_STRING);
                } else {
                  fprintf(f, "  call i64 @\"nyash.array.slot_load_hi\"(%s)\n", ab);
                }
                fprintf(f, "  %%r%lld = call i64 @\"nyash.array.string_len_hi\"(%s)\n", string_len_match.len_dst, ab);
                set_type(string_len_match.len_dst, T_I64);
                if (dst) remember_string_length_call(string_len_match.len_dst, dst);
                skip_inst[string_len_match.len_ii] = 1;
                return 1;
              }
              if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
              fprintf(f, "  %%r%lld = call i64 @\"nyash.array.string_len_hi\"(%s)\n", string_len_match.len_dst, ab);
              set_type(string_len_match.len_dst, T_I64);
              for (int si = 0; si < string_len_match.copy_n; si++) skip_inst[string_len_match.copy_iis[si]] = 1;
              skip_inst[string_len_match.len_ii] = 1;
              return 1;
            }
            if (try_defer_array_string_indexof_get_window(
                    f, blocks, bid, ii, ilen, insts, bname, mname, recv, a0, dst, recv_org,
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

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_LEN:
            if (!emit_generic_method_len_by_policy(f, bname, dst, recv, &plan, ab)) {
              return -1;
            }
            return 1;

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_PUSH:
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

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_SUBSTRING:
            if (try_emit_string_insert_mid_window_call(
                    f, ii, ilen, insts, recv, dst, a0, a1, skip_inst)) {
              return 1;
            }
            if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
            if (a1) append_i64_arg_ref(ab, ab_cap, a1, 0);
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.string.substring_hii\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
              set_origin(dst, ORG_STRING);
              set_scan_origin(dst, ORG_STRING);
              remember_string_substring_call(dst, recv, a0, a1);
            } else {
              fprintf(f, "  call i64 @\"nyash.string.substring_hii\"(%s)\n", ab);
            }
            return 1;

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_HAS:
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

          case HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE:
          default:
            return 0;
        }
      }
