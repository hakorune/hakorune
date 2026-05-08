      #include "hako_llvmc_ffi_string_chain_terms.inc"

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

      auto int string_concat_deferred_pair_can_materialize_later(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          long long lhs,
          long long rhs) {
        size_t use_ii = 0;
        long long final_reg = 0;
        yyjson_val* use_inst = NULL;
        const char* op = NULL;
        const char* bop = NULL;
        long long use_lhs = 0;
        long long use_rhs = 0;
        long long other = 0;
        if (!string_concat_arg_is_string_handle(lhs) ||
            !string_concat_arg_is_string_handle(rhs)) {
          return 0;
        }
        if (!follow_single_use_copy_chain_to_use(
                ii + 1, ilen, insts, dst, &use_ii, &final_reg, NULL, NULL)) {
          return 0;
        }
        use_inst = yyjson_arr_get(insts, use_ii);
        op = read_str(use_inst, "op");
        bop = read_str(use_inst, "operation");
        if (!(op && !strcmp(op, "binop") && bop && !strcmp(bop, "+"))) {
          return 1;
        }
        use_lhs = resolve_copy_chain_source_reg(insts, use_ii, read_int(use_inst, "lhs"));
        use_rhs = resolve_copy_chain_source_reg(insts, use_ii, read_int(use_inst, "rhs"));
        final_reg = resolve_copy_chain_source_reg(insts, use_ii, final_reg);
        if (use_lhs == final_reg) {
          other = use_rhs;
        } else if (use_rhs == final_reg) {
          other = use_lhs;
        } else {
          return 0;
        }
        return string_concat_arg_is_string_handle(other);
      }

      auto enum StringConcatPairRouteKind classify_string_concat_pair_route(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          long long lhs,
          long long rhs,
          int* direct_set_out,
          int* shared_receiver_out) {
        int direct_set = hako_llvmc_value_has_direct_set_consumer(dst);
        int shared_receiver = 0;
        if (get_str_const(rhs) != NULL) {
          shared_receiver = hako_llvmc_string_kernel_plan_shared_receiver(dst);
        }
        if (direct_set_out) *direct_set_out = direct_set || shared_receiver;
        if (shared_receiver_out) *shared_receiver_out = shared_receiver;
        if (should_defer_string_concat_pair(ii, ilen, insts, dst) &&
            string_concat_deferred_pair_can_materialize_later(
                ii, ilen, insts, dst, lhs, rhs)) {
          return HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_DEFER_PAIR;
        }
        if (get_str_const(rhs) != NULL && direct_set) {
          return HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONST_SUFFIX;
        }
        if (get_str_const(rhs) != NULL && shared_receiver) {
          return HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONST_SUFFIX_SHARED;
        }
        if (direct_set || shared_receiver) {
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
        if (get_str_const(dst) != NULL) {
          char route_extra[384];
          if (hako_llvmc_route_trace_enabled()) {
            snprintf(
                route_extra,
                sizeof(route_extra),
                "ii=%zu dst=%lld lhs=%lld rhs=%lld folded=1",
                ii,
                dst,
                lhs,
                rhs);
            hako_llvmc_emit_route_trace(
                "string_concat_add_route",
                hako_llvmc_fast_enabled() ? "const_fold_hoisted" : "const_fold_emit",
                "literal_pair",
                route_extra);
          }
          return emit_string_const_box_result(f, dst);
        }
        int direct_set = 0;
        int shared_receiver = 0;
        char route_extra[384];
        enum StringConcatPairRouteKind route = classify_string_concat_pair_route(
            ii, ilen, insts, dst, lhs, rhs, &direct_set, &shared_receiver);
        if (route == HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_DEFER_PAIR) {
          if (hako_llvmc_route_trace_enabled()) {
            snprintf(
                route_extra,
                sizeof(route_extra),
                "ii=%zu dst=%lld lhs=%lld rhs=%lld direct_set=%d shared_receiver=%d",
                ii,
                dst,
                lhs,
                rhs,
                direct_set,
                shared_receiver);
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
        if (direct_set && try_emit_string_insert_mid_const_call(f, ii, ilen, insts, dst, lhs, rhs)) {
          if (hako_llvmc_route_trace_enabled()) {
            snprintf(
                route_extra,
                sizeof(route_extra),
                "ii=%zu dst=%lld lhs=%lld rhs=%lld direct_set=%d shared_receiver=%d",
                ii,
                dst,
                lhs,
                rhs,
                direct_set,
                shared_receiver);
            hako_llvmc_emit_route_trace(
                "string_concat_add_route",
                shared_receiver ? "defer_insert_mid_kernel_slot_shared"
                                : "defer_insert_mid_kernel_slot",
                shared_receiver ? "insert_mid_shared_receiver"
                                : "insert_mid_const",
                route_extra);
          }
          return 1;
        }
        if (route == HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONST_SUFFIX ||
            route == HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONST_SUFFIX_SHARED) {
          /* Current concrete lowering for the owner-side `const_suffix` route.
           * phase-151x visibility lock keeps the intended canonical reading:
           *   thaw.str + lit.str + str.concat2 + freeze.str
           * while this shim now defers direct-set-only publication into the
           * array-store kernel-slot consumer, and now also lets the same
           * producer feed the active shared-receiver exact front
           * (`set(...)` + trailing `substring(...)`) without early publish.
           */
          if (try_emit_string_concat_const_suffix_call(
                  f,
                  ii,
                  ilen,
                  insts,
                  dst,
                  lhs,
                  rhs,
                  route == HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONST_SUFFIX_SHARED)) {
            if (hako_llvmc_route_trace_enabled()) {
              snprintf(
                  route_extra,
                  sizeof(route_extra),
                  "ii=%zu dst=%lld lhs=%lld rhs=%lld direct_set=%d shared_receiver=%d",
                  ii,
                  dst,
                  lhs,
                  rhs,
                  direct_set,
                  shared_receiver);
              hako_llvmc_emit_route_trace(
                  "string_concat_add_route",
                  route == HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONST_SUFFIX_SHARED
                      ? "defer_const_suffix_kernel_slot_shared"
                      : "defer_const_suffix_kernel_slot",
                  route == HAKO_LLVMC_STRING_CONCAT_PAIR_ROUTE_CONST_SUFFIX_SHARED
                      ? "const_suffix_shared_receiver"
                      : "const_suffix",
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
                "ii=%zu dst=%lld lhs=%lld rhs=%lld direct_set=%d shared_receiver=%d",
                ii,
                dst,
                lhs,
                rhs,
                direct_set,
                shared_receiver);
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
              "ii=%zu dst=%lld lhs=%lld rhs=%lld direct_set=%d shared_receiver=%d",
              ii,
              dst,
              lhs,
              rhs,
              direct_set,
              shared_receiver);
          hako_llvmc_emit_route_trace(
              "string_concat_add_route",
              "emit_concat_hh",
              "fallback_hh",
              route_extra);
        }
        emit_string_concat_hh_call(f, dst, lhs, rhs);
        return 1;
      }
