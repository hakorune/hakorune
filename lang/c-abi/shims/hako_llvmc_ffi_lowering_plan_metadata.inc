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

      auto int lowering_plan_read_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
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

      auto int lowering_plan_entry_is_extern_registry(yyjson_val* entry) {
        const char* proof = read_str(entry, "proof");
        return proof && !strcmp(proof, "extern_registry");
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
            !lowering_plan_entry_is_contract_manifest(entry)) {
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
