      enum GenericMethodLenRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_NONE = 0,
        HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_MAP_ENTRY_COUNT,
        HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_ARRAY_SLOT_LEN,
        HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_STRING_LEN,
      };

      auto enum GenericMethodLenRouteKind classify_generic_method_len_route(
          const char* bname,
          const struct GenericMethodEmitPlan* plan) {
        if (!plan) return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_NONE;
        if ((bname && !strcmp(bname, "MapBox")) || plan->runtime_map_size) {
          return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_MAP_ENTRY_COUNT;
        }
        if ((bname && !strcmp(bname, "ArrayBox")) || plan->runtime_array_len ||
            plan->runtime_array_string) {
          return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_ARRAY_SLOT_LEN;
        }
        if (plan->runtime_string) {
          return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_STRING_LEN;
        }
        return HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_NONE;
      }

      auto int emit_generic_method_len_by_policy(
          FILE* f,
          const char* bname,
          long long dst,
          long long recv,
          const struct GenericMethodEmitPlan* plan,
          const char* ab) {
        long long concat_known_len = 0;
        long long substring_src = 0;
        long long substring_start = 0;
        long long substring_end = 0;
        long long resolved_recv = resolve_copy_source_reg(recv);
        char substring_ab[128];
        switch (classify_generic_method_len_route(bname, plan)) {
          case HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_MAP_ENTRY_COUNT:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.map.entry_count_h\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.map.entry_count_h\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_ARRAY_SLOT_LEN:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.array.slot_len_h\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.array.slot_len_h\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_STRING_LEN:
            if (known_string_concat_chain_len(resolved_recv, &concat_known_len)) {
              if (dst) {
                fprintf(f, "  %%r%lld = add i64 0, %lld\n", dst, concat_known_len);
                set_type(dst, T_I64);
                put_const(dst, concat_known_len);
              }
              if (hako_llvmc_route_trace_enabled()) {
                char extra[128];
                snprintf(
                    extra,
                    sizeof(extra),
                    "recv=%lld len=%lld",
                    resolved_recv,
                    concat_known_len);
                hako_llvmc_emit_route_trace(
                    "string_len_corridor",
                    "hit",
                    "concat_chain_known_len",
                    extra);
              }
              return 1;
            }
            if (hako_llvmc_string_corridor_has_candidate(
                    resolved_recv, "direct_kernel_entry") &&
                find_string_substring_call(
                    resolved_recv,
                    &substring_src,
                    &substring_start,
                    &substring_end)) {
              substring_ab[0] = '\0';
              append_i64_arg_ref(substring_ab, sizeof(substring_ab), substring_src, 1);
              append_i64_arg_ref(substring_ab, sizeof(substring_ab), substring_start, 0);
              append_i64_arg_ref(substring_ab, sizeof(substring_ab), substring_end, 0);
              if (dst) {
                fprintf(
                    f,
                    "  %%r%lld = call i64 @\"nyash.string.substring_len_hii\"(%s)\n",
                    dst,
                    substring_ab);
                set_type(dst, T_I64);
              } else {
                fprintf(f, "  call i64 @\"nyash.string.substring_len_hii\"(%s)\n", substring_ab);
              }
              if (hako_llvmc_route_trace_enabled()) {
                char extra[192];
                snprintf(
                    extra,
                    sizeof(extra),
                    "recv=%lld src=%lld start=%lld end=%lld",
                    resolved_recv,
                    substring_src,
                    substring_start,
                    substring_end);
                hako_llvmc_emit_route_trace(
                    "string_len_corridor",
                    "hit",
                    "substring_len_direct_kernel_entry",
                    extra);
              }
              return 1;
            }
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.string.len_h\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
              remember_string_length_call(dst, recv);
            } else {
              fprintf(f, "  call i64 @\"nyash.string.len_h\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_LEN_ROUTE_NONE:
          default:
            return 0;
        }
      }
