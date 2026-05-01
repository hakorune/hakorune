      struct GenericPureNeedFlags {
        int map_birth;
        int map_set;
        int map_size;
        int map_get;
        int map_has;
        int runtime_data_has;
        int arr_birth;
        int arr_push;
        int arr_len;
        int arr_set;
        int arr_get;
        int arr_has;
        int arr_slot_load;
        int arr_kernel_slot_concat_by_index;
        int arr_string_indexof;
        int arr_string_indexof_const_utf8;
        int string_const_box;
        int string_len;
        int string_substring;
        int string_substring_len;
        int string_indexof;
        int string_concat;
        int printf;
      };

      enum MirCallNeedKind {
        HAKO_LLVMC_MIR_CALL_NEED_NONE = 0,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_SET,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_GET,
        HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS,
        HAKO_LLVMC_MIR_CALL_NEED_RUNTIME_DATA_HAS,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS,
        HAKO_LLVMC_MIR_CALL_NEED_ARRAY_STRING_INDEXOF,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING_LEN,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF,
        HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT,
        HAKO_LLVMC_MIR_CALL_NEED_PRINTF,
      };

      auto void apply_mir_call_need_kind(
          struct GenericPureNeedFlags* needs,
          enum MirCallNeedKind kind) {
        if (!needs) return;
        switch (kind) {
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH:
            needs->map_birth = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_SET:
            needs->map_set = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE:
            needs->map_size = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_GET:
            needs->map_get = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS:
            needs->map_has = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_RUNTIME_DATA_HAS:
            needs->runtime_data_has = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH:
            needs->arr_birth = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH:
            needs->arr_push = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN:
            needs->arr_len = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET:
            needs->arr_set = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET:
            needs->arr_get = 1;
            needs->arr_slot_load = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS:
            needs->arr_has = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_ARRAY_STRING_INDEXOF:
            needs->arr_string_indexof = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN:
            needs->string_len = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING:
            needs->string_substring = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING_LEN:
            needs->string_substring_len = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF:
            needs->string_indexof = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT:
            needs->string_concat = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_PRINTF:
            needs->printf = 1;
            break;
          case HAKO_LLVMC_MIR_CALL_NEED_NONE:
          default:
            break;
        }
      }

      auto enum MirCallNeedKind classify_mir_call_constructor_need_kind(
          const char* bname) {
        if (bname && !strcmp(bname, "MapBox")) {
          return HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH;
        }
        if (bname && !strcmp(bname, "ArrayBox")) {
          return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto int mir_call_need_route_read_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto enum MirCallNeedKind
      mir_call_need_kind_from_route_metadata_tokens(
          const char* route_id,
          const char* route_kind,
          const char* helper_symbol,
          const char* proof) {
        if (route_id &&
            !strcmp(route_id, "generic_method.has") &&
            route_kind &&
            !strcmp(route_kind, "runtime_data_contains_any") &&
            helper_symbol &&
            !strcmp(helper_symbol, "nyash.runtime_data.has_hh") &&
            proof &&
            !strcmp(proof, "has_surface_policy")) {
          return HAKO_LLVMC_MIR_CALL_NEED_RUNTIME_DATA_HAS;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind
      mir_call_need_kind_from_core_method_route_tokens(
          const char* route_id,
          const char* op,
          const char* route_kind,
          enum CoreMethodLoweringTierMetadata lowering_tier) {
        if (!(route_id && op && route_kind)) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        if (!strcmp(route_id, "generic_method.get") &&
            !strcmp(op, "MapGet") &&
            !strcmp(route_kind, "runtime_data_load_any") &&
            core_method_lowering_tier_is_cold_fallback(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_MAP_GET;
        }
        if (!strcmp(route_id, "generic_method.get") &&
            !strcmp(op, "MapGet") &&
            !strcmp(route_kind, "map_load_any") &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_MAP_GET;
        }
        if (!strcmp(route_id, "generic_method.get") &&
            !strcmp(op, "ArrayGet") &&
            !strcmp(route_kind, "array_slot_load_any") &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET;
        }
        if (!strcmp(route_id, "generic_method.has") &&
            !strcmp(op, "MapHas") &&
            (!strcmp(route_kind, "map_contains_any") ||
             !strcmp(route_kind, "map_contains_i64")) &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS;
        }
        if (!strcmp(route_id, "generic_method.has") &&
            !strcmp(op, "ArrayHas") &&
            !strcmp(route_kind, "array_contains_any") &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS;
        }
        if (!strcmp(route_id, "generic_method.len") &&
            !strcmp(op, "MapLen") &&
            !strcmp(route_kind, "map_entry_count") &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE;
        }
        if (!strcmp(route_id, "generic_method.len") &&
            !strcmp(op, "ArrayLen") &&
            !strcmp(route_kind, "array_slot_len") &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN;
        }
        if (!strcmp(route_id, "generic_method.push") &&
            !strcmp(op, "ArrayPush") &&
            !strcmp(route_kind, "array_append_any") &&
            core_method_lowering_tier_is_cold_fallback(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH;
        }
        if (!strcmp(route_id, "generic_method.set") &&
            !strcmp(op, "ArraySet") &&
            !strcmp(route_kind, "array_store_any") &&
            core_method_lowering_tier_is_cold_fallback(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET;
        }
        if (!strcmp(route_id, "generic_method.set") &&
            !strcmp(op, "MapSet") &&
            !strcmp(route_kind, "map_store_any") &&
            core_method_lowering_tier_is_cold_fallback(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_MAP_SET;
        }
        if (!strcmp(route_id, "generic_method.len") &&
            !strcmp(op, "StringLen") &&
            !strcmp(route_kind, "string_len") &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN;
        }
        if (!strcmp(route_id, "generic_method.substring") &&
            !strcmp(op, "StringSubstring") &&
            !strcmp(route_kind, "string_substring") &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING;
        }
        if (!strcmp(route_id, "generic_method.indexOf") &&
            !strcmp(op, "StringIndexOf") &&
            !strcmp(route_kind, "string_indexof") &&
            core_method_lowering_tier_is_warm_direct_abi(lowering_tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind
      mir_call_need_kind_from_lowering_plan_entry(yyjson_val* entry) {
        struct LoweringPlanGenericMethodView view;
        if (!read_lowering_plan_generic_method_view(entry, &view)) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        if (!strcmp(view.route_id, "generic_method.has") &&
            !strcmp(view.core_op, "MapHas") &&
            (!strcmp(view.route_kind, "map_contains_any") ||
             !strcmp(view.route_kind, "map_contains_i64")) &&
            view.symbol &&
            (!strcmp(view.symbol, "nyash.map.probe_hh") ||
             !strcmp(view.symbol, "nyash.map.probe_hi")) &&
            view.route_proof &&
            !strcmp(view.route_proof, "has_surface_policy") &&
            lowering_plan_tier_is_direct_abi(view.tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS;
        }
        if (!strcmp(view.route_id, "generic_method.len") &&
            !strcmp(view.core_op, "MapLen") &&
            !strcmp(view.route_kind, "map_entry_count") &&
            view.symbol &&
            !strcmp(view.symbol, "nyash.map.entry_count_i64") &&
            view.route_proof &&
            !strcmp(view.route_proof, "len_surface_policy") &&
            lowering_plan_tier_is_direct_abi(view.tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE;
        }
        if (!strcmp(view.route_id, "generic_method.len") &&
            !strcmp(view.core_op, "ArrayLen") &&
            !strcmp(view.route_kind, "array_slot_len") &&
            view.symbol &&
            !strcmp(view.symbol, "nyash.array.slot_len_h") &&
            view.route_proof &&
            !strcmp(view.route_proof, "len_surface_policy") &&
            lowering_plan_tier_is_direct_abi(view.tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN;
        }
        if (!strcmp(view.route_id, "generic_method.len") &&
            !strcmp(view.core_op, "StringLen") &&
            !strcmp(view.route_kind, "string_len") &&
            view.symbol &&
            !strcmp(view.symbol, "nyash.string.len_h") &&
            view.route_proof &&
            !strcmp(view.route_proof, "len_surface_policy") &&
            lowering_plan_tier_is_direct_abi(view.tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN;
        }
        if (!strcmp(view.route_id, "generic_method.get") &&
            !strcmp(view.core_op, "ArrayGet") &&
            !strcmp(view.route_kind, "array_slot_load_any") &&
            view.symbol &&
            !strcmp(view.symbol, "nyash.array.slot_load_hi") &&
            view.route_proof &&
            !strcmp(view.route_proof, "get_surface_policy") &&
            lowering_plan_tier_is_direct_abi(view.tier)) {
          return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind
      classify_mir_call_method_need_kind_from_core_method_metadata(
          long long bid,
          size_t ii) {
        yyjson_val* plan_entry = find_lowering_plan_entry_for_site(bid, ii);
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (!fn_metadata) return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        if (plan_entry) {
          enum MirCallNeedKind plan_need =
              mir_call_need_kind_from_lowering_plan_entry(plan_entry);
          if (plan_need != HAKO_LLVMC_MIR_CALL_NEED_NONE) {
            trace_lowering_plan_entry_hit(plan_entry, "mir_call_need");
            return plan_need;
          }
        }
        routes = yyjson_obj_get(fn_metadata, "generic_method_routes");
        if (!(routes && yyjson_is_arr(routes))) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
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
          if (!mir_call_need_route_read_i64_field(route, "block", &block) ||
              !mir_call_need_route_read_i64_field(
                  route,
                  "instruction_index",
                  &inst) ||
              block != bid ||
              (size_t)inst != ii) {
            continue;
          }
          {
            enum MirCallNeedKind route_need =
                mir_call_need_kind_from_route_metadata_tokens(
                    route_id, route_kind, helper_symbol, proof_raw);
            if (route_need != HAKO_LLVMC_MIR_CALL_NEED_NONE) {
              return route_need;
            }
          }
          core_method = yyjson_obj_get(route, "core_method");
          if (!(core_method && yyjson_is_obj(core_method))) {
            return HAKO_LLVMC_MIR_CALL_NEED_NONE;
          }
          op = read_str(core_method, "op");
          proof = read_core_method_proof_metadata(core_method, &invalid);
          lowering_tier = read_core_method_lowering_tier_metadata(core_method, &invalid);
          if (invalid ||
              !core_method_metadata_proof_is_contract_manifest(proof)) {
            return HAKO_LLVMC_MIR_CALL_NEED_NONE;
          }
          return mir_call_need_kind_from_core_method_route_tokens(
              route_id,
              op,
              route_kind,
              lowering_tier);
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind classify_mir_call_global_need_kind(
          const char* mname) {
        if (mname && !strcmp(mname, "print")) {
          return HAKO_LLVMC_MIR_CALL_NEED_PRINTF;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind classify_mir_call_extern_need_kind(
          const char* mname) {
        if (mname &&
            (!strcmp(mname, "nyash.string.concat_hh") ||
             !strcmp(mname, "nyash.string.concat3_hhh") ||
             !strcmp(mname, "nyash.string.insert_hsi"))) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT;
        }
        if (mname &&
            !strcmp(mname, "nyash.string.substring_hii")) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING;
        }
        if (mname &&
            (!strcmp(mname, "nyash.string.substring_concat_hhii") ||
             !strcmp(mname, "nyash.string.substring_concat3_hhhii"))) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING;
        }
        if (mname && !strcmp(mname, "nyash.string.substring_len_hii")) {
          return HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING_LEN;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }
