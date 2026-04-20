#include "hako_llvmc_ffi_string_concat_match.inc"

      auto int emit_piecewise_slot_hop_publish_call(
          FILE* f,
          size_t ii,
          long long intermediate_value,
          long long final_dst,
          long long source_h,
          long long middle_reg,
          long long split,
          long long start,
          long long end,
          long long hop_start,
          long long hop_end,
          const char* route_reason);

      auto int hako_llvmc_string_kernel_plan_shared_receiver(long long dst) {
        int shared_receiver = 0;
        if (!dst) return 0;
        if (hako_llvmc_string_kernel_plan_read_alias_fact(
                dst, "shared_receiver", &shared_receiver)) {
          return shared_receiver;
        }
        return 0;
      }

      auto int hako_llvmc_string_kernel_plan_direct_set_consumer(long long dst) {
        int direct_set_consumer = 0;
        if (!dst) return 0;
        if (hako_llvmc_string_kernel_plan_read_alias_fact(
                dst, "direct_set_consumer", &direct_set_consumer)) {
          return direct_set_consumer;
        }
        return 0;
      }

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

      auto void remember_deferred_const_suffix_kernel_slot(
          long long dst,
          long long lhs,
          long long rhs) {
        if (!dst || !lhs || !rhs) return;
        for (size_t i = 0; i < deferred_const_suffix_kernel_slots_n; i++) {
          if (deferred_const_suffix_kernel_slots[i].dst != dst) continue;
          deferred_const_suffix_kernel_slots[i].lhs = lhs;
          deferred_const_suffix_kernel_slots[i].rhs = rhs;
          return;
        }
        if (deferred_const_suffix_kernel_slots_n <
            ARR_LEN(deferred_const_suffix_kernel_slots)) {
          deferred_const_suffix_kernel_slots[deferred_const_suffix_kernel_slots_n].dst = dst;
          deferred_const_suffix_kernel_slots[deferred_const_suffix_kernel_slots_n].lhs = lhs;
          deferred_const_suffix_kernel_slots[deferred_const_suffix_kernel_slots_n].rhs = rhs;
          deferred_const_suffix_kernel_slots_n++;
        }
      }

      auto void remember_deferred_insert_mid_kernel_slot(
          long long dst,
          long long source,
          long long middle,
          long long split) {
        if (!dst || !source || !middle) return;
        for (size_t i = 0; i < deferred_insert_mid_kernel_slots_n; i++) {
          if (deferred_insert_mid_kernel_slots[i].dst != dst) continue;
          deferred_insert_mid_kernel_slots[i].source = source;
          deferred_insert_mid_kernel_slots[i].middle = middle;
          deferred_insert_mid_kernel_slots[i].split = split;
          return;
        }
        if (deferred_insert_mid_kernel_slots_n <
            ARR_LEN(deferred_insert_mid_kernel_slots)) {
          deferred_insert_mid_kernel_slots[deferred_insert_mid_kernel_slots_n].dst = dst;
          deferred_insert_mid_kernel_slots[deferred_insert_mid_kernel_slots_n].source = source;
          deferred_insert_mid_kernel_slots[deferred_insert_mid_kernel_slots_n].middle = middle;
          deferred_insert_mid_kernel_slots[deferred_insert_mid_kernel_slots_n].split = split;
          deferred_insert_mid_kernel_slots_n++;
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

      auto void remember_deferred_piecewise_subrange(
          long long dst,
          long long source,
          long long middle,
          long long split) {
        if (!dst || !source || !middle) return;
        for (size_t i = 0; i < deferred_piecewise_subranges_n; i++) {
          if (deferred_piecewise_subranges[i].dst != dst) continue;
          deferred_piecewise_subranges[i].source = source;
          deferred_piecewise_subranges[i].middle = middle;
          deferred_piecewise_subranges[i].split = split;
          return;
        }
        if (deferred_piecewise_subranges_n < ARR_LEN(deferred_piecewise_subranges)) {
          deferred_piecewise_subranges[deferred_piecewise_subranges_n].dst = dst;
          deferred_piecewise_subranges[deferred_piecewise_subranges_n].source = source;
          deferred_piecewise_subranges[deferred_piecewise_subranges_n].middle = middle;
          deferred_piecewise_subranges[deferred_piecewise_subranges_n].split = split;
          deferred_piecewise_subranges_n++;
        }
      }

      auto int find_deferred_piecewise_subrange(
          long long reg,
          long long* source,
          long long* middle,
          long long* split) {
        reg = resolve_copy_source_reg(reg);
        if (!reg) return 0;
        for (size_t i = 0; i < deferred_piecewise_subranges_n; i++) {
          if (deferred_piecewise_subranges[i].dst != reg) continue;
          if (source) *source = deferred_piecewise_subranges[i].source;
          if (middle) *middle = deferred_piecewise_subranges[i].middle;
          if (split) *split = deferred_piecewise_subranges[i].split;
          return 1;
        }
        return 0;
      }

      auto int find_deferred_insert_mid_kernel_slot(
          long long reg,
          long long* source,
          long long* middle,
          long long* split) {
        reg = resolve_copy_source_reg(reg);
        if (!reg) return 0;
        for (size_t i = 0; i < deferred_insert_mid_kernel_slots_n; i++) {
          if (deferred_insert_mid_kernel_slots[i].dst != reg) continue;
          if (source) *source = deferred_insert_mid_kernel_slots[i].source;
          if (middle) *middle = deferred_insert_mid_kernel_slots[i].middle;
          if (split) *split = deferred_insert_mid_kernel_slots[i].split;
          return 1;
        }
        return 0;
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

      auto int emit_string_const_box_result(FILE* f, long long dst) {
        const char* sval = get_str_const(dst);
        if (!sval) return 0;
        if (hako_llvmc_fast_enabled()) {
          finalize_string_concat_result(dst);
          return 1;
        }
        size_t slen = strlen(sval);
        fprintf(
            f,
            "  %%r%lld = call i64 @\"nyash.box.from_i8_string_const\"(ptr getelementptr inbounds ([%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0))\n",
            dst,
            slen + 1,
            dst);
        finalize_string_concat_result(dst);
        return 1;
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
          long long rhs,
          int emit_placeholder) {
        const char* rhs_const = get_str_const(rhs);
        if (!(dst && lhs && rhs_const)) return 0;
        if (!emit_placeholder && !hako_llvmc_value_has_direct_set_consumer(dst)) return 0;
        remember_deferred_const_suffix_kernel_slot(dst, lhs, rhs);
        if (emit_placeholder) {
          fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
        }
        finalize_string_concat_result(dst);
        return 1;
      }
