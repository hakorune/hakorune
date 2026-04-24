      enum GenericMethodGetRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE = 0,
        HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_RUNTIME_DATA_LOAD_ANY,
        HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY,
        HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY,
      };

      auto enum GenericMethodGetRouteKind classify_generic_method_get_route(
          const char* bname,
          const struct GenericMethodEmitPlan* plan) {
        if (bname && !strcmp(bname, "MapBox")) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY;
        }
        if ((bname && !strcmp(bname, "ArrayBox")) ||
            (plan && (plan->runtime_array_get || plan->runtime_array_string))) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY;
        }
        if (bname && !strcmp(bname, "RuntimeDataBox")) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_RUNTIME_DATA_LOAD_ANY;
        }
        return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
      }

      auto int classify_generic_method_get_value_demand_stable_object(
          enum GenericMethodGetRouteKind route) {
        return route != HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
      }

      auto int classify_generic_method_get_publish_demand_need_stable_object(
          enum GenericMethodGetRouteKind route) {
        return route != HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
      }

      auto int emit_generic_method_get_fallback_by_policy(
          FILE* f,
          long long bid,
          size_t ii,
          const char* bname,
          long long dst,
          long long recv,
          long long a0,
          const struct GenericMethodEmitPlan* plan,
          char* ab) {
        struct MapLookupFusionRouteMetadata fusion;
        int fusion_result = match_map_lookup_fusion_route_metadata(
            bid,
            ii,
            HAKO_LLVMC_MAP_LOOKUP_FUSION_ROLE_GET,
            dst,
            recv,
            a0,
            &fusion);
        if (fusion.invalid) {
          if (hako_llvmc_route_trace_enabled()) {
            char extra[128];
            snprintf(
                extra,
                sizeof(extra),
                "bid=%lld ii=%zu recv=%lld key=%lld",
                bid,
                ii,
                recv,
                a0);
            hako_llvmc_emit_route_trace(
                "map_lookup_fusion_get",
                "miss",
                "malformed_mir_route_metadata",
                extra);
          }
          return -1;
        }
        if (fusion_result && fusion.has_stored_value_const) {
          if (hako_llvmc_route_trace_enabled()) {
            char extra[224];
            snprintf(
                extra,
                sizeof(extra),
                "bid=%lld ii=%zu recv=%lld key=%lld get=%lld has=%lld stored=%lld",
                bid,
                ii,
                fusion.receiver_reg,
                fusion.key_reg,
                fusion.get_result_reg,
                fusion.has_result_reg,
                fusion.stored_value_const);
            hako_llvmc_emit_route_trace(
                "map_lookup_fusion_get",
                "hit",
                "mir_route_metadata",
                extra);
          }
          if (dst) {
            fprintf(f, "  %%r%lld = add i64 0, %lld\n", dst, fusion.stored_value_const);
            set_type(dst, T_I64);
          }
          return 1;
        }
        enum GenericMethodGetRouteKind route = classify_generic_method_get_route(bname, plan);
        int value_demand_stable_object =
            classify_generic_method_get_value_demand_stable_object(route);
        int publish_demand_need_stable_object =
            classify_generic_method_get_publish_demand_need_stable_object(route);
        (void)value_demand_stable_object;
        (void)publish_demand_need_stable_object;
        switch (route) {
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_RUNTIME_DATA_LOAD_ANY:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.runtime_data.get_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.runtime_data.get_hh\"(%s)\n", ab);
            }
            return 1;
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
              enum HakoLlvmcArraySlotEmitDemand demand =
                  (plan && plan->runtime_array_string)
                      ? HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_REF
                      : HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_HANDLE;
              hako_llvmc_emit_array_slot_load(f, dst, ab, demand);
            } else {
              hako_llvmc_emit_array_slot_load(
                  f, dst, ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_HANDLE);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE:
          default:
            return 0;
        }
      }
