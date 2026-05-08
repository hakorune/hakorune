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

      auto enum MirCallNeedKind classify_mir_call_constructor_need_kind(
          const char* bname) {
        static const struct MirCallNeedNameRule rules[] = {
          {"MapBox", HAKO_LLVMC_MIR_CALL_NEED_MAP_BIRTH},
          {"ArrayBox", HAKO_LLVMC_MIR_CALL_NEED_ARRAY_BIRTH},
        };
        return mir_call_need_kind_from_name_rules(bname, rules, ARR_LEN(rules));
      }

      auto enum MirCallNeedKind classify_mir_call_global_need_kind(
          const char* mname) {
        static const struct MirCallNeedNameRule rules[] = {
          {"print", HAKO_LLVMC_MIR_CALL_NEED_PRINTF},
        };
        return mir_call_need_kind_from_name_rules(mname, rules, ARR_LEN(rules));
      }

      auto enum MirCallNeedKind classify_mir_call_extern_need_kind(
          const char* mname) {
        static const struct MirCallNeedNameRule rules[] = {
          {"nyash.string.concat_hh", HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT},
          {"nyash.string.concat3_hhh", HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT},
          {"nyash.string.insert_hsi", HAKO_LLVMC_MIR_CALL_NEED_STRING_CONCAT},
          {"nyash.string.substring_hii", HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING},
          {"nyash.string.substring_concat_hhii",
           HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING},
          {"nyash.string.substring_concat3_hhhii",
           HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING},
          {"nyash.string.substring_len_hii",
           HAKO_LLVMC_MIR_CALL_NEED_STRING_SUBSTRING_LEN},
        };
        return mir_call_need_kind_from_name_rules(mname, rules, ARR_LEN(rules));
      }
