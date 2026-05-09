      /*
       * MIR call declaration need metadata rules.
       *
       * These helpers consume MIR-owned route metadata and LoweringPlan rows.
       * Site scanning and name fallback stay outside this file.
       */
      struct MirCallRouteMetadataNeedRule {
        const char* route_id;
        const char* route_kind;
        const char* helper_symbol;
        const char* proof;
        enum MirCallNeedKind need_kind;
      };

      auto int mir_call_route_metadata_need_rule_matches(
          const char* route_id,
          const char* route_kind,
          const char* helper_symbol,
          const char* proof,
          const struct MirCallRouteMetadataNeedRule* rule) {
        if (!(route_id && route_kind && helper_symbol && proof && rule &&
              rule->route_id && rule->route_kind && rule->helper_symbol &&
              rule->proof)) {
          return 0;
        }
        return !strcmp(route_id, rule->route_id) &&
               !strcmp(route_kind, rule->route_kind) &&
               !strcmp(helper_symbol, rule->helper_symbol) &&
               !strcmp(proof, rule->proof);
      }

      enum MirCallNeedTierGuard {
        HAKO_LLVMC_MIR_CALL_NEED_TIER_ANY = 0,
        HAKO_LLVMC_MIR_CALL_NEED_TIER_COLD_FALLBACK,
        HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
      };

      struct MirCallCoreMethodNeedRule {
        const char* route_id;
        const char* op;
        const char* route_kind;
        enum MirCallNeedTierGuard tier_guard;
        enum MirCallNeedKind need_kind;
      };

      auto int mir_call_need_tier_guard_matches(
          enum CoreMethodLoweringTierMetadata lowering_tier,
          enum MirCallNeedTierGuard tier_guard) {
        switch (tier_guard) {
          case HAKO_LLVMC_MIR_CALL_NEED_TIER_COLD_FALLBACK:
            return core_method_lowering_tier_is_cold_fallback(lowering_tier);
          case HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI:
            return core_method_lowering_tier_is_warm_direct_abi(lowering_tier);
          case HAKO_LLVMC_MIR_CALL_NEED_TIER_ANY:
          default:
            return 1;
        }
      }

      auto int mir_call_core_method_need_rule_matches(
          const char* route_id,
          const char* op,
          const char* route_kind,
          enum CoreMethodLoweringTierMetadata lowering_tier,
          const struct MirCallCoreMethodNeedRule* rule) {
        if (!(route_id && op && route_kind && rule && rule->route_id &&
              rule->op && rule->route_kind)) {
          return 0;
        }
        return !strcmp(route_id, rule->route_id) &&
               !strcmp(op, rule->op) &&
               !strcmp(route_kind, rule->route_kind) &&
               mir_call_need_tier_guard_matches(lowering_tier, rule->tier_guard);
      }

      auto enum MirCallNeedKind
      mir_call_need_kind_from_route_metadata_tokens(
          const char* route_id,
          const char* route_kind,
          const char* helper_symbol,
          const char* proof) {
        static const struct MirCallRouteMetadataNeedRule rules[] = {
          {
            "generic_method.has",
            "runtime_data_contains_any",
            "nyash.runtime_data.has_hh",
            "has_surface_policy",
            HAKO_LLVMC_MIR_CALL_NEED_RUNTIME_DATA_HAS,
          },
          {
            "generic_method.keys",
            "map_keys_array",
            "nyash.map.keys_h",
            "mir_json_flags_keys",
            HAKO_LLVMC_MIR_CALL_NEED_MAP_KEYS,
          },
          {
            "generic_method.keys",
            "map_keys_array",
            "nyash.map.keys_h",
            "keys_surface_policy",
            HAKO_LLVMC_MIR_CALL_NEED_MAP_KEYS,
          },
        };
        size_t ri = 0;
        for (ri = 0; ri < ARR_LEN(rules); ri++) {
          if (mir_call_route_metadata_need_rule_matches(
                  route_id, route_kind, helper_symbol, proof, &rules[ri])) {
            return rules[ri].need_kind;
          }
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind
      mir_call_need_kind_from_core_method_route_tokens(
          const char* route_id,
          const char* op,
          const char* route_kind,
          enum CoreMethodLoweringTierMetadata lowering_tier) {
        static const struct MirCallCoreMethodNeedRule rules[] = {
          {"generic_method.get", "MapGet", "runtime_data_load_any",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_COLD_FALLBACK,
           HAKO_LLVMC_MIR_CALL_NEED_MAP_GET},
          {"generic_method.get", "MapGet", "map_load_any",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_MAP_GET},
          {"generic_method.get", "ArrayGet", "array_slot_load_any",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET},
          {"generic_method.has", "MapHas", "map_contains_any",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS},
          {"generic_method.has", "MapHas", "map_contains_i64",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS},
          {"generic_method.has", "ArrayHas", "array_contains_any",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS},
          {"generic_method.delete", "MapDelete", "map_delete_any",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_COLD_FALLBACK,
           HAKO_LLVMC_MIR_CALL_NEED_MAP_DELETE},
          {"generic_method.len", "MapLen", "map_entry_count",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE},
          {"generic_method.keys", "MapKeys", "map_keys_array",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_MAP_KEYS},
          {"generic_method.len", "ArrayLen", "array_slot_len",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN},
          {"generic_method.push", "ArrayPush", "array_append_any",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_COLD_FALLBACK,
           HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH},
          {"generic_method.set", "ArraySet", "array_store_any",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_COLD_FALLBACK,
           HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET},
          {"generic_method.set", "MapSet", "map_store_any",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_COLD_FALLBACK,
           HAKO_LLVMC_MIR_CALL_NEED_MAP_SET},
          {"generic_method.len", "StringLen", "string_len",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN},
          {"generic_method.substring", "StringSubstring", "string_substring",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING},
          {"generic_method.indexOf", "StringIndexOf", "string_indexof",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF},
          {"generic_method.lastIndexOf", "StringLastIndexOf", "string_last_indexof",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF},
          {"generic_method.contains", "StringContains", "string_contains",
           HAKO_LLVMC_MIR_CALL_NEED_TIER_WARM_DIRECT_ABI,
           HAKO_LLVMC_MIR_CALL_NEED_STRING_CONTAINS},
        };
        size_t ri = 0;
        for (ri = 0; ri < ARR_LEN(rules); ri++) {
          if (mir_call_core_method_need_rule_matches(
                  route_id, op, route_kind, lowering_tier, &rules[ri])) {
            return rules[ri].need_kind;
          }
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      struct LoweringPlanGenericMethodNeedRule {
        const char* route_id;
        const char* core_op;
        const char* route_kind;
        const char* symbol;
        const char* route_proof;
        enum LoweringPlanTierMetadata tier;
        enum MirCallNeedKind need_kind;
      };

      auto int lowering_plan_generic_method_need_rule_matches(
          const struct LoweringPlanGenericMethodView* view,
          const struct LoweringPlanGenericMethodNeedRule* rule) {
        if (!(view && rule)) return 0;
        return lowering_plan_generic_method_view_matches_symbolic_route(
            view,
            rule->route_id,
            rule->core_op,
            rule->route_kind,
            rule->symbol,
            rule->route_proof,
            rule->tier);
      }

      auto enum MirCallNeedKind
      mir_call_generic_method_need_kind_from_lowering_plan_entry(yyjson_val* entry) {
        static const struct LoweringPlanGenericMethodNeedRule rules[] = {
          {
            "generic_method.has",
            "MapHas",
            "map_contains_any",
            "nyash.map.probe_hh",
            "has_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS,
          },
          {
            "generic_method.has",
            "MapHas",
            "map_contains_i64",
            "nyash.map.probe_hi",
            "has_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_MAP_HAS,
          },
          {
            "generic_method.has",
            "ArrayHas",
            "array_contains_any",
            "nyash.array.has_hh",
            "has_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_ARRAY_HAS,
          },
          {
            "generic_method.len",
            "MapLen",
            "map_entry_count",
            "nyash.map.entry_count_i64",
            "len_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_MAP_SIZE,
          },
          {
            "generic_method.len",
            "ArrayLen",
            "array_slot_len",
            "nyash.array.slot_len_h",
            "len_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN,
          },
          {
            "generic_method.len",
            "StringLen",
            "string_len",
            "nyash.string.len_fast_h",
            "len_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_STRING_LEN,
          },
          {
            "generic_method.keys",
            "MapKeys",
            "map_keys_array",
            "nyash.map.keys_h",
            "keys_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_MAP_KEYS,
          },
          {
            "generic_method.get",
            "ArrayGet",
            "array_slot_load_any",
            "nyash.array.slot_load_hi",
            "get_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET,
          },
          {
            "generic_method.get",
            "MapGet",
            "map_load_any",
            "nyash.map.slot_load_hh",
            "get_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_MAP_GET,
          },
          {
            "generic_method.push",
            "ArrayPush",
            "array_append_any",
            "nyash.array.slot_append_hh",
            "push_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
            HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH,
          },
          {
            "generic_method.set",
            "MapSet",
            "map_store_any",
            "nyash.map.slot_store_hhh",
            "set_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
            HAKO_LLVMC_MIR_CALL_NEED_MAP_SET,
          },
          {
            "generic_method.set",
            "ArraySet",
            "array_store_any",
            "nyash.array.slot_store_hii",
            "set_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
            HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET,
          },
          {
            "generic_method.set",
            "ArraySet",
            "array_store_any",
            "nyash.array.slot_store_hih",
            "set_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
            HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET,
          },
          {
            "generic_method.set",
            "ArraySet",
            "array_store_any",
            "nyash.array.set_his",
            "set_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
            HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET,
          },
          {
            "generic_method.delete",
            "MapDelete",
            "map_delete_any",
            "nyash.map.delete_hh",
            "delete_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
            HAKO_LLVMC_MIR_CALL_NEED_MAP_DELETE,
          },
          {
            "generic_method.substring",
            "StringSubstring",
            "string_substring",
            "nyash.string.substring_hii",
            "substring_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING,
          },
          {
            "generic_method.indexOf",
            "StringIndexOf",
            "string_indexof",
            "nyash.string.indexOf_hh",
            "indexof_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF,
          },
          {
            "generic_method.lastIndexOf",
            "StringLastIndexOf",
            "string_last_indexof",
            "nyash.string.lastIndexOf_hh",
            "lastindexof_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_STRING_INDEXOF,
          },
          {
            "generic_method.contains",
            "StringContains",
            "string_contains",
            "nyash.string.contains_hh",
            "contains_surface_policy",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
            HAKO_LLVMC_MIR_CALL_NEED_STRING_CONTAINS,
          },
        };
        struct LoweringPlanGenericMethodView view;
        size_t ri = 0;
        if (!read_lowering_plan_generic_method_view(entry, &view)) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        for (ri = 0; ri < ARR_LEN(rules); ri++) {
          if (lowering_plan_generic_method_need_rule_matches(&view, &rules[ri])) {
            return rules[ri].need_kind;
          }
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      struct LoweringPlanExternNeedRule {
        const char* route_id;
        const char* core_op;
        const char* route_kind;
        const char* symbol;
        enum MirCallNeedKind need_kind;
      };

      auto int lowering_plan_extern_need_rule_matches(
          const struct LoweringPlanExternCallView* view,
          const struct LoweringPlanExternNeedRule* rule) {
        if (!(view && rule)) return 0;
        return lowering_plan_extern_call_view_matches_runtime_route(
            view,
            rule->route_id,
            rule->core_op,
            rule->route_kind,
            rule->symbol,
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME);
      }

      auto enum MirCallNeedKind
      mir_call_extern_need_kind_from_lowering_plan_entry(yyjson_val* entry) {
        static const struct LoweringPlanExternNeedRule rules[] = {
          {
            "extern.env.get",
            "EnvGet",
            "extern.env.get",
            "nyash.env.get",
            HAKO_LLVMC_MIR_CALL_NEED_ENV_GET,
          },
          {
            "extern.env.set",
            "EnvSet",
            "extern.env.set",
            "nyash.env.set",
            HAKO_LLVMC_MIR_CALL_NEED_ENV_SET,
          },
          {
            "extern.any.handle_live",
            "AnyHandleLive",
            "extern.any.handle_live",
            "nyash.any.handle_live_h",
            HAKO_LLVMC_MIR_CALL_NEED_ANY_HANDLE_LIVE,
          },
          {
            "extern.array.slot_append_any",
            "ArraySlotAppendAny",
            "extern.array.slot_append_any",
            "nyash.array.slot_append_hh",
            HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH,
          },
          {
            "extern.array.slot_len_i64",
            "ArraySlotLenI64",
            "extern.array.slot_len_i64",
            "nyash.array.slot_len_h",
            HAKO_LLVMC_MIR_CALL_NEED_ARRAY_LEN,
          },
          {
            "extern.array.slot_load_i64",
            "ArraySlotLoadI64",
            "extern.array.slot_load_i64",
            "nyash.array.slot_load_hi",
            HAKO_LLVMC_MIR_CALL_NEED_ARRAY_GET,
          },
          {
            "extern.array.slot_store_i64",
            "ArraySlotStoreI64",
            "extern.array.slot_store_i64",
            "nyash.array.slot_store_hii",
            HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET,
          },
          {
            "extern.hako_mem.alloc",
            "HakoMemAlloc",
            "extern.hako_mem.alloc",
            "hako_mem_alloc",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_MEM_ALLOC,
          },
          {
            "extern.hako_mem.free",
            "HakoMemFree",
            "extern.hako_mem.free",
            "hako_mem_free",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_MEM_FREE,
          },
          {
            "extern.hako_osvm.reserve_bytes_i64",
            "HakoOsvmReserveBytesI64",
            "extern.hako_osvm.reserve_bytes_i64",
            "hako_osvm_reserve_bytes_i64",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_RESERVE_BYTES_I64,
          },
          {
            "extern.hako_osvm.commit_bytes_i64",
            "HakoOsvmCommitBytesI64",
            "extern.hako_osvm.commit_bytes_i64",
            "hako_osvm_commit_bytes_i64",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_COMMIT_BYTES_I64,
          },
          {
            "extern.hako_osvm.decommit_bytes_i64",
            "HakoOsvmDecommitBytesI64",
            "extern.hako_osvm.decommit_bytes_i64",
            "hako_osvm_decommit_bytes_i64",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_DECOMMIT_BYTES_I64,
          },
          {
            "extern.hako_tls.cache_slot_get_i64",
            "HakoTlsCacheSlotGetI64",
            "extern.hako_tls.cache_slot_get_i64",
            "hako_tls_cache_slot_get_i64",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_TLS_CACHE_SLOT_GET_I64,
          },
          {
            "extern.hako_tls.cache_slot_set_i64",
            "HakoTlsCacheSlotSetI64",
            "extern.hako_tls.cache_slot_set_i64",
            "hako_tls_cache_slot_set_i64",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_TLS_CACHE_SLOT_SET_I64,
          },
          {
            "extern.hostbridge.extern_invoke",
            "HostBridgeExternInvoke",
            "extern.hostbridge.extern_invoke",
            "nyash.hostbridge.extern_invoke",
            HAKO_LLVMC_MIR_CALL_NEED_HOSTBRIDGE_EXTERN_INVOKE_TRAP,
          },
          {
            "extern.stage1.emit_program_json_v0",
            "Stage1EmitProgramJson",
            "extern.stage1.emit_program_json_v0",
            "nyash.stage1.emit_program_json_v0_h",
            HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_PROGRAM_JSON,
          },
          {
            "extern.stage1.emit_mir_from_source_v0",
            "Stage1EmitMirFromSource",
            "extern.stage1.emit_mir_from_source_v0",
            "nyash.stage1.emit_mir_from_source_v0_h",
            HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_MIR_FROM_SOURCE,
          },
          {
            "extern.stage1.emit_mir_from_program_json_v0",
            "Stage1EmitMirFromProgramJson",
            "extern.stage1.emit_mir_from_program_json_v0",
            "nyash.stage1.emit_mir_from_program_json_v0_h",
            HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_MIR_FROM_PROGRAM_JSON,
          },
        };
        struct LoweringPlanExternCallView view;
        size_t ri = 0;
        if (!read_lowering_plan_extern_call_view(entry, &view)) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        for (ri = 0; ri < ARR_LEN(rules); ri++) {
          if (lowering_plan_extern_need_rule_matches(&view, &rules[ri])) {
            return rules[ri].need_kind;
          }
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }
