      auto int try_emit_substring_concat3_piecewise_direct_set_call(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          yyjson_val* args) {
        long long prefix = 0;
        long long middle = 0;
        long long suffix = 0;
        long long start = 0;
        long long end = 0;
        long long source_prefix = 0;
        long long source_suffix = 0;
        long long prefix_start = 0;
        long long prefix_end = 0;
        long long suffix_start = 0;
        long long suffix_end = 0;
        long long len_src = 0;
        long long zero = 0;
        int used_plan_window = 0;
        const char* middle_const = NULL;
        const char* reason = "shape_match";
        char extra[256];

        if (!(dst && args && yyjson_is_arr(args) && yyjson_arr_size(args) == 5)) return 0;
        if (!hako_llvmc_string_kernel_plan_direct_set_consumer(dst)) return 0;

        prefix = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
        middle = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
        suffix = (long long)yyjson_get_sint(yyjson_arr_get(args, 2));
        start = (long long)yyjson_get_sint(yyjson_arr_get(args, 3));
        end = (long long)yyjson_get_sint(yyjson_arr_get(args, 4));

        prefix = resolve_copy_chain_source_reg(insts, ii + 1, prefix);
        middle = resolve_copy_chain_source_reg(insts, ii + 1, middle);
        suffix = resolve_copy_chain_source_reg(insts, ii + 1, suffix);
        middle_const = get_str_const(middle);
        if (!(prefix && middle && suffix && middle_const)) return 0;

        if (hako_llvmc_string_corridor_read_any_plan_source_window(
                prefix,
                &source_prefix,
                &prefix_start,
                &prefix_end) &&
            hako_llvmc_string_corridor_read_any_plan_source_window(
                suffix,
                &source_suffix,
                &suffix_start,
                &suffix_end) &&
            source_prefix &&
            source_prefix == source_suffix &&
            has_const(prefix_start, &zero) &&
            zero == 0 &&
            prefix_end &&
            prefix_end == suffix_start) {
          used_plan_window = 1;
          reason = "plan_window_match";
        }

        if (!used_plan_window) {
          if (!find_string_substring_call(prefix, &source_prefix, &prefix_start, &prefix_end))
            return 0;
          if (!find_string_substring_call(suffix, &source_suffix, &suffix_start, &suffix_end))
            return 0;
          source_prefix = resolve_copy_chain_source_reg(insts, ii + 1, source_prefix);
          source_suffix = resolve_copy_chain_source_reg(insts, ii + 1, source_suffix);
          prefix_start = resolve_copy_chain_source_reg(insts, ii + 1, prefix_start);
          prefix_end = resolve_copy_chain_source_reg(insts, ii + 1, prefix_end);
          suffix_start = resolve_copy_chain_source_reg(insts, ii + 1, suffix_start);
          suffix_end = resolve_copy_chain_source_reg(insts, ii + 1, suffix_end);
          source_prefix = resolve_copy_source_reg(source_prefix);
          source_suffix = resolve_copy_source_reg(source_suffix);
          prefix_start = resolve_copy_source_reg(prefix_start);
          prefix_end = resolve_copy_source_reg(prefix_end);
          suffix_start = resolve_copy_source_reg(suffix_start);
          suffix_end = resolve_copy_source_reg(suffix_end);
          if (!(source_prefix && source_prefix == source_suffix)) return 0;
          if (!(has_const(prefix_start, &zero) && zero == 0)) return 0;
          if (!(prefix_end && prefix_end == suffix_start)) return 0;
          if (!find_string_length_call(suffix_end, &len_src)) return 0;
          len_src = resolve_copy_chain_source_reg(insts, ii + 1, len_src);
          if (len_src != source_prefix) return 0;
        }

        remember_deferred_piecewise_subrange(dst, source_prefix, middle, prefix_end);
        remember_string_substring_call(dst, dst, start, end);
        fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
        finalize_string_concat_result(dst);
        if (hako_llvmc_route_trace_enabled()) {
          snprintf(
              extra,
              sizeof(extra),
              "ii=%zu dst=%lld source=%lld prefix=%lld middle=%lld suffix=%lld split=%lld start=%lld end=%lld plan=%d",
              ii,
              dst,
              source_prefix,
              prefix,
              middle,
              suffix,
              prefix_end,
              start,
              end,
              used_plan_window);
          hako_llvmc_emit_route_trace(
              "substring_concat3_piecewise_direct_set",
              "hit",
              reason,
              extra);
        }
        return 1;
      }

      auto int try_emit_string_concat3_chain_call(
          FILE* f,
          long long dst,
          long long lhs,
          long long rhs) {
        long long a = 0;
        long long b = 0;
        char ab[384];
        ab[0] = '\0';
        if (find_string_concat_pair(lhs, &a, &b)) {
          if (!string_concat_arg_is_string_handle(a) ||
              !string_concat_arg_is_string_handle(b) ||
              !string_concat_arg_is_string_handle(rhs)) {
            return 0;
          }
          append_string_concat_marshaled_arg_ref(f, ab, sizeof(ab), a, 1, dst, "a");
          append_string_concat_marshaled_arg_ref(f, ab, sizeof(ab), b, 0, dst, "b");
          append_string_concat_marshaled_arg_ref(f, ab, sizeof(ab), rhs, 0, dst, "rhs");
          fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat3_hhh\"(%s)\n", dst, ab);
          finalize_string_concat_result(dst);
          remember_string_concat_triple(dst, a, b, rhs);
          if (hako_llvmc_route_trace_enabled()) {
            char extra[256];
            snprintf(
                extra,
                sizeof(extra),
                "dst=%lld lhs=%lld rhs=%lld producer_kind=Concat3 boundary_kind=Store post_store_use=None known_len=-1 direct_set_only=0",
                dst,
                lhs,
                rhs);
            hako_llvmc_emit_route_trace("string_concat3_window", "hit", "pair_lhs_match", extra);
          }
          return 1;
        }
        if (find_string_concat_pair(rhs, &a, &b)) {
          if (!string_concat_arg_is_string_handle(lhs) ||
              !string_concat_arg_is_string_handle(a) ||
              !string_concat_arg_is_string_handle(b)) {
            return 0;
          }
          append_string_concat_marshaled_arg_ref(f, ab, sizeof(ab), lhs, 1, dst, "lhs");
          append_string_concat_marshaled_arg_ref(f, ab, sizeof(ab), a, 0, dst, "a");
          append_string_concat_marshaled_arg_ref(f, ab, sizeof(ab), b, 0, dst, "b");
          fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat3_hhh\"(%s)\n", dst, ab);
          finalize_string_concat_result(dst);
          remember_string_concat_triple(dst, lhs, a, b);
          if (hako_llvmc_route_trace_enabled()) {
            char extra[256];
            snprintf(
                extra,
                sizeof(extra),
                "dst=%lld lhs=%lld rhs=%lld producer_kind=Concat3 boundary_kind=Store post_store_use=None known_len=-1 direct_set_only=0",
                dst,
                lhs,
                rhs);
            hako_llvmc_emit_route_trace("string_concat3_window", "hit", "pair_rhs_match", extra);
          }
          return 1;
        }
        return 0;
      }
