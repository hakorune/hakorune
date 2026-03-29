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
