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
        HAKO_LLVMC_MIR_CALL_RECV_SURFACE_STRING_BOX,
        HAKO_LLVMC_MIR_CALL_RECV_SURFACE_RUNTIME_DATA_BOX,
      };

      enum MirCallMethodSurfaceKind {
        HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_UNKNOWN = 0,
        HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_GET,
        HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SET,
        HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SIZE,
        HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_HAS,
        HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_PUSH,
        HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_INDEXOF,
        HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SUBSTRING,
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
        if (!strcmp(bname, "StringBox")) return HAKO_LLVMC_MIR_CALL_RECV_SURFACE_STRING_BOX;
        if (!strcmp(bname, "RuntimeDataBox")) {
          return HAKO_LLVMC_MIR_CALL_RECV_SURFACE_RUNTIME_DATA_BOX;
        }
        return HAKO_LLVMC_MIR_CALL_RECV_SURFACE_UNKNOWN;
      }

      auto enum MirCallMethodSurfaceKind classify_mir_call_method_surface(
          const char* mname) {
        if (!mname) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_UNKNOWN;
        if (!strcmp(mname, "get")) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_GET;
        if (!strcmp(mname, "set")) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SET;
        if (!strcmp(mname, "size") || !strcmp(mname, "len") || !strcmp(mname, "length")) {
          return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SIZE;
        }
        if (!strcmp(mname, "has")) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_HAS;
        if (!strcmp(mname, "push")) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_PUSH;
        if (!strcmp(mname, "indexOf")) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_INDEXOF;
        if (!strcmp(mname, "substring")) return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SUBSTRING;
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

        if (recv_surface == HAKO_LLVMC_MIR_CALL_RECV_SURFACE_MAP_BOX) {
          if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_GET) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_GET;
          }
          if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SIZE) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_SIZE;
          }
          if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_HAS) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_HAS;
          }
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }

        if (recv_surface == HAKO_LLVMC_MIR_CALL_RECV_SURFACE_ARRAY_BOX) {
          if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SIZE) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_LEN;
          }
          if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_GET) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_GET;
          }
          if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_PUSH) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_PUSH;
          }
          if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_HAS) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_HAS;
          }
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }

        if (recv_surface == HAKO_LLVMC_MIR_CALL_RECV_SURFACE_STRING_BOX) {
          if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SIZE) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
          }
          if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_INDEXOF) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
          }
          if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SUBSTRING) {
            return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
          }
          return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
        }

        if (recv_surface == HAKO_LLVMC_MIR_CALL_RECV_SURFACE_RUNTIME_DATA_BOX) {
          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_MAP_BIRTH) {
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_GET) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_GET;
            }
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SIZE) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_SIZE;
            }
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_HAS) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_MAP_HAS;
            }
            return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
          }

          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_BIRTH ||
              recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_STRING_BIRTH) {
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SIZE) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_LEN;
            }
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_GET &&
                recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_ARRAY_STRING_BIRTH) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_STRING;
            }
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_GET) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_GET;
            }
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_PUSH) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_PUSH;
            }
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_HAS) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_HAS;
            }
            return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
          }

          if (recv_family == HAKO_LLVMC_MIR_CALL_RECV_FAMILY_STRING) {
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SIZE) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
            }
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SUBSTRING) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
            }
            if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_INDEXOF) {
              return HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING;
            }
            return HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK;
          }

          if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_INDEXOF) {
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
        recv_family = classify_mir_call_receiver_family(route.recv_org);
        recv_surface = classify_mir_call_receiver_surface(bname);
        method_surface = classify_mir_call_method_surface(mname);
        route_kind = classify_mir_call_generic_method_route_kind(
            recv_surface, method_surface, recv_family);
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
            route.array_store_string_source_preserve = 1;
            route.array_store_string_identity_demand_stable_object = 0;
            route.array_store_string_publication_demand_publish_handle = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_STRING:
            route.runtime_string = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_ROUTE_FALLBACK:
          default:
            break;
        }
        if (method_surface == HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_INDEXOF &&
            route.runtime_string) {
          route.deferred_array_string_indexof =
              find_deferred_array_string_indexof_select(bid, ii);
        }
        return route;
      }
