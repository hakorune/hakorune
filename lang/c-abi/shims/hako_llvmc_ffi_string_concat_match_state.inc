      struct StringConcatPairRec {
        long long dst;
        long long lhs;
        long long rhs;
      };
      struct StringConcatTripleRec {
        long long dst;
        long long first;
        long long second;
        long long third;
      };
      struct StringLenCallRec {
        long long dst;
        long long src;
      };
      struct StringSubstringCallRec {
        long long dst;
        long long src;
        long long start;
        long long end;
      };
      struct DeferredPiecewiseSubrangeRec {
        long long dst;
        long long source;
        long long middle;
        long long split;
      };
      struct DeferredInsertMidKernelSlotRec {
        long long dst;
        long long source;
        long long middle;
        long long split;
      };
      struct DeferredConstSuffixKernelSlotRec {
        long long dst;
        long long lhs;
        long long rhs;
      };
      struct StringConcatPairRec string_concat_pairs[512];
      size_t string_concat_pairs_n = 0;
      struct StringConcatTripleRec string_concat_triples[256];
      size_t string_concat_triples_n = 0;
      struct StringLenCallRec string_len_calls[256];
      size_t string_len_calls_n = 0;
      struct StringSubstringCallRec string_substring_calls[256];
      size_t string_substring_calls_n = 0;
      struct DeferredPiecewiseSubrangeRec deferred_piecewise_subranges[128];
      size_t deferred_piecewise_subranges_n = 0;
      struct DeferredInsertMidKernelSlotRec deferred_insert_mid_kernel_slots[128];
      size_t deferred_insert_mid_kernel_slots_n = 0;
      struct DeferredConstSuffixKernelSlotRec deferred_const_suffix_kernel_slots[256];
      size_t deferred_const_suffix_kernel_slots_n = 0;

      auto void remember_string_concat_triple(
          long long dst,
          long long first,
          long long second,
          long long third);
      auto void remember_deferred_const_suffix_kernel_slot(
          long long dst,
          long long lhs,
          long long rhs);
      auto void remember_deferred_insert_mid_kernel_slot(
          long long dst,
          long long source,
          long long middle,
          long long split);
      auto void finalize_string_concat_result(long long dst);
      auto int should_skip_string_const_box_for_insert_mid_window(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long reg);

      auto int find_string_concat_pair(long long reg, long long* lhs, long long* rhs) {
        if (!reg) return 0;
        for (size_t i = 0; i < string_concat_pairs_n; i++) {
          if (string_concat_pairs[i].dst != reg) continue;
          if (lhs) *lhs = string_concat_pairs[i].lhs;
          if (rhs) *rhs = string_concat_pairs[i].rhs;
          return 1;
        }
        return 0;
      }

      auto int find_deferred_const_suffix_kernel_slot(
          long long reg,
          long long* lhs,
          long long* rhs) {
        reg = resolve_copy_source_reg(reg);
        if (!reg) return 0;
        for (size_t i = 0; i < deferred_const_suffix_kernel_slots_n; i++) {
          if (deferred_const_suffix_kernel_slots[i].dst != reg) continue;
          if (lhs) *lhs = deferred_const_suffix_kernel_slots[i].lhs;
          if (rhs) *rhs = deferred_const_suffix_kernel_slots[i].rhs;
          return 1;
        }
        return 0;
      }

      auto int find_string_concat_triple(
          long long reg,
          long long* first,
          long long* second,
          long long* third) {
        if (!reg) return 0;
        for (size_t i = 0; i < string_concat_triples_n; i++) {
          if (string_concat_triples[i].dst != reg) continue;
          if (first) *first = string_concat_triples[i].first;
          if (second) *second = string_concat_triples[i].second;
          if (third) *third = string_concat_triples[i].third;
          return 1;
        }
        return 0;
      }

      auto int find_string_length_call(long long reg, long long* src) {
        if (!reg) return 0;
        for (size_t i = 0; i < string_len_calls_n; i++) {
          if (string_len_calls[i].dst != reg) continue;
          if (src) *src = string_len_calls[i].src;
          return 1;
        }
        return 0;
      }

      auto int find_string_substring_call(
          long long reg,
          long long* src,
          long long* start,
          long long* end) {
        if (!reg) return 0;
        for (size_t i = 0; i < string_substring_calls_n; i++) {
          if (string_substring_calls[i].dst != reg) continue;
          if (src) *src = string_substring_calls[i].src;
          if (start) *start = string_substring_calls[i].start;
          if (end) *end = string_substring_calls[i].end;
          return 1;
        }
        return 0;
      }

      auto int known_string_value_len_depth(
          long long reg,
          long long* out_len,
          int depth);

      auto int known_string_concat_chain_len(long long reg, long long* out_len) {
        long long lhs = 0;
        long long rhs = 0;
        long long first = 0;
        long long second = 0;
        long long third = 0;
        long long total = 0;
        reg = resolve_copy_source_reg(reg);
        if (!reg) return 0;
        if (find_string_concat_pair(reg, &lhs, &rhs)) {
          long long lhs_len = 0;
          long long rhs_len = 0;
          if (!known_string_value_len_depth(lhs, &lhs_len, 0)) return 0;
          if (!known_string_value_len_depth(rhs, &rhs_len, 0)) return 0;
          total = lhs_len + rhs_len;
          if (out_len) *out_len = total;
          return 1;
        }
        if (find_string_concat_triple(reg, &first, &second, &third)) {
          long long first_len = 0;
          long long second_len = 0;
          long long third_len = 0;
          if (!known_string_value_len_depth(first, &first_len, 0)) return 0;
          if (!known_string_value_len_depth(second, &second_len, 0)) return 0;
          if (!known_string_value_len_depth(third, &third_len, 0)) return 0;
          total = first_len + second_len + third_len;
          if (out_len) *out_len = total;
          return 1;
        }
        return 0;
      }

      auto int known_string_value_len_depth(
          long long reg,
          long long* out_len,
          int depth) {
        long long nested_len = 0;
        const char* text = NULL;
        reg = resolve_copy_source_reg(reg);
        if (!reg || depth > 8) return 0;
        if (get_known_string_len(reg, &nested_len)) {
          if (out_len) *out_len = nested_len;
          return 1;
        }
        text = get_str_const(reg);
        if (text != NULL) {
          if (out_len) *out_len = (long long)strlen(text);
          return 1;
        }
        if (known_string_concat_chain_len(reg, &nested_len)) {
          if (out_len) *out_len = nested_len;
          return 1;
        }
        return 0;
      }

      auto int known_string_value_len(long long reg, long long* out_len) {
        return known_string_value_len_depth(reg, out_len, 0);
      }

      auto const char* string_concat_producer_kind_for_reg(long long reg) {
        long long lhs = 0;
        long long rhs = 0;
        long long first = 0;
        long long second = 0;
        long long third = 0;
        long long src = 0;
        long long start = 0;
        long long end = 0;
        long long base = resolve_copy_source_reg(reg);
        if (!base) return "Unknown";
        if (find_string_concat_triple(base, &first, &second, &third)) return "Concat3";
        if (find_string_concat_pair(base, &lhs, &rhs)) return "ConcatPair";
        if (find_string_substring_call(base, &src, &start, &end)) return "Substring";
        if (get_str_const(base) != NULL) return "ConstString";
        return "Unknown";
      }

      auto int is_known_string_concat_operand(long long reg) {
        if (!reg) return 0;
        if (get_origin(reg) == ORG_STRING) return 1;
        if (get_scan_origin(reg) == ORG_STRING) return 1;
        return get_str_const(reg) != NULL;
      }

      auto int inst_defines_string_concat_operand(
          yyjson_val* ins,
          yyjson_val* insts,
          size_t start_ii,
          size_t end_ii,
          int depth,
          long long reg);

      auto int future_reg_becomes_string_operand(
          yyjson_val* insts,
          size_t start_ii,
          size_t end_ii,
          int depth,
          long long reg) {
        if (!reg) return 0;
        if (is_known_string_concat_operand(reg)) return 1;
        if (depth > 8) return 0;
        for (size_t jj = end_ii; jj-- > start_ii;) {
          yyjson_val* later = yyjson_arr_get(insts, jj);
          if (read_int(later, "dst") != reg) continue;
          return inst_defines_string_concat_operand(
              later, insts, start_ii, jj, depth + 1, reg);
        }
        return 0;
      }

      auto int inst_defines_string_concat_operand(
          yyjson_val* ins,
          yyjson_val* insts,
          size_t start_ii,
          size_t end_ii,
          int depth,
          long long reg) {
        const char* op = read_str(ins, "op");
        if (!op) return 0;
        if (!strcmp(op, "copy")) {
          return future_reg_becomes_string_operand(
              insts, start_ii, end_ii, depth, read_int(ins, "src"));
        }
        if (!strcmp(op, "const")) {
          return get_str_const(reg) != NULL;
        }
        if (!strcmp(op, "newbox")) {
          const char* tname = read_str(ins, "type");
          return tname && !strcmp(tname, "StringBox");
        }
        if (!strcmp(op, "mir_call")) {
          yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
          yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
          const char* ctype = cal ? read_str(cal, "type") : NULL;
          const char* mname =
              cal ? (read_str(cal, "method") ? read_str(cal, "method")
                                             : read_str(cal, "name"))
                  : NULL;
          return ctype && !strcmp(ctype, "Method") &&
                 mname && !strcmp(mname, "substring");
        }
        if (!strcmp(op, "binop")) {
          const char* k = read_str(ins, "op_kind");
          long long lhs = 0;
          long long rhs = 0;
          if (!k) k = read_str(ins, "operation");
          if (!(k && (!strcmp(k, "Add") || !strcmp(k, "+")))) return 0;
          lhs = read_int(ins, "lhs");
          rhs = read_int(ins, "rhs");
          return future_reg_becomes_string_operand(insts, start_ii, end_ii, depth, lhs) &&
                 future_reg_becomes_string_operand(insts, start_ii, end_ii, depth, rhs);
        }
        return 0;
      }

      auto int is_string_concat_chain_consumer_inst(
          yyjson_val* ins,
          yyjson_val* insts,
          size_t start_ii,
          size_t use_ii,
          long long reg) {
        const char* op = read_str(ins, "op");
        const char* k = NULL;
        long long lhs = 0;
        long long rhs = 0;
        long long other = 0;
        if (!(op && !strcmp(op, "binop"))) return 0;
        k = read_str(ins, "op_kind");
        if (!k) k = read_str(ins, "operation");
        if (!(k && (!strcmp(k, "Add") || !strcmp(k, "+")))) return 0;
        lhs = read_int(ins, "lhs");
        rhs = read_int(ins, "rhs");
        if (lhs != reg && rhs != reg) return 0;
        other = lhs == reg ? rhs : lhs;
        if (is_known_string_concat_operand(other)) return 1;
        return future_reg_becomes_string_operand(insts, start_ii, use_ii, 0, other);
      }

      auto int is_string_concat_len_consumer_inst(
          yyjson_val* ins,
          long long reg) {
        yyjson_val* mc = NULL;
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        const char* op = read_str(ins, "op");
        const char* ctype = NULL;
        const char* mname = NULL;
        long long recv = 0;
        long long known_len = 0;
        if (!(op && !strcmp(op, "mir_call"))) return 0;
        mc = yyjson_obj_get(ins, "mir_call");
        cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        args = mc ? yyjson_obj_get(mc, "args") : NULL;
        ctype = cal ? read_str(cal, "type") : NULL;
        mname = cal ? (read_str(cal, "method") ? read_str(cal, "method")
                                               : read_str(cal, "name"))
                    : NULL;
        recv = cal ? resolve_copy_source_reg(read_int(cal, "receiver")) : 0;
        if (!(ctype && !strcmp(ctype, "Method"))) return 0;
        if (!(mname &&
              (!strcmp(mname, "length") || !strcmp(mname, "len") ||
               !strcmp(mname, "size")))) {
          return 0;
        }
        if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) == 0)) return 0;
        if (recv != resolve_copy_source_reg(reg)) return 0;
        return known_string_concat_chain_len(recv, &known_len);
      }

      auto int resolve_string_concat_chain_terms(
          long long reg,
          long long* first,
          long long* second,
          long long* third,
          int* term_count) {
        long long lhs = 0;
        long long rhs = 0;
        long long nested_a = 0;
        long long nested_b = 0;
        long long nested_c = 0;
        int lhs_nested = 0;
        int rhs_nested = 0;
        reg = resolve_copy_source_reg(reg);
        if (!reg) return 0;
        if (find_string_concat_triple(reg, &nested_a, &nested_b, &nested_c)) {
          if (first) *first = resolve_copy_source_reg(nested_a);
          if (second) *second = resolve_copy_source_reg(nested_b);
          if (third) *third = resolve_copy_source_reg(nested_c);
          if (term_count) *term_count = 3;
          return 1;
        }
        if (!find_string_concat_pair(reg, &lhs, &rhs)) return 0;
        lhs = resolve_copy_source_reg(lhs);
        rhs = resolve_copy_source_reg(rhs);
        lhs_nested = find_string_concat_pair(lhs, &nested_a, &nested_b);
        if (!lhs_nested) {
          rhs_nested = find_string_concat_pair(rhs, &nested_b, &nested_c);
        } else {
          rhs_nested = 0;
        }
        if (lhs_nested && !rhs_nested) {
          if (first) *first = resolve_copy_source_reg(nested_a);
          if (second) *second = resolve_copy_source_reg(nested_b);
          if (third) *third = rhs;
          if (term_count) *term_count = 3;
          return 1;
        }
        if (rhs_nested && !lhs_nested) {
          if (first) *first = lhs;
          if (second) *second = resolve_copy_source_reg(nested_b);
          if (third) *third = resolve_copy_source_reg(nested_c);
          if (term_count) *term_count = 3;
          return 1;
        }
        if (lhs_nested || rhs_nested) return 0;
        if (first) *first = lhs;
        if (second) *second = rhs;
        if (third) *third = 0;
        if (term_count) *term_count = 2;
        return 1;
      }

      auto int is_string_concat_substring_consumer_inst(
          yyjson_val* ins,
          long long reg) {
        yyjson_val* mc = NULL;
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        const char* op = read_str(ins, "op");
        const char* ctype = NULL;
        const char* mname = NULL;
        long long recv = 0;
        long long concat_first = 0;
        long long concat_second = 0;
        long long concat_third = 0;
        int concat_term_count = 0;
        if (!(op && !strcmp(op, "mir_call"))) return 0;
        mc = yyjson_obj_get(ins, "mir_call");
        cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        args = mc ? yyjson_obj_get(mc, "args") : NULL;
        ctype = cal ? read_str(cal, "type") : NULL;
        mname = cal ? (read_str(cal, "method") ? read_str(cal, "method")
                                               : read_str(cal, "name"))
                    : NULL;
        recv = cal ? resolve_copy_source_reg(read_int(cal, "receiver")) : 0;
        if (!(ctype && !strcmp(ctype, "Method"))) return 0;
        if (!(mname && !strcmp(mname, "substring"))) return 0;
        if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) == 2)) return 0;
        if (recv != resolve_copy_source_reg(reg)) return 0;
        return resolve_string_concat_chain_terms(
            recv,
            &concat_first,
            &concat_second,
            &concat_third,
            &concat_term_count);
      }

      auto int is_string_concat_array_set_consumer_inst(yyjson_val* ins, long long reg) {
        yyjson_val* mc = NULL;
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        const char* op = read_str(ins, "op");
        const char* ctype = NULL;
        const char* mname = NULL;
        if (!(op && !strcmp(op, "mir_call"))) return 0;
        mc = yyjson_obj_get(ins, "mir_call");
        cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        ctype = cal ? read_str(cal, "type") : NULL;
        mname = cal ? (read_str(cal, "method") ? read_str(cal, "method")
                                               : read_str(cal, "name"))
                    : NULL;
        if (!(ctype && !strcmp(ctype, "Method"))) return 0;
        if (!(mname && !strcmp(mname, "set"))) return 0;
        args = mc ? yyjson_obj_get(mc, "args") : NULL;
        if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) >= 2)) return 0;
        if ((long long)yyjson_get_sint(yyjson_arr_get(args, 1)) != reg) return 0;
        return 1;
      }

      #include "hako_llvmc_ffi_string_concat_window.inc"
