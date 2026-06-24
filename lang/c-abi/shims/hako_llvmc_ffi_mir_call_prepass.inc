      auto void mark_global_call_result_origin_from_view(
          const struct LoweringPlanGlobalCallView* view,
          long long dst) {
        int origin_kind = ORG_NONE;
        if (!dst || !view) return;
        origin_kind = lowering_plan_global_call_view_result_origin_kind(view);
        if (origin_kind == ORG_NONE) return;
        set_scan_origin(dst, origin_kind);
        set_origin(dst, origin_kind);
      }

      auto void mark_global_call_result_origin_from_lowering_plan(
          long long bid,
          size_t ii,
          long long dst) {
        yyjson_val* entry = NULL;
        const char* source = NULL;
        struct LoweringPlanGlobalCallView view;
        if (!dst) return;
        entry = find_lowering_plan_entry_for_site(bid, ii);
        source = entry ? read_str(entry, "source") : NULL;
        if (!(source && !strcmp(source, "global_call_routes"))) return;
        if (!read_lowering_plan_global_call_view(entry, &view)) return;
        mark_global_call_result_origin_from_view(&view, dst);
      }

      auto int mir_call_generic_method_value_origin_is_string_from_lowering_plan(
          long long bid,
          size_t ii) {
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanGenericMethodView view;
        if (!read_lowering_plan_generic_method_view(entry, &view)) return 0;
        return lowering_plan_generic_method_view_value_origin_is_string(&view);
      }

      auto void mark_user_box_method_result_from_lowering_plan(
          long long bid,
          size_t ii,
          long long dst) {
        yyjson_val* entry = NULL;
        const char* source = NULL;
        struct LoweringPlanUserBoxMethodView view;
        int typed_plan_index = -1;
        int origin_kind = ORG_NONE;
        if (!dst) return;
        entry = find_lowering_plan_entry_for_site(bid, ii);
        source = entry ? read_str(entry, "source") : NULL;
        if (!(source && !strcmp(source, "user_box_method_routes"))) return;
        if (!read_lowering_plan_user_box_method_view(entry, &view)) return;
        if (!lowering_plan_user_box_method_view_is_direct_runtime_i64_or_handle(&view)) return;
        if (view.target_result_box_name &&
            find_typed_object_plan_index(view.target_result_box_name, &typed_plan_index)) {
          set_origin(dst, ORG_TYPED_OBJECT);
          set_scan_origin(dst, ORG_TYPED_OBJECT);
          set_typed_object_binding(dst, typed_plan_index);
          return;
        }
        origin_kind = lowering_plan_user_box_method_view_result_origin_kind(&view);
        if (origin_kind != ORG_NONE) {
          set_origin(dst, origin_kind);
          set_scan_origin(dst, origin_kind);
        }
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

      auto void mark_extern_call_result_origin_from_lowering_plan(
          long long bid,
          size_t ii,
          long long dst) {
        yyjson_val* entry = NULL;
        struct LoweringPlanExternCallView view;
        int origin_kind = ORG_NONE;
        if (!dst) return;
        entry = find_lowering_plan_entry_for_site(bid, ii);
        if (!read_lowering_plan_extern_call_view(entry, &view)) return;
        origin_kind = lowering_plan_extern_call_view_result_origin_kind(&view);
        if (origin_kind == ORG_NONE) return;
        set_scan_origin(dst, origin_kind);
      }

      auto void scan_mir_call_need_flags(
          yyjson_val* blocks,
          long long bid,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          yyjson_val* ins,
          struct MirCallNeedFlags* needs) {
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        const char* ctype = cal ? read_str(cal, "type") : NULL;
        const char* bname = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
        yyjson_val* site_plan_entry = find_lowering_plan_entry_for_site(bid, ii);

        (void)blocks;
        (void)ilen;
        (void)insts;
        if (!needs) return;
        {
          enum MirCallNeedKind plan_extern_need =
              classify_mir_call_extern_need_kind_from_lowering_plan(bid, ii);
          if (plan_extern_need != HAKO_LLVMC_MIR_CALL_NEED_NONE) {
            apply_mir_call_need_kind(needs, plan_extern_need);
            mark_extern_call_result_origin_from_lowering_plan(
                bid, ii, read_int(ins, "dst"));
            return;
          }
        }
        if (ctype && !strcmp(ctype, "Constructor")) {
          long long dst = read_int(ins, "dst");
          if (mark_constructor_call_from_lowering_plan(bid, ii, dst, needs)) {
            return;
          }
          (void)bname;
          return;
        }

        if (ctype && !strcmp(ctype, "Method")) {
          long long recv = cal ? read_int(cal, "receiver") : 0;
          yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
          long long a0 = 0;
          long long a1 = 0;
          struct GenericMethodRouteState route;
          enum MirCallNeedKind need_kind;
          if (args && yyjson_is_arr(args)) {
            if (yyjson_arr_size(args) >= 1) a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
            if (yyjson_arr_size(args) >= 2) a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
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
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_PUSH &&
              route.recv_org == ORG_ARRAY_BIRTH &&
              mir_call_generic_method_value_origin_is_string_from_lowering_plan(bid, ii)) {
            hako_llvmc_promote_array_string_birth_if_value_is_string(
                recv, a0, 1);
          }
          if (need_kind == HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET &&
              route.recv_org == ORG_ARRAY_BIRTH &&
              mir_call_generic_method_value_origin_is_string_from_lowering_plan(bid, ii)) {
            hako_llvmc_promote_array_string_birth_if_value_is_string(
                recv, a1, 1);
          }
          return;
        }

        if (ctype && !strcmp(ctype, "Global")) {
          enum MirCallNeedKind plan_need =
              classify_mir_call_global_need_kind_from_lowering_plan(bid, ii);
          mark_global_call_result_origin_from_lowering_plan(
              bid, ii, read_int(ins, "dst"));
          if (plan_need != HAKO_LLVMC_MIR_CALL_NEED_NONE) {
            apply_mir_call_need_kind(needs, plan_need);
          }
          return;
        }

      }
