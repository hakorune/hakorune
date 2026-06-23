      /*
       * MIR call declaration need metadata rules.
       *
       * These helpers consume MIR-owned route metadata and LoweringPlan rows.
       * Site scanning and name fallback stay outside this file.
       */
      auto enum MirCallNeedKind
      mir_call_need_kind_from_route_metadata_tokens(
          const char* route_id,
          const char* route_kind,
          const char* helper_symbol,
          const char* proof) {
        const struct HakoLlvmcGenericMethodRouteRegistryRow* row = NULL;
        if (!(route_id && route_kind && helper_symbol && proof)) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        row = hako_llvmc_generic_method_route_registry_find_by_route_tokens(
            route_id, route_kind, helper_symbol, proof);
        if (row && row->need_kind != 0) {
          return (enum MirCallNeedKind)row->need_kind;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind
      mir_call_need_kind_from_core_method_route_tokens(
          const char* route_id,
          const char* op,
          const char* route_kind,
          enum CoreMethodLoweringTierMetadata lowering_tier) {
        const struct HakoLlvmcGenericMethodRouteRegistryRow* row = NULL;
        (void)lowering_tier;
        if (!(route_id && op && route_kind)) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        row = hako_llvmc_generic_method_route_registry_find_by_tuple(
            route_id, op, route_kind, lowering_tier);
        if (row && row->need_kind != 0) {
          return (enum MirCallNeedKind)row->need_kind;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind
      mir_call_generic_method_need_kind_from_lowering_plan_entry(yyjson_val* entry) {
        struct LoweringPlanGenericMethodView view;
        const struct HakoLlvmcGenericMethodRouteRegistryRow* row = NULL;
        if (!read_lowering_plan_generic_method_view(entry, &view)) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        row = hako_llvmc_generic_method_route_registry_find_by_tuple(
            view.route_id, view.core_op, view.route_kind, view.tier);
        if (row && row->need_kind != 0) {
          return (enum MirCallNeedKind)row->need_kind;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind
      mir_call_constructor_need_kind_from_lowering_plan_entry(yyjson_val* entry) {
        struct LoweringPlanConstructorCallView view;
        if (!read_lowering_plan_constructor_call_view(entry, &view)) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        if (view.need_kind && !strcmp(view.need_kind, "map_birth")) {
          return HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH;
        }
        if (view.need_kind && !strcmp(view.need_kind, "array_birth")) {
          return HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH;
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
            "extern.env.now_ms",
            "EnvNowMs",
            "extern.env.now_ms",
            "nyash.env.now_ms",
            HAKO_LLVMC_MIR_CALL_NEED_ENV_NOW_MS,
          },
          {
            "extern.string.concat_hh",
            "StringConcat",
            "extern.string.concat_hh",
            "nyash.string.concat_hh",
            HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT,
          },
          {
            "extern.string.concat3_hhh",
            "StringConcat3",
            "extern.string.concat3_hhh",
            "nyash.string.concat3_hhh",
            HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT,
          },
          {
            "extern.string.insert_hsi",
            "StringInsert",
            "extern.string.insert_hsi",
            "nyash.string.insert_hsi",
            HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT,
          },
          {
            "extern.string.substring_hii",
            "StringSubstring",
            "extern.string.substring_hii",
            "nyash.string.substring_hii",
            HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING,
          },
          {
            "extern.string.substring_concat_hhii",
            "StringSubstringConcat",
            "extern.string.substring_concat_hhii",
            "nyash.string.substring_concat_hhii",
            HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING,
          },
          {
            "extern.string.substring_concat3_hhhii",
            "StringSubstringConcat3",
            "extern.string.substring_concat3_hhhii",
            "nyash.string.substring_concat3_hhhii",
            HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING,
          },
          {
            "extern.string.substring_len_hii",
            "StringSubstringLen",
            "extern.string.substring_len_hii",
            "nyash.string.substring_len_hii",
            HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING_LEN,
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
            "extern.hako_atomic.slot_cas_i64",
            "HakoAtomicSlotCasI64",
            "extern.hako_atomic.slot_cas_i64",
            "hako_atomic_slot_cas_i64",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_CAS_I64,
          },
          {
            "extern.hako_atomic.slot_fetch_add_i64",
            "HakoAtomicSlotFetchAddI64",
            "extern.hako_atomic.slot_fetch_add_i64",
            "hako_atomic_slot_fetch_add_i64",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_FETCH_ADD_I64,
          },
          {
            "extern.hako_atomic.slot_load_i64",
            "HakoAtomicSlotLoadI64",
            "extern.hako_atomic.slot_load_i64",
            "hako_atomic_slot_load_i64",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_LOAD_I64,
          },
          {
            "extern.hako_atomic.slot_store_i64",
            "HakoAtomicSlotStoreI64",
            "extern.hako_atomic.slot_store_i64",
            "hako_atomic_slot_store_i64",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_SLOT_STORE_I64,
          },
          {
            "extern.hako_atomic.ptr_cas_ordered",
            "HakoAtomicPtrCasOrdered",
            "extern.hako_atomic.ptr_cas_ordered",
            "hako_atomic_ptr_cas_ordered",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_PTR_CAS_ORDERED,
          },
          {
            "extern.hako_atomic.ptr_load_ordered",
            "HakoAtomicPtrLoadOrdered",
            "extern.hako_atomic.ptr_load_ordered",
            "hako_atomic_ptr_load_ordered",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_PTR_LOAD_ORDERED,
          },
          {
            "extern.hako_atomic.ptr_store_ordered",
            "HakoAtomicPtrStoreOrdered",
            "extern.hako_atomic.ptr_store_ordered",
            "hako_atomic_ptr_store_ordered",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_ATOMIC_PTR_STORE_ORDERED,
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
            "extern.hako_osvm.unreserve_bytes_i64",
            "HakoOsvmUnreserveBytesI64",
            "extern.hako_osvm.unreserve_bytes_i64",
            "hako_osvm_unreserve_bytes_i64",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_OSVM_UNRESERVE_BYTES_I64,
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
            "extern.hako_worker.current_id_i64",
            "HakoWorkerCurrentIdI64",
            "extern.hako_worker.current_id_i64",
            "hako_worker_current_id_i64",
            HAKO_LLVMC_MIR_CALL_NEED_HAKO_WORKER_CURRENT_ID_I64,
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
