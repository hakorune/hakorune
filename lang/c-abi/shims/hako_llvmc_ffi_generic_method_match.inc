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

      struct GenericMethodEmitPlan {
        enum GenericMethodEmitKind kind;
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
      };

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
        if (mname && !strcmp(mname, "get")) return HAKO_LLVMC_GENERIC_METHOD_EMIT_GET;
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

      auto int classify_generic_method_emit_plan(
          const char* bname,
          const char* mname,
          const struct GenericMethodRouteState* route,
          long long a0,
          long long a1,
          struct GenericMethodEmitPlan* plan) {
        if (!plan) return 0;
        memset(plan, 0, sizeof(*plan));
        if (route) {
          plan->recv_org = route->recv_org;
          plan->runtime_array_len = route->runtime_array_len;
          plan->runtime_array_get = route->runtime_array_get;
          plan->runtime_array_push = route->runtime_array_push;
          plan->runtime_array_has = route->runtime_array_has;
          plan->runtime_string = route->runtime_string;
          plan->runtime_array_string = route->runtime_array_string;
          plan->runtime_map_get = route->runtime_map_get;
          plan->runtime_map_size = route->runtime_map_size;
          plan->runtime_map_has = route->runtime_map_has;
        }
        plan->kind = classify_generic_method_emit_kind(bname, mname, route, a0, a1);
        return plan->kind != HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
      }
