      /*
       * MIR call declaration need name fallback.
       *
       * These rows are compatibility fallbacks for surfaces that are not yet
       * represented by typed route metadata at the call site.
       */
      struct MirCallNeedNameRule {
        const char* name;
        enum MirCallNeedKind need_kind;
      };

      enum MirCallNeedNameFallbackSurface {
        HAKO_LLVMC_MIR_CALL_NEED_NAME_FALLBACK_GLOBAL = 2,
      };

      auto const char* mir_call_need_name_fallback_surface_name(
          enum MirCallNeedNameFallbackSurface surface) {
        switch (surface) {
          case HAKO_LLVMC_MIR_CALL_NEED_NAME_FALLBACK_GLOBAL:
            return "global";
          default:
            return "unknown";
        }
      }

      auto void record_mir_call_need_name_fallback(
          struct MirCallNeedFlags* needs,
          enum MirCallNeedNameFallbackSurface surface,
          const char* name,
          enum MirCallNeedKind need_kind,
          long long bid,
          size_t ii) {
        if (!(needs && need_kind != HAKO_LLVMC_MIR_CALL_NEED_NONE)) return;
        switch (surface) {
          case HAKO_LLVMC_MIR_CALL_NEED_NAME_FALLBACK_GLOBAL:
            needs->name_fallback_global_hits++;
            break;
          default:
            break;
        }
        if (hako_llvmc_route_trace_enabled()) {
          fprintf(
              stderr,
              "[mir-call/need-name-fallback] surface=%s block=%lld inst=%zu name=%s need_kind=%d\n",
              mir_call_need_name_fallback_surface_name(surface),
              bid,
              ii,
              name ? name : "",
              (int)need_kind);
        }
      }

      auto enum MirCallNeedKind mir_call_need_kind_from_name_rules(
          const char* name,
          const struct MirCallNeedNameRule* rules,
          size_t rule_count) {
        size_t ri = 0;
        if (!(name && rules)) return HAKO_LLVMC_MIR_CALL_NEED_NONE;
        for (ri = 0; ri < rule_count; ri++) {
          if (rules[ri].name && !strcmp(name, rules[ri].name)) {
            return rules[ri].need_kind;
          }
        }
        return HAKO_LLVMC_MIR_CALL_NEED_NONE;
      }

      auto enum MirCallNeedKind classify_mir_call_global_need_kind(
          const char* mname) {
        static const struct MirCallNeedNameRule rules[] = {
          {"print", HAKO_LLVMC_MIR_CALL_NEED_PRINTF},
        };
        return mir_call_need_kind_from_name_rules(mname, rules, ARR_LEN(rules));
      }
