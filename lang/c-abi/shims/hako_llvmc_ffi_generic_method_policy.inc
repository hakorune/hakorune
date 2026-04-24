      enum GenericMethodEmitKind {
        HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE = 0,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_BIRTH_ZERO,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_SET,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_GET,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_LEN,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_PUSH,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_SUBSTRING,
        HAKO_LLVMC_GENERIC_METHOD_EMIT_HAS,
      };

      enum GenericMethodSetRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE = 0,
        HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_RUNTIME_DATA_STORE_ANY,
        HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY,
        HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_I64,
        HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING,
        HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_ANY,
      };

      struct GenericMethodEmitPlan {
        enum GenericMethodEmitKind kind;
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
        enum GenericMethodSetRouteKind set_route;
        int runtime_map_get;
        int runtime_map_size;
        int runtime_map_has;
      };

      auto int classify_array_store_string_source_preserve(
          enum GenericMethodSetRouteKind set_route) {
        return set_route == HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING;
      }

      auto int classify_array_store_string_identity_demand_stable_object(
          enum GenericMethodSetRouteKind set_route) {
        (void)set_route;
        return 0;
      }

      auto int classify_array_store_string_publication_demand_publish_handle(
          enum GenericMethodSetRouteKind set_route) {
        return set_route == HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING;
      }

      auto enum GenericMethodEmitKind classify_generic_method_emit_kind(
          const char* bname,
          const char* mname,
          const struct GenericMethodRouteState* route,
          long long a0,
          long long a1) {
        (void)route;
        if (mname && !strcmp(mname, "birth") && !a0 && !a1) {
          if ((bname && !strcmp(bname, "ArrayBox")) || (bname && !strcmp(bname, "MapBox"))) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_BIRTH_ZERO;
          }
        }
        if (mname && !strcmp(mname, "set")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_SET;
        if (mname && (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size"))) {
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_LEN;
        }
        if (mname && !strcmp(mname, "push")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_PUSH;
        if (mname && !strcmp(mname, "substring") && route && route->runtime_string) {
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_SUBSTRING;
        }
        if (mname && !strcmp(mname, "has")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_HAS;
        return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
      }

      auto enum GenericMethodSetRouteKind classify_generic_method_set_route(
          const char* bname,
          const struct GenericMethodEmitPlan* plan,
          int value_is_i64,
          int value_is_string_handle) {
        int arrayish = 0;
        if (plan &&
            (plan->recv_org == ORG_ARRAY_BIRTH ||
             plan->recv_org == ORG_ARRAY_STRING_BIRTH ||
             plan->runtime_array_string ||
             plan->runtime_array_push ||
             plan->runtime_array_get ||
             plan->runtime_array_len ||
             plan->runtime_array_has)) {
          arrayish = 1;
        }
        if (arrayish) {
          if (value_is_i64) return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_I64;
          if (value_is_string_handle) return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING;
          return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_ANY;
        }
        if (bname && !strcmp(bname, "RuntimeDataBox")) {
          return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_RUNTIME_DATA_STORE_ANY;
        }
        return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
      }

      auto int should_promote_array_string_birth_after_set(
          const struct GenericMethodEmitPlan* plan,
          int value_is_string_handle) {
        if (!plan || !value_is_string_handle) return 0;
        return plan->runtime_array_string || plan->runtime_array_push ||
               plan->runtime_array_get || plan->runtime_array_len ||
               plan->runtime_array_has;
      }
