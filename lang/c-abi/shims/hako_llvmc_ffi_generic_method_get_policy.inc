      enum GenericMethodGetRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE = 0,
        HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY,
        HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY,
      };

      auto enum GenericMethodGetRouteKind classify_generic_method_get_route(
          const char* bname,
          const struct GenericMethodEmitPlan* plan) {
        if ((bname && !strcmp(bname, "MapBox")) ||
            (plan && plan->runtime_map_get)) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY;
        }
        if ((bname && !strcmp(bname, "ArrayBox")) ||
            (plan && (plan->runtime_array_get || plan->runtime_array_string))) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY;
        }
        return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
      }

      auto int emit_generic_method_get_fallback_by_policy(
          FILE* f,
          const char* bname,
          long long dst,
          const struct GenericMethodEmitPlan* plan,
          char* ab) {
        switch (classify_generic_method_get_route(bname, plan)) {
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.map.slot_load_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
              set_origin(dst, ORG_MAP_GET);
            } else {
              fprintf(f, "  call i64 @\"nyash.map.slot_load_hh\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.array.slot_load_hi\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
              if (plan && plan->runtime_array_string) {
                set_origin(dst, ORG_STRING);
                set_scan_origin(dst, ORG_STRING);
              }
            } else {
              fprintf(f, "  call i64 @\"nyash.array.slot_load_hi\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE:
          default:
            return 0;
        }
      }
