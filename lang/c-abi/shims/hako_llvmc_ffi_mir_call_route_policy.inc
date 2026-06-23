      enum MirCallGenericMethodRouteKind {
        HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK = 0,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_GET,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_SIZE,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_HAS,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_LEN,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_GET,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_PUSH,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_STRING,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ANY_LENGTH,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING_INDEXOF,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING_LASTINDEXOF,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING_CONTAINS,
      };

      struct GenericMethodRouteState {
        int recv_org;
        int runtime_array_len;
        int runtime_array_get;
        int runtime_array_push;
        int runtime_string;
        int runtime_string_indexof;
        int runtime_string_lastindexof;
        int runtime_string_contains;
        int runtime_array_string;
        int runtime_any_length;
        int array_store_string_source_preserve;
        int array_store_string_identity_demand_stable_object;
        int array_store_string_publication_demand_publish_handle;
        int runtime_map_get;
        int runtime_map_size;
        int runtime_map_has;
        struct DeferredArrayStringIndexofSelect* deferred_array_string_indexof;
      };

      auto enum MirCallGenericMethodRouteKind
      classify_mir_call_generic_method_route_kind_from_lowering_plan(
          yyjson_val* entry) {
        struct LoweringPlanGenericMethodView view;
        const struct HakoLlvmcGenericMethodRouteRegistryRow* row = NULL;
        if (!read_lowering_plan_generic_method_view(entry, &view)) {
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }
        row = hako_llvmc_generic_method_route_registry_find_by_tuple(
            view.route_id,
            view.core_op,
            view.route_kind,
            view.tier);
        if (row && row->route_result != HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK) {
          trace_lowering_plan_entry_hit(entry, "mir_call_route");
          return (enum MirCallGenericMethodRouteKind)row->route_result;
        }
        return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
      }

      auto enum MirCallGenericMethodRouteKind
      classify_mir_call_generic_method_route_kind_from_lowering_plan_site(
          long long bid,
          size_t ii) {
        yyjson_val* plan_entry = find_lowering_plan_entry_for_site(bid, ii);
        if (plan_entry) {
          enum MirCallGenericMethodRouteKind plan_route =
              classify_mir_call_generic_method_route_kind_from_lowering_plan(
                  plan_entry);
          if (plan_route != HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK) {
            return plan_route;
          }
        }
        return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
      }

      auto int lowering_plan_generic_method_receiver_origin_kind(
          const struct LoweringPlanGenericMethodView* view) {
        if (!(view && view->receiver_origin_box)) return ORG_NONE;
        if (!strcmp(view->receiver_origin_box, "StringBox")) return ORG_STRING;
        if (!strcmp(view->receiver_origin_box, "ArrayBox") ||
            !strcmp(view->receiver_origin_box, "DirectArrayI64")) {
          return ORG_ARRAY_BIRTH;
        }
        if (!strcmp(view->receiver_origin_box, "MapBox")) return ORG_MAP_BIRTH;
        return ORG_NONE;
      }

      auto int classify_mir_call_generic_method_receiver_origin_from_lowering_plan_site(
          long long bid,
          size_t ii) {
        yyjson_val* plan_entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanGenericMethodView view;
        if (!plan_entry) return ORG_NONE;
        if (!read_lowering_plan_generic_method_view(plan_entry, &view)) {
          return ORG_NONE;
        }
        return lowering_plan_generic_method_receiver_origin_kind(&view);
      }

      auto void apply_mir_call_generic_method_route_kind(
          enum MirCallGenericMethodRouteKind route_kind,
          struct GenericMethodRouteState* route) {
        if (!route) return;
        switch (route_kind) {
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_GET:
            route->runtime_map_get = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_SIZE:
            route->runtime_map_size = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_HAS:
            route->runtime_map_has = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_LEN:
            route->runtime_array_len = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_GET:
            route->runtime_array_get = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_PUSH:
            route->runtime_array_push = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_STRING:
            route->runtime_array_string = 1;
            route->array_store_string_source_preserve = 1;
            route->array_store_string_identity_demand_stable_object = 0;
            route->array_store_string_publication_demand_publish_handle = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING:
            route->runtime_string = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ANY_LENGTH:
            route->runtime_any_length = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING_INDEXOF:
            route->runtime_string = 1;
            route->runtime_string_indexof = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING_LASTINDEXOF:
            route->runtime_string = 1;
            route->runtime_string_lastindexof = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING_CONTAINS:
            route->runtime_string = 1;
            route->runtime_string_contains = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK:
          default:
            break;
        }
      }

      auto struct GenericMethodRouteState classify_generic_method_route(
          long long bid,
          size_t ii,
          long long recv) {
        struct GenericMethodRouteState route;
        enum MirCallGenericMethodRouteKind route_kind;
        (void)recv;
        memset(&route, 0, sizeof(route));
        route.recv_org =
            classify_mir_call_generic_method_receiver_origin_from_lowering_plan_site(
                bid,
                ii);
        route_kind =
            classify_mir_call_generic_method_route_kind_from_lowering_plan_site(
                bid,
                ii);
        apply_mir_call_generic_method_route_kind(route_kind, &route);
        route.deferred_array_string_indexof =
            find_deferred_array_string_indexof_select(bid, ii);
        if (route.deferred_array_string_indexof) {
          route.runtime_string = 1;
          route.runtime_string_indexof = 1;
        }
        return route;
      }
