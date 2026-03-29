#include "hako_llvmc_ffi_string_concat_match.inc"

      auto void remember_string_concat_pair(long long dst, long long lhs, long long rhs) {
        if (!dst || !lhs || !rhs) return;
        for (size_t i = 0; i < string_concat_pairs_n; i++) {
          if (string_concat_pairs[i].dst != dst) continue;
          string_concat_pairs[i].lhs = lhs;
          string_concat_pairs[i].rhs = rhs;
          return;
        }
        if (string_concat_pairs_n < ARR_LEN(string_concat_pairs)) {
          string_concat_pairs[string_concat_pairs_n].dst = dst;
          string_concat_pairs[string_concat_pairs_n].lhs = lhs;
          string_concat_pairs[string_concat_pairs_n].rhs = rhs;
          string_concat_pairs_n++;
        }
      }

      auto void remember_string_concat_triple(
          long long dst,
          long long first,
          long long second,
          long long third) {
        if (!dst || !first || !second || !third) return;
        for (size_t i = 0; i < string_concat_triples_n; i++) {
          if (string_concat_triples[i].dst != dst) continue;
          string_concat_triples[i].first = first;
          string_concat_triples[i].second = second;
          string_concat_triples[i].third = third;
          return;
        }
        if (string_concat_triples_n < ARR_LEN(string_concat_triples)) {
          string_concat_triples[string_concat_triples_n].dst = dst;
          string_concat_triples[string_concat_triples_n].first = first;
          string_concat_triples[string_concat_triples_n].second = second;
          string_concat_triples[string_concat_triples_n].third = third;
          string_concat_triples_n++;
        }
      }

      auto void remember_string_length_call(long long dst, long long src) {
        if (!dst || !src) return;
        for (size_t i = 0; i < string_len_calls_n; i++) {
          if (string_len_calls[i].dst != dst) continue;
          string_len_calls[i].src = src;
          return;
        }
        if (string_len_calls_n < ARR_LEN(string_len_calls)) {
          string_len_calls[string_len_calls_n].dst = dst;
          string_len_calls[string_len_calls_n].src = src;
          string_len_calls_n++;
        }
      }

      auto void remember_string_substring_call(
          long long dst,
          long long src,
          long long start,
          long long end) {
        if (!dst || !src) return;
        for (size_t i = 0; i < string_substring_calls_n; i++) {
          if (string_substring_calls[i].dst != dst) continue;
          string_substring_calls[i].src = src;
          string_substring_calls[i].start = start;
          string_substring_calls[i].end = end;
          return;
        }
        if (string_substring_calls_n < ARR_LEN(string_substring_calls)) {
          string_substring_calls[string_substring_calls_n].dst = dst;
          string_substring_calls[string_substring_calls_n].src = src;
          string_substring_calls[string_substring_calls_n].start = start;
          string_substring_calls[string_substring_calls_n].end = end;
          string_substring_calls_n++;
        }
      }

      auto void append_string_concat_arg_ref(
          char* out,
          size_t out_cap,
          long long reg,
          int first) {
        long long cv = 0;
        if (has_const(reg, &cv)) {
          char buf[64];
          snprintf(buf, sizeof(buf), "%si64 %lld", first ? "" : ", ", cv);
          strncat(out, buf, out_cap - strlen(out) - 1);
          return;
        }
        append_i64_arg_ref(out, out_cap, reg, first);
      }

      auto void finalize_string_concat_result(long long dst) {
        if (!dst) return;
        set_type(dst, T_I64);
        set_origin(dst, ORG_STRING);
        set_scan_origin(dst, ORG_STRING);
      }

      auto void emit_string_concat_hh_call(
          FILE* f,
          long long dst,
          long long lhs,
          long long rhs) {
        char ab[128];
        ab[0] = '\0';
        append_string_concat_arg_ref(ab, sizeof(ab), lhs, 1);
        append_string_concat_arg_ref(ab, sizeof(ab), rhs, 0);
        if (dst) {
          fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat_hh\"(%s)\n", dst, ab);
          finalize_string_concat_result(dst);
          remember_string_concat_pair(dst, lhs, rhs);
        } else {
          fprintf(f, "  call i64 @\"nyash.string.concat_hh\"(%s)\n", ab);
        }
      }

      auto int try_emit_string_concat_const_suffix_call(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          long long lhs,
          long long rhs) {
        const char* rhs_const = get_str_const(rhs);
        if (!(dst && lhs && rhs_const)) return 0;
        if (!has_direct_array_set_consumer(ii, ilen, insts, dst)) return 0;
        fprintf(
            f,
            "  %%r%lld = call i64 @\"nyash.string.concat_hs\"(",
            dst);
        {
          char ab[192];
          size_t slen = strlen(rhs_const);
          ab[0] = '\0';
          append_i64_arg_ref(ab, sizeof(ab), lhs, 1);
          strncat(ab, ", ptr getelementptr inbounds (", sizeof(ab) - strlen(ab) - 1);
          {
            char tail[128];
            snprintf(
                tail,
                sizeof(tail),
                "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
                slen + 1,
                rhs);
            strncat(ab, tail, sizeof(ab) - strlen(ab) - 1);
          }
          fprintf(f, "%s)\n", ab);
        }
        finalize_string_concat_result(dst);
        return 1;
      }

      auto int try_emit_string_insert_mid_const_call(
          FILE* f,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst,
          long long lhs,
          long long rhs) {
        long long prefix = 0;
        long long middle = 0;
        long long src_prefix = 0;
        long long src_suffix = 0;
        long long prefix_start = 0;
        long long prefix_end = 0;
        long long suffix_start = 0;
        long long suffix_end = 0;
        long long len_src = 0;
        long long zero = 0;
        const char* middle_const = NULL;
        char ab[224];
        if (!(dst && lhs && rhs)) return 0;
        if (!has_direct_array_set_consumer(ii, ilen, insts, dst)) return 0;
        lhs = resolve_copy_chain_source_reg(insts, ii + 1, lhs);
        rhs = resolve_copy_chain_source_reg(insts, ii + 1, rhs);
        if (!find_string_concat_pair(lhs, &prefix, &middle)) return 0;
        prefix = resolve_copy_chain_source_reg(insts, ii + 1, prefix);
        middle = resolve_copy_chain_source_reg(insts, ii + 1, middle);
        middle_const = get_str_const(middle);
        if (!middle_const) return 0;
        if (!find_string_substring_call(prefix, &src_prefix, &prefix_start, &prefix_end)) return 0;
        if (!find_string_substring_call(rhs, &src_suffix, &suffix_start, &suffix_end)) return 0;
        src_prefix = resolve_copy_chain_source_reg(insts, ii + 1, src_prefix);
        src_suffix = resolve_copy_chain_source_reg(insts, ii + 1, src_suffix);
        prefix_start = resolve_copy_chain_source_reg(insts, ii + 1, prefix_start);
        prefix_end = resolve_copy_chain_source_reg(insts, ii + 1, prefix_end);
        suffix_start = resolve_copy_chain_source_reg(insts, ii + 1, suffix_start);
        suffix_end = resolve_copy_chain_source_reg(insts, ii + 1, suffix_end);
        if (!(src_prefix && src_prefix == src_suffix)) return 0;
        if (!(has_const(prefix_start, &zero) && zero == 0)) return 0;
        if (!(prefix_end && prefix_end == suffix_start)) return 0;
        if (!find_string_length_call(suffix_end, &len_src)) return 0;
        len_src = resolve_copy_chain_source_reg(insts, ii + 1, len_src);
        if (len_src != src_prefix) return 0;

        fprintf(f, "  %%r%lld = call i64 @\"nyash.string.insert_hsi\"(", dst);
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), src_prefix, 1);
        strncat(ab, ", ptr getelementptr inbounds (", sizeof(ab) - strlen(ab) - 1);
        {
          char tail[128];
          snprintf(
              tail,
              sizeof(tail),
              "[%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0)",
              strlen(middle_const) + 1,
              middle);
          strncat(ab, tail, sizeof(ab) - strlen(ab) - 1);
        }
        append_i64_arg_ref(ab, sizeof(ab), prefix_end, 0);
        fprintf(f, "%s)\n", ab);
        finalize_string_concat_result(dst);
        remember_string_concat_triple(dst, prefix, middle, rhs);
        return 1;
      }

      auto int try_emit_string_concat3_chain_call(
          FILE* f,
          long long dst,
          long long lhs,
          long long rhs) {
        long long a = 0;
        long long b = 0;
        char ab[192];
        ab[0] = '\0';
        if (find_string_concat_pair(lhs, &a, &b)) {
          append_string_concat_arg_ref(ab, sizeof(ab), a, 1);
          append_string_concat_arg_ref(ab, sizeof(ab), b, 0);
          append_string_concat_arg_ref(ab, sizeof(ab), rhs, 0);
          fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat3_hhh\"(%s)\n", dst, ab);
          finalize_string_concat_result(dst);
          remember_string_concat_triple(dst, a, b, rhs);
          return 1;
        }
        if (find_string_concat_pair(rhs, &a, &b)) {
          append_string_concat_arg_ref(ab, sizeof(ab), lhs, 1);
          append_string_concat_arg_ref(ab, sizeof(ab), a, 0);
          append_string_concat_arg_ref(ab, sizeof(ab), b, 0);
          fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat3_hhh\"(%s)\n", dst, ab);
          finalize_string_concat_result(dst);
          remember_string_concat_triple(dst, lhs, a, b);
          return 1;
        }
        return 0;
      }

      auto int emit_supported_string_extern_mir_call(
          FILE* f,
          const char* mname,
          long long dst,
          yyjson_val* args) {
        char ab[192];
        size_t argc = 0;
        if (!(mname && args && yyjson_is_arr(args))) return -1;
        argc = yyjson_arr_size(args);
        ab[0] = '\0';

        if (!strcmp(mname, "nyash.string.concat_hh")) {
          if (argc != 2) return -1;
          append_string_concat_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 0)), 1);
          append_string_concat_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 1)), 0);
          if (dst) {
            fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat_hh\"(%s)\n", dst, ab);
            finalize_string_concat_result(dst);
            remember_string_concat_pair(
                dst,
                (long long)yyjson_get_sint(yyjson_arr_get(args, 0)),
                (long long)yyjson_get_sint(yyjson_arr_get(args, 1)));
          } else {
            fprintf(f, "  call i64 @\"nyash.string.concat_hh\"(%s)\n", ab);
          }
          return 1;
        }

        if (!strcmp(mname, "nyash.string.concat3_hhh")) {
          if (argc != 3) return -1;
          append_string_concat_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 0)), 1);
          append_string_concat_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 1)), 0);
          append_string_concat_arg_ref(
              ab, sizeof(ab), (long long)yyjson_get_sint(yyjson_arr_get(args, 2)), 0);
          if (dst) {
            fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat3_hhh\"(%s)\n", dst, ab);
            finalize_string_concat_result(dst);
            remember_string_concat_triple(
                dst,
                (long long)yyjson_get_sint(yyjson_arr_get(args, 0)),
                (long long)yyjson_get_sint(yyjson_arr_get(args, 1)),
                (long long)yyjson_get_sint(yyjson_arr_get(args, 2)));
          } else {
            fprintf(f, "  call i64 @\"nyash.string.concat3_hhh\"(%s)\n", ab);
          }
          return 1;
        }

        return -1;
      }
