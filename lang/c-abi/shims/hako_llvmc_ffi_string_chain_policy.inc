      enum StringConcatPairRouteKind {
        HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_DEFER_PAIR = 1,
        HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONST_SUFFIX = 2,
        HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_INSERT_MID_CONST = 3,
        HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONCAT3_CHAIN = 4,
        HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONCAT_HH = 5
      };

      enum StringConcat3RouteKind {
        HAKO_LLVMC_STRING_CONCAT3_ROUTE_CHAIN = 1,
        HAKO_LLVMC_STRING_CONCAT3_ROUTE_FALLBACK = 2
      };

      auto enum StringConcat3RouteKind classify_string_concat3_route(
          long long lhs,
          long long rhs) {
        long long a = 0;
        long long b = 0;
        if (find_string_concat_pair(lhs, &a, &b)) {
          return HAKO_LLVMC_STRING_CONCAT3_ROUTE_CHAIN;
        }
        if (find_string_concat_pair(rhs, &a, &b)) {
          return HAKO_LLVMC_STRING_CONCAT3_ROUTE_CHAIN;
        }
        return HAKO_LLVMC_STRING_CONCAT3_ROUTE_FALLBACK;
      }

      auto enum StringConcatPairRouteKind classify_string_concat_pair_route(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          long long lhs,
          long long rhs,
          int* direct_set_out) {
        int direct_set = has_direct_array_set_consumer(ii, ilen, insts, dst);
        if (direct_set_out) *direct_set_out = direct_set;
        if (should_defer_string_concat_pair(ii, ilen, insts, dst)) {
          return HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_DEFER_PAIR;
        }
        if (get_str_const(rhs) != NULL && direct_set) {
          return HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONST_SUFFIX;
        }
        if (direct_set) {
          return HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_INSERT_MID_CONST;
        }
        if (classify_string_concat3_route(lhs, rhs) ==
            HAKO_LLVMC_STRING_CONCAT3_ROUTE_CHAIN) {
          return HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONCAT3_CHAIN;
        }
        return HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONCAT_HH;
      }

      auto int emit_string_concat_pair_by_policy(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          long long lhs,
          long long rhs) {
        int direct_set = 0;
        char route_extra[384];
        enum StringConcatPairRouteKind route = classify_string_concat_pair_route(
            ii, ilen, insts, dst, lhs, rhs, &direct_set);
        if (route == HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_DEFER_PAIR) {
          if (hako_llvmc_route_trace_enabled()) {
            snprintf(
                route_extra,
                sizeof(route_extra),
                "ii=%zu dst=%lld lhs=%lld rhs=%lld direct_set=%d",
                ii,
                dst,
                lhs,
                rhs,
                direct_set);
            hako_llvmc_emit_route_trace(
                "string_concat_add_route",
                "defer_pair",
                "pair_chain",
                route_extra);
          }
          remember_string_concat_pair(dst, lhs, rhs);
          finalize_string_concat_result(dst);
          return 1;
        }
        if (route == HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONST_SUFFIX) {
          if (try_emit_string_concat_const_suffix_call(f, ii, ilen, insts, dst, lhs, rhs)) {
            if (hako_llvmc_route_trace_enabled()) {
              snprintf(
                  route_extra,
                  sizeof(route_extra),
                  "ii=%zu dst=%lld lhs=%lld rhs=%lld direct_set=%d",
                  ii,
                  dst,
                  lhs,
                  rhs,
                  direct_set);
              hako_llvmc_emit_route_trace(
                  "string_concat_add_route",
                  "emit_concat_hs",
                  "const_suffix",
                  route_extra);
            }
            return 1;
          }
        }
        if (route == HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_INSERT_MID_CONST) {
          if (try_emit_string_insert_mid_const_call(f, ii, ilen, insts, dst, lhs, rhs)) {
            if (hako_llvmc_route_trace_enabled()) {
              snprintf(
                  route_extra,
                  sizeof(route_extra),
                  "ii=%zu dst=%lld lhs=%lld rhs=%lld direct_set=%d",
                  ii,
                  dst,
                  lhs,
                  rhs,
                  direct_set);
              hako_llvmc_emit_route_trace(
                  "string_concat_add_route",
                  "emit_insert_hsi",
                  "insert_mid_const",
                  route_extra);
            }
            return 1;
          }
        }
        if ((route == HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONCAT3_CHAIN ||
             route == HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_INSERT_MID_CONST) &&
            try_emit_string_concat3_chain_call(f, dst, lhs, rhs)) {
          if (hako_llvmc_route_trace_enabled()) {
            snprintf(
                route_extra,
                sizeof(route_extra),
                "ii=%zu dst=%lld lhs=%lld rhs=%lld direct_set=%d",
                ii,
                dst,
                lhs,
                rhs,
                direct_set);
            hako_llvmc_emit_route_trace(
                "string_concat_add_route",
                "emit_concat3_hhh",
                "concat3_chain",
                route_extra);
          }
          return 1;
        }
        if (hako_llvmc_route_trace_enabled()) {
          snprintf(
              route_extra,
              sizeof(route_extra),
              "ii=%zu dst=%lld lhs=%lld rhs=%lld direct_set=%d",
              ii,
              dst,
              lhs,
              rhs,
              direct_set);
          hako_llvmc_emit_route_trace(
              "string_concat_add_route",
              "emit_concat_hh",
              "fallback_hh",
              route_extra);
        }
        emit_string_concat_hh_call(f, dst, lhs, rhs);
        return 1;
      }
