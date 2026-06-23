      /*
       * Array-string origin promotion helper.
       *
       * This consumes already-known value origins. It does not select a route.
       */
      auto int hako_llvmc_value_has_string_origin(long long reg) {
        long long base = resolve_copy_source_reg(reg);
        return get_origin(reg) == ORG_STRING ||
               get_scan_origin(reg) == ORG_STRING ||
               get_origin(base) == ORG_STRING ||
               get_scan_origin(base) == ORG_STRING;
      }

      auto void hako_llvmc_promote_array_birth_origin_to_string(long long recv) {
        promote_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
        promote_scan_origin_kind(ORG_ARRAY_BIRTH, ORG_ARRAY_STRING_BIRTH);
        if (recv) {
          set_origin(recv, ORG_ARRAY_STRING_BIRTH);
          set_scan_origin(recv, ORG_ARRAY_STRING_BIRTH);
        }
      }

      auto int hako_llvmc_promote_array_string_birth_if_value_is_string(
          long long recv,
          long long value,
          int route_allows_array_store) {
        if (!route_allows_array_store || !value) return 0;
        if (!hako_llvmc_value_has_string_origin(value)) return 0;
        hako_llvmc_promote_array_birth_origin_to_string(recv);
        return 1;
      }
