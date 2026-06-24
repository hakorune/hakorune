      #include "hako_llvmc_ffi_mir_call_need_vocab.inc"
      #include "hako_llvmc_ffi_mir_call_need_metadata_rules.inc"
      #include "hako_llvmc_ffi_mir_call_need_apply.inc"

      auto enum MirCallNeedKind
      classify_mir_call_method_need_kind_from_site_lowering_plan(
          long long bid,
          size_t ii);

      auto enum MirCallNeedKind
      classify_mir_call_global_need_kind_from_lowering_plan(
          long long bid,
          size_t ii);

      auto enum MirCallNeedKind
      classify_mir_call_extern_need_kind_from_lowering_plan(
          long long bid,
          size_t ii);

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

      auto int scan_mir_extern_call_need_flags_from_lowering_plan(
          long long bid,
          size_t ii,
          yyjson_val* ins,
          struct MirCallNeedFlags* needs) {
        enum MirCallNeedKind plan_extern_need =
            classify_mir_call_extern_need_kind_from_lowering_plan(bid, ii);
        if (plan_extern_need == HAKO_LLVMC_MIR_CALL_NEED_NONE) return 0;
        apply_mir_call_need_kind(needs, plan_extern_need);
        mark_extern_call_result_origin_from_lowering_plan(
            bid, ii, read_int(ins, "dst"));
        return 1;
      }

      auto void scan_mir_method_call_need_flags_from_lowering_plan(
          long long bid,
          size_t ii,
          yyjson_val* ins,
          yyjson_val* cal,
          yyjson_val* mc,
          struct MirCallNeedFlags* needs) {
        long long recv = cal ? read_int(cal, "receiver") : 0;
        yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
        long long a0 = 0;
        long long a1 = 0;
        struct GenericMethodRouteState route;
        enum MirCallNeedKind need_kind;
        if (!needs) return;
        if (args && yyjson_is_arr(args)) {
          if (yyjson_arr_size(args) >= 1) {
            a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
          }
          if (yyjson_arr_size(args) >= 2) {
            a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
          }
        }
        mark_user_box_method_result_from_lowering_plan(bid, ii, read_int(ins, "dst"));
        mark_generic_method_result_origin_from_lowering_plan(
            bid, ii, read_int(ins, "dst"));
        if (match_array_text_observer_route_metadata(bid, ii, NULL)) {
          mark_array_text_observer_need_flags_from_metadata(
              bid, ii, read_int(ins, "dst"), needs);
          return;
        }
        route = classify_generic_method_route(bid, ii, recv);
        need_kind =
            classify_mir_call_method_need_kind_from_site_lowering_plan(bid, ii);
        if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_STRING_INDEXOF) {
          mark_array_text_observer_need_flags_from_metadata(
              bid, ii, read_int(ins, "dst"), needs);
          return;
        }
        apply_mir_call_need_kind(needs, need_kind);
        maybe_promote_array_string_birth_for_mir_call_method(
            bid, ii, need_kind, &route, recv, a0, a1);
      }

      auto void scan_mir_global_call_need_flags_from_lowering_plan(
          long long bid,
          size_t ii,
          yyjson_val* ins,
          struct MirCallNeedFlags* needs) {
        enum MirCallNeedKind plan_need =
            classify_mir_call_global_need_kind_from_lowering_plan(bid, ii);
        mark_global_call_result_origin_from_lowering_plan(
            bid, ii, read_int(ins, "dst"));
        if (plan_need != HAKO_LLVMC_MIR_CALL_NEED_NONE) {
          apply_mir_call_need_kind(needs, plan_need);
        }
      }

      auto enum MirCallNeedKind
      classify_mir_call_generic_method_need_kind_from_metadata(
          long long bid,
          size_t ii) {
        yyjson_val* routes = fn_metadata ? yyjson_obj_get(fn_metadata, "generic_method_routes") : NULL;
        size_t ri = 0;
        if (!(routes && yyjson_is_arr(routes))) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          yyjson_val* core_method = route ? yyjson_obj_get(route, "core_method") : NULL;
          long long block = -1;
          long long inst = -1;
          const char* route_id = route ? read_str(route, "route_id") : NULL;
          const char* route_kind = route ? read_str(route, "route_kind") : NULL;
          const char* core_op = core_method ? read_str(core_method, "op") : NULL;
          int tier = hako_llvmc_generic_method_route_registry_tier_from_label(
              core_method ? read_str(core_method, "lowering_tier") : NULL);
          const struct HakoLlvmcGenericMethodRouteRegistryRow* row = NULL;
          if (!hako_llvmc_json_read_i64_field(route, "block", &block) ||
              !hako_llvmc_json_read_i64_field(route, "instruction_index", &inst) ||
              block != bid ||
              inst != (long long)ii) {
            continue;
          }
          row = hako_llvmc_generic_method_route_registry_find_by_tuple(
              route_id,
              core_op,
              route_kind,
              tier);
          if (row && row->need_kind != 0) {
            if (hako_llvmc_route_trace_enabled()) {
              hako_llvmc_emit_route_trace(
                  "mir_call_need_metadata",
                  "hit",
                  "generic_method_descriptor",
                  route_kind ? route_kind : "-");
            }
            return (enum MirCallNeedKind)row->need_kind;
          }
          if (hako_llvmc_route_trace_enabled()) {
            hako_llvmc_emit_route_trace(
                "mir_call_need_metadata",
                "skip",
                "generic_method_descriptor_mismatch",
                route_kind ? route_kind : "-");
          }
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
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
        return classify_mir_call_generic_method_need_kind_from_metadata(bid, ii);
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
