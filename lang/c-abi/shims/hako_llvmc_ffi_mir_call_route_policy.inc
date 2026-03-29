      enum MirCallReceiverFamilyKind {
        HAKO_LLVMC_MIR_CALL_RECV_FAMILY_NONE = 0,
        HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_BIRTH,
        HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_STRING_BIRTH,
        HAKO_LLVMC_MIR_CALL_RECV_FAMILY_MAP_BIRTH,
        HAKO_LLVMC_MIR_CALL_RECV_FAMILY_STRING,
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
      };

      struct GenericMethodRouteState {
        int recv_org;
        int runtime_array_len;
        int runtime_array_get;
        int runtime_array_push;
        int runtime_array_has;
        int runtime_string;
        int runtime_array_string;
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

      auto enum MirCallGenericMethodRouteKind
      classify_mir_call_generic_method_route_kind(
          const char* bname,
          const char* mname,
          enum MirCallReceiverFamilyKind recv_family) {
        if (!(bname && mname)) {
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }

        if (!strcmp(bname, "MapBox")) {
          if (!strcmp(mname, "get")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_GET;
          if (!strcmp(mname, "size") || !strcmp(mname, "len") || !strcmp(mname, "length")) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_SIZE;
          }
          if (!strcmp(mname, "has")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_HAS;
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }

        if (!strcmp(bname, "ArrayBox")) {
          if (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size")) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_LEN;
          }
          if (!strcmp(mname, "get")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_GET;
          if (!strcmp(mname, "push")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_PUSH;
          if (!strcmp(mname, "has")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_HAS;
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }

        if (!strcmp(bname, "StringBox")) {
          if (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size")) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
          }
          if (!strcmp(mname, "indexOf")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
          if (!strcmp(mname, "substring")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }

        if (!strcmp(bname, "RuntimeDataBox")) {
          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_MAP_BIRTH) {
            if (!strcmp(mname, "get")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_GET;
            if (!strcmp(mname, "size") || !strcmp(mname, "len") || !strcmp(mname, "length")) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_SIZE;
            }
            if (!strcmp(mname, "has")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_HAS;
            return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
          }

          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_BIRTH ||
              recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_STRING_BIRTH) {
            if (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size")) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_LEN;
            }
            if (!strcmp(mname, "get") &&
                recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_STRING_BIRTH) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_STRING;
            }
            if (!strcmp(mname, "get")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_GET;
            if (!strcmp(mname, "push")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_PUSH;
            if (!strcmp(mname, "has")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_HAS;
            return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
          }

          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_STRING) {
            if (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size")) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
            }
            if (!strcmp(mname, "substring")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
            if (!strcmp(mname, "indexOf")) return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
            return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
          }

          if (!strcmp(mname, "indexOf")) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
          }
        }

        return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
      }

      auto struct GenericMethodRouteState classify_generic_method_route(
          long long bid,
          size_t ii,
          const char* bname,
          const char* mname,
          long long recv) {
        struct GenericMethodRouteState route;
        enum MirCallReceiverFamilyKind recv_family;
        enum MirCallGenericMethodRouteKind route_kind;
        (void)bid;
        (void)ii;
        memset(&route, 0, sizeof(route));
        route.recv_org = recv ? get_origin(recv) : ORG_NONE;
        if (route.recv_org == ORG_NONE && recv) {
          route.recv_org = get_scan_origin(recv);
        }
        recv_family = classify_mir_call_receiver_family(route.recv_org);
        route_kind = classify_mir_call_generic_method_route_kind(bname, mname, recv_family);
        switch (route_kind) {
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_GET:
            route.runtime_map_get = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_SIZE:
            route.runtime_map_size = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_HAS:
            route.runtime_map_has = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_LEN:
            route.runtime_array_len = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_GET:
            route.runtime_array_get = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_PUSH:
            route.runtime_array_push = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_HAS:
            route.runtime_array_has = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_STRING:
            route.runtime_array_string = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING:
            route.runtime_string = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK:
          default:
            break;
        }
        if (mname && !strcmp(mname, "indexOf") && route.runtime_string) {
          route.deferred_array_string_indexof =
              find_deferred_array_string_indexof_select(bid, ii);
        }
        return route;
      }
