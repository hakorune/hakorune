      /*
       * LoweringPlan generic-method rows accepted by the module generic string
       * emitter. Keep this table-like surface out of body emission so route
       * vocabulary does not drift across emit sites.
       */
      auto int module_generic_string_method_view_is_direct_string_len(
          const struct LoweringPlanGenericMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.len") &&
               view->core_op &&
               !strcmp(view->core_op, "StringLen") &&
               view->route_kind &&
               !strcmp(view->route_kind, "string_len") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.string.len_fast_h") &&
               view->route_proof &&
               !strcmp(view->route_proof, "len_surface_policy") &&
               view->return_shape &&
               !strcmp(view->return_shape, "scalar_i64") &&
               view->value_demand &&
               !strcmp(view->value_demand, "scalar_i64") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
      }

      auto int module_generic_string_method_view_is_direct_array_len(
          const struct LoweringPlanGenericMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.len") &&
               view->core_op &&
               !strcmp(view->core_op, "ArrayLen") &&
               view->route_kind &&
               !strcmp(view->route_kind, "array_slot_len") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.array.slot_len_h") &&
               view->route_proof &&
               !strcmp(view->route_proof, "len_surface_policy") &&
               view->receiver_origin_box &&
               !strcmp(view->receiver_origin_box, "ArrayBox") &&
               view->return_shape &&
               !strcmp(view->return_shape, "scalar_i64") &&
               view->value_demand &&
               !strcmp(view->value_demand, "scalar_i64") &&
               view->publication_policy &&
               !strcmp(view->publication_policy, "no_publication") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
      }

      auto int module_generic_string_method_view_is_direct_array_push(
          const struct LoweringPlanGenericMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.push") &&
               view->core_op &&
               !strcmp(view->core_op, "ArrayPush") &&
               view->route_kind &&
               !strcmp(view->route_kind, "array_append_any") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.array.slot_append_hh") &&
               view->route_proof &&
               !strcmp(view->route_proof, "push_surface_policy") &&
               view->receiver_origin_box &&
               !strcmp(view->receiver_origin_box, "ArrayBox") &&
               view->return_shape &&
               !strcmp(view->return_shape, "scalar_i64") &&
               view->value_demand &&
               !strcmp(view->value_demand, "write_any") &&
               view->publication_policy &&
               !strcmp(view->publication_policy, "no_publication") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME;
      }

      auto int module_generic_string_method_view_is_direct_array_get(
          const struct LoweringPlanGenericMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.get") &&
               view->core_op &&
               !strcmp(view->core_op, "ArrayGet") &&
               view->route_kind &&
               !strcmp(view->route_kind, "array_slot_load_any") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.array.slot_load_hi") &&
               view->route_proof &&
               !strcmp(view->route_proof, "get_surface_policy") &&
               view->receiver_origin_box &&
               !strcmp(view->receiver_origin_box, "ArrayBox") &&
               !view->return_shape &&
               view->value_demand &&
               !strcmp(view->value_demand, "read_ref") &&
               !view->publication_policy &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
      }

      auto int module_generic_string_method_view_is_direct_mir_json_array_item_get_with(
          const struct LoweringPlanGenericMethodView* view,
          const char* route_proof,
          const char* return_shape,
          const char* value_demand) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.get") &&
               view->core_op &&
               !strcmp(view->core_op, "ArrayGet") &&
               view->route_kind &&
               !strcmp(view->route_kind, "array_slot_load_any") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.array.slot_load_hi") &&
               view->route_proof &&
               route_proof &&
               !strcmp(view->route_proof, route_proof) &&
               !view->receiver_origin_box &&
               view->return_shape &&
               return_shape &&
               !strcmp(view->return_shape, return_shape) &&
               view->value_demand &&
               value_demand &&
               !strcmp(view->value_demand, value_demand) &&
               view->publication_policy &&
               !strcmp(view->publication_policy, "no_publication") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
      }

      auto int module_generic_string_method_view_is_direct_mir_json_block_inst_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return module_generic_string_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_block_inst_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int module_generic_string_method_view_is_direct_mir_json_effects_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return module_generic_string_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_effects_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int module_generic_string_method_view_is_direct_mir_json_function_block_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return module_generic_string_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_function_block_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int module_generic_string_method_view_is_direct_mir_json_module_function_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return module_generic_string_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_module_function_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int module_generic_string_method_view_is_direct_mir_json_params_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return module_generic_string_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_params_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int module_generic_string_method_view_is_direct_mir_json_phi_incoming_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return module_generic_string_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_phi_incoming_array_item",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int module_generic_string_method_view_is_direct_mir_json_phi_incoming_pair_scalar_get(
          const struct LoweringPlanGenericMethodView* view) {
        return module_generic_string_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_phi_incoming_pair_scalar",
            "scalar_i64_or_missing_zero",
            "scalar_i64");
      }

      auto int module_generic_string_method_view_is_direct_mir_json_vid_array_item_get(
          const struct LoweringPlanGenericMethodView* view) {
        return module_generic_string_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_vid_array_item",
            "scalar_i64_or_missing_zero",
            "scalar_i64");
      }

      auto int module_generic_string_method_view_is_direct_mir_json_flags_rec_array_get(
          const struct LoweringPlanGenericMethodView* view) {
        return module_generic_string_method_view_is_direct_mir_json_array_item_get_with(
            view,
            "mir_json_flags_rec_access",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int module_generic_string_method_view_is_direct_mir_json_flags_keys(
          const struct LoweringPlanGenericMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.keys") &&
               view->core_op &&
               !strcmp(view->core_op, "MapKeys") &&
               view->route_kind &&
               !strcmp(view->route_kind, "map_keys_array") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.map.keys_h") &&
               view->route_proof &&
               !strcmp(view->route_proof, "mir_json_flags_keys") &&
               !view->receiver_origin_box &&
               view->return_shape &&
               !strcmp(view->return_shape, "mixed_runtime_i64_or_handle") &&
               view->value_demand &&
               !strcmp(view->value_demand, "runtime_i64_or_handle") &&
               view->publication_policy &&
               !strcmp(view->publication_policy, "no_publication") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
      }

      auto int module_generic_string_method_view_is_direct_map_get(
          const struct LoweringPlanGenericMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.get") &&
               view->core_op &&
               !strcmp(view->core_op, "MapGet") &&
               view->route_kind &&
               !strcmp(view->route_kind, "runtime_data_load_any") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.runtime_data.get_hh") &&
               view->route_proof &&
               !strcmp(view->route_proof, "get_surface_policy") &&
               view->receiver_origin_box &&
               !strcmp(view->receiver_origin_box, "MapBox") &&
               view->return_shape &&
               !strcmp(view->return_shape, "mixed_runtime_i64_or_handle") &&
               view->value_demand &&
               !strcmp(view->value_demand, "runtime_i64_or_handle") &&
               view->publication_policy &&
               !strcmp(view->publication_policy, "runtime_data_facade") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME;
      }

      auto int module_generic_string_method_view_is_direct_mir_json_map_get_with(
          const struct LoweringPlanGenericMethodView* view,
          const char* route_proof,
          const char* return_shape,
          const char* value_demand) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.get") &&
               view->core_op &&
               !strcmp(view->core_op, "MapGet") &&
               view->route_kind &&
               !strcmp(view->route_kind, "runtime_data_load_any") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.runtime_data.get_hh") &&
               view->route_proof &&
               route_proof &&
               !strcmp(view->route_proof, route_proof) &&
               !view->receiver_origin_box &&
               view->return_shape &&
               return_shape &&
               !strcmp(view->return_shape, return_shape) &&
               view->value_demand &&
               value_demand &&
               !strcmp(view->value_demand, value_demand) &&
               view->publication_policy &&
               !strcmp(view->publication_policy, "no_publication") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME;
      }

      auto int module_generic_string_method_view_is_direct_mir_json_block_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        const char* key = view && view->entry ? read_str(view->entry, "key_const_text") : NULL;
        return module_generic_string_method_view_is_direct_mir_json_map_get_with(
                   view,
                   "mir_json_block_field",
                   "mixed_runtime_i64_or_handle",
                   "runtime_i64_or_handle") &&
               key &&
               (!strcmp(key, "id") || !strcmp(key, "instructions"));
      }

      auto int module_generic_string_method_view_is_direct_mir_json_const_value_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        const char* key = view && view->entry ? read_str(view->entry, "key_const_text") : NULL;
        return module_generic_string_method_view_is_direct_mir_json_map_get_with(
                   view,
                   "mir_json_const_value_field",
                   "mixed_runtime_i64_or_handle",
                   "runtime_i64_or_handle") &&
               key &&
               (!strcmp(key, "type") || !strcmp(key, "value"));
      }

      auto int module_generic_string_method_view_is_direct_mir_json_numeric_value_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        const char* key = view && view->entry ? read_str(view->entry, "key_const_text") : NULL;
        return module_generic_string_method_view_is_direct_mir_json_map_get_with(
                   view,
                   "mir_json_numeric_value_field",
                   "scalar_i64_or_missing_zero",
                   "scalar_i64") &&
               key &&
               !strcmp(key, "value");
      }

      auto int module_generic_string_method_view_is_direct_mir_json_callee_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        const char* key = view && view->entry ? read_str(view->entry, "key_const_text") : NULL;
        return module_generic_string_method_view_is_direct_mir_json_map_get_with(
                   view,
                   "mir_json_callee_field",
                   "mixed_runtime_i64_or_handle",
                   "runtime_i64_or_handle") &&
               key &&
               (!strcmp(key, "type") ||
                !strcmp(key, "name") ||
                !strcmp(key, "box_name") ||
                !strcmp(key, "method") ||
                !strcmp(key, "receiver") ||
                !strcmp(key, "box_type"));
      }

      auto int module_generic_string_method_view_is_direct_mir_json_inst_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        const char* key = view && view->entry ? read_str(view->entry, "key_const_text") : NULL;
        return module_generic_string_method_view_is_direct_mir_json_map_get_with(
                   view,
                   "mir_json_inst_field",
                   "mixed_runtime_i64_or_handle",
                   "runtime_i64_or_handle") &&
               key &&
               (!strcmp(key, "op") ||
                !strcmp(key, "operation") ||
                !strcmp(key, "op_kind") ||
                !strcmp(key, "cmp") ||
                !strcmp(key, "value") ||
                !strcmp(key, "args") ||
                !strcmp(key, "effects") ||
                !strcmp(key, "dst") ||
                !strcmp(key, "lhs") ||
                !strcmp(key, "rhs") ||
                !strcmp(key, "cond") ||
                !strcmp(key, "then") ||
                !strcmp(key, "else") ||
                !strcmp(key, "target") ||
                !strcmp(key, "incoming") ||
                !strcmp(key, "values") ||
                !strcmp(key, "mir_call") ||
                !strcmp(key, "callee") ||
                !strcmp(key, "func") ||
                !strcmp(key, "name"));
      }

      auto int module_generic_string_method_view_is_direct_mir_json_function_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        const char* key = view && view->entry ? read_str(view->entry, "key_const_text") : NULL;
        return module_generic_string_method_view_is_direct_mir_json_map_get_with(
                   view,
                   "mir_json_function_field",
                   "mixed_runtime_i64_or_handle",
                   "runtime_i64_or_handle") &&
               key &&
               (!strcmp(key, "name") ||
                !strcmp(key, "params") ||
                !strcmp(key, "flags") ||
                !strcmp(key, "blocks"));
      }

      auto int module_generic_string_method_view_is_direct_mir_json_module_field_get(
          const struct LoweringPlanGenericMethodView* view) {
        const char* key = view && view->entry ? read_str(view->entry, "key_const_text") : NULL;
        return module_generic_string_method_view_is_direct_mir_json_map_get_with(
                   view,
                   "mir_json_module_field",
                   "mixed_runtime_i64_or_handle",
                   "runtime_i64_or_handle") &&
               key &&
               (!strcmp(key, "functions") || !strcmp(key, "functions_0"));
      }

      auto int module_generic_string_method_view_is_direct_mir_json_flags_rec_map_get(
          const struct LoweringPlanGenericMethodView* view) {
        return module_generic_string_method_view_is_direct_mir_json_map_get_with(
            view,
            "mir_json_flags_rec_access",
            "mixed_runtime_i64_or_handle",
            "runtime_i64_or_handle");
      }

      auto int module_generic_string_method_view_is_direct_map_set(
          const struct LoweringPlanGenericMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.set") &&
               view->core_op &&
               !strcmp(view->core_op, "MapSet") &&
               view->route_kind &&
               !strcmp(view->route_kind, "map_store_any") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.map.slot_store_hhh") &&
               view->route_proof &&
               !strcmp(view->route_proof, "set_surface_policy") &&
               view->receiver_origin_box &&
               !strcmp(view->receiver_origin_box, "MapBox") &&
               !view->return_shape &&
               view->value_demand &&
               !strcmp(view->value_demand, "write_any") &&
               !view->publication_policy &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME;
      }

      auto int module_generic_string_method_view_is_direct_string_substring(
          const struct LoweringPlanGenericMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.substring") &&
               view->core_op &&
               !strcmp(view->core_op, "StringSubstring") &&
               view->route_kind &&
               !strcmp(view->route_kind, "string_substring") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.string.substring_hii") &&
               view->route_proof &&
               !strcmp(view->route_proof, "substring_surface_policy") &&
               view->receiver_origin_box &&
               !strcmp(view->receiver_origin_box, "StringBox") &&
               !view->return_shape &&
               view->value_demand &&
               !strcmp(view->value_demand, "read_ref") &&
               !view->publication_policy &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
      }

      auto int module_generic_string_method_view_is_direct_string_indexof(
          const struct LoweringPlanGenericMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.indexOf") &&
               view->core_op &&
               !strcmp(view->core_op, "StringIndexOf") &&
               view->route_kind &&
               !strcmp(view->route_kind, "string_indexof") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.string.indexOf_hh") &&
               view->route_proof &&
               !strcmp(view->route_proof, "indexof_surface_policy") &&
               view->receiver_origin_box &&
               !strcmp(view->receiver_origin_box, "StringBox") &&
               view->return_shape &&
               !strcmp(view->return_shape, "scalar_i64") &&
               view->value_demand &&
               !strcmp(view->value_demand, "scalar_i64") &&
               view->publication_policy &&
               !strcmp(view->publication_policy, "no_publication") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
      }

      auto int module_generic_string_method_view_is_direct_string_lastindexof(
          const struct LoweringPlanGenericMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.lastIndexOf") &&
               view->core_op &&
               !strcmp(view->core_op, "StringLastIndexOf") &&
               view->route_kind &&
               !strcmp(view->route_kind, "string_last_indexof") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.string.lastIndexOf_hh") &&
               view->route_proof &&
               !strcmp(view->route_proof, "lastindexof_surface_policy") &&
               view->receiver_origin_box &&
               !strcmp(view->receiver_origin_box, "StringBox") &&
               view->return_shape &&
               !strcmp(view->return_shape, "scalar_i64") &&
               view->value_demand &&
               !strcmp(view->value_demand, "scalar_i64") &&
               view->publication_policy &&
               !strcmp(view->publication_policy, "no_publication") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
      }

      auto int module_generic_string_method_view_is_direct_string_contains(
          const struct LoweringPlanGenericMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "generic_method.contains") &&
               view->core_op &&
               !strcmp(view->core_op, "StringContains") &&
               view->route_kind &&
               !strcmp(view->route_kind, "string_contains") &&
               view->symbol &&
               !strcmp(view->symbol, "nyash.string.contains_hh") &&
               view->route_proof &&
               !strcmp(view->route_proof, "contains_surface_policy") &&
               view->receiver_origin_box &&
               !strcmp(view->receiver_origin_box, "StringBox") &&
               view->return_shape &&
               !strcmp(view->return_shape, "scalar_i64") &&
               view->value_demand &&
               !strcmp(view->value_demand, "scalar_i64") &&
               view->publication_policy &&
               !strcmp(view->publication_policy, "no_publication") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
      }
