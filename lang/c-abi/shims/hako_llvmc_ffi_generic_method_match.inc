      #include "hako_llvmc_ffi_generic_method_policy.inc"

      auto int generic_method_emit_kind_route_read_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto enum GenericMethodEmitKind
      classify_generic_method_emit_kind_from_lowering_plan(yyjson_val* entry) {
        struct LoweringPlanGenericMethodView view;
        if (!read_lowering_plan_generic_method_view(entry, &view)) {
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
        }
        if (!strcmp(view.route_id, "generic_method.has") &&
            (!strcmp(view.core_op, "MapHas") ||
             !strcmp(view.core_op, "ArrayHas")) &&
            lowering_plan_tier_is_direct_abi(view.tier)) {
          trace_lowering_plan_entry_hit(entry, "generic_method_emit");
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_HAS;
        }
        if (!strcmp(view.route_id, "generic_method.get") &&
            !strcmp(view.core_op, "MapGet") &&
            (lowering_plan_tier_is_cold_runtime(view.tier) ||
             lowering_plan_tier_is_direct_abi(view.tier))) {
          trace_lowering_plan_entry_hit(entry, "generic_method_emit");
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_GET;
        }
        if (!strcmp(view.route_id, "generic_method.get") &&
            !strcmp(view.core_op, "ArrayGet") &&
            lowering_plan_tier_is_direct_abi(view.tier)) {
          trace_lowering_plan_entry_hit(entry, "generic_method_emit");
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_GET;
        }
        if (!strcmp(view.route_id, "generic_method.len") &&
            (!strcmp(view.core_op, "ArrayLen") ||
             !strcmp(view.core_op, "MapLen") ||
             !strcmp(view.core_op, "StringLen")) &&
            lowering_plan_tier_is_direct_abi(view.tier)) {
          trace_lowering_plan_entry_hit(entry, "generic_method_emit");
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_LEN;
        }
        if (!strcmp(view.route_id, "generic_method.push") &&
            !strcmp(view.core_op, "ArrayPush") &&
            lowering_plan_tier_is_cold_runtime(view.tier)) {
          trace_lowering_plan_entry_hit(entry, "generic_method_emit");
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_PUSH;
        }
        if (!strcmp(view.route_id, "generic_method.set") &&
            (!strcmp(view.core_op, "ArraySet") ||
             !strcmp(view.core_op, "MapSet")) &&
            lowering_plan_tier_is_cold_runtime(view.tier)) {
          trace_lowering_plan_entry_hit(entry, "generic_method_emit");
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_SET;
        }
        if (!strcmp(view.route_id, "generic_method.substring") &&
            !strcmp(view.core_op, "StringSubstring") &&
            lowering_plan_tier_is_direct_abi(view.tier)) {
          trace_lowering_plan_entry_hit(entry, "generic_method_emit");
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_SUBSTRING;
        }
        return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
      }

      auto enum GenericMethodEmitKind
      classify_generic_method_emit_kind_from_core_method_metadata(
          long long bid,
          size_t ii) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        yyjson_val* plan_entry = find_lowering_plan_entry_for_site(bid, ii);
        if (plan_entry) {
          enum GenericMethodEmitKind plan_kind =
              classify_generic_method_emit_kind_from_lowering_plan(plan_entry);
          if (plan_kind != HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE) {
            return plan_kind;
          }
        }
        if (!fn_metadata) return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
        routes = yyjson_obj_get(fn_metadata, "generic_method_routes");
        if (!(routes && yyjson_is_arr(routes))) {
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
        }
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* core_method = NULL;
          const char* route_id = route ? read_str(route, "route_id") : NULL;
          const char* route_kind = route ? read_str(route, "route_kind") : NULL;
          const char* helper_symbol = route ? read_str(route, "helper_symbol") : NULL;
          const char* proof_raw = route ? read_str(route, "proof") : NULL;
          const char* op = NULL;
          long long block = 0;
          long long inst = 0;
          int invalid = 0;
          enum CoreMethodMetadataProof proof =
              HAKO_LLVMC_CORE_METHOD_METADATA_PROOF_NONE;
          enum CoreMethodLoweringTierMetadata lowering_tier =
              HAKO_LLVMC_CORE_METHOD_LOWERING_TIER_NONE;
          if (!generic_method_emit_kind_route_read_i64_field(route, "block", &block) ||
              !generic_method_emit_kind_route_read_i64_field(
                  route,
                  "instruction_index",
                  &inst) ||
              block != bid ||
              (size_t)inst != ii) {
            continue;
          }
          if (!(route_id &&
                (!strcmp(route_id, "generic_method.has") ||
                 !strcmp(route_id, "generic_method.get") ||
                 !strcmp(route_id, "generic_method.len") ||
                 !strcmp(route_id, "generic_method.push") ||
                 !strcmp(route_id, "generic_method.set") ||
                 !strcmp(route_id, "generic_method.substring")))) {
            continue;
          }
          if (route_id &&
              !strcmp(route_id, "generic_method.has") &&
              route_kind &&
              !strcmp(route_kind, "runtime_data_contains_any") &&
              helper_symbol &&
              !strcmp(helper_symbol, "nyash.runtime_data.has_hh") &&
              proof_raw &&
              !strcmp(proof_raw, "has_surface_policy")) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_HAS;
          }
          core_method = yyjson_obj_get(route, "core_method");
          if (!(core_method && yyjson_is_obj(core_method))) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
          }
          op = read_str(core_method, "op");
          proof = read_core_method_proof_metadata(core_method, &invalid);
          lowering_tier = read_core_method_lowering_tier_metadata(core_method, &invalid);
          if (!invalid &&
              op &&
              (!strcmp(op, "MapHas") ||
               !strcmp(op, "ArrayHas")) &&
              route_id &&
              !strcmp(route_id, "generic_method.has") &&
              core_method_metadata_proof_is_contract_manifest(proof) &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_HAS;
          }
          if (!invalid &&
              op &&
              !strcmp(op, "MapGet") &&
              route_id &&
              !strcmp(route_id, "generic_method.get") &&
              core_method_metadata_proof_is_contract_manifest(proof) &&
              (core_method_lowering_tier_is_cold_fallback(lowering_tier) ||
               core_method_lowering_tier_is_warm_direct_abi(lowering_tier))) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_GET;
          }
          if (!invalid &&
              op &&
              !strcmp(op, "ArrayGet") &&
              route_id &&
              !strcmp(route_id, "generic_method.get") &&
              core_method_metadata_proof_is_contract_manifest(proof) &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_GET;
          }
          if (!invalid &&
              op &&
              (!strcmp(op, "ArrayLen") ||
               !strcmp(op, "MapLen") ||
               !strcmp(op, "StringLen")) &&
              route_id &&
              !strcmp(route_id, "generic_method.len") &&
              core_method_metadata_proof_is_contract_manifest(proof) &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_LEN;
          }
          if (!invalid &&
              op &&
              !strcmp(op, "ArrayPush") &&
              route_id &&
              !strcmp(route_id, "generic_method.push") &&
              core_method_metadata_proof_is_contract_manifest(proof) &&
              core_method_lowering_tier_is_cold_fallback(lowering_tier)) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_PUSH;
          }
          if (!invalid &&
              op &&
              (!strcmp(op, "ArraySet") ||
               !strcmp(op, "MapSet")) &&
              route_id &&
              !strcmp(route_id, "generic_method.set") &&
              core_method_metadata_proof_is_contract_manifest(proof) &&
              core_method_lowering_tier_is_cold_fallback(lowering_tier)) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_SET;
          }
          if (!invalid &&
              op &&
              !strcmp(op, "StringSubstring") &&
              route_id &&
              !strcmp(route_id, "generic_method.substring") &&
              core_method_metadata_proof_is_contract_manifest(proof) &&
              core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_SUBSTRING;
          }
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
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
        if (view->route_id &&
            !strcmp(view->route_id, rule->route_id) &&
            view->core_op &&
            !strcmp(view->core_op, rule->core_op) &&
            view->route_kind &&
            !strcmp(view->route_kind, rule->route_kind) &&
            view->symbol &&
            !strcmp(view->symbol, rule->symbol) &&
            view->route_proof &&
            !strcmp(view->route_proof, rule->route_proof) &&
            view->tier == rule->tier &&
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
      classify_generic_method_set_route_from_core_method_metadata(
          long long bid,
          size_t ii,
          int value_is_i64,
          int value_is_string_handle) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        enum GenericMethodSetRouteKind plan_route =
            classify_generic_method_set_route_from_lowering_plan(
                bid,
                ii,
                value_is_i64,
                value_is_string_handle);
        if (plan_route != HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE) {
          return plan_route;
        }
        if (!fn_metadata) return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
        routes = yyjson_obj_get(fn_metadata, "generic_method_routes");
        if (!(routes && yyjson_is_arr(routes))) {
          return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
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
          if (!generic_method_emit_kind_route_read_i64_field(route, "block", &block) ||
              !generic_method_emit_kind_route_read_i64_field(
                  route,
                  "instruction_index",
                  &inst) ||
              block != bid ||
              (size_t)inst != ii) {
            continue;
          }
          if (!(route_id && !strcmp(route_id, "generic_method.set"))) {
            continue;
          }
          core_method = yyjson_obj_get(route, "core_method");
          if (!(core_method && yyjson_is_obj(core_method))) {
            return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
          }
          op = read_str(core_method, "op");
          proof = read_core_method_proof_metadata(core_method, &invalid);
          lowering_tier = read_core_method_lowering_tier_metadata(core_method, &invalid);
          if (invalid ||
              !core_method_metadata_proof_is_contract_manifest(proof) ||
              !core_method_lowering_tier_is_cold_fallback(lowering_tier)) {
            return HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE;
          }
          return generic_method_set_route_from_metadata_value_shape(
              op,
              route_kind,
              value_is_i64,
              value_is_string_handle);
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
        plan->kind = preselected_kind != HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE
            ? preselected_kind
            : classify_generic_method_emit_kind(bname, mname, route, a0, a1);
        if (plan->kind == HAKO_LLVMC_GENERIC_METHOD_EMIT_SET) {
          int value_is_i64 = a1 && is_plain_i64_value(a1);
          int value_is_string_handle = a1 && get_origin(a1) == ORG_STRING;
          enum GenericMethodSetRouteKind metadata_set_route =
              classify_generic_method_set_route_from_core_method_metadata(
                  bid,
                  ii,
                  value_is_i64,
                  value_is_string_handle);
          enum GenericMethodSetRouteKind set_route = classify_generic_method_set_route(
              bname, plan, value_is_i64, value_is_string_handle);
          if (metadata_set_route != HAKO_LLVMC_GENERIC_METHOD_SET_ROUTE_NONE) {
            set_route = metadata_set_route;
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
