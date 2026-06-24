      #include "hako_llvmc_ffi_mir_call_need_vocab.inc"
      #include "hako_llvmc_ffi_mir_call_need_metadata_rules.inc"
      #include "hako_llvmc_ffi_mir_call_need_apply.inc"

      auto int maybe_promote_array_string_birth_for_mir_call_method(
          long long bid,
          size_t ii,
          enum MirCallNeedKind need_kind,
          const struct GenericMethodRouteState* route,
          long long recv,
          long long arg0,
          long long arg1) {
        yyjson_val* entry = NULL;
        struct LoweringPlanGenericMethodView view;
        long long value = 0;
        if (!route) return 0;
        if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH) {
          value = arg0;
        } else if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET) {
          value = arg1;
        } else {
          return 0;
        }
        if (route->recv_org != ORG_ARRAY_BIRTH) return 0;
        entry = find_lowering_plan_entry_for_site(bid, ii);
        if (!read_lowering_plan_generic_method_view(entry, &view)) return 0;
        if (!lowering_plan_generic_method_view_value_origin_is_string(&view)) {
          return 0;
        }
        return hako_llvmc_promote_array_string_birth_if_value_is_string(
            recv, value, 1);
      }

      auto int mark_constructor_call_from_lowering_plan(
          long long bid,
          size_t ii,
          long long dst,
          struct MirCallNeedFlags* needs) {
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanConstructorCallView view;
        enum MirCallNeedKind need_kind;
        int origin_kind;
        if (!entry) return 0;
        if (!read_lowering_plan_constructor_call_view(entry, &view)) return 0;
        need_kind = mir_call_constructor_need_kind_from_lowering_plan_entry(entry);
        apply_mir_call_need_kind(needs, need_kind);
        origin_kind = lowering_plan_constructor_call_view_result_origin_kind(&view);
        if (dst && origin_kind != ORG_NONE) {
          set_scan_origin(dst, origin_kind);
        }
        return 1;
      }

      auto enum MirCallNeedKind
      classify_mir_call_method_need_kind_from_site_lowering_plan(
          long long bid,
          size_t ii) {
        yyjson_val* plan_entry = find_lowering_plan_entry_for_site(bid, ii);
        if (plan_entry) {
          enum MirCallNeedKind plan_need =
              mir_call_generic_method_need_kind_from_lowering_plan_entry(plan_entry);
          if (plan_need != HAKO_LLVMC_MIR_CALL_NEED_NONE) {
            trace_lowering_plan_entry_hit(plan_entry, "mir_call_need");
            return plan_need;
          }
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind
      classify_mir_call_global_need_kind_from_lowering_plan(
          long long bid,
          size_t ii) {
        yyjson_val* plan_entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanGlobalCallView view;
        if (!plan_entry) return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        if (!read_lowering_plan_global_call_view(plan_entry, &view)) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        if (lowering_plan_global_call_view_is_stage1_emit_program_json(&view)) {
          trace_lowering_plan_entry_hit(plan_entry, "mir_call_global_need");
          return HAKO_LLVMC_MIR_CALL_NEED_STAGE1_EMIT_PROGRAM_JSON;
        }
        {
          const char* need_kind = read_str(plan_entry, "need_kind");
          if (need_kind && !strcmp(need_kind, "printf")) {
            trace_lowering_plan_entry_hit(plan_entry, "mir_call_global_need");
            return HAKO_LLVMC_MIR_CALL_NEED_PRINTF;
          }
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }


      auto enum MirCallNeedKind
      classify_mir_call_extern_need_kind_from_lowering_plan(
          long long bid,
          size_t ii) {
        yyjson_val* plan_entry = find_lowering_plan_entry_for_site(bid, ii);
        if (plan_entry) {
          enum MirCallNeedKind plan_need =
              mir_call_extern_need_kind_from_lowering_plan_entry(plan_entry);
          if (plan_need != HAKO_LLVMC_MIR_CALL_NEED_NONE) {
            trace_lowering_plan_entry_hit(plan_entry, "mir_call_extern_need");
            return plan_need;
          }
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }
