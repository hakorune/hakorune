      #include "hako_llvmc_ffi_mir_call_need_vocab.inc"
      #include "hako_llvmc_ffi_mir_call_need_name_fallback.inc"
      #include "hako_llvmc_ffi_mir_call_need_metadata_rules.inc"
      #include "hako_llvmc_ffi_mir_call_need_apply.inc"

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
              mir_call_generic_method_need_kind_from_lowering_plan_entry(plan_entry);
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
          if (!hako_llvmc_json_read_i64_field(route, "block", &block) ||
              !hako_llvmc_json_read_i64_field(
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
