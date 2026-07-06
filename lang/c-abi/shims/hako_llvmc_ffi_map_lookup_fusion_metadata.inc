      enum MapLookupFusionMetadataRole {
        HAKO_LLVMC_MAP_LOOKUP_FUSION_ROLE_GET = 0,
        HAKO_LLVMC_MAP_LOOKUP_FUSION_ROLE_HAS,
      };

      struct MapLookupFusionRouteMetadata {
        int invalid;
        long long receiver_reg;
        long long key_reg;
        long long get_result_reg;
        long long has_result_reg;
        long long stored_value_const;
        int has_stored_value_const;
      };

      auto int map_lookup_fusion_read_optional_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out,
          int* present) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (present) *present = 0;
        if (!val || yyjson_is_null(val)) return 1;
        if (!yyjson_is_num(val)) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        if (present) *present = 1;
        return 1;
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

      auto int map_lookup_fusion_stored_value_proof_valid(const char* proof) {
        return proof &&
               (!strcmp(proof, "scalar_i64_nonzero") ||
                !strcmp(proof, "scalar_i64_const") ||
                !strcmp(proof, "unknown_scalar"));
      }

      auto int map_lookup_fusion_route_site_matches(
          yyjson_val* route,
          const char* target_inst_key,
          long long bid,
          size_t ii,
          int* invalid) {
        const char* route_id = route ? read_str(route, "route_id") : NULL;
        long long block = 0;
        long long target_inst = 0;
        if (!(route_id && !strcmp(route_id, "map_lookup.same_key"))) {
          return 0;
        }
        if (!hako_llvmc_json_read_i64_field(route, "block", &block) ||
            !hako_llvmc_json_read_i64_field(route, target_inst_key, &target_inst)) {
          if (invalid) *invalid = 1;
          return 0;
        }
        return block == bid && (size_t)target_inst == ii;
      }

      auto int map_lookup_fusion_route_shape_valid(yyjson_val* route) {
        const char* fusion_op = read_str(route, "fusion_op");
        const char* receiver_origin_box = read_str(route, "receiver_origin_box");
        const char* key_route = read_str(route, "key_route");
        const char* get_return_shape = read_str(route, "get_return_shape");
        const char* get_value_demand = read_str(route, "get_value_demand");
        const char* get_publication_policy = read_str(route, "get_publication_policy");
        const char* has_result_shape = read_str(route, "has_result_shape");
        const char* stored_value_proof = read_str(route, "stored_value_proof");
        const char* proof = read_str(route, "proof");
        const char* lowering_tier = read_str(route, "lowering_tier");
        return (fusion_op && !strcmp(fusion_op, "MapLookupSameKey")) &&
               (receiver_origin_box && !strcmp(receiver_origin_box, "MapBox")) &&
               (key_route && !strcmp(key_route, "i64_const")) &&
               (get_return_shape &&
                !strcmp(get_return_shape, "scalar_i64_or_missing_zero")) &&
               (get_value_demand && !strcmp(get_value_demand, "scalar_i64")) &&
               (get_publication_policy &&
                !strcmp(get_publication_policy, "no_publication")) &&
               (has_result_shape && !strcmp(has_result_shape, "presence_bool")) &&
               map_lookup_fusion_stored_value_proof_valid(stored_value_proof) &&
               (proof && !strcmp(proof, "same_receiver_same_i64_key_scalar_get_has")) &&
               (lowering_tier && !strcmp(lowering_tier, "cold_fallback")) &&
               map_lookup_fusion_effects_valid(yyjson_obj_get(route, "effects"));
      }

      auto int map_lookup_fusion_route_registers_match(
          yyjson_val* route,
          enum MapLookupFusionMetadataRole role,
          long long dst,
          long long recv,
          long long a0,
          struct MapLookupFusionRouteMetadata* out) {
        long long receiver_reg = 0;
        long long key_reg = 0;
        long long get_result_reg = 0;
        long long has_result_reg = 0;
        long long stored_value_const = 0;
        int has_stored_value_const = 0;
        if (!hako_llvmc_json_read_i64_field(route, "receiver_value", &receiver_reg) ||
            !hako_llvmc_json_read_i64_field(route, "key_value", &key_reg) ||
            !hako_llvmc_json_read_i64_field(route, "get_result_value", &get_result_reg) ||
            !hako_llvmc_json_read_i64_field(route, "has_result_value", &has_result_reg) ||
            !map_lookup_fusion_read_optional_i64_field(
                route,
                "stored_value_const",
                &stored_value_const,
                &has_stored_value_const) ||
            receiver_reg != recv ||
            key_reg != a0 ||
            (role == HAKO_LLVMC_MAP_LOOKUP_FUSION_ROLE_GET && get_result_reg != dst) ||
            (role == HAKO_LLVMC_MAP_LOOKUP_FUSION_ROLE_HAS && has_result_reg != dst)) {
          return 0;
        }
        if (out) {
          out->receiver_reg = receiver_reg;
          out->key_reg = key_reg;
          out->get_result_reg = get_result_reg;
          out->has_result_reg = has_result_reg;
          out->stored_value_const = stored_value_const;
          out->has_stored_value_const = has_stored_value_const;
        }
        return 1;
      }

      auto int match_map_lookup_fusion_route_metadata(
          long long bid,
          size_t ii,
          enum MapLookupFusionMetadataRole role,
          long long dst,
          long long recv,
          long long a0,
          struct MapLookupFusionRouteMetadata* out) {
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
          int invalid_site = 0;
          const char* target_inst_key =
              role == HAKO_LLVMC_MAP_LOOKUP_FUSION_ROLE_GET
                  ? "get_instruction_index"
                  : "has_instruction_index";
          if (!map_lookup_fusion_route_site_matches(
                  route,
                  target_inst_key,
                  bid,
                  ii,
                  &invalid_site)) {
            if (invalid_site && out) out->invalid = 1;
            if (invalid_site) return 0;
            continue;
          }

          if (!map_lookup_fusion_route_shape_valid(route) ||
              !map_lookup_fusion_route_registers_match(
                  route,
                  role,
                  dst,
                  recv,
                  a0,
                  out)) {
            if (out) out->invalid = 1;
            return 0;
          }
          return 1;
        }
        return 0;
      }
