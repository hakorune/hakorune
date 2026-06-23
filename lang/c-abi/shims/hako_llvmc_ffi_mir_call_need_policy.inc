      #include "hako_llvmc_ffi_mir_call_need_vocab.inc"
      #include "hako_llvmc_ffi_mir_call_need_name_fallback.inc"
      #include "hako_llvmc_ffi_mir_call_need_metadata_rules.inc"
      #include "hako_llvmc_ffi_mir_call_need_apply.inc"

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
