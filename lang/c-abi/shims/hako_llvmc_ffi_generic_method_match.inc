      #include "hako_llvmc_ffi_generic_method_policy.inc"

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
          plan->array_store_string_source_preserve =
              route->array_store_string_source_preserve;
          plan->array_store_string_identity_demand_stable_object =
              route->array_store_string_identity_demand_stable_object;
          plan->array_store_string_publication_demand_publish_handle =
              route->array_store_string_publication_demand_publish_handle;
          plan->runtime_map_get = route->runtime_map_get;
          plan->runtime_map_size = route->runtime_map_size;
          plan->runtime_map_has = route->runtime_map_has;
        }
        plan->kind = classify_generic_method_emit_kind(bname, mname, route, a0, a1);
        return plan->kind != HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
      }
