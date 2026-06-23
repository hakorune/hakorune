      #include "hako_llvmc_ffi_generic_method_policy.inc"
      auto enum GenericMethodEmitKind
      classify_generic_method_emit_kind_from_lowering_plan(yyjson_val* entry) {
        struct LoweringPlanGenericMethodView view;
        const struct HakoLlvmcGenericMethodRouteRegistryRow* row = NULL;
        long long block = -1;
        long long inst = -1;
        if (!read_lowering_plan_generic_method_view(entry, &view)) {
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
        }
        row = hako_llvmc_generic_method_route_registry_find_by_tuple(
            view.route_id, view.core_op, view.route_kind, view.tier);
        if (row && row->emit_kind != 0) {
          trace_lowering_plan_entry_hit(entry, "generic_method_emit");
          return (enum GenericMethodEmitKind)row->emit_kind;
        }
        lowering_plan_read_i64_field(entry, "block", &block);
        lowering_plan_read_i64_field(entry, "instruction_index", &inst);
        note_pure_unsupported_shape_route_detail(
            block,
            inst >= 0 ? (size_t)inst : 0,
            "mir_call",
            "generic_method_route_contract_mismatch",
            view.return_shape,
            "descriptor_mismatch",
            view.core_op,
            hako_llvmc_generic_method_route_registry_first_tuple_mismatch(
                view.route_id,
                view.core_op,
                view.route_kind,
                view.tier,
                view.symbol,
                view.route_proof),
            view.symbol,
            view.receiver_origin_box,
            view.route_id,
            view.route_kind,
            "check spec/mir/generic_method_routes.toml");
        return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
      }

      auto enum GenericMethodEmitKind
      classify_generic_method_emit_kind_from_site_lowering_plan(
          long long bid,
          size_t ii) {
        yyjson_val* plan_entry = find_lowering_plan_entry_for_site(bid, ii);
        if (plan_entry) {
          enum GenericMethodEmitKind plan_kind =
              classify_generic_method_emit_kind_from_lowering_plan(plan_entry);
          if (plan_kind != HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE) {
            return plan_kind;
          }
        }
        return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
      }

      auto enum GenericMethodSetRouteKind
      generic_method_set_route_from_metadata_value_shape(
          const char* op,
          const char* route_kind,
          int value_is_i64,
          int value_is_string_handle) {
        if (op && !strcmp(op, "MapSet") &&
            route_kind && !strcmp(route_kind, "map_store_i64")) {
          return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_I64;
        }
        if (op && !strcmp(op, "MapSet") &&
            route_kind && !strcmp(route_kind, "map_store_any")) {
          return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY;
        }
        if (op && !strcmp(op, "ArraySet") &&
            route_kind && !strcmp(route_kind, "array_store_any")) {
          if (value_is_i64) {
            return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_I64;
          }
          if (value_is_string_handle) {
            return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING;
          }
          return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_ANY;
        }
        return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
      }

      enum LoweringPlanSetValueShape {
        HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_ANY = 0,
        HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_I64,
        HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_NON_STRING_HANDLE,
        HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_STRING_HANDLE,
      };

      struct LoweringPlanSetRouteRule {
        const char* route_id;
        const char* core_op;
        const char* route_kind;
        const char* symbol;
        const char* route_proof;
        enum LoweringPlanTierMetadata tier;
        enum LoweringPlanSetValueShape value_shape;
        enum GenericMethodSetRouteKind result;
      };

      auto int lowering_plan_set_value_shape_matches(
          enum LoweringPlanSetValueShape shape,
          int value_is_i64,
          int value_is_string_handle) {
        switch (shape) {
          case HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_ANY:
            return 1;
          case HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_I64:
            return value_is_i64;
          case HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_NON_STRING_HANDLE:
            return !value_is_i64 && !value_is_string_handle;
          case HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_STRING_HANDLE:
            return value_is_string_handle;
          default:
            return 0;
        }
      }

      auto enum GenericMethodSetRouteKind
      lowering_plan_set_route_rule_matches(
          const struct LoweringPlanGenericMethodView* view,
          const struct LoweringPlanSetRouteRule* rule,
          int value_is_i64,
          int value_is_string_handle) {
        if (!(view && rule)) return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
        if (lowering_plan_generic_method_view_matches_route_tuple(
                view,
                rule->route_id,
                rule->core_op,
                rule->route_kind,
                rule->tier) &&
            view->symbol &&
            !strcmp(view->symbol, rule->symbol) &&
            view->route_proof &&
            !strcmp(view->route_proof, rule->route_proof) &&
            lowering_plan_set_value_shape_matches(
                rule->value_shape,
                value_is_i64,
                value_is_string_handle)) {
          return rule->result;
        }
        return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
      }

      auto enum GenericMethodSetRouteKind
      classify_generic_method_set_route_from_lowering_plan(
          long long bid,
          size_t ii,
          int value_is_i64,
          int value_is_string_handle) {
        static const struct LoweringPlanSetRouteRule rules[] = {
          {
            "generic_method.set",
            "MapSet",
            "map_store_i64",
            "nyash.map.slot_store_hih",
            "set_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
            HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_ANY,
            HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_I64,
          },
          {
            "generic_method.set",
            "MapSet",
            "map_store_any",
            "nyash.map.slot_store_hhh",
            "set_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
            HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_ANY,
            HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_MAP_STORE_ANY,
          },
          {
            "generic_method.set",
            "ArraySet",
            "array_store_any",
            "nyash.array.slot_store_hii",
            "set_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
            HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_I64,
            HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_I64,
          },
          {
            "generic_method.set",
            "ArraySet",
            "array_store_any",
            "nyash.array.slot_store_hih",
            "set_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
            HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_NON_STRING_HANDLE,
            HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_ANY,
          },
          {
            "generic_method.set",
            "ArraySet",
            "array_store_any",
            "nyash.array.set_his",
            "set_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
            HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_STRING_HANDLE,
            HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_ARRAY_STORE_STRING,
          },
        };
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanGenericMethodView view;
        size_t ri = 0;
        if (!entry || !read_lowering_plan_generic_method_view(entry, &view)) {
          return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
        }
        for (ri = 0; ri < ARR_LEN(rules); ri++) {
          enum GenericMethodSetRouteKind route =
              lowering_plan_set_route_rule_matches(
                  &view,
                  &rules[ri],
                  value_is_i64,
                  value_is_string_handle);
          if (route != HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE) {
            return route;
          }
        }
        return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
      }

      auto enum GenericMethodSetRouteKind
      classify_generic_method_set_route_from_site_lowering_plan(
          long long bid,
          size_t ii,
          int value_is_i64,
          int value_is_string_handle) {
        enum GenericMethodSetRouteKind plan_route =
            classify_generic_method_set_route_from_lowering_plan(
                bid,
                ii,
                value_is_i64,
                value_is_string_handle);
        if (plan_route != HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE) {
          return plan_route;
        }
        return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
      }

      auto int classify_generic_method_emit_plan(
          long long bid,
          size_t ii,
          const char* bname,
          const char* mname,
          const struct GenericMethodRouteState* route,
          enum GenericMethodEmitKind preselected_kind,
          long long a0,
          long long a1,
          struct GenericMethodEmitPlan* plan) {
        if (!plan) return 0;
        memset(plan, 0, sizeof(*plan));
        plan->set_route = HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
        if (route) {
          plan->recv_org = route->recv_org;
          plan->runtime_array_len = route->runtime_array_len;
          plan->runtime_array_get = route->runtime_array_get;
          plan->runtime_array_push = route->runtime_array_push;
          plan->runtime_string = route->runtime_string;
          plan->runtime_any_length = route->runtime_any_length;
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
        (void)mname;
        (void)a0;
        plan->kind = preselected_kind;
        if (plan->kind == HAKO_LLVMC_GENERIC_METHOD_EMIT_SET) {
          int value_is_i64 = a1 && is_plain_i64_value(a1);
          int value_is_string_handle = a1 && get_origin(a1) == ORG_STRING;
          enum GenericMethodSetRouteKind plan_set_route =
              classify_generic_method_set_route_from_site_lowering_plan(
                  bid,
                  ii,
                  value_is_i64,
                  value_is_string_handle);
          enum GenericMethodSetRouteKind set_route = classify_generic_method_set_route(
              bname, plan, value_is_i64, value_is_string_handle);
          if (plan_set_route != HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE) {
            set_route = plan_set_route;
          }
          plan->set_route = set_route;
          plan->array_store_string_source_preserve =
              classify_array_store_string_source_preserve(set_route);
          plan->array_store_string_identity_demand_stable_object =
              classify_array_store_string_identity_demand_stable_object(set_route);
          plan->array_store_string_publication_demand_publish_handle =
              classify_array_store_string_publication_demand_publish_handle(set_route);
        }
        return plan->kind != HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
      }
