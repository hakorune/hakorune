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
      classify_generic_method_emit_kind_from_metadata(
          long long bid,
          size_t ii) {
        yyjson_val* routes = fn_metadata ? yyjson_obj_get(fn_metadata, "generic_method_routes") : NULL;
        size_t ri = 0;
        if (!(routes && yyjson_is_arr(routes))) {
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
        }
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* core_method = route ? yyjson_obj_get(route, "core_method") : NULL;
          long long block = -1;
          long long inst = -1;
          const char* route_id = route ? read_str(route, "route_id") : NULL;
          const char* route_kind = route ? read_str(route, "route_kind") : NULL;
          const char* core_op = core_method ? read_str(core_method, "op") : NULL;
          int tier =
              hako_llvmc_generic_method_route_registry_tier_from_label(
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
          if (row && row->emit_kind != 0) {
            if (hako_llvmc_route_trace_enabled()) {
              hako_llvmc_emit_route_trace(
                  "generic_method_emit_metadata",
                  "hit",
                  "descriptor_row",
                  route_kind ? route_kind : "-");
            }
            return (enum GenericMethodEmitKind)row->emit_kind;
          }
          if (hako_llvmc_route_trace_enabled()) {
            hako_llvmc_emit_route_trace(
                "generic_method_emit_metadata",
                "skip",
                "descriptor_mismatch",
                route_kind ? route_kind : "-");
          }
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
        }
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
        return classify_generic_method_emit_kind_from_metadata(bid, ii);
      }

      enum LoweringPlanSetValueShape {
        HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_NONE = 0,
        HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_ANY,
        HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_I64,
        HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_NON_STRING_HANDLE,
        HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_STRING_HANDLE,
      };

      auto enum LoweringPlanSetValueShape lowering_plan_set_value_shape_from_runtime_value(
          int value_is_i64,
          int value_is_string_handle) {
        if (value_is_i64) {
          return HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_I64;
        }
        if (value_is_string_handle) {
          return HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_STRING_HANDLE;
        }
        return HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_NON_STRING_HANDLE;
      }

      auto enum GenericMethodSetRouteKind
      classify_generic_method_set_route_from_metadata(
          long long bid,
          size_t ii,
          enum LoweringPlanSetValueShape value_shape) {
        yyjson_val* routes = fn_metadata ? yyjson_obj_get(fn_metadata, "generic_method_routes") : NULL;
        size_t ri = 0;
        if (!(routes && yyjson_is_arr(routes))) {
          return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
        }
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* core_method = route ? yyjson_obj_get(route, "core_method") : NULL;
          long long block = -1;
          long long inst = -1;
          const char* route_id = route ? read_str(route, "route_id") : NULL;
          const char* route_kind = route ? read_str(route, "route_kind") : NULL;
          const char* proof = route ? read_str(route, "proof") : NULL;
          const char* core_op = core_method ? read_str(core_method, "op") : NULL;
          int tier =
              hako_llvmc_generic_method_route_registry_tier_from_label(
                  core_method ? read_str(core_method, "lowering_tier") : NULL);
          const struct HakoLlvmcGenericMethodRouteRegistryRow* row = NULL;
          if (!hako_llvmc_json_read_i64_field(route, "block", &block) ||
              !hako_llvmc_json_read_i64_field(route, "instruction_index", &inst) ||
              block != bid ||
              inst != (long long)ii ||
              !(route_id && !strcmp(route_id, "generic_method.set"))) {
            continue;
          }
          row = hako_llvmc_generic_method_route_registry_find_set_rule(
              route_id,
              core_op,
              route_kind,
              tier,
              NULL,
              proof,
              (int)value_shape);
          if (row && row->set_route_result != 0) {
            if (hako_llvmc_route_trace_enabled()) {
              hako_llvmc_emit_route_trace(
                  "generic_method_set_metadata",
                  "hit",
                  "descriptor_row",
                  route_kind ? route_kind : "-");
            }
            return (enum GenericMethodSetRouteKind)row->set_route_result;
          }
          if (hako_llvmc_route_trace_enabled()) {
            hako_llvmc_emit_route_trace(
                "generic_method_set_metadata",
                "skip",
                "descriptor_mismatch",
                route_kind ? route_kind : "-");
          }
          return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
        }
        return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
      }

      auto enum GenericMethodSetRouteKind
      classify_generic_method_set_route_from_lowering_plan(
          long long bid,
          size_t ii,
          int value_is_i64,
          int value_is_string_handle) {
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanGenericMethodView view;
        const struct HakoLlvmcGenericMethodRouteRegistryRow* row = NULL;
        enum LoweringPlanSetValueShape value_shape =
            HAKO_LLVMC_LOWERING_PLAN_SET_VALUE_NONE;
        if (!entry || !read_lowering_plan_generic_method_view(entry, &view)) {
          return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
        }
        value_shape = lowering_plan_set_value_shape_from_runtime_value(
            value_is_i64,
            value_is_string_handle);
        row = hako_llvmc_generic_method_route_registry_find_set_rule(
            view.route_id,
            view.core_op,
            view.route_kind,
            view.tier,
            NULL,
            view.route_proof,
            (int)value_shape);
        if (row && row->set_route_result != 0) {
          return (enum GenericMethodSetRouteKind)row->set_route_result;
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
        return classify_generic_method_set_route_from_metadata(
            bid,
            ii,
            lowering_plan_set_value_shape_from_runtime_value(
                value_is_i64,
                value_is_string_handle));
      }

      auto int generic_method_value_origin_is_string_from_lowering_plan_site(
          long long bid,
          size_t ii) {
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanGenericMethodView view;
        if (!read_lowering_plan_generic_method_view(entry, &view)) return 0;
        return lowering_plan_generic_method_view_value_origin_is_string(&view);
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
        plan->value_is_string_handle =
            generic_method_value_origin_is_string_from_lowering_plan_site(bid, ii);
        plan->kind = preselected_kind;
        if (plan->kind == HAKO_LLVMC_GENERIC_METHOD_EMIT_SET) {
          int value_is_i64 = a1 && is_plain_i64_value(a1);
          int value_is_string_handle = plan->value_is_string_handle;
          plan->set_route =
              classify_generic_method_set_route_from_site_lowering_plan(
                  bid,
                  ii,
                  value_is_i64,
                  value_is_string_handle);
          plan->array_store_string_source_preserve =
              classify_array_store_string_source_preserve(plan->set_route);
          plan->array_store_string_identity_demand_stable_object =
              classify_array_store_string_identity_demand_stable_object(plan->set_route);
          plan->array_store_string_publication_demand_publish_handle =
              classify_array_store_string_publication_demand_publish_handle(plan->set_route);
        }
        return plan->kind != HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
      }
