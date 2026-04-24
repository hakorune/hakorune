      enum GenericMethodHasRouteKind {
        HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE = 0,
        HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_RUNTIME_DATA_CONTAINS_ANY,
        HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_ARRAY_CONTAINS_ANY,
        HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_ANY,
        HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_I64,
      };

      struct GenericMethodHasRouteMetadata {
        int matched;
        int invalid;
        enum GenericMethodHasRouteKind route;
        long long receiver_reg;
        long long key_reg;
      };

      struct MapLookupFusionHasMetadata {
        int matched;
        int invalid;
        long long receiver_reg;
        long long key_reg;
        long long get_result_reg;
        long long has_result_reg;
      };

      auto int generic_method_has_route_read_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto const char* generic_method_has_helper_symbol(
          enum GenericMethodHasRouteKind route) {
        switch (route) {
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_RUNTIME_DATA_CONTAINS_ANY:
            return "nyash.runtime_data.has_hh";
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_ANY:
            return "nyash.map.probe_hh";
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_I64:
            return "nyash.map.probe_hi";
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_ARRAY_CONTAINS_ANY:
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE:
          default:
            return NULL;
        }
      }

      auto enum GenericMethodHasRouteKind generic_method_has_route_kind_from_metadata(
          const char* route_kind) {
        if (route_kind && !strcmp(route_kind, "runtime_data_contains_any")) {
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_RUNTIME_DATA_CONTAINS_ANY;
        }
        if (route_kind && !strcmp(route_kind, "map_contains_any")) {
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_ANY;
        }
        if (route_kind && !strcmp(route_kind, "map_contains_i64")) {
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_I64;
        }
        return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE;
      }

      auto enum GenericMethodHasRouteKind
      generic_method_has_route_kind_from_core_and_route_metadata(
          enum GenericMethodHasRouteKind core_route_kind,
          enum GenericMethodHasRouteKind metadata_route_kind) {
        if (core_route_kind == HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_ANY &&
            metadata_route_kind == HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_I64) {
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_I64;
        }
        return core_route_kind;
      }

      auto enum GenericMethodHasRouteKind
      generic_method_has_route_kind_from_core_method_metadata(
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
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE;
        }
        if (present) *present = 1;
        if (!yyjson_is_obj(core_method)) {
          if (invalid) *invalid = 1;
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE;
        }
        op = read_str(core_method, "op");
        proof = read_core_method_proof_metadata(core_method, invalid);
        lowering_tier = read_core_method_lowering_tier_metadata(core_method, invalid);
        if (op && !strcmp(op, "MapHas") &&
            core_method_metadata_proof_is_contract_manifest(proof) &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_ANY;
        }
        if (invalid) *invalid = 1;
        return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE;
      }

      auto int generic_method_has_route_effects_valid(yyjson_val* effects) {
        yyjson_val* e0 = NULL;
        const char* effect = NULL;
        if (!(effects && yyjson_is_arr(effects) && yyjson_arr_size(effects) == 1)) {
          return 0;
        }
        e0 = yyjson_arr_get(effects, 0);
        if (!(e0 && yyjson_is_str(e0))) return 0;
        effect = yyjson_get_str(e0);
        return effect && !strcmp(effect, "probe.key");
      }

      auto int map_lookup_fusion_effects_valid(yyjson_val* effects) {
        yyjson_val* e0 = NULL;
        yyjson_val* e1 = NULL;
        yyjson_val* e2 = NULL;
        const char* s0 = NULL;
        const char* s1 = NULL;
        const char* s2 = NULL;
        if (!(effects && yyjson_is_arr(effects) && yyjson_arr_size(effects) == 3)) {
          return 0;
        }
        e0 = yyjson_arr_get(effects, 0);
        e1 = yyjson_arr_get(effects, 1);
        e2 = yyjson_arr_get(effects, 2);
        if (!(e0 && yyjson_is_str(e0) && e1 && yyjson_is_str(e1) &&
              e2 && yyjson_is_str(e2))) {
          return 0;
        }
        s0 = yyjson_get_str(e0);
        s1 = yyjson_get_str(e1);
        s2 = yyjson_get_str(e2);
        return s0 && !strcmp(s0, "read.key") &&
               s1 && !strcmp(s1, "probe.key") &&
               s2 && !strcmp(s2, "metadata.only");
      }

      auto int map_lookup_stored_value_proof_valid(const char* proof) {
        return proof &&
               (!strcmp(proof, "scalar_i64_nonzero") ||
                !strcmp(proof, "scalar_i64_const") ||
                !strcmp(proof, "unknown_scalar"));
      }

      auto int generic_method_has_key_route_valid(const char* key_route) {
        if (!key_route) return 1; /* compat with older MIR JSON metadata */
        if (!strcmp(key_route, "i64_const")) return 1;
        if (!strcmp(key_route, "unknown_any")) return 1;
        return 0;
      }

      auto int generic_method_has_route_key_route_valid(
          enum GenericMethodHasRouteKind route_kind,
          const char* key_route) {
        if (!generic_method_has_key_route_valid(key_route)) return 0;
        if (route_kind == HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_I64) {
          return key_route && !strcmp(key_route, "i64_const");
        }
        return 1;
      }

      auto int match_map_lookup_fusion_has_metadata(
          long long bid,
          size_t ii,
          long long dst,
          long long recv,
          long long a0,
          struct MapLookupFusionHasMetadata* out) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "map_lookup_fusion_routes");
        if (!routes) return 0;
        if (!yyjson_is_arr(routes)) {
          if (out) out->invalid = 1;
          return 0;
        }
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          const char* route_id = route ? read_str(route, "route_id") : NULL;
          long long block = 0;
          long long has_inst = 0;
          long long receiver_reg = 0;
          long long key_reg = 0;
          long long get_result_reg = 0;
          long long has_result_reg = 0;
          const char* fusion_op = NULL;
          const char* receiver_origin_box = NULL;
          const char* key_route = NULL;
          const char* get_return_shape = NULL;
          const char* get_value_demand = NULL;
          const char* get_publication_policy = NULL;
          const char* has_result_shape = NULL;
          const char* stored_value_proof = NULL;
          const char* proof = NULL;
          const char* lowering_tier = NULL;
          if (!(route_id && !strcmp(route_id, "map_lookup.same_key"))) {
            continue;
          }
          if (!generic_method_has_route_read_i64_field(route, "block", &block) ||
              !generic_method_has_route_read_i64_field(
                  route,
                  "has_instruction_index",
                  &has_inst)) {
            if (out) out->invalid = 1;
            return 0;
          }
          if (block != bid || (size_t)has_inst != ii) {
            continue;
          }

          fusion_op = read_str(route, "fusion_op");
          receiver_origin_box = read_str(route, "receiver_origin_box");
          key_route = read_str(route, "key_route");
          get_return_shape = read_str(route, "get_return_shape");
          get_value_demand = read_str(route, "get_value_demand");
          get_publication_policy = read_str(route, "get_publication_policy");
          has_result_shape = read_str(route, "has_result_shape");
          stored_value_proof = read_str(route, "stored_value_proof");
          proof = read_str(route, "proof");
          lowering_tier = read_str(route, "lowering_tier");

          if (!(fusion_op && !strcmp(fusion_op, "MapLookupSameKey")) ||
              !(receiver_origin_box && !strcmp(receiver_origin_box, "MapBox")) ||
              !(key_route && !strcmp(key_route, "i64_const")) ||
              !(get_return_shape &&
                !strcmp(get_return_shape, "scalar_i64_or_missing_zero")) ||
              !(get_value_demand && !strcmp(get_value_demand, "scalar_i64")) ||
              !(get_publication_policy &&
                !strcmp(get_publication_policy, "no_publication")) ||
              !(has_result_shape && !strcmp(has_result_shape, "presence_bool")) ||
              !map_lookup_stored_value_proof_valid(stored_value_proof) ||
              !(proof && !strcmp(proof, "same_receiver_same_i64_key_scalar_get_has")) ||
              !(lowering_tier && !strcmp(lowering_tier, "cold_fallback")) ||
              !generic_method_has_route_read_i64_field(
                  route,
                  "receiver_value",
                  &receiver_reg) ||
              !generic_method_has_route_read_i64_field(route, "key_value", &key_reg) ||
              !generic_method_has_route_read_i64_field(
                  route,
                  "get_result_value",
                  &get_result_reg) ||
              !generic_method_has_route_read_i64_field(
                  route,
                  "has_result_value",
                  &has_result_reg) ||
              receiver_reg != recv ||
              key_reg != a0 ||
              has_result_reg != dst ||
              !map_lookup_fusion_effects_valid(yyjson_obj_get(route, "effects"))) {
            if (out) out->invalid = 1;
            return 0;
          }

          if (out) {
            out->matched = 1;
            out->receiver_reg = receiver_reg;
            out->key_reg = key_reg;
            out->get_result_reg = get_result_reg;
            out->has_result_reg = has_result_reg;
          }
          if (hako_llvmc_route_trace_enabled()) {
            char extra[192];
            snprintf(
                extra,
                sizeof(extra),
                "bid=%lld ii=%zu recv=%lld key=%lld get=%lld has=%lld",
                bid,
                ii,
                receiver_reg,
                key_reg,
                get_result_reg,
                has_result_reg);
            hako_llvmc_emit_route_trace(
                "map_lookup_fusion_has",
                "hit",
                "mir_route_metadata",
                extra);
          }
          return 1;
        }
        return 0;
      }

      auto int match_generic_method_has_route_metadata(
          long long bid,
          size_t ii,
          const char* bname,
          long long recv,
          long long a0,
          struct GenericMethodHasRouteMetadata* out) {
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
          const char* value_demand = NULL;
          int core_method_present = 0;
          int core_method_invalid = 0;
          enum GenericMethodHasRouteKind core_route_kind =
              HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE;
          enum GenericMethodHasRouteKind metadata_route_kind =
              HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE;
          enum GenericMethodHasRouteKind route_kind =
              HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE;
          if (!(route_id && !strcmp(route_id, "generic_method.has"))) {
            continue;
          }
          if (!generic_method_has_route_read_i64_field(route, "block", &block) ||
              !generic_method_has_route_read_i64_field(
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
          value_demand = read_str(route, "value_demand");
          metadata_route_kind = generic_method_has_route_kind_from_metadata(route_kind_raw);
          core_route_kind = generic_method_has_route_kind_from_core_method_metadata(
              route,
              &core_method_present,
              &core_method_invalid);
          route_kind = core_method_present
              ? generic_method_has_route_kind_from_core_and_route_metadata(
                    core_route_kind,
                    metadata_route_kind)
              : metadata_route_kind;
          if (!(route_box && bname && !strcmp(route_box, bname)) ||
              !(method && !strcmp(method, "has")) ||
              !(emit_kind && !strcmp(emit_kind, "has")) ||
              !(proof && !strcmp(proof, "has_surface_policy")) ||
              !(value_demand && !strcmp(value_demand, "read_ref")) ||
              !generic_method_has_route_key_route_valid(route_kind, key_route) ||
              (core_method_present &&
               !(receiver_origin_box && !strcmp(receiver_origin_box, "MapBox"))) ||
              core_method_invalid ||
              route_kind == HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE ||
              !generic_method_has_route_read_i64_field(route, "arity", &arity) ||
              arity != 1 ||
              !generic_method_has_route_read_i64_field(
                  route,
                  "receiver_value",
                  &receiver_reg) ||
              !generic_method_has_route_read_i64_field(route, "key_value", &key_reg) ||
              receiver_reg != recv ||
              key_reg != a0 ||
              !helper_symbol ||
              strcmp(helper_symbol, generic_method_has_helper_symbol(route_kind)) ||
              !generic_method_has_route_effects_valid(
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
          if (hako_llvmc_route_trace_enabled()) {
            char extra[192];
            snprintf(
                extra,
                sizeof(extra),
                "bid=%lld ii=%zu recv=%lld key=%lld route_kind=%s key_route=%s helper=%s",
                bid,
                ii,
                receiver_reg,
                key_reg,
                route_kind_raw,
                key_route ? key_route : "none",
                helper_symbol);
            hako_llvmc_emit_route_trace(
                "generic_method_has_route",
                "hit",
                "mir_route_metadata",
                extra);
          }
          return 1;
        }
        return 0;
      }

      auto enum GenericMethodHasRouteKind classify_generic_method_has_route(
          const char* bname,
          const struct GenericMethodEmitPlan* plan) {
        if (bname && !strcmp(bname, "MapBox")) {
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_ANY;
        }
        if ((bname && !strcmp(bname, "ArrayBox")) ||
            (bname && !strcmp(bname, "RuntimeDataBox")) ||
            (plan && (plan->runtime_array_has || plan->runtime_map_has))) {
          return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_RUNTIME_DATA_CONTAINS_ANY;
        }
        return HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE;
      }

      auto int classify_generic_method_has_value_demand_read_ref(
          enum GenericMethodHasRouteKind route) {
        return route != HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE;
      }

      auto int emit_generic_method_has_by_policy(
          FILE* f,
          long long bid,
          size_t ii,
          const char* bname,
          long long dst,
          long long recv,
          long long a0,
          const struct GenericMethodEmitPlan* plan,
          char* ab,
          size_t ab_cap) {
        struct GenericMethodHasRouteMetadata metadata;
        struct MapLookupFusionHasMetadata fusion;
        int metadata_result = 0;
        int fusion_result = 0;
        if (a0) append_i64_arg_ref(ab, ab_cap, a0, ab[0]=='\0');
        fusion_result =
            match_map_lookup_fusion_has_metadata(bid, ii, dst, recv, a0, &fusion);
        if (fusion.invalid) {
          if (hako_llvmc_route_trace_enabled()) {
            char extra[128];
            snprintf(extra, sizeof(extra), "bid=%lld ii=%zu recv=%lld key=%lld", bid, ii, recv, a0);
            hako_llvmc_emit_route_trace(
                "map_lookup_fusion_has",
                "miss",
                "malformed_mir_route_metadata",
                extra);
          }
          return -1;
        }
        if (fusion_result) {
          if (dst) {
            fprintf(f, "  %%r%lld = add i64 0, 1\n", dst);
            set_type(dst, T_I64);
          }
          return 1;
        }
        metadata_result =
            match_generic_method_has_route_metadata(bid, ii, bname, recv, a0, &metadata);
        if (metadata.invalid) {
          if (hako_llvmc_route_trace_enabled()) {
            char extra[128];
            snprintf(extra, sizeof(extra), "bid=%lld ii=%zu recv=%lld key=%lld", bid, ii, recv, a0);
            hako_llvmc_emit_route_trace(
                "generic_method_has_route",
                "miss",
                "malformed_mir_route_metadata",
                extra);
          }
          return -1;
        }
        enum GenericMethodHasRouteKind route = metadata_result
            ? metadata.route
            : classify_generic_method_has_route(bname, plan);
        int value_demand_read_ref =
            classify_generic_method_has_value_demand_read_ref(route);
        (void)value_demand_read_ref;
        switch (route) {
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_RUNTIME_DATA_CONTAINS_ANY:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.runtime_data.has_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.runtime_data.has_hh\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_ARRAY_CONTAINS_ANY:
            return 0;
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_ANY:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.map.probe_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.map.probe_hh\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_MAP_CONTAINS_I64:
            if (dst) {
              fprintf(f, "  %%r%lld = call i64 @\"nyash.map.probe_hi\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              fprintf(f, "  call i64 @\"nyash.map.probe_hi\"(%s)\n", ab);
            }
            return 1;
          case HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_NONE:
          default:
            return 0;
        }
      }
