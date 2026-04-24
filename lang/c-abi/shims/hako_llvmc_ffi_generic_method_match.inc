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
      classify_generic_method_emit_kind_from_core_method_metadata(
          long long bid,
          size_t ii) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (!fn_metadata) return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
        routes = yyjson_obj_get(fn_metadata, "generic_method_routes");
        if (!(routes && yyjson_is_arr(routes))) {
          return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
        }
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* core_method = NULL;
          const char* route_id = route ? read_str(route, "route_id") : NULL;
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
          core_method = yyjson_obj_get(route, "core_method");
          if (!(core_method && yyjson_is_obj(core_method))) {
            return HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
          }
          op = read_str(core_method, "op");
          proof = read_core_method_proof_metadata(core_method, &invalid);
          lowering_tier = read_core_method_lowering_tier_metadata(core_method, &invalid);
          if (!invalid &&
              op &&
              !strcmp(op, "MapHas") &&
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
              core_method_lowering_tier_is_cold_fallback(lowering_tier)) {
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

      auto int classify_generic_method_emit_plan(
          const char* bname,
          const char* mname,
          const struct GenericMethodRouteState* route,
          enum GenericMethodEmitKind preselected_kind,
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
        plan->kind = preselected_kind != HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE
            ? preselected_kind
            : classify_generic_method_emit_kind(bname, mname, route, a0, a1);
        if (plan->kind == HAKO_LLVMC_GENERIC_METHOD_EMIT_SET) {
          int value_is_i64 = a1 && is_plain_i64_value(a1);
          int value_is_string_handle = a1 && get_origin(a1) == ORG_STRING;
          enum GenericMethodSetRouteKind set_route = classify_generic_method_set_route(
              bname, plan, value_is_i64, value_is_string_handle);
          plan->array_store_string_source_preserve =
              classify_array_store_string_source_preserve(set_route);
          plan->array_store_string_identity_demand_stable_object =
              classify_array_store_string_identity_demand_stable_object(set_route);
          plan->array_store_string_publication_demand_publish_handle =
              classify_array_store_string_publication_demand_publish_handle(set_route);
        }
        return plan->kind != HAKO_LLVMC_GENERIC_METHOD_EMIT_NONE;
      }
