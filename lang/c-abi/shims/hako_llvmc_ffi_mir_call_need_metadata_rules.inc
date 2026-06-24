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

      auto enum MirCallNeedKind
      mir_call_extern_need_kind_from_lowering_plan_entry(yyjson_val* entry) {
        struct LoweringPlanExternCallView view;
        const struct HakoLlvmcExternCallRouteRegistryRow* row = NULL;
        if (!read_lowering_plan_extern_call_view(entry, &view)) {
          return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        }
        row = hako_llvmc_extern_call_route_registry_find_need_rule(
            view.route_id,
            view.core_op,
            view.route_kind,
            view.symbol,
            view.tier);
        if (row && row->need_kind != HAKO_LLVMC_MIR_CALL_NEED_NONE) {
          return (enum MirCallNeedKind)row->need_kind;
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }
