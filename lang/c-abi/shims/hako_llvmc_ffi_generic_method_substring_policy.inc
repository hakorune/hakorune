      enum GenericMethodSubstringRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_INSERT_MID_WINDOW = 0,
        HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_STRING_SUBSTRING_CALL = 1,
      };

      auto enum GenericMethodSubstringRouteKind classify_generic_method_substring_route(
          int matched_insert_window) {
        if (matched_insert_window) {
          return HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_INSERT_MID_WINDOW;
        }
        return HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_STRING_SUBSTRING_CALL;
      }

      auto int emit_generic_method_substring_by_policy(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long recv,
          long long dst,
          long long a0,
          long long a1,
          unsigned char* skip_inst,
          char* ab,
          size_t ab_cap) {
        if (classify_generic_method_substring_route(
                try_emit_string_insert_mid_window_call(
                    f, ii, ilen, insts, recv, dst, a0, a1, skip_inst)) ==
            HAKO_LLVMC_GENERIC_METHOD_SUBSTRING_ROUTE_INSERT_MID_WINDOW) {
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
      }
