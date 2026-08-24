      enum MirCallExternEmitBehavior {
        HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL = 0,
        HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR_RETURN_I64,
        HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR_ARG_VOID,
        HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR1_I64_ARG_RETURN_I64,
        HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR3_I64_I64_ARG_RETURN_I64,
        HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR2_I64_ARG_CALL,
        HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_TRAP_SENTINEL,
      };

      struct MirCallExternEmitRule {
        const char* route_id;
        const char* core_op;
        const char* route_kind;
        const char* symbol;
        const char* return_shape;
        const char* value_demand;
        long long arity;
        int read_value_value;
        int require_nonzero_method_arg;
        int set_string_origin;
        const char* trace_consumer;
        enum MirCallExternEmitBehavior behavior;
      };

      auto int lowering_plan_extern_emit_rule_matches(
          const struct LoweringPlanExternCallView* view,
          const struct MirCallExternEmitRule* rule) {
        if (!(view && rule)) return 0;
        return lowering_plan_extern_call_view_is_valid_with(
            view,
            rule->route_id,
            rule->core_op,
            rule->route_kind,
            rule->symbol,
            rule->return_shape,
            rule->value_demand);
      }

      auto const struct MirCallExternEmitRule* mir_call_extern_emit_rule_table(
          size_t* count) {
        static const struct MirCallExternEmitRule rules[] = {
          {"extern.env.get",
           "EnvGet",
           "extern.env.get",
           "nyash.env.get",
           "string_handle_or_null",
           "runtime_i64_or_handle",
           1,
           0,
           0,
           1,
           "mir_call_extern_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.env.set",
           "EnvSet",
           "extern.env.set",
           "nyash.env.set",
           "scalar_i64",
           "runtime_i64",
           2,
           1,
           0,
           0,
           "mir_call_extern_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.env.now_ms",
           "EnvNowMs",
           "extern.env.now_ms",
           "nyash.env.now_ms",
           "scalar_i64",
           "runtime_i64",
           0,
           0,
           0,
           0,
           "mir_call_env_now_ms_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.any.handle_live",
           "AnyHandleLive",
           "extern.any.handle_live",
           "nyash.any.handle_live_h",
           "scalar_i64",
           "runtime_i64",
           1,
           0,
           0,
           0,
           "mir_call_any_handle_live_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.array.slot_append_any",
           "ArraySlotAppendAny",
           "extern.array.slot_append_any",
           "nyash.array.slot_append_hh",
           "scalar_i64",
           "runtime_i64",
           2,
           1,
           0,
           0,
           "mir_call_array_slot_append_any_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.array.slot_len_i64",
           "ArraySlotLenI64",
           "extern.array.slot_len_i64",
           "nyash.array.slot_len_h",
           "scalar_i64",
           "runtime_i64",
           1,
           0,
           0,
           0,
           "mir_call_array_slot_len_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.array.slot_load_i64",
           "ArraySlotLoadI64",
           "extern.array.slot_load_i64",
           "nyash.array.slot_load_hi",
           "scalar_i64",
           "runtime_i64",
           2,
           1,
           0,
           0,
           "mir_call_array_slot_load_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.array.slot_store_i64",
           "ArraySlotStoreI64",
           "extern.array.slot_store_i64",
           "nyash.array.slot_store_hii",
           "scalar_i64",
           "runtime_i64",
           3,
           1,
           0,
           0,
           "mir_call_array_slot_store_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hako_mem.alloc",
           "HakoMemAlloc",
           "extern.hako_mem.alloc",
           "hako_mem_alloc",
           "native_ptr_nullable",
           "native_ptr_nullable",
           1,
           0,
           0,
           0,
           "mir_call_hako_mem_alloc_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR_RETURN_I64},
          {"extern.hako_mem.free",
           "HakoMemFree",
           "extern.hako_mem.free",
           "hako_mem_free",
           "c_void",
           "no_payload",
           1,
           0,
           0,
           0,
           "mir_call_hako_mem_free_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR_ARG_VOID},
          {"extern.hako_atomic.slot_cas_i64",
           "HakoAtomicSlotCasI64",
           "extern.hako_atomic.slot_cas_i64",
           "hako_atomic_slot_cas_i64",
           "scalar_i64",
           "runtime_i64",
           3,
           1,
           0,
           0,
           "mir_call_hako_atomic_slot_cas_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hako_atomic.slot_fetch_add_i64",
           "HakoAtomicSlotFetchAddI64",
           "extern.hako_atomic.slot_fetch_add_i64",
           "hako_atomic_slot_fetch_add_i64",
           "scalar_i64",
           "runtime_i64",
           2,
           1,
           0,
           0,
           "mir_call_hako_atomic_slot_fetch_add_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hako_atomic.slot_load_i64",
           "HakoAtomicSlotLoadI64",
           "extern.hako_atomic.slot_load_i64",
           "hako_atomic_slot_load_i64",
           "scalar_i64",
           "runtime_i64",
           1,
           0,
           0,
           0,
           "mir_call_hako_atomic_slot_load_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hako_atomic.slot_store_i64",
           "HakoAtomicSlotStoreI64",
           "extern.hako_atomic.slot_store_i64",
           "hako_atomic_slot_store_i64",
           "scalar_i64",
           "runtime_i64",
           2,
           1,
           0,
           0,
           "mir_call_hako_atomic_slot_store_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hako_atomic.ptr_store_ordered",
           "HakoAtomicPtrStoreOrdered",
           "extern.hako_atomic.ptr_store_ordered",
           "hako_atomic_ptr_store_ordered",
           "scalar_i64",
           "native_ptr_nullable",
           3,
           0,
           0,
           0,
           "mir_call_hako_atomic_ptr_store_ordered_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR2_I64_ARG_CALL},
          {"extern.hako_atomic.ptr_cas_ordered",
           "HakoAtomicPtrCasOrdered",
           "extern.hako_atomic.ptr_cas_ordered",
           "hako_atomic_ptr_cas_ordered",
           "native_ptr_nullable",
           "native_ptr_nullable",
           5,
           0,
           0,
           0,
           "mir_call_hako_atomic_ptr_cas_ordered_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR3_I64_I64_ARG_RETURN_I64},
          {"extern.hako_atomic.ptr_load_ordered",
           "HakoAtomicPtrLoadOrdered",
           "extern.hako_atomic.ptr_load_ordered",
           "hako_atomic_ptr_load_ordered",
           "native_ptr_nullable",
           "native_ptr_nullable",
           2,
           0,
           0,
           0,
           "mir_call_hako_atomic_ptr_load_ordered_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR1_I64_ARG_RETURN_I64},
          {"extern.hako_osvm.reserve_bytes_i64",
           "HakoOsvmReserveBytesI64",
           "extern.hako_osvm.reserve_bytes_i64",
           "hako_osvm_reserve_bytes_i64",
           "native_ptr_nullable",
           "native_ptr_nullable",
           1,
           0,
           0,
           0,
           "mir_call_hako_osvm_reserve_bytes_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hako_osvm.commit_bytes_i64",
           "HakoOsvmCommitBytesI64",
           "extern.hako_osvm.commit_bytes_i64",
           "hako_osvm_commit_bytes_i64",
           "scalar_i64",
           "runtime_i64",
           2,
           1,
           0,
           0,
           "mir_call_hako_osvm_commit_bytes_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hako_osvm.decommit_bytes_i64",
           "HakoOsvmDecommitBytesI64",
           "extern.hako_osvm.decommit_bytes_i64",
           "hako_osvm_decommit_bytes_i64",
           "scalar_i64",
           "runtime_i64",
           2,
           1,
           0,
           0,
           "mir_call_hako_osvm_decommit_bytes_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hako_osvm.unreserve_bytes_i64",
           "HakoOsvmUnreserveBytesI64",
           "extern.hako_osvm.unreserve_bytes_i64",
           "hako_osvm_unreserve_bytes_i64",
           "scalar_i64",
           "runtime_i64",
           2,
           1,
           0,
           0,
           "mir_call_hako_osvm_unreserve_bytes_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hako_tls.cache_slot_get_i64",
           "HakoTlsCacheSlotGetI64",
           "extern.hako_tls.cache_slot_get_i64",
           "hako_tls_cache_slot_get_i64",
           "scalar_i64",
           "runtime_i64",
           1,
           0,
           0,
           0,
           "mir_call_hako_tls_cache_slot_get_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hako_tls.cache_slot_set_i64",
           "HakoTlsCacheSlotSetI64",
           "extern.hako_tls.cache_slot_set_i64",
           "hako_tls_cache_slot_set_i64",
           "scalar_i64",
           "runtime_i64",
           2,
           1,
           0,
           0,
           "mir_call_hako_tls_cache_slot_set_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hako_worker.current_id_i64",
           "HakoWorkerCurrentIdI64",
           "extern.hako_worker.current_id_i64",
           "hako_worker_current_id_i64",
           "scalar_i64",
           "runtime_i64",
           1,
           0,
           0,
           0,
           "mir_call_hako_worker_current_id_i64_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.hostbridge.extern_invoke",
           "HostBridgeExternInvoke",
           "extern.hostbridge.extern_invoke",
           "nyash.hostbridge.extern_invoke",
           "string_handle_or_null",
           "runtime_i64_or_handle",
           3,
           1,
           1,
           1,
           "mir_call_hostbridge_extern_invoke_trap_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_TRAP_SENTINEL},
          {"extern.stage1.emit_program_json_v0",
           "Stage1EmitProgramJson",
           "extern.stage1.emit_program_json_v0",
           "nyash.stage1.emit_program_json_v0_h",
           "string_handle",
           "runtime_i64_or_handle",
           1,
           0,
           0,
           1,
           "mir_call_stage1_emit_program_json_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.stage1.emit_mir_from_source_v0",
           "Stage1EmitMirFromSource",
           "extern.stage1.emit_mir_from_source_v0",
           "nyash.stage1.emit_mir_from_source_v0_h",
           "string_handle",
           "runtime_i64_or_handle",
           1,
           0,
           0,
           1,
           "mir_call_stage1_emit_mir_from_source_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
          {"extern.stage1.emit_mir_from_program_json_v0",
           "Stage1EmitMirFromProgramJson",
           "extern.stage1.emit_mir_from_program_json_v0",
           "nyash.stage1.emit_mir_from_program_json_v0_h",
           "string_handle",
           "runtime_i64_or_handle",
           1,
           0,
           0,
           1,
           "mir_call_stage1_emit_mir_from_program_json_emit",
           HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_CALL},
        };
        if (count) *count = ARR_LEN(rules);
        return rules;
      }

      auto const struct MirCallExternEmitRule* find_mir_call_extern_emit_rule(
          const struct LoweringPlanExternCallView* view) {
        size_t count = 0;
        const struct MirCallExternEmitRule* rules =
            mir_call_extern_emit_rule_table(&count);
        size_t ri = 0;
        for (ri = 0; ri < count; ri++) {
          if (lowering_plan_extern_emit_rule_matches(view, &rules[ri])) {
            return &rules[ri];
          }
        }
        return NULL;
      }

      auto const struct MirCallExternEmitRule*
      find_mir_call_extern_emit_rule_by_route_id(const char* route_id) {
        size_t count = 0;
        const struct MirCallExternEmitRule* rules =
            mir_call_extern_emit_rule_table(&count);
        size_t ri = 0;
        if (!route_id) return NULL;
        for (ri = 0; ri < count; ri++) {
          if (rules[ri].route_id && !strcmp(rules[ri].route_id, route_id)) {
            return &rules[ri];
          }
        }
        return NULL;
      }

      auto int lowering_plan_extern_emit_rule_view_is_valid_by_route_id(
          const struct LoweringPlanExternCallView* view,
          const char* route_id) {
        return lowering_plan_extern_emit_rule_matches(
            view,
            find_mir_call_extern_emit_rule_by_route_id(route_id));
      }

      auto int lowering_plan_env_get_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_emit_rule_view_is_valid_by_route_id(
            view, "extern.env.get");
      }

      auto int lowering_plan_env_set_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_emit_rule_view_is_valid_by_route_id(
            view, "extern.env.set");
      }

      auto int lowering_plan_hostbridge_extern_invoke_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_emit_rule_view_is_valid_by_route_id(
            view, "extern.hostbridge.extern_invoke");
      }

      auto int lowering_plan_stage1_emit_program_json_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_emit_rule_view_is_valid_by_route_id(
            view, "extern.stage1.emit_program_json_v0");
      }

      auto int lowering_plan_stage1_emit_mir_from_source_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_emit_rule_view_is_valid_by_route_id(
            view, "extern.stage1.emit_mir_from_source_v0");
      }

      auto int lowering_plan_stage1_emit_mir_from_program_json_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_emit_rule_view_is_valid_by_route_id(
            view, "extern.stage1.emit_mir_from_program_json_v0");
      }

      auto void format_extern_i64_value_ref(
          char* out,
          size_t out_cap,
          long long reg) {
        long long cv = 0;
        long long base = resolve_copy_source_reg(reg);
        if (has_const(base, &cv)) snprintf(out, out_cap, "%lld", cv);
        else snprintf(out, out_cap, "%%r%lld", base);
      }
