      enum LoweringPlanTierMetadata {
        HAKO_LLVMC_LOWERING_PLAN_TIER_NONE = 0,
        HAKO_LLVMC_LOWERING_PLAN_TIER_HOT_INLINE,
        HAKO_LLVMC_LOWERING_PLAN_TIER_DIRECT_ABI,
        HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME,
        HAKO_LLVMC_LOWERING_PLAN_TIER_UNSUPPORTED,
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
