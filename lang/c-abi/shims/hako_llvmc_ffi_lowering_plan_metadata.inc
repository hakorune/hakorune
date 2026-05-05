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
        enum LoweringPlanTierMetadata tier;
      };

      struct LoweringPlanGlobalCallView {
        yyjson_val* entry;
        const char* route_id;
        const char* callee_name;
        const char* target_symbol;
        const char* target_return_type;
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

      auto int lowering_plan_read_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
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
                !strcmp(proof, "typed_global_call_static_string_array") ||
                !strcmp(proof, "typed_global_call_mir_schema_map_constructor") ||
                !strcmp(proof, "typed_global_call_box_type_inspector_describe") ||
                !strcmp(proof, "typed_global_call_pattern_util_local_value_probe"));
      }

      auto int lowering_plan_entry_is_typed_global_call_contract(
          yyjson_val* entry) {
        return lowering_plan_proof_is_typed_global_call_contract(
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

      auto int read_lowering_plan_extern_call_view(
          yyjson_val* entry,
          struct LoweringPlanExternCallView* out) {
        const char* source = entry ? read_str(entry, "source") : NULL;
        int invalid = 0;
        enum LoweringPlanTierMetadata tier =
            read_lowering_plan_tier_metadata(entry, &invalid);
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
