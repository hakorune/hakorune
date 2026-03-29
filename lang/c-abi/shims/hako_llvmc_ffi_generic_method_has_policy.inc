      enum GenericMethodHasRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE = 0,
        HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_RUNTIME_DATA_CONTAINS_ANY,
        HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_ARRAY_CONTAINS_ANY,
        HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_ANY,
      };

      auto enum GenericMethodHasRouteKind classify_generic_method_has_route(
          const char* bname,
          const struct GenericMethodEmitPlan* plan) {
        if (bname && !strcmp(bname, "RuntimeDataBox")) {
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_RUNTIME_DATA_CONTAINS_ANY;
        }
        if (plan && plan->runtime_array_has) {
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_ARRAY_CONTAINS_ANY;
        }
        if ((bname && !strcmp(bname, "MapBox")) ||
            (plan && plan->runtime_map_has)) {
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_ANY;
        }
        return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE;
      }

      auto int emit_generic_method_has_by_policy(
          FILE* f,
          const char* bname,
          long long dst,
          long long a0,
          const struct GenericMethodEmitPlan* plan,
          char* ab,
          size_t ab_cap) {
        if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
        switch (classify_generic_method_has_route(bname, plan)) {
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_RUNTIME_DATA_CONTAINS_ANY:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.runtime_data.has_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.runtime_data.has_hh\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_ARRAY_CONTAINS_ANY:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.array.has_hi\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.array.has_hi\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_ANY:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.map.probe_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.map.probe_hh\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE:
          default:
            return 0;
        }
      }
