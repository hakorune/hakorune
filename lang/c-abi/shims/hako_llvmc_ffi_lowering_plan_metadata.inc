      enum LoweringPlanTierMetadata {
        HAKO_LLVMC_LOWERING_PLAN_TIER_NONE = 0,
        HAKO_LLVMC_LOWERING_PLAN_TIER_HOT_INLINE,
        HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
        HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
        HAKO_LLVMC_LOWERING_PLAN_TIER_UNSUPPORTED,
      };

      struct LoweringPlanGenericMethodView {
        yyjson_val* entry;
        const char* route_id;
        const char* core_op;
        const char* route_kind;
        const char* symbol;
        const char* route_proof;
        const char* receiver_origin_box;
        const char* result_origin_box;
        const char* key_route;
        const char* return_shape;
        const char* value_demand;
        const char* publication_policy;
        enum LoweringPlanTierMetadata tier;
      };

      struct LoweringPlanExternCallView {
        yyjson_val* entry;
        const char* route_id;
        const char* source_symbol;
        const char* core_op;
        const char* route_kind;
        const char* emit_kind;
        const char* symbol;
        const char* proof;
        const char* route_proof;
        const char* return_shape;
        const char* value_demand;
        long long result_value;
        int has_result_value;
        enum LoweringPlanTierMetadata tier;
      };

      struct LoweringPlanGlobalCallView {
        yyjson_val* entry;
        const char* route_id;
        const char* callee_name;
        const char* target_symbol;
        const char* target_return_type;
        const char* target_result_box_name;
        const char* core_op;
        const char* route_kind;
        const char* emit_kind;
        const char* proof;
        const char* route_proof;
        const char* reason;
        const char* target_shape_reason;
        const char* target_shape_blocker_symbol;
        const char* target_shape_blocker_reason;
        const char* return_shape;
        const char* value_demand;
        const char* result_origin;
        const char* arg0_origin_box;
        const char* definition_owner;
        const char* emit_trace_consumer;
        long long arity;
        long long target_arity;
        long long result_value;
        int has_target_arity;
        int has_result_value;
        int target_exists;
        int has_arity_matches;
        int arity_matches;
        enum LoweringPlanTierMetadata tier;
      };

      struct LoweringPlanUserBoxMethodView {
        yyjson_val* entry;
        const char* route_id;
        const char* box_name;
        const char* method;
        const char* target_symbol;
        const char* target_result_box_name;
        const char* core_op;
        const char* route_kind;
        const char* emit_kind;
        const char* proof;
        const char* route_proof;
        const char* reason;
        const char* return_shape;
        const char* value_demand;
        const char* result_origin;
        const char* definition_owner;
        const char* emit_trace_consumer;
        long long arity;
        long long target_arity;
        long long receiver_value;
        long long result_value;
        long long type_id;
        int has_target_arity;
        int has_receiver_value;
        int has_result_value;
        int has_type_id;
        int target_exists;
        int has_arity_matches;
        int arity_matches;
        enum LoweringPlanTierMetadata tier;
      };

      struct LoweringPlanDirectArrayAccessView {
        yyjson_val* entry;
        const char* route_id;
        const char* op;
        const char* array_kind;
        const char* element_type;
        const char* route;
        const char* bounds_policy;
        const char* proof_kind;
        yyjson_val* proof_ids;
        const char* fallback_policy;
        const char* cfg_shape;
        const char* store_semantics;
        long long receiver_value;
        long long index_value;
        long long value_value;
        long long result_value;
        int has_value_value;
        int has_result_value;
      };

      auto int lowering_plan_read_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        return hako_llvmc_json_read_i64_field(obj, key, out);
      }

      auto int lowering_plan_read_bool_field(
          yyjson_val* obj,
          const char* key,
          int* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_bool(val))) return 0;
        if (out) *out = yyjson_get_bool(val) ? 1 : 0;
        return 1;
      }

      auto enum LoweringPlanTierMetadata read_lowering_plan_tier_metadata(
          yyjson_val* entry,
          int* invalid) {
        const char* tier = read_str(entry, "tier");
        if (tier && !strcmp(tier, "HotInline")) {
          return HAKO_LLVMC_LOWERING_PLAN_TIER_HOT_INLINE;
        }
        if (tier && !strcmp(tier, "DirectAbi")) {
          return HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
        }
        if (tier && !strcmp(tier, "ColdRuntime")) {
          return HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME;
        }
        if (tier && !strcmp(tier, "Unsupported")) {
          return HAKO_LLVMC_LOWERING_PLAN_TIER_UNSUPPORTED;
        }
        if (invalid) *invalid = 1;
        return HAKO_LLVMC_LOWERING_PLAN_TIER_NONE;
      }

      auto int lowering_plan_tier_is_direct_abi(
          enum LoweringPlanTierMetadata tier) {
        return tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI;
      }

      auto int lowering_plan_tier_is_cold_runtime(
          enum LoweringPlanTierMetadata tier) {
        return tier == HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME;
      }

      auto yyjson_val* find_lowering_plan_entry_for_site(long long bid, size_t ii) {
        yyjson_val* entries = NULL;
        size_t pi = 0;
        if (!fn_metadata) return NULL;
        entries = yyjson_obj_get(fn_metadata, "lowering_plan");
        if (!(entries && yyjson_is_arr(entries))) {
          return NULL;
        }
        for (pi = 0; pi < yyjson_arr_size(entries); pi++) {
          yyjson_val* entry = yyjson_arr_get(entries, pi);
          long long block = 0;
          long long inst = 0;
          if (!lowering_plan_read_i64_field(entry, "block", &block) ||
              !lowering_plan_read_i64_field(entry, "instruction_index", &inst)) {
            continue;
          }
          if (block == bid && (size_t)inst == ii) {
            return entry;
          }
        }
        return NULL;
      }

      auto yyjson_val* find_direct_array_access_plan_for_site(long long bid, size_t ii) {
        yyjson_val* entries = NULL;
        size_t pi = 0;
        if (!fn_metadata) return NULL;
        entries = yyjson_obj_get(fn_metadata, "direct_array_access_plans");
        if (!(entries && yyjson_is_arr(entries))) {
          return NULL;
        }
        for (pi = 0; pi < yyjson_arr_size(entries); pi++) {
          yyjson_val* entry = yyjson_arr_get(entries, pi);
          long long block = 0;
          long long inst = 0;
          if (!lowering_plan_read_i64_field(entry, "block", &block) ||
              !lowering_plan_read_i64_field(entry, "instruction_index", &inst)) {
            continue;
          }
          if (block == bid && (size_t)inst == ii) {
            return entry;
          }
        }
        return NULL;
      }

      auto int read_lowering_plan_direct_array_access_view(
          yyjson_val* entry,
          struct LoweringPlanDirectArrayAccessView* out) {
        long long receiver_value = 0;
        long long index_value = 0;
        long long value_value = 0;
        long long result_value = 0;
        if (!(entry && out)) return 0;
        memset(out, 0, sizeof(*out));
        out->entry = entry;
        out->route_id = read_str(entry, "route_id");
        out->op = read_str(entry, "op");
        out->array_kind = read_str(entry, "array_kind");
        out->element_type = read_str(entry, "element_type");
        out->route = read_str(entry, "route");
        out->bounds_policy = read_str(entry, "bounds_policy");
        out->proof_kind = read_str(entry, "proof_kind");
        out->proof_ids = yyjson_obj_get(entry, "proof_ids");
        out->fallback_policy = read_str(entry, "fallback_policy");
        out->cfg_shape = read_str(entry, "cfg_shape");
        out->store_semantics = read_str(entry, "store_semantics");
        if (!lowering_plan_read_i64_field(entry, "receiver_value", &receiver_value) ||
            !lowering_plan_read_i64_field(entry, "index_value", &index_value)) {
          return 0;
        }
        out->receiver_value = receiver_value;
        out->index_value = index_value;
        if (lowering_plan_read_i64_field(entry, "value_value", &value_value)) {
          out->value_value = value_value;
          out->has_value_value = 1;
        }
        if (lowering_plan_read_i64_field(entry, "result_value", &result_value)) {
          out->result_value = result_value;
          out->has_result_value = 1;
        }
        return out->route_id &&
               !strcmp(out->route_id, "direct_array.access") &&
               out->op &&
               out->array_kind &&
               !strcmp(out->array_kind, "DirectArrayI64") &&
               out->element_type &&
               !strcmp(out->element_type, "i64") &&
               out->route &&
               out->bounds_policy &&
               out->proof_kind &&
               out->proof_ids &&
               yyjson_is_arr(out->proof_ids) &&
               out->fallback_policy &&
               out->cfg_shape &&
               out->store_semantics;
      }

      auto int lowering_plan_direct_array_access_view_has_proof_id(
          const struct LoweringPlanDirectArrayAccessView* view,
          const char* proof_id) {
        size_t pi = 0;
        if (!(view && view->proof_ids && yyjson_is_arr(view->proof_ids) && proof_id)) return 0;
        for (pi = 0; pi < yyjson_arr_size(view->proof_ids); pi++) {
          const char* actual = yyjson_get_str(yyjson_arr_get(view->proof_ids, pi));
          if (actual && !strcmp(actual, proof_id)) return 1;
        }
        return 0;
      }

      auto int lowering_plan_direct_array_access_view_is_checked_i64(
          const struct LoweringPlanDirectArrayAccessView* view,
          const char* op,
          const char* route) {
        return view &&
               view->op &&
               !strcmp(view->op, op) &&
               view->route &&
               !strcmp(view->route, route) &&
               view->bounds_policy &&
               !strcmp(view->bounds_policy, "checked") &&
               view->proof_kind &&
               !strcmp(view->proof_kind, "exact_front_contract") &&
               lowering_plan_direct_array_access_view_has_proof_id(view, "exact_front_contract") &&
               view->fallback_policy &&
               !strcmp(view->fallback_policy, "allow_checked") &&
               view->cfg_shape &&
               !strcmp(view->cfg_shape, "checked_branching") &&
               view->store_semantics &&
               ((!strcmp(op, "store") &&
                 !strcmp(view->store_semantics, "append_or_overwrite")) ||
                (!strcmp(op, "load") &&
                 !strcmp(view->store_semantics, "not_store")));
      }

      auto int lowering_plan_direct_array_access_view_is_proved_unchecked_range_index_store_i64(
          const struct LoweringPlanDirectArrayAccessView* view,
          const char* route) {
        return view &&
               view->op &&
               !strcmp(view->op, "store") &&
               view->route &&
               !strcmp(view->route, route) &&
               view->bounds_policy &&
               !strcmp(view->bounds_policy, "proved_unchecked") &&
               view->proof_kind &&
               !strcmp(view->proof_kind, "range_index") &&
               lowering_plan_direct_array_access_view_has_proof_id(view, "range_index") &&
               view->fallback_policy &&
               !strcmp(view->fallback_policy, "fail_fast") &&
               view->cfg_shape &&
               !strcmp(view->cfg_shape, "branchless") &&
               view->store_semantics &&
               !strcmp(view->store_semantics, "append_or_overwrite");
      }

      auto int lowering_plan_direct_array_access_view_is_proved_unchecked_stack_top_pop_i64(
          const struct LoweringPlanDirectArrayAccessView* view,
          const char* op,
          const char* route) {
        return view &&
               view->op &&
               !strcmp(view->op, op) &&
               view->route &&
               !strcmp(view->route, route) &&
               view->bounds_policy &&
               !strcmp(view->bounds_policy, "proved_unchecked") &&
               view->proof_kind &&
               !strcmp(view->proof_kind, "stack_top_pop") &&
               lowering_plan_direct_array_access_view_has_proof_id(view, "stack_top_pop") &&
               view->fallback_policy &&
               !strcmp(view->fallback_policy, "fail_fast") &&
               view->cfg_shape &&
               !strcmp(view->cfg_shape, "branchless") &&
               view->store_semantics &&
               ((!strcmp(op, "store") &&
                 !strcmp(view->store_semantics, "overwrite_existing")) ||
                (!strcmp(op, "load") &&
                 !strcmp(view->store_semantics, "not_store")));
      }

      auto int lowering_plan_direct_array_access_view_is_proved_unchecked_caller_precondition_store_i64(
          const struct LoweringPlanDirectArrayAccessView* view,
          const char* route) {
        return view &&
               view->op &&
               !strcmp(view->op, "store") &&
               view->route &&
               !strcmp(view->route, route) &&
               view->bounds_policy &&
               !strcmp(view->bounds_policy, "proved_unchecked") &&
               view->proof_kind &&
               !strcmp(view->proof_kind, "caller_precondition") &&
               lowering_plan_direct_array_access_view_has_proof_id(view, "caller_precondition") &&
               view->fallback_policy &&
               !strcmp(view->fallback_policy, "fail_fast") &&
               view->cfg_shape &&
               !strcmp(view->cfg_shape, "branchless") &&
               view->store_semantics &&
               !strcmp(view->store_semantics, "overwrite_existing");
      }

      auto int lowering_plan_direct_array_access_site_matches_checked_i64(
          long long bid,
          size_t ii,
          const char* op,
          const char* route,
          long long receiver_value,
          long long index_value,
          long long value_value,
          int require_value,
          long long result_value,
          int require_result) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        if (!lowering_plan_direct_array_access_view_is_checked_i64(&view, op, route)) return 0;
        if (resolve_copy_source_reg(view.receiver_value) != resolve_copy_source_reg(receiver_value) ||
            resolve_copy_source_reg(view.index_value) != resolve_copy_source_reg(index_value)) return 0;
        if (require_value &&
            (!view.has_value_value ||
             resolve_copy_source_reg(view.value_value) != resolve_copy_source_reg(value_value))) return 0;
        if (require_result &&
            (!view.has_result_value ||
             resolve_copy_source_reg(view.result_value) != resolve_copy_source_reg(result_value))) return 0;
        return 1;
      }

      auto int lowering_plan_direct_array_access_site_matches_proved_unchecked_range_index_store_i64(
          long long bid,
          size_t ii,
          const char* route,
          long long receiver_value,
          long long index_value,
          long long value_value,
          long long result_value,
          int require_result) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        if (!lowering_plan_direct_array_access_view_is_proved_unchecked_range_index_store_i64(&view, route)) return 0;
        if (resolve_copy_source_reg(view.receiver_value) != resolve_copy_source_reg(receiver_value) ||
            resolve_copy_source_reg(view.index_value) != resolve_copy_source_reg(index_value)) return 0;
        if (!view.has_value_value ||
            resolve_copy_source_reg(view.value_value) != resolve_copy_source_reg(value_value)) return 0;
        if (require_result &&
            (!view.has_result_value ||
             resolve_copy_source_reg(view.result_value) != resolve_copy_source_reg(result_value))) return 0;
        return 1;
      }

      auto int lowering_plan_direct_array_access_site_has_proved_unchecked_range_index_store_i64(
          long long bid,
          size_t ii,
          const char* route) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        return lowering_plan_direct_array_access_view_is_proved_unchecked_range_index_store_i64(
            &view,
            route);
      }

      auto int lowering_plan_direct_array_access_site_matches_proved_unchecked_stack_top_pop_load_i64(
          long long bid,
          size_t ii,
          const char* route,
          long long receiver_value,
          long long index_value,
          long long result_value,
          int require_result) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        if (!lowering_plan_direct_array_access_view_is_proved_unchecked_stack_top_pop_i64(&view, "load", route)) return 0;
        if (resolve_copy_source_reg(view.receiver_value) != resolve_copy_source_reg(receiver_value) ||
            resolve_copy_source_reg(view.index_value) != resolve_copy_source_reg(index_value)) return 0;
        if (require_result &&
            (!view.has_result_value ||
             resolve_copy_source_reg(view.result_value) != resolve_copy_source_reg(result_value))) return 0;
        return 1;
      }

      auto int lowering_plan_direct_array_access_site_matches_proved_unchecked_stack_top_pop_store_i64(
          long long bid,
          size_t ii,
          const char* route,
          long long receiver_value,
          long long index_value,
          long long value_value,
          long long result_value,
          int require_result) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        if (!lowering_plan_direct_array_access_view_is_proved_unchecked_stack_top_pop_i64(&view, "store", route)) return 0;
        if (resolve_copy_source_reg(view.receiver_value) != resolve_copy_source_reg(receiver_value) ||
            resolve_copy_source_reg(view.index_value) != resolve_copy_source_reg(index_value)) return 0;
        if (!view.has_value_value ||
            resolve_copy_source_reg(view.value_value) != resolve_copy_source_reg(value_value)) return 0;
        if (require_result &&
            (!view.has_result_value ||
             resolve_copy_source_reg(view.result_value) != resolve_copy_source_reg(result_value))) return 0;
        return 1;
      }

      auto int lowering_plan_direct_array_access_site_matches_proved_unchecked_caller_precondition_store_i64(
          long long bid,
          size_t ii,
          const char* route,
          long long receiver_value,
          long long index_value,
          long long value_value,
          long long result_value,
          int require_result) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        if (!lowering_plan_direct_array_access_view_is_proved_unchecked_caller_precondition_store_i64(&view, route)) return 0;
        if (resolve_copy_source_reg(view.receiver_value) != resolve_copy_source_reg(receiver_value) ||
            resolve_copy_source_reg(view.index_value) != resolve_copy_source_reg(index_value)) return 0;
        if (!view.has_value_value ||
            resolve_copy_source_reg(view.value_value) != resolve_copy_source_reg(value_value)) return 0;
        if (require_result &&
            (!view.has_result_value ||
             resolve_copy_source_reg(view.result_value) != resolve_copy_source_reg(result_value))) return 0;
        return 1;
      }

      auto int lowering_plan_entry_is_contract_manifest(yyjson_val* entry) {
        const char* proof = read_str(entry, "proof");
        return proof && !strcmp(proof, "core_method_contract_manifest");
      }

      auto int lowering_plan_entry_is_mir_json_flags_keys(yyjson_val* entry) {
        const char* source = entry ? read_str(entry, "source") : NULL;
        const char* route_id = entry ? read_str(entry, "source_route_id") : NULL;
        const char* proof = entry ? read_str(entry, "proof") : NULL;
        const char* route_proof = entry ? read_str(entry, "route_proof") : NULL;
        const char* route_kind = entry ? read_str(entry, "route_kind") : NULL;
        return source &&
               !strcmp(source, "generic_method_routes") &&
               route_id &&
               !strcmp(route_id, "generic_method.keys") &&
               proof &&
               !strcmp(proof, "mir_json_flags_keys") &&
               route_proof &&
               !strcmp(route_proof, "mir_json_flags_keys") &&
               route_kind &&
               !strcmp(route_kind, "map_keys_array");
      }

      auto int lowering_plan_entry_is_extern_registry(yyjson_val* entry) {
        const char* proof = read_str(entry, "proof");
        return proof && !strcmp(proof, "extern_registry");
      }

      auto int lowering_plan_proof_is_typed_global_call_contract(
          const char* proof) {
        return proof &&
               (!strcmp(proof, "typed_global_call_contract_missing") ||
                !strcmp(proof, "typed_global_call_leaf_numeric_i64") ||
                !strcmp(proof, "typed_global_call_generic_pure_string") ||
                !strcmp(proof, "typed_global_call_generic_string_or_void_sentinel") ||
                !strcmp(proof, "typed_global_call_generic_string_void_logging") ||
                !strcmp(proof, "typed_global_call_generic_i64") ||
                !strcmp(proof, "typed_global_call_parser_program_json") ||
                !strcmp(proof, "typed_global_call_stage1_emit_program_json") ||
                !strcmp(proof, "typed_global_call_static_string_array") ||
                !strcmp(proof, "typed_global_call_mir_schema_map_constructor") ||
                !strcmp(proof, "typed_global_call_box_type_inspector_describe") ||
                !strcmp(proof, "typed_global_call_same_module_object_handle") ||
                !strcmp(proof, "typed_global_call_same_module_scalar_i64") ||
                !strcmp(proof, "typed_global_call_same_module_void_sentinel") ||
                !strcmp(proof, "typed_global_call_same_module_mixed_runtime") ||
                !strcmp(proof, "typed_global_call_void_side_effect") ||
                !strcmp(proof, "typed_global_call_pattern_util_local_value_probe"));
      }

      auto int lowering_plan_entry_is_typed_global_call_contract(
          yyjson_val* entry) {
        return lowering_plan_proof_is_typed_global_call_contract(
            read_str(entry, "proof"));
      }

      auto int lowering_plan_proof_is_typed_user_box_method_contract(
          const char* proof) {
        return proof &&
               (!strcmp(proof, "typed_user_box_birth_same_module") ||
                !strcmp(proof, "typed_user_box_method_same_module") ||
                !strcmp(proof, "typed_user_box_method_contract_missing"));
      }

      auto int lowering_plan_entry_is_typed_user_box_method_contract(
          yyjson_val* entry) {
        return lowering_plan_proof_is_typed_user_box_method_contract(
            read_str(entry, "proof"));
      }

      auto int read_lowering_plan_generic_method_view(
          yyjson_val* entry,
          struct LoweringPlanGenericMethodView* out) {
        const char* source = entry ? read_str(entry, "source") : NULL;
        int invalid = 0;
        enum LoweringPlanTierMetadata tier =
            read_lowering_plan_tier_metadata(entry, &invalid);
        if (out) memset(out, 0, sizeof(*out));
        if (!(source && !strcmp(source, "generic_method_routes")) ||
            invalid ||
            !(lowering_plan_entry_is_contract_manifest(entry) ||
              lowering_plan_entry_is_mir_json_flags_keys(entry))) {
          return 0;
        }
        if (out) {
          out->entry = entry;
          out->route_id = read_str(entry, "source_route_id");
          out->core_op = read_str(entry, "core_op");
          out->route_kind = read_str(entry, "route_kind");
          out->symbol = read_str(entry, "symbol");
          out->route_proof = read_str(entry, "route_proof");
          out->receiver_origin_box = read_str(entry, "receiver_origin_box");
          out->result_origin_box = read_str(entry, "result_origin_box");
          out->key_route = read_str(entry, "key_route");
          out->return_shape = read_str(entry, "return_shape");
          out->value_demand = read_str(entry, "value_demand");
          out->publication_policy = read_str(entry, "publication_policy");
          out->tier = tier;
        }
        return out &&
               out->route_id &&
               out->core_op &&
               out->route_kind;
      }

      auto int lowering_plan_generic_method_view_matches_route_tuple(
          const struct LoweringPlanGenericMethodView* view,
          const char* route_id,
          const char* core_op,
          const char* route_kind,
          enum LoweringPlanTierMetadata tier) {
        if (!(view && route_id && core_op && route_kind)) return 0;
        return view->route_id &&
               !strcmp(view->route_id, route_id) &&
               view->core_op &&
               !strcmp(view->core_op, core_op) &&
               view->route_kind &&
               !strcmp(view->route_kind, route_kind) &&
               view->tier == tier;
      }

      auto int lowering_plan_generic_method_view_matches_symbolic_route(
          const struct LoweringPlanGenericMethodView* view,
          const char* route_id,
          const char* core_op,
          const char* route_kind,
          const char* symbol,
          const char* route_proof,
          enum LoweringPlanTierMetadata tier) {
        return lowering_plan_generic_method_view_matches_route_tuple(
                   view,
                   route_id,
                   core_op,
                   route_kind,
                   tier) &&
               view->symbol &&
               symbol &&
               !strcmp(view->symbol, symbol) &&
               view->route_proof &&
               route_proof &&
               !strcmp(view->route_proof, route_proof);
      }

      auto int read_lowering_plan_extern_call_view(
          yyjson_val* entry,
          struct LoweringPlanExternCallView* out) {
        const char* source = entry ? read_str(entry, "source") : NULL;
        int invalid = 0;
        enum LoweringPlanTierMetadata tier =
            read_lowering_plan_tier_metadata(entry, &invalid);
        long long result_value = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!(source && !strcmp(source, "extern_call_routes")) ||
            invalid ||
            !lowering_plan_entry_is_extern_registry(entry)) {
          return 0;
        }
        if (out) {
          out->entry = entry;
          out->route_id = read_str(entry, "source_route_id");
          out->source_symbol = read_str(entry, "source_symbol");
          out->core_op = read_str(entry, "core_op");
          out->route_kind = read_str(entry, "route_kind");
          out->emit_kind = read_str(entry, "emit_kind");
          out->symbol = read_str(entry, "symbol");
          out->proof = read_str(entry, "proof");
          out->route_proof = read_str(entry, "route_proof");
          out->return_shape = read_str(entry, "return_shape");
          out->value_demand = read_str(entry, "value_demand");
          if (lowering_plan_read_i64_field(entry, "result_value", &result_value)) {
            out->result_value = result_value;
            out->has_result_value = 1;
          }
          out->tier = tier;
        }
        return out &&
               out->route_id &&
               out->core_op &&
               out->route_kind &&
               out->emit_kind &&
               out->symbol &&
               out->proof;
      }

      auto int lowering_plan_extern_call_view_matches_runtime_route(
          const struct LoweringPlanExternCallView* view,
          const char* route_id,
          const char* core_op,
          const char* route_kind,
          const char* symbol,
          enum LoweringPlanTierMetadata tier) {
        return view &&
               view->route_id &&
               route_id &&
               !strcmp(view->route_id, route_id) &&
               view->core_op &&
               core_op &&
               !strcmp(view->core_op, core_op) &&
               view->route_kind &&
               route_kind &&
               !strcmp(view->route_kind, route_kind) &&
               view->emit_kind &&
               !strcmp(view->emit_kind, "runtime_call") &&
               view->symbol &&
               symbol &&
               !strcmp(view->symbol, symbol) &&
               view->proof &&
               !strcmp(view->proof, "extern_registry") &&
               view->route_proof &&
                !strcmp(view->route_proof, "extern_registry") &&
                view->tier == tier;
      }

      auto int lowering_plan_extern_call_view_result_origin_kind(
          const struct LoweringPlanExternCallView* view) {
        if (!(view && view->return_shape && view->value_demand)) return ORG_NONE;
        if (strcmp(view->value_demand, "runtime_i64_or_handle")) return ORG_NONE;
        if (!strcmp(view->return_shape, "string_handle") ||
            !strcmp(view->return_shape, "string_handle_or_null")) {
          return ORG_STRING;
        }
        return ORG_NONE;
      }

      auto int lowering_plan_extern_call_view_returns_scalar_i64(
          const struct LoweringPlanExternCallView* view) {
        return view &&
               view->return_shape &&
               !strcmp(view->return_shape, "scalar_i64") &&
               view->value_demand;
      }

      auto int lowering_plan_extern_call_view_returns_native_ptr_nullable_transport_i64(
          const struct LoweringPlanExternCallView* view) {
        return view &&
               view->return_shape &&
               !strcmp(view->return_shape, "native_ptr_nullable") &&
               view->value_demand &&
               !strcmp(view->value_demand, "native_ptr_nullable");
      }

      auto int lowering_plan_extern_call_view_returns_void_sentinel_i64_zero(
          const struct LoweringPlanExternCallView* view) {
        return view &&
               view->return_shape &&
               !strcmp(view->return_shape, "void_sentinel_i64_zero") &&
               view->value_demand &&
               !strcmp(view->value_demand, "scalar_i64");
      }

      auto int read_lowering_plan_global_call_view(
          yyjson_val* entry,
          struct LoweringPlanGlobalCallView* out) {
        const char* source = entry ? read_str(entry, "source") : NULL;
        int invalid = 0;
        enum LoweringPlanTierMetadata tier =
            read_lowering_plan_tier_metadata(entry, &invalid);
        long long target_arity = 0;
        long long result_value = 0;
        int target_exists = 0;
        int arity_matches = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!(source && !strcmp(source, "global_call_routes")) ||
            invalid ||
            !lowering_plan_entry_is_typed_global_call_contract(entry)) {
          return 0;
        }
        if (out) {
          out->entry = entry;
          out->route_id = read_str(entry, "source_route_id");
          out->callee_name = read_str(entry, "callee_name");
          out->target_symbol = read_str(entry, "target_symbol");
          out->target_return_type = read_str(entry, "target_return_type");
          out->target_result_box_name = read_str(entry, "target_result_box_name");
          out->core_op = read_str(entry, "core_op");
          out->route_kind = read_str(entry, "route_kind");
          out->emit_kind = read_str(entry, "emit_kind");
          out->proof = read_str(entry, "proof");
          out->route_proof = read_str(entry, "route_proof");
          out->reason = read_str(entry, "reason");
          out->target_shape_reason = read_str(entry, "target_shape_reason");
          out->target_shape_blocker_symbol = read_str(entry, "target_shape_blocker_symbol");
          out->target_shape_blocker_reason = read_str(entry, "target_shape_blocker_reason");
          out->return_shape = read_str(entry, "return_shape");
          out->value_demand = read_str(entry, "value_demand");
          out->result_origin = read_str(entry, "result_origin");
          out->arg0_origin_box = read_str(entry, "arg0_origin_box");
          out->definition_owner = read_str(entry, "definition_owner");
          out->emit_trace_consumer = read_str(entry, "emit_trace_consumer");
          lowering_plan_read_i64_field(entry, "arity", &out->arity);
          if (lowering_plan_read_i64_field(entry, "target_arity", &target_arity)) {
            out->target_arity = target_arity;
            out->has_target_arity = 1;
          }
          if (lowering_plan_read_i64_field(entry, "result_value", &result_value)) {
            out->result_value = result_value;
            out->has_result_value = 1;
          }
          if (lowering_plan_read_bool_field(entry, "target_exists", &target_exists)) {
            out->target_exists = target_exists;
          }
          if (lowering_plan_read_bool_field(entry, "arity_matches", &arity_matches)) {
            out->arity_matches = arity_matches;
            out->has_arity_matches = 1;
          }
          out->tier = tier;
        }
        return out &&
               out->route_id &&
               out->callee_name &&
               out->core_op &&
               out->route_kind &&
               out->emit_kind &&
               out->proof;
      }

      auto int lowering_plan_global_call_view_has_direct_target(
          const struct LoweringPlanGlobalCallView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "global.user_call") &&
               view->callee_name &&
               view->target_symbol &&
               view->core_op &&
               !strcmp(view->core_op, "UserGlobalCall") &&
               view->route_kind &&
               !strcmp(view->route_kind, "global.user_call") &&
               lowering_plan_proof_is_typed_global_call_contract(view->proof) &&
               view->target_exists &&
               view->has_target_arity &&
               view->has_arity_matches &&
               view->arity_matches &&
               view->target_arity == view->arity &&
               llvm_quoted_symbol_name_is_safe(view->target_symbol);
      }

      auto int lowering_plan_global_call_view_definition_owner_is(
          const struct LoweringPlanGlobalCallView* view,
          const char* owner) {
        return view &&
               view->definition_owner &&
               owner &&
               !strcmp(view->definition_owner, owner);
      }

      auto int lowering_plan_global_call_view_uses_uniform_mir_definition(
          const struct LoweringPlanGlobalCallView* view) {
        return lowering_plan_global_call_view_has_direct_target(view) &&
               lowering_plan_global_call_view_definition_owner_is(view, "uniform_mir");
      }

      auto int lowering_plan_global_call_view_uses_leaf_i64_definition(
          const struct LoweringPlanGlobalCallView* view) {
        return lowering_plan_global_call_view_has_direct_target(view) &&
               lowering_plan_global_call_view_definition_owner_is(view, "leaf_i64");
      }

      auto int lowering_plan_global_call_view_is_direct_void_sentinel_i64_zero(
          const struct LoweringPlanGlobalCallView* view) {
        return lowering_plan_global_call_view_has_direct_target(view) &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI &&
               view->emit_kind &&
               !strcmp(view->emit_kind, "direct_function_call") &&
               view->return_shape &&
               !strcmp(view->return_shape, "void_sentinel_i64_zero") &&
               view->value_demand &&
               !strcmp(view->value_demand, "scalar_i64");
      }

      auto int lowering_plan_global_call_view_uses_generic_i64_or_leaf_definition(
          const struct LoweringPlanGlobalCallView* view) {
        return lowering_plan_global_call_view_has_direct_target(view) &&
               lowering_plan_global_call_view_definition_owner_is(view, "generic_i64_or_leaf");
      }

      auto int lowering_plan_global_call_view_uses_module_generic_or_i64_definition(
          const struct LoweringPlanGlobalCallView* view) {
        return lowering_plan_global_call_view_has_direct_target(view) &&
               (lowering_plan_global_call_view_definition_owner_is(view, "module_generic") ||
                lowering_plan_global_call_view_uses_generic_i64_or_leaf_definition(view));
      }

      auto int lowering_plan_global_call_view_requires_same_module_function_definition(
          const struct LoweringPlanGlobalCallView* view) {
        return lowering_plan_global_call_view_uses_module_generic_or_i64_definition(view) ||
               lowering_plan_global_call_view_uses_uniform_mir_definition(view);
      }

      auto int read_lowering_plan_user_box_method_view(
          yyjson_val* entry,
          struct LoweringPlanUserBoxMethodView* out) {
        const char* source = entry ? read_str(entry, "source") : NULL;
        int invalid = 0;
        enum LoweringPlanTierMetadata tier =
            read_lowering_plan_tier_metadata(entry, &invalid);
        long long target_arity = 0;
        long long receiver_value = 0;
        long long result_value = 0;
        long long type_id = 0;
        int target_exists = 0;
        int arity_matches = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!(source && !strcmp(source, "user_box_method_routes")) ||
            invalid ||
            !lowering_plan_entry_is_typed_user_box_method_contract(entry)) {
          return 0;
        }
        if (out) {
          out->entry = entry;
          out->route_id = read_str(entry, "source_route_id");
          out->box_name = read_str(entry, "box_name");
          out->method = read_str(entry, "method");
          out->target_symbol = read_str(entry, "target_symbol");
          out->target_result_box_name = read_str(entry, "target_result_box_name");
          out->core_op = read_str(entry, "core_op");
          out->route_kind = read_str(entry, "route_kind");
          out->emit_kind = read_str(entry, "emit_kind");
          out->proof = read_str(entry, "proof");
          out->route_proof = read_str(entry, "route_proof");
          out->reason = read_str(entry, "reason");
          out->return_shape = read_str(entry, "return_shape");
          out->value_demand = read_str(entry, "value_demand");
          out->result_origin = read_str(entry, "result_origin");
          out->definition_owner = read_str(entry, "definition_owner");
          out->emit_trace_consumer = read_str(entry, "emit_trace_consumer");
          lowering_plan_read_i64_field(entry, "arity", &out->arity);
          if (lowering_plan_read_i64_field(entry, "target_arity", &target_arity)) {
            out->target_arity = target_arity;
            out->has_target_arity = 1;
          }
          if (lowering_plan_read_i64_field(entry, "receiver_value", &receiver_value)) {
            out->receiver_value = receiver_value;
            out->has_receiver_value = 1;
          }
          if (lowering_plan_read_i64_field(entry, "result_value", &result_value)) {
            out->result_value = result_value;
            out->has_result_value = 1;
          }
          if (lowering_plan_read_i64_field(entry, "type_id", &type_id)) {
            out->type_id = type_id;
            out->has_type_id = 1;
          }
          if (lowering_plan_read_bool_field(entry, "target_exists", &target_exists)) {
            out->target_exists = target_exists;
          }
          if (lowering_plan_read_bool_field(entry, "arity_matches", &arity_matches)) {
            out->arity_matches = arity_matches;
            out->has_arity_matches = 1;
          }
          out->tier = tier;
        }
        return out &&
               out->route_id &&
               out->box_name &&
               out->method &&
               out->target_symbol &&
               out->core_op &&
               out->route_kind &&
               out->emit_kind &&
               out->proof;
      }

      auto int lowering_plan_user_box_method_view_has_direct_target(
          const struct LoweringPlanUserBoxMethodView* view) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "user_box.method_call") &&
               view->core_op &&
               !strcmp(view->core_op, "UserBoxMethodCall") &&
               view->route_kind &&
               view->method &&
               view->proof &&
               view->route_proof &&
               ((view->method &&
                 !strcmp(view->method, "birth") &&
                 !strcmp(view->route_kind, "user_box.birth") &&
                 !strcmp(view->proof, "typed_user_box_birth_same_module") &&
                 !strcmp(view->route_proof, "typed_user_box_birth_same_module")) ||
                (strcmp(view->method, "birth") &&
                 !strcmp(view->route_kind, "user_box.method") &&
                 !strcmp(view->proof, "typed_user_box_method_same_module") &&
                 !strcmp(view->route_proof, "typed_user_box_method_same_module"))) &&
               view->target_exists &&
               view->has_target_arity &&
               view->has_arity_matches &&
               view->arity_matches &&
               view->target_arity == view->arity + 1 &&
               view->has_receiver_value &&
               view->has_type_id &&
               view->type_id > 0 &&
               llvm_quoted_symbol_name_is_safe(view->target_symbol);
      }

      auto int lowering_plan_user_box_method_view_requires_same_module_function_definition(
          const struct LoweringPlanUserBoxMethodView* view) {
        return lowering_plan_user_box_method_view_has_direct_target(view) &&
               view->definition_owner &&
               !strcmp(view->definition_owner, "typed_object_method");
      }

      auto int lowering_plan_user_box_method_view_is_direct_void_sentinel_i64_zero(
          const struct LoweringPlanUserBoxMethodView* view) {
        return lowering_plan_user_box_method_view_requires_same_module_function_definition(view) &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI &&
               view->emit_kind &&
               !strcmp(view->emit_kind, "direct_function_call") &&
               view->return_shape &&
               !strcmp(view->return_shape, "void_sentinel_i64_zero") &&
               view->value_demand &&
               !strcmp(view->value_demand, "scalar_i64");
      }

      auto int lowering_plan_user_box_method_view_is_direct_scalar_i64(
          const struct LoweringPlanUserBoxMethodView* view) {
        return lowering_plan_user_box_method_view_requires_same_module_function_definition(view) &&
               view->method &&
               strcmp(view->method, "birth") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI &&
               view->emit_kind &&
               !strcmp(view->emit_kind, "direct_function_call") &&
               view->return_shape &&
               !strcmp(view->return_shape, "scalar_i64") &&
               view->value_demand &&
               (!strcmp(view->value_demand, "scalar_i64") ||
                !strcmp(view->value_demand, "runtime_i64_or_handle"));
      }

      auto int lowering_plan_user_box_method_view_is_direct_runtime_i64_or_handle(
          const struct LoweringPlanUserBoxMethodView* view) {
        return lowering_plan_user_box_method_view_requires_same_module_function_definition(view) &&
               view->method &&
               strcmp(view->method, "birth") &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI &&
               view->emit_kind &&
               !strcmp(view->emit_kind, "direct_function_call") &&
               view->return_shape &&
               (!strcmp(view->return_shape, "string_handle") ||
                !strcmp(view->return_shape, "object_handle") ||
                !strcmp(view->return_shape, "mixed_runtime_i64_or_handle")) &&
               view->value_demand &&
               !strcmp(view->value_demand, "runtime_i64_or_handle");
      }

      auto int lowering_plan_user_box_method_view_result_origin_kind(
          const struct LoweringPlanUserBoxMethodView* view) {
        if (!(view && view->result_origin)) return ORG_NONE;
        if (!strcmp(view->result_origin, "string")) return ORG_STRING;
        return ORG_NONE;
      }

      auto int lowering_plan_global_call_view_matches_runtime_route(
          const struct LoweringPlanGlobalCallView* view,
          const char* route_kind,
          const char* proof,
          const char* symbol,
          const char* return_shape,
          const char* value_demand) {
        return view &&
               view->route_id &&
               !strcmp(view->route_id, "global.user_call") &&
               view->core_op &&
               !strcmp(view->core_op, "UserGlobalCall") &&
               view->route_kind &&
               route_kind &&
               !strcmp(view->route_kind, route_kind) &&
               view->proof &&
               proof &&
               !strcmp(view->proof, proof) &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME &&
               view->emit_kind &&
               !strcmp(view->emit_kind, "runtime_call") &&
               view->target_symbol &&
               symbol &&
               !strcmp(view->target_symbol, symbol) &&
               view->return_shape &&
               return_shape &&
               !strcmp(view->return_shape, return_shape) &&
               view->value_demand &&
               value_demand &&
               !strcmp(view->value_demand, value_demand) &&
               llvm_quoted_symbol_name_is_safe(view->target_symbol);
      }

      auto int lowering_plan_global_call_view_is_stage1_emit_program_json(
          const struct LoweringPlanGlobalCallView* view) {
        return lowering_plan_global_call_view_matches_runtime_route(
            view,
            "stage1.emit_program_json_v0",
            "typed_global_call_stage1_emit_program_json",
            "nyash.stage1.emit_program_json_v0_h",
            "string_handle",
            "runtime_i64_or_handle");
      }

      auto int lowering_plan_global_call_view_result_origin_kind(
          const struct LoweringPlanGlobalCallView* view) {
        if (!(view && view->result_origin)) return ORG_NONE;
        if (!strcmp(view->result_origin, "none")) return ORG_NONE;
        if (!strcmp(view->result_origin, "string")) {
          return ORG_STRING;
        }
        if (!strcmp(view->result_origin, "array_string_birth")) {
          return ORG_ARRAY_STRING_BIRTH;
        }
        if (!strcmp(view->result_origin, "map_birth")) {
          return ORG_MAP_BIRTH;
        }
        return ORG_NONE;
      }

      auto int lowering_plan_global_call_view_arg0_origin_is_string(
          const struct LoweringPlanGlobalCallView* view) {
        return view &&
               view->arg0_origin_box &&
               !strcmp(view->arg0_origin_box, "StringBox");
      }

      auto const char* lowering_plan_global_call_view_emit_trace_consumer(
          const struct LoweringPlanGlobalCallView* view) {
        if (view && view->emit_trace_consumer) return view->emit_trace_consumer;
        return "mir_call_global_unknown_emit";
      }

      auto void trace_lowering_plan_entry_hit(
          yyjson_val* entry,
          const char* consumer) {
        if (!hako_llvmc_route_trace_enabled()) return;
        const char* site = entry ? read_str(entry, "site") : NULL;
        const char* source_route_id = entry ? read_str(entry, "source_route_id") : NULL;
        const char* core_op = entry ? read_str(entry, "core_op") : NULL;
        const char* tier = entry ? read_str(entry, "tier") : NULL;
        const char* symbol = entry ? read_str(entry, "symbol") : NULL;
        char extra[384];
        snprintf(
            extra,
            sizeof(extra),
            "consumer=%s site=%s route=%s core_op=%s tier=%s symbol=%s",
            consumer ? consumer : "unknown",
            site ? site : "-",
            source_route_id ? source_route_id : "-",
            core_op ? core_op : "-",
            tier ? tier : "-",
            symbol ? symbol : "-");
        hako_llvmc_emit_route_trace(
            "lowering_plan",
            "hit",
            "mir_lowering_plan_v0",
            extra);
      }
