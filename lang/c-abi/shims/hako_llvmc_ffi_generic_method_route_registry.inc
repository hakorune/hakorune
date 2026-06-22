      /*
       * Generic-method route tuple registry.
       *
       * This is the shared row table for the generic-method route-policy,
       * emit-kind, and need-metadata consumers. Keep tuple rows here instead of
       * re-listing them in each consumer include.
       */
      struct HakoLlvmcGenericMethodRouteRegistryRow {
        const char* route_id;
        const char* core_op;
        const char* route_kind;
        const char* helper_symbol;
        const char* route_proof;
        enum LoweringPlanTierMetadata tier;
        int route_result;
        int emit_kind;
        int need_kind;
      };

      /*
       * Numeric result codes are used here so the shared registry does not need
       * the consumer-specific enum definitions to be in scope.
       *
       * MirCallGenericMethodRouteKind:
       *   0 fallback, 1 map_get, 2 map_size, 3 map_has, 4 array_len,
       *   5 array_get, 6 array_push, 7 array_string, 8 string,
       *   9 any_length, 10 string_indexof, 11 string_lastindexof,
       *   12 string_contains
       *
       * GenericMethodEmitKind:
       *   0 none, 1 set, 2 get, 3 len, 4 push, 5 substring, 6 has, 7 keys
       *
       * MirCallNeedKind:
       *   0 none, 1 map_birth, 2 map_set, 3 map_size, 4 map_get, 5 map_has,
       *   6 map_delete, 7 map_keys, 8 runtime_data_has, 9 array_birth,
       *   10 array_push, 11 array_len, 12 array_set, 13 array_get,
       *   14 array_has, 15 array_string_indexof, 16 string_len,
       *   17 string_substring, 18 string_substring_len, 19 string_indexof,
       *   20 string_contains, 21 string_concat, 22 any_length,
       *   23 any_handle_live, 24 env_get, 25 env_set, 26 env_now_ms,
       *   27 hako_atomic_slot_cas_i64, 28 hako_atomic_slot_fetch_add_i64,
       *   29 hako_atomic_slot_load_i64, 30 hako_atomic_slot_store_i64,
       *   31 hako_atomic_ptr_cas_ordered, 32 hako_atomic_ptr_load_ordered,
       *   33 hako_atomic_ptr_store_ordered, 34 hako_mem_alloc, 35 hako_mem_free,
       *   36 hako_osvm_reserve_bytes_i64, 37 hako_osvm_commit_bytes_i64,
       *   38 hako_osvm_decommit_bytes_i64, 39 hako_osvm_unreserve_bytes_i64,
       *   40 hako_tls_cache_slot_get_i64, 41 hako_tls_cache_slot_set_i64,
       *   42 hako_worker_current_id_i64, 43 hostbridge_extern_invoke_trap,
       *   44 stage1_emit_program_json, 45 stage1_emit_mir_from_source,
       *   46 stage1_emit_mir_from_program_json, 47 printf
       */
      static const struct HakoLlvmcGenericMethodRouteRegistryRow
          hako_llvmc_generic_method_route_registry_rows[] = {
              {
                  "generic_method.get",
                  "AnyGet",
                  "runtime_data_load_any",
                  "nyash.runtime_data.get_hh",
                  "get_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
                  1,
                  2,
                  4,
              },
              {
                  "generic_method.get",
                  "MapGet",
                  "runtime_data_load_any",
                  NULL,
                  NULL,
                  HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
                  1,
                  2,
                  4,
              },
              {
                  "generic_method.get",
                  "MapGet",
                  "map_load_scalar_i64",
                  "nyash.map.scalar_load_hi",
                  "map_set_scalar_i64_same_key_no_escape",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  1,
                  2,
                  4,
              },
              {
                  "generic_method.get",
                  "MapGet",
                  "map_load_scalar_i64",
                  "nyash.map.scalar_load_hi",
                  "map_set_scalar_i64_dominates_no_escape",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  1,
                  2,
                  4,
              },
              {
                  "generic_method.get",
                  "MapGet",
                  "map_load_scalar_i64",
                  "nyash.map.scalar_load_hi",
                  "map_set_scalar_i64_covered_dynamic_i64_key_no_escape",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  1,
                  2,
                  4,
              },
              {
                  "generic_method.get",
                  "MapGet",
                  "map_load_any",
                  "nyash.map.slot_load_hh",
                  "get_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  1,
                  2,
                  4,
              },
              {
                  "generic_method.get",
                  "ArrayGet",
                  "array_slot_load_any",
                  "nyash.array.slot_load_hi",
                  "get_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  5,
                  2,
                  13,
              },
              {
                  "generic_method.has",
                  "AnyHas",
                  "runtime_data_contains_any",
                  "nyash.runtime_data.has_hh",
                  "has_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  3,
                  6,
                  8,
              },
              {
                  "generic_method.has",
                  "MapHas",
                  "map_contains_any",
                  "nyash.map.probe_hh",
                  "has_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  3,
                  6,
                  5,
              },
              {
                  "generic_method.has",
                  "MapHas",
                  "map_contains_i64",
                  "nyash.map.probe_hi",
                  "has_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  3,
                  6,
                  5,
              },
              {
                  "generic_method.has",
                  "ArrayHas",
                  "array_contains_any",
                  "nyash.array.has_hh",
                  "has_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  3,
                  6,
                  14,
              },
              {
                  "generic_method.len",
                  "MapLen",
                  "map_entry_count",
                  "nyash.map.entry_count_i64",
                  "len_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  2,
                  3,
                  3,
              },
              {
                  "generic_method.len",
                  "ArrayLen",
                  "array_slot_len",
                  "nyash.array.slot_len_h",
                  "len_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  4,
                  3,
                  11,
              },
              {
                  "generic_method.len",
                  "AnyLen",
                  "any_length",
                  "nyash.any.length_h",
                  "len_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  9,
                  3,
                  22,
              },
              {
                  "generic_method.len",
                  "StringLen",
                  "string_len",
                  "nyash.string.len_fast_h",
                  "len_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  8,
                  3,
                  16,
              },
              {
                  "generic_method.push",
                  "ArrayPush",
                  "array_append_any",
                  "nyash.array.slot_append_hh",
                  "push_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
                  6,
                  4,
                  10,
              },
              {
                  "generic_method.keys",
                  "MapKeys",
                  "map_keys_array",
                  "nyash.map.keys_h",
                  "keys_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  0,
                  7,
                  7,
              },
              {
                  "generic_method.set",
                  "MapSet",
                  "map_store_i64",
                  "nyash.map.slot_store_hih",
                  "set_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
                  0,
                  1,
                  2,
              },
              {
                  "generic_method.set",
                  "MapSet",
                  "map_store_any",
                  "nyash.map.slot_store_hhh",
                  "set_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
                  0,
                  1,
                  2,
              },
              {
                  "generic_method.set",
                  "ArraySet",
                  "array_store_any",
                  "nyash.array.slot_store_hii",
                  "set_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
                  0,
                  1,
                  12,
              },
              {
                  "generic_method.set",
                  "ArraySet",
                  "array_store_any",
                  "nyash.array.slot_store_hih",
                  "set_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
                  0,
                  1,
                  12,
              },
              {
                  "generic_method.set",
                  "ArraySet",
                  "array_store_any",
                  "nyash.array.set_his",
                  "set_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
                  0,
                  1,
                  12,
              },
              {
                  "generic_method.delete",
                  "MapDelete",
                  "map_delete_any",
                  "nyash.map.delete_hh",
                  "delete_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
                  0,
                  0,
                  6,
              },
              {
                  "generic_method.substring",
                  "StringSubstring",
                  "string_substring",
                  "nyash.string.substring_hii",
                  "substring_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  8,
                  5,
                  17,
              },
              {
                  "generic_method.indexOf",
                  "StringIndexOf",
                  "string_indexof",
                  "nyash.string.indexOf_hh",
                  "indexof_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  10,
                  0,
                  19,
              },
              {
                  "generic_method.lastIndexOf",
                  "StringLastIndexOf",
                  "string_last_indexof",
                  "nyash.string.lastIndexOf_hh",
                  "lastindexof_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  11,
                  0,
                  19,
              },
              {
                  "generic_method.contains",
                  "StringContains",
                  "string_contains",
                  "nyash.string.contains_hh",
                  "contains_surface_policy",
                  HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
                  12,
                  0,
                  20,
              },
          };

      static const struct HakoLlvmcGenericMethodRouteRegistryRow*
      hako_llvmc_generic_method_route_registry_find_by_tuple(
          const char* route_id,
          const char* core_op,
          const char* route_kind,
          enum LoweringPlanTierMetadata tier) {
        size_t i = 0;
        for (i = 0;
             i < sizeof(hako_llvmc_generic_method_route_registry_rows) /
                     sizeof(hako_llvmc_generic_method_route_registry_rows[0]);
             i++) {
          const struct HakoLlvmcGenericMethodRouteRegistryRow* row =
              &hako_llvmc_generic_method_route_registry_rows[i];
          if (row->route_id && row->core_op && row->route_kind &&
              route_id && core_op && route_kind &&
              !strcmp(row->route_id, route_id) &&
              !strcmp(row->core_op, core_op) &&
              !strcmp(row->route_kind, route_kind) &&
              row->tier == tier) {
            return row;
          }
        }
        return NULL;
      }

      static const struct HakoLlvmcGenericMethodRouteRegistryRow*
      hako_llvmc_generic_method_route_registry_find_by_route_tokens(
          const char* route_id,
          const char* route_kind,
          const char* helper_symbol,
          const char* route_proof) {
        size_t i = 0;
        for (i = 0;
             i < sizeof(hako_llvmc_generic_method_route_registry_rows) /
                     sizeof(hako_llvmc_generic_method_route_registry_rows[0]);
             i++) {
          const struct HakoLlvmcGenericMethodRouteRegistryRow* row =
              &hako_llvmc_generic_method_route_registry_rows[i];
          if (row->route_id && row->route_kind && row->helper_symbol &&
              row->route_proof && route_id && route_kind && helper_symbol &&
              route_proof &&
              !strcmp(row->route_id, route_id) &&
              !strcmp(row->route_kind, route_kind) &&
              !strcmp(row->helper_symbol, helper_symbol) &&
              !strcmp(row->route_proof, route_proof)) {
            return row;
          }
        }
        return NULL;
      }
