      enum MirCallReceiverFamilyKind {
        HAKO_LLVMC_MIR_CALL_RECV_FAMILY_NONE = 0,
        HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_BIRTH,
        HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_STRING_BIRTH,
        HAKO_LLVMC_MIR_CALL_RECV_FAMILY_MAP_BIRTH,
        HAKO_LLVMC_MIR_CALL_RECV_FAMILY_STRING,
      };

      enum MirCallReceiverSurfaceKind {
        HAKO_LLVMC_MIR_CALL_RECV_SURFACE_UNKNOWN = 0,
        HAKO_LLVMC_MIR_CALL_RECV_SURFACE_MAP_BOX,
        HAKO_LLVMC_MIR_CALL_RECV_SURFACE_ARRAY_BOX,
        HAKO_LLVMC_MIR_CALL_RECV_SURFACE_RUNTIME_DATA_BOX,
      };

      enum MirCallMethodSurfaceKind {
        HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_UNKNOWN = 0,
        HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_HAS,
      };

      enum MirCallGenericMethodRouteKind {
        HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK = 0,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_GET,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_SIZE,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_HAS,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_LEN,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_GET,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_PUSH,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_HAS,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_STRING,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING,
        HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING_INDEXOF,
      };

      struct GenericMethodRouteState {
        int recv_org;
        int runtime_array_len;
        int runtime_array_get;
        int runtime_array_push;
        int runtime_array_has;
        int runtime_string;
        int runtime_string_indexof;
        int runtime_array_string;
        int array_store_string_source_preserve;
        int array_store_string_identity_demand_stable_object;
        int array_store_string_publication_demand_publish_handle;
        int runtime_map_get;
        int runtime_map_size;
        int runtime_map_has;
        struct DeferredArrayStringIndexofSelect* deferred_array_string_indexof;
      };

      auto enum MirCallReceiverFamilyKind classify_mir_call_receiver_family(
          int recv_org) {
        switch (recv_org) {
          case ORG_ARRAY_BIRTH:
            return HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_BIRTH;
          case ORG_ARRAY_STRING_BIRTH:
            return HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_STRING_BIRTH;
          case ORG_MAP_BIRTH:
            return HAKO_LLVMC_MIR_CALL_RECV_FAMILY_MAP_BIRTH;
          case ORG_STRING:
            return HAKO_LLVMC_MIR_CALL_RECV_FAMILY_STRING;
          case ORG_NONE:
          default:
            return HAKO_LLVMC_MIR_CALL_RECV_FAMILY_NONE;
        }
      }

      auto enum MirCallReceiverSurfaceKind classify_mir_call_receiver_surface(
          const char* bname) {
        if (!bname) return HAKO_LLVMC_MIR_CALL_RECV_SURFACE_UNKNOWN;
        if (!strcmp(bname, "MapBox")) return HAKO_LLVMC_MIR_CALL_RECV_SURFACE_MAP_BOX;
        if (!strcmp(bname, "ArrayBox")) return HAKO_LLVMC_MIR_CALL_RECV_SURFACE_ARRAY_BOX;
        if (!strcmp(bname, "RuntimeDataBox")) {
          return HAKO_LLVMC_MIR_CALL_RECV_SURFACE_RUNTIME_DATA_BOX;
        }
        return HAKO_LLVMC_MIR_CALL_RECV_SURFACE_UNKNOWN;
      }

      auto enum MirCallMethodSurfaceKind classify_mir_call_method_surface(
          const char* mname) {
        if (!mname) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_UNKNOWN;
        if (!strcmp(mname, "has")) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_HAS;
        return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_UNKNOWN;
      }

      auto enum MirCallGenericMethodRouteKind
      classify_mir_call_generic_method_route_kind(
          enum MirCallReceiverSurfaceKind recv_surface,
          enum MirCallMethodSurfaceKind method_surface,
          enum MirCallReceiverFamilyKind recv_family) {
        if (recv_surface == HAKO_LLVMC_MIR_CALL_RECV_SURFACE_UNKNOWN ||
            method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_UNKNOWN) {
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }

        if (recv_surface == HAKO_LLVMC_MIR_CALL_RECV_SURFACE_MAP_BOX &&
            method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_HAS) {
          return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_HAS;
        }

        if (recv_surface == HAKO_LLVMC_MIR_CALL_RECV_SURFACE_ARRAY_BOX &&
            method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_HAS) {
          return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_HAS;
        }

        if (recv_surface == HAKO_LLVMC_MIR_CALL_RECV_SURFACE_RUNTIME_DATA_BOX &&
            method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_HAS) {
          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_MAP_BIRTH) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_HAS;
          }
          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_BIRTH ||
              recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_STRING_BIRTH) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_HAS;
          }
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }

        return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
      }

      auto int mir_call_route_policy_read_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto enum MirCallGenericMethodRouteKind
      classify_mir_call_generic_method_route_kind_from_core_method_metadata(
          long long bid,
          size_t ii) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (!fn_metadata) return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        routes = yyjson_obj_get(fn_metadata, "generic_method_routes");
        if (!(routes && yyjson_is_arr(routes))) {
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* core_method = NULL;
          const char* route_id = route ? read_str(route, "route_id") : NULL;
          const char* route_kind = route ? read_str(route, "route_kind") : NULL;
          const char* op = NULL;
          long long block = 0;
          long long inst = 0;
          int invalid = 0;
          enum CoreMethodMetadataProof proof =
              HAKO_LLVMC_CORE_METHOD_METADATA_PROOF_NONE;
          enum CoreMethodLoweringTierMetadata lowering_tier =
              HAKO_LLVMC_CORE_METHOD_LOWERING_TIER_NONE;
          if (!mir_call_route_policy_read_i64_field(route, "block", &block) ||
              !mir_call_route_policy_read_i64_field(
                  route,
                  "instruction_index",
                  &inst) ||
              block != bid ||
              (size_t)inst != ii) {
            continue;
          }
          core_method = yyjson_obj_get(route, "core_method");
          if (!(core_method && yyjson_is_obj(core_method))) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
          }
          op = read_str(core_method, "op");
          proof = read_core_method_proof_metadata(core_method, &invalid);
          lowering_tier = read_core_method_lowering_tier_metadata(core_method, &invalid);
          if (invalid ||
              !core_method_metadata_proof_is_contract_manifest(proof) ||
              !(op && route_id && route_kind)) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
          }
          if (!strcmp(route_id, "generic_method.get") &&
              !strcmp(op, "MapGet") &&
              !strcmp(route_kind, "runtime_data_load_any") &&
              core_method_lowering_tier_is_cold_fallback(lowering_tier)) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_GET;
          }
          if (!strcmp(route_id, "generic_method.get") &&
              !strcmp(op, "MapGet") &&
              !strcmp(route_kind, "map_load_any") &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_GET;
          }
          if (!strcmp(route_id, "generic_method.get") &&
              !strcmp(op, "ArrayGet") &&
              !strcmp(route_kind, "array_slot_load_any") &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_GET;
          }
          if (!strcmp(route_id, "generic_method.has") &&
              !strcmp(op, "MapHas") &&
              (!strcmp(route_kind, "map_contains_any") ||
               !strcmp(route_kind, "map_contains_i64")) &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_HAS;
          }
          if (!strcmp(route_id, "generic_method.len") &&
              !strcmp(op, "MapLen") &&
              !strcmp(route_kind, "map_entry_count") &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_SIZE;
          }
          if (!strcmp(route_id, "generic_method.len") &&
              !strcmp(op, "ArrayLen") &&
              !strcmp(route_kind, "array_slot_len") &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_LEN;
          }
          if (!strcmp(route_id, "generic_method.push") &&
              !strcmp(op, "ArrayPush") &&
              !strcmp(route_kind, "array_append_any") &&
              core_method_lowering_tier_is_cold_fallback(lowering_tier)) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_PUSH;
          }
          if (!strcmp(route_id, "generic_method.len") &&
              !strcmp(op, "StringLen") &&
              !strcmp(route_kind, "string_len") &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
          }
          if (!strcmp(route_id, "generic_method.substring") &&
              !strcmp(op, "StringSubstring") &&
              !strcmp(route_kind, "string_substring") &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
          }
          if (!strcmp(route_id, "generic_method.indexOf") &&
              !strcmp(op, "StringIndexOf") &&
              !strcmp(route_kind, "string_indexof") &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING_INDEXOF;
          }
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }
        return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
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
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_HAS:
            route->runtime_array_has = 1;
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
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING_INDEXOF:
            route->runtime_string = 1;
            route->runtime_string_indexof = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK:
          default:
            break;
        }
      }

      auto struct GenericMethodRouteState classify_generic_method_route(
          long long bid,
          size_t ii,
          const char* bname,
          const char* mname,
          long long recv) {
        struct GenericMethodRouteState route;
        enum MirCallReceiverFamilyKind recv_family;
        enum MirCallReceiverSurfaceKind recv_surface;
        enum MirCallMethodSurfaceKind method_surface;
        enum MirCallGenericMethodRouteKind route_kind;
        (void)bid;
        (void)ii;
        memset(&route, 0, sizeof(route));
        route.recv_org = recv ? get_origin(recv) : ORG_NONE;
        if (route.recv_org == ORG_NONE && recv) {
          route.recv_org = get_scan_origin(recv);
        }
        route_kind =
            classify_mir_call_generic_method_route_kind_from_core_method_metadata(
                bid,
                ii);
        if (route_kind == HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK) {
          recv_family = classify_mir_call_receiver_family(route.recv_org);
          recv_surface = classify_mir_call_receiver_surface(bname);
          method_surface = classify_mir_call_method_surface(mname);
          route_kind = classify_mir_call_generic_method_route_kind(
              recv_surface, method_surface, recv_family);
        } else {
          method_surface = classify_mir_call_method_surface(mname);
        }
        apply_mir_call_generic_method_route_kind(route_kind, &route);
        route.deferred_array_string_indexof =
            find_deferred_array_string_indexof_select(bid, ii);
        if (route.deferred_array_string_indexof) {
          route.runtime_string = 1;
          route.runtime_string_indexof = 1;
        }
        return route;
      }
