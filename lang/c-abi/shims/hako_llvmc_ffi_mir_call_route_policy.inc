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
      classify_mir_call_generic_method_route_kind_from_metadata(
          long long bid,
          size_t ii) {
        yyjson_val* routes = fn_metadata ? yyjson_obj_get(fn_metadata, "generic_method_routes") : NULL;
        size_t ri = 0;
        if (!(routes && yyjson_is_arr(routes))) {
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* core_method = route ? yyjson_obj_get(route, "core_method") : NULL;
          long long block = -1;
          long long inst = -1;
          const char* route_id = route ? read_str(route, "route_id") : NULL;
          const char* route_kind = route ? read_str(route, "route_kind") : NULL;
          const char* core_op = core_method ? read_str(core_method, "op") : NULL;
          int tier = hako_llvmc_generic_method_route_registry_tier_from_label(
              core_method ? read_str(core_method, "lowering_tier") : NULL);
          const struct HakoLlvmcGenericMethodRouteRegistryRow* row = NULL;
          if (!hako_llvmc_json_read_i64_field(route, "block", &block) ||
              !hako_llvmc_json_read_i64_field(route, "instruction_index", &inst) ||
              block != bid ||
              inst != (long long)ii) {
            continue;
          }
          row = hako_llvmc_generic_method_route_registry_find_by_tuple(
              route_id,
              core_op,
              route_kind,
              tier);
          if (row && row->route_result != HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK) {
            if (hako_llvmc_route_trace_enabled()) {
              hako_llvmc_emit_route_trace(
                  "mir_call_route_metadata",
                  "hit",
                  "generic_method_descriptor",
                  route_kind ? route_kind : "-");
            }
            return (enum MirCallGenericMethodRouteKind)row->route_result;
          }
          if (hako_llvmc_route_trace_enabled()) {
            hako_llvmc_emit_route_trace(
                "mir_call_route_metadata",
                "skip",
                "generic_method_descriptor_mismatch",
                route_kind ? route_kind : "-");
          }
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
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
        return classify_mir_call_generic_method_route_kind_from_metadata(bid, ii);
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
        if (!plan_entry) {
          yyjson_val* routes = fn_metadata ? yyjson_obj_get(fn_metadata, "generic_method_routes") : NULL;
          size_t ri = 0;
          if (!(routes && yyjson_is_arr(routes))) return ORG_NONE;
          for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
            yyjson_val* route = yyjson_arr_get(routes, ri);
            long long block = -1;
            long long inst = -1;
            const char* receiver_origin_box = route ? read_str(route, "receiver_origin_box") : NULL;
            if (!hako_llvmc_json_read_i64_field(route, "block", &block) ||
                !hako_llvmc_json_read_i64_field(route, "instruction_index", &inst) ||
                block != bid ||
                inst != (long long)ii) {
              continue;
            }
            if (!receiver_origin_box) return ORG_NONE;
            if (!strcmp(receiver_origin_box, "StringBox")) return ORG_STRING;
            if (!strcmp(receiver_origin_box, "ArrayBox") ||
                !strcmp(receiver_origin_box, "DirectArrayI64")) {
              return ORG_ARRAY_BIRTH;
            }
            if (!strcmp(receiver_origin_box, "MapBox")) return ORG_MAP_BIRTH;
            return ORG_NONE;
          }
          return ORG_NONE;
        }
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
