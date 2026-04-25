      enum GenericMethodGetRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE = 0,
        HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_RUNTIME_DATA_LOAD_ANY,
        HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY,
        HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY,
      };

      struct GenericMethodGetRouteMetadata {
        int matched;
        int invalid;
        enum GenericMethodGetRouteKind route;
        long long receiver_reg;
        long long key_reg;
      };

      auto int generic_method_get_route_read_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto const char* generic_method_get_helper_symbol(
          enum GenericMethodGetRouteKind route) {
        switch (route) {
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_RUNTIME_DATA_LOAD_ANY:
            return "nyash.runtime_data.get_hh";
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY:
            return "nyash.map.slot_load_hh";
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY:
            return "nyash.array.slot_load_hi";
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE:
          default:
            return NULL;
        }
      }

      auto enum GenericMethodGetRouteKind generic_method_get_route_kind_from_metadata(
          const char* route_kind) {
        if (route_kind && !strcmp(route_kind, "runtime_data_load_any")) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_RUNTIME_DATA_LOAD_ANY;
        }
        if (route_kind && !strcmp(route_kind, "map_load_any")) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY;
        }
        if (route_kind && !strcmp(route_kind, "array_slot_load_any")) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY;
        }
        return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
      }

      auto enum GenericMethodGetRouteKind
      generic_method_get_route_kind_from_core_method_metadata(
          yyjson_val* route,
          int* present,
          int* invalid) {
        yyjson_val* core_method = route ? yyjson_obj_get(route, "core_method") : NULL;
        const char* op = NULL;
        enum CoreMethodMetadataProof proof =
            HAKO_LLVMC_CORE_METHOD_METADATA_PROOF_NONE;
        enum CoreMethodLoweringTierMetadata lowering_tier =
            HAKO_LLVMC_CORE_METHOD_LOWERING_TIER_NONE;
        if (present) *present = 0;
        if (invalid) *invalid = 0;
        if (!core_method || yyjson_is_null(core_method)) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
        }
        if (present) *present = 1;
        if (!yyjson_is_obj(core_method)) {
          if (invalid) *invalid = 1;
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
        }
        op = read_str(core_method, "op");
        proof = read_core_method_proof_metadata(core_method, invalid);
        lowering_tier = read_core_method_lowering_tier_metadata(core_method, invalid);
        if (op && !strcmp(op, "MapGet") &&
            core_method_metadata_proof_is_contract_manifest(proof) &&
            core_method_lowering_tier_is_cold_fallback(lowering_tier)) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_RUNTIME_DATA_LOAD_ANY;
        }
        if (op && !strcmp(op, "MapGet") &&
            core_method_metadata_proof_is_contract_manifest(proof) &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY;
        }
        if (op && !strcmp(op, "ArrayGet") &&
            core_method_metadata_proof_is_contract_manifest(proof) &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY;
        }
        if (invalid) *invalid = 1;
        return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
      }

      auto int generic_method_get_receiver_origin_valid(
          enum GenericMethodGetRouteKind route,
          const char* receiver_origin_box) {
        if (!receiver_origin_box) return 0;
        switch (route) {
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_RUNTIME_DATA_LOAD_ANY:
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY:
            return !strcmp(receiver_origin_box, "MapBox");
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY:
            return !strcmp(receiver_origin_box, "ArrayBox");
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE:
          default:
            return 0;
        }
      }

      auto int generic_method_get_route_effects_valid(yyjson_val* effects) {
        yyjson_val* e0 = NULL;
        const char* effect = NULL;
        if (!(effects && yyjson_is_arr(effects) && yyjson_arr_size(effects) == 1)) {
          return 0;
        }
        e0 = yyjson_arr_get(effects, 0);
        if (!(e0 && yyjson_is_str(e0))) return 0;
        effect = yyjson_get_str(e0);
        return effect && !strcmp(effect, "read.key");
      }

      auto int generic_method_get_key_route_valid(const char* key_route) {
        if (!key_route) return 1; /* compat with older MIR JSON metadata */
        if (!strcmp(key_route, "i64_const")) return 1;
        if (!strcmp(key_route, "unknown_any")) return 1;
        return 0;
      }

      auto int generic_method_get_route_proof_valid(const char* proof) {
        if (!proof) return 0;
        if (!strcmp(proof, "get_surface_policy")) return 1;
        if (!strcmp(proof, "map_set_scalar_i64_dominates_no_escape")) return 1;
        if (!strcmp(proof, "map_set_scalar_i64_same_key_no_escape")) return 1;
        return 0;
      }

      auto int generic_method_get_contract_shape_valid(
          enum GenericMethodGetRouteKind route,
          const char* return_shape,
          const char* value_demand,
          const char* publication_policy) {
        if (return_shape && value_demand && publication_policy &&
            !strcmp(return_shape, "mixed_runtime_i64_or_handle") &&
            !strcmp(value_demand, "runtime_i64_or_handle") &&
            !strcmp(publication_policy, "runtime_data_facade")) {
          return 1;
        }
        if (return_shape && value_demand && publication_policy &&
            !strcmp(return_shape, "scalar_i64_or_missing_zero") &&
            !strcmp(value_demand, "scalar_i64") &&
            !strcmp(publication_policy, "no_publication")) {
          return 1;
        }
        if ((route == HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY ||
             route == HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY) &&
            !return_shape && value_demand && !publication_policy &&
            !strcmp(value_demand, "read_ref")) {
          return 1;
        }
        return 0;
      }

      auto int match_generic_method_get_route_metadata(
          long long bid,
          size_t ii,
          const char* bname,
          long long recv,
          long long a0,
          struct GenericMethodGetRouteMetadata* out) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "generic_method_routes");
        if (!routes) return 0;
        if (!yyjson_is_arr(routes)) {
          if (out) out->invalid = 1;
          return 0;
        }
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          const char* route_id = route ? read_str(route, "route_id") : NULL;
          long long block = 0;
          long long inst = 0;
          long long receiver_reg = 0;
          long long key_reg = 0;
          long long arity = 0;
          const char* route_box = NULL;
          const char* method = NULL;
          const char* receiver_origin_box = NULL;
          const char* key_route = NULL;
          const char* emit_kind = NULL;
          const char* route_kind_raw = NULL;
          const char* helper_symbol = NULL;
          const char* proof = NULL;
          const char* return_shape = NULL;
          const char* value_demand = NULL;
          const char* publication_policy = NULL;
          int core_method_present = 0;
          int core_method_invalid = 0;
          enum GenericMethodGetRouteKind core_route_kind =
              HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
          enum GenericMethodGetRouteKind metadata_route_kind =
              HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
          enum GenericMethodGetRouteKind route_kind =
              HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
          if (!(route_id && !strcmp(route_id, "generic_method.get"))) {
            continue;
          }
          if (!generic_method_get_route_read_i64_field(route, "block", &block) ||
              !generic_method_get_route_read_i64_field(
                  route,
                  "instruction_index",
                  &inst)) {
            if (out) out->invalid = 1;
            return 0;
          }
          if (block != bid || (size_t)inst != ii) {
            continue;
          }
          route_box = read_str(route, "box_name");
          method = read_str(route, "method");
          receiver_origin_box = read_str(route, "receiver_origin_box");
          key_route = read_str(route, "key_route");
          emit_kind = read_str(route, "emit_kind");
          route_kind_raw = read_str(route, "route_kind");
          helper_symbol = read_str(route, "helper_symbol");
          proof = read_str(route, "proof");
          return_shape = read_str(route, "return_shape");
          value_demand = read_str(route, "value_demand");
          publication_policy = read_str(route, "publication_policy");
          metadata_route_kind = generic_method_get_route_kind_from_metadata(route_kind_raw);
          core_route_kind = generic_method_get_route_kind_from_core_method_metadata(
              route,
              &core_method_present,
              &core_method_invalid);
          route_kind = core_method_present ? core_route_kind : metadata_route_kind;
          if (!(route_box && bname && !strcmp(route_box, bname)) ||
              !(method && !strcmp(method, "get")) ||
              !generic_method_get_key_route_valid(key_route) ||
              !(emit_kind && !strcmp(emit_kind, "get")) ||
              !generic_method_get_route_proof_valid(proof) ||
              !generic_method_get_contract_shape_valid(
                  route_kind,
                  return_shape,
                  value_demand,
                  publication_policy) ||
              core_method_invalid ||
              route_kind == HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE ||
              !generic_method_get_receiver_origin_valid(
                  route_kind,
                  receiver_origin_box) ||
              !generic_method_get_route_read_i64_field(route, "arity", &arity) ||
              arity != 1 ||
              !generic_method_get_route_read_i64_field(
                  route,
                  "receiver_value",
                  &receiver_reg) ||
              !generic_method_get_route_read_i64_field(route, "key_value", &key_reg) ||
              receiver_reg != recv ||
              key_reg != a0 ||
              !helper_symbol ||
              strcmp(helper_symbol, generic_method_get_helper_symbol(route_kind)) ||
              !generic_method_get_route_effects_valid(
                  yyjson_obj_get(route, "effects"))) {
            if (out) out->invalid = 1;
            return 0;
          }
          if (out) {
            out->matched = 1;
            out->route = route_kind;
            out->receiver_reg = receiver_reg;
            out->key_reg = key_reg;
          }
          return 1;
        }
        return 0;
      }

      auto enum GenericMethodGetRouteKind classify_generic_method_get_route(
          const char* bname,
          const struct GenericMethodEmitPlan* plan) {
        if (bname && !strcmp(bname, "MapBox")) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY;
        }
        if ((bname && !strcmp(bname, "ArrayBox")) ||
            (plan && (plan->runtime_array_get || plan->runtime_array_string))) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY;
        }
        if (bname && !strcmp(bname, "RuntimeDataBox")) {
          return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_RUNTIME_DATA_LOAD_ANY;
        }
        return HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE;
      }

      auto int emit_generic_method_get_fallback_by_policy(
          FILE* f,
          long long bid,
          size_t ii,
          const char* bname,
          long long dst,
          long long recv,
          long long a0,
          const struct GenericMethodEmitPlan* plan,
          char* ab) {
        struct GenericMethodGetRouteMetadata metadata;
        struct MapLookupFusionRouteMetadata fusion;
        int metadata_result = 0;
        int fusion_result = match_map_lookup_fusion_route_metadata(
            bid,
            ii,
            HAKO_LLVMC_MAP_LOOKUP_FUSION_ROLE_GET,
            dst,
            recv,
            a0,
            &fusion);
        if (fusion.invalid) {
          if (hako_llvmc_route_trace_enabled()) {
            char extra[128];
            snprintf(
                extra,
                sizeof(extra),
                "bid=%lld ii=%zu recv=%lld key=%lld",
                bid,
                ii,
                recv,
                a0);
            hako_llvmc_emit_route_trace(
                "map_lookup_fusion_get",
                "miss",
                "malformed_mir_route_metadata",
                extra);
          }
          return -1;
        }
        if (fusion_result && fusion.has_stored_value_const) {
          if (hako_llvmc_route_trace_enabled()) {
            char extra[224];
            snprintf(
                extra,
                sizeof(extra),
                "bid=%lld ii=%zu recv=%lld key=%lld get=%lld has=%lld stored=%lld",
                bid,
                ii,
                fusion.receiver_reg,
                fusion.key_reg,
                fusion.get_result_reg,
                fusion.has_result_reg,
                fusion.stored_value_const);
            hako_llvmc_emit_route_trace(
                "map_lookup_fusion_get",
                "hit",
                "mir_route_metadata",
                extra);
          }
          if (dst) {
            fprintf(f, "  %%r%lld = add i64 0, %lld\n", dst, fusion.stored_value_const);
            set_type(dst, T_I64);
          }
          return 1;
        }
        metadata_result =
            match_generic_method_get_route_metadata(bid, ii, bname, recv, a0, &metadata);
        if (metadata.invalid) {
          return -1;
        }
        enum GenericMethodGetRouteKind route = metadata_result
            ? metadata.route
            : classify_generic_method_get_route(bname, plan);
        switch (route) {
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_RUNTIME_DATA_LOAD_ANY:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.runtime_data.get_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.runtime_data.get_hh\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_MAP_LOAD_ANY:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.map.slot_load_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
              set_origin(dst, ORG_MAP_GET);
            } else {
              fprintf(f, "  call i64 @\"nyash.map.slot_load_hh\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_ARRAY_SLOT_LOAD_ANY:
            if (dst) {
              enum HakoLlvmcArraySlotEmitDemand demand =
                  (plan && plan->runtime_array_string)
                      ? HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_TEXT_REF
                      : HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_HANDLE;
              hako_llvmc_emit_array_slot_load(f, dst, ab, demand);
            } else {
              hako_llvmc_emit_array_slot_load(
                  f, dst, ab, HAKO_LLVMC_ARRAY_SLOT_DEMAND_READ_HANDLE);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_GET_ROUTE_NONE:
          default:
            return 0;
        }
      }
