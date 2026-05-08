      /*
       * LoweringPlan generic-method rows accepted by the same-module function
       * emitter. Keep this table-like surface out of body emission so route
       * vocabulary does not drift across emit sites.
       */
      auto int same_module_method_view_str_eq(
          const char* actual,
          const char* expected) {
        return actual && expected && !strcmp(actual, expected);
      }

      auto int same_module_method_view_optional_str_eq(
          const char* actual,
          const char* expected) {
        if (!expected) return !actual;
        return actual && !strcmp(actual, expected);
      }

      enum SameModuleMethodViewReceiverRuleKind {
        SAME_MODULE_METHOD_VIEW_RECEIVER_ANY = 0,
        SAME_MODULE_METHOD_VIEW_RECEIVER_NONE,
        SAME_MODULE_METHOD_VIEW_RECEIVER_EXACT,
      };

      enum SameModuleMethodViewStringRuleKind {
        SAME_MODULE_METHOD_VIEW_STRING_REQUIRED = 0,
        SAME_MODULE_METHOD_VIEW_STRING_OPTIONAL,
        SAME_MODULE_METHOD_VIEW_STRING_IGNORED,
      };

      struct SameModuleMethodViewRouteRule {
        const char* route_id;
        const char* core_op;
        const char* route_kind;
        const char* symbol;
        const char* route_proof;
        const char* receiver_origin_box;
        enum SameModuleMethodViewReceiverRuleKind receiver_rule;
        const char* return_shape;
        enum SameModuleMethodViewStringRuleKind return_shape_rule;
        const char* value_demand;
        enum SameModuleMethodViewStringRuleKind value_demand_rule;
        const char* publication_policy;
        enum SameModuleMethodViewStringRuleKind publication_policy_rule;
        enum LoweringPlanTierMetadata tier;
      };

      auto int same_module_method_view_string_rule_matches(
          const char* actual,
          const char* expected,
          enum SameModuleMethodViewStringRuleKind rule_kind) {
        switch (rule_kind) {
          case SAME_MODULE_METHOD_VIEW_STRING_IGNORED:
            return 1;
          case SAME_MODULE_METHOD_VIEW_STRING_OPTIONAL:
            return same_module_method_view_optional_str_eq(actual, expected);
          case SAME_MODULE_METHOD_VIEW_STRING_REQUIRED:
          default:
            return same_module_method_view_str_eq(actual, expected);
        }
      }

      auto int same_module_method_view_receiver_rule_matches(
          const struct LoweringPlanGenericMethodView* view,
          const struct SameModuleMethodViewRouteRule* rule) {
        if (!(view && rule)) return 0;
        switch (rule->receiver_rule) {
          case SAME_MODULE_METHOD_VIEW_RECEIVER_NONE:
            return !view->receiver_origin_box;
          case SAME_MODULE_METHOD_VIEW_RECEIVER_EXACT:
            return same_module_method_view_str_eq(
                view->receiver_origin_box,
                rule->receiver_origin_box);
          case SAME_MODULE_METHOD_VIEW_RECEIVER_ANY:
          default:
            return 1;
        }
      }

      auto int same_module_method_view_matches_route_rule(
          const struct LoweringPlanGenericMethodView* view,
          const struct SameModuleMethodViewRouteRule* rule) {
        return view &&
               rule &&
               lowering_plan_generic_method_view_matches_route_tuple(
                   view,
                   rule->route_id,
                   rule->core_op,
                   rule->route_kind,
                   rule->tier) &&
               same_module_method_view_str_eq(view->symbol, rule->symbol) &&
               same_module_method_view_str_eq(
                   view->route_proof,
                   rule->route_proof) &&
               same_module_method_view_receiver_rule_matches(view, rule) &&
               same_module_method_view_string_rule_matches(
                   view->return_shape,
                   rule->return_shape,
                   rule->return_shape_rule) &&
               same_module_method_view_string_rule_matches(
                   view->value_demand,
                   rule->value_demand,
                   rule->value_demand_rule) &&
               same_module_method_view_string_rule_matches(
                   view->publication_policy,
                   rule->publication_policy,
                   rule->publication_policy_rule);
      }

      auto int same_module_method_view_is_direct_receiver_with(
          const struct LoweringPlanGenericMethodView* view,
          const char* route_id,
          const char* core_op,
          const char* route_kind,
          const char* symbol,
          const char* route_proof,
          const char* receiver_origin_box,
          const char* return_shape,
          const char* value_demand,
          const char* publication_policy,
          enum LoweringPlanTierMetadata tier) {
        struct SameModuleMethodViewRouteRule rule;
        rule.route_id = route_id;
        rule.core_op = core_op;
        rule.route_kind = route_kind;
        rule.symbol = symbol;
        rule.route_proof = route_proof;
        rule.receiver_origin_box = receiver_origin_box;
        rule.receiver_rule = SAME_MODULE_METHOD_VIEW_RECEIVER_EXACT;
        rule.return_shape = return_shape;
        rule.return_shape_rule = SAME_MODULE_METHOD_VIEW_STRING_OPTIONAL;
        rule.value_demand = value_demand;
        rule.value_demand_rule = SAME_MODULE_METHOD_VIEW_STRING_REQUIRED;
        rule.publication_policy = publication_policy;
        rule.publication_policy_rule = SAME_MODULE_METHOD_VIEW_STRING_OPTIONAL;
        rule.tier = tier;
        return same_module_method_view_matches_route_rule(view, &rule);
      }

      auto int same_module_method_view_is_ownerless_with(
          const struct LoweringPlanGenericMethodView* view,
          const char* route_id,
          const char* core_op,
          const char* route_kind,
          const char* symbol,
          const char* route_proof,
          const char* return_shape,
          const char* value_demand,
          const char* publication_policy,
          enum LoweringPlanTierMetadata tier) {
        struct SameModuleMethodViewRouteRule rule;
        rule.route_id = route_id;
        rule.core_op = core_op;
        rule.route_kind = route_kind;
        rule.symbol = symbol;
        rule.route_proof = route_proof;
        rule.receiver_origin_box = NULL;
        rule.receiver_rule = SAME_MODULE_METHOD_VIEW_RECEIVER_NONE;
        rule.return_shape = return_shape;
        rule.return_shape_rule = SAME_MODULE_METHOD_VIEW_STRING_REQUIRED;
        rule.value_demand = value_demand;
        rule.value_demand_rule = SAME_MODULE_METHOD_VIEW_STRING_REQUIRED;
        rule.publication_policy = publication_policy;
        rule.publication_policy_rule = SAME_MODULE_METHOD_VIEW_STRING_REQUIRED;
        rule.tier = tier;
        return same_module_method_view_matches_route_rule(view, &rule);
      }

      auto int same_module_method_view_is_direct_string_len(
          const struct LoweringPlanGenericMethodView* view) {
        static const struct SameModuleMethodViewRouteRule rule = {
          "generic_method.len",
          "StringLen",
          "string_len",
          "nyash.string.len_fast_h",
          "len_surface_policy",
          NULL,
          SAME_MODULE_METHOD_VIEW_RECEIVER_ANY,
          "scalar_i64",
          SAME_MODULE_METHOD_VIEW_STRING_REQUIRED,
          "scalar_i64",
          SAME_MODULE_METHOD_VIEW_STRING_REQUIRED,
          NULL,
          SAME_MODULE_METHOD_VIEW_STRING_IGNORED,
          HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
        };
        return same_module_method_view_matches_route_rule(view, &rule);
      }

      auto int same_module_method_view_is_direct_array_len(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_receiver_with(
            view,
            "generic_method.len",
            "ArrayLen",
            "array_slot_len",
            "nyash.array.slot_len_h",
            "len_surface_policy",
            "ArrayBox",
            "scalar_i64",
            "scalar_i64",
            "no_publication",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI);
      }

      auto int same_module_method_view_is_direct_array_push(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_receiver_with(
            view,
            "generic_method.push",
            "ArrayPush",
            "array_append_any",
            "nyash.array.slot_append_hh",
            "push_surface_policy",
            "ArrayBox",
            "scalar_i64",
            "write_any",
            "no_publication",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME);
      }

      auto int same_module_method_view_is_direct_array_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_receiver_with(
            view,
            "generic_method.get",
            "ArrayGet",
            "array_slot_load_any",
            "nyash.array.slot_load_hi",
            "get_surface_policy",
            "ArrayBox",
            NULL,
            "read_ref",
            NULL,
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI);
      }

      auto int same_module_method_view_is_direct_mir_json_array_item_get_with(
          const struct LoweringPlanGenericMethodView* view,
          const char* route_proof,
          const char* return_shape,
          const char* value_demand) {
        return same_module_method_view_is_ownerless_with(
            view,
            "generic_method.get",
            "ArrayGet",
            "array_slot_load_any",
            "nyash.array.slot_load_hi",
            route_proof,
            return_shape,
            value_demand,
            "no_publication",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI);
      }

      auto int same_module_method_view_is_direct_mir_json_block_inst_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_block_inst_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_effects_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_effects_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_function_block_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_function_block_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_module_function_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_module_function_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_params_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_params_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_phi_incoming_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_phi_incoming_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_phi_incoming_pair_scalar_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_phi_incoming_pair_scalar",
            "scalar_i64_or_missing_zero",
            "scalar_i64");
      }

      auto int same_module_method_view_is_direct_mir_json_vid_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_vid_array_item",
            "scalar_i64_or_missing_zero",
            "scalar_i64");
      }

      auto int same_module_method_view_is_direct_mir_json_flags_rec_array_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_flags_rec_access",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_flags_keys(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_ownerless_with(
            view,
            "generic_method.keys",
            "MapKeys",
            "map_keys_array",
            "nyash.map.keys_h",
            "mir_json_flags_keys",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle",
            "no_publication",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI);
      }

      auto int same_module_method_view_is_direct_map_keys(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_receiver_with(
            view,
            "generic_method.keys",
            "MapKeys",
            "map_keys_array",
            "nyash.map.keys_h",
            "keys_surface_policy",
            "MapBox",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle",
            "no_publication",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI);
      }

      auto int same_module_method_view_is_direct_map_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_receiver_with(
                   view,
                   "generic_method.get",
                   "MapGet",
                   "runtime_data_load_any",
                   "nyash.runtime_data.get_hh",
                   "get_surface_policy",
                   "MapBox",
                   "mixed_runtime_i64_or_handle",
                   "runtime_i64_or_handle",
                   "runtime_data_facade",
                   HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME) ||
               same_module_method_view_is_direct_receiver_with(
                   view,
                   "generic_method.get",
                   "MapGet",
                   "map_load_any",
                   "nyash.map.slot_load_hh",
                   "get_surface_policy",
                   "MapBox",
                   NULL,
                   "read_ref",
                   NULL,
                   HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI);
      }

      auto int same_module_method_view_is_direct_mir_json_map_get_with(
          const struct LoweringPlanGenericMethodView* view,
          const char* route_proof,
          const char* return_shape,
          const char* value_demand) {
        return same_module_method_view_is_ownerless_with(
            view,
            "generic_method.get",
            "MapGet",
            "runtime_data_load_any",
            "nyash.runtime_data.get_hh",
            route_proof,
            return_shape,
            value_demand,
            "no_publication",
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME);
      }

      auto int same_module_method_view_is_direct_mir_json_block_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_map_get_with(
            view,
            "mir_json_block_field",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_const_value_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_map_get_with(
            view,
            "mir_json_const_value_field",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_numeric_value_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_map_get_with(
            view,
            "mir_json_numeric_value_field",
            "scalar_i64_or_missing_zero",
            "scalar_i64");
      }

      auto int same_module_method_view_is_direct_mir_json_callee_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_map_get_with(
            view,
            "mir_json_callee_field",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_inst_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_map_get_with(
            view,
            "mir_json_inst_field",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_function_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_map_get_with(
            view,
            "mir_json_function_field",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_module_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_map_get_with(
            view,
            "mir_json_module_field",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_mir_json_flags_rec_map_get(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_mir_json_map_get_with(
            view,
            "mir_json_flags_rec_access",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int same_module_method_view_is_direct_map_set(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_receiver_with(
            view,
            "generic_method.set",
            "MapSet",
            "map_store_any",
            "nyash.map.slot_store_hhh",
            "set_surface_policy",
            "MapBox",
            NULL,
            "write_any",
            NULL,
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME);
      }

      auto int same_module_method_view_is_direct_array_set(
          const struct LoweringPlanGenericMethodView* view) {
        /*
         * The star is a MIR-owned symbolic helper token for the array-set
         * variant family. This predicate still performs exact string matching.
         */
        return same_module_method_view_is_direct_receiver_with(
            view,
            "generic_method.set",
            "ArraySet",
            "array_store_any",
            "nyash.array.slot_store_*",
            "set_surface_policy",
            "ArrayBox",
            NULL,
            "write_any",
            NULL,
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME);
      }

      auto int same_module_method_view_is_direct_set(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_map_set(view) ||
               same_module_method_view_is_direct_array_set(view);
      }

      auto int same_module_method_view_is_direct_map_delete(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_receiver_with(
            view,
            "generic_method.delete",
            "MapDelete",
            "map_delete_any",
            "nyash.map.delete_hh",
            "delete_surface_policy",
            "MapBox",
            "scalar_i64",
            "write_any",
            NULL,
            HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME);
      }

      auto int same_module_method_view_is_direct_map_has_any(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_receiver_with(
            view,
            "generic_method.has",
            "MapHas",
            "map_contains_any",
            "nyash.map.probe_hh",
            "has_surface_policy",
            "MapBox",
            NULL,
            "read_ref",
            NULL,
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI);
      }

      auto int same_module_method_view_is_direct_map_has_i64(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_receiver_with(
            view,
            "generic_method.has",
            "MapHas",
            "map_contains_i64",
            "nyash.map.probe_hi",
            "has_surface_policy",
            "MapBox",
            NULL,
            "read_ref",
            NULL,
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI);
      }

      auto int same_module_method_view_is_direct_array_has(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_receiver_with(
            view,
            "generic_method.has",
            "ArrayHas",
            "array_contains_any",
            "nyash.array.has_hh",
            "has_surface_policy",
            "ArrayBox",
            NULL,
            "read_ref",
            NULL,
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI);
      }

      auto int same_module_method_view_is_direct_has(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_map_has_any(view) ||
               same_module_method_view_is_direct_map_has_i64(view) ||
               same_module_method_view_is_direct_array_has(view);
      }

      auto int same_module_method_view_is_direct_string_substring(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_receiver_with(
            view,
            "generic_method.substring",
            "StringSubstring",
            "string_substring",
            "nyash.string.substring_hii",
            "substring_surface_policy",
            "StringBox",
            NULL,
            "read_ref",
            NULL,
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI);
      }

      auto int same_module_method_view_is_direct_string_scalar_with(
          const struct LoweringPlanGenericMethodView* view,
          const char* route_id,
          const char* core_op,
          const char* route_kind,
          const char* symbol,
          const char* route_proof) {
        return same_module_method_view_is_direct_receiver_with(
            view,
            route_id,
            core_op,
            route_kind,
            symbol,
            route_proof,
            "StringBox",
            "scalar_i64",
            "scalar_i64",
            "no_publication",
            HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI);
      }

      auto int same_module_method_view_is_direct_string_indexof(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_string_scalar_with(
            view,
            "generic_method.indexOf",
            "StringIndexOf",
            "string_indexof",
            "nyash.string.indexOf_hh",
            "indexof_surface_policy");
      }

      auto int same_module_method_view_is_direct_string_lastindexof(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_string_scalar_with(
            view,
            "generic_method.lastIndexOf",
            "StringLastIndexOf",
            "string_last_indexof",
            "nyash.string.lastIndexOf_hh",
            "lastindexof_surface_policy");
      }

      auto int same_module_method_view_is_direct_string_contains(
          const struct LoweringPlanGenericMethodView* view) {
        return same_module_method_view_is_direct_string_scalar_with(
            view,
            "generic_method.contains",
            "StringContains",
            "string_contains",
            "nyash.string.contains_hh",
            "contains_surface_policy");
      }
