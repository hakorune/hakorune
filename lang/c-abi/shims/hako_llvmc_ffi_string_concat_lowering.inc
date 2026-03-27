      struct StringConcatPairRec {
        long long dst;
        long long lhs;
        long long rhs;
      };
      struct StringConcatPairRec string_concat_pairs[512];
      size_t string_concat_pairs_n = 0;

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

      auto int is_string_concat_array_set_consumer_inst(yyjson_val* ins, long long reg) {
        yyjson_val* mc = NULL;
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        const char* op = read_str(ins, "op");
        const char* ctype = NULL;
        const char* bname = NULL;
        const char* mname = NULL;
        long long recv = 0;
        if (!(op && !strcmp(op, "mir_call"))) return 0;
        mc = yyjson_obj_get(ins, "mir_call");
        cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        ctype = cal ? read_str(cal, "type") : NULL;
        bname = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name")
                                                 : read_str(cal, "box_type"))
                    : NULL;
        mname = cal ? (read_str(cal, "method") ? read_str(cal, "method")
                                               : read_str(cal, "name"))
                    : NULL;
        if (!(ctype && !strcmp(ctype, "Method"))) return 0;
        if (!(mname && !strcmp(mname, "set"))) return 0;
        args = mc ? yyjson_obj_get(mc, "args") : NULL;
        if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) >= 2)) return 0;
        if ((long long)yyjson_get_sint(yyjson_arr_get(args, 1)) != reg) return 0;
        if (bname && !strcmp(bname, "ArrayBox")) return 1;
        if (!(bname && !strcmp(bname, "RuntimeDataBox"))) return 0;
        recv = read_int(cal, "receiver");
        return get_origin(recv) == ORG_ARRAY_BIRTH ||
               get_origin(recv) == ORG_ARRAY_STRING_BIRTH ||
               get_scan_origin(recv) == ORG_ARRAY_BIRTH ||
               get_scan_origin(recv) == ORG_ARRAY_STRING_BIRTH;
      }

      auto int find_single_use_inst(
          size_t start_ii,
          size_t ilen,
          yyjson_val* insts,
          long long reg,
          size_t* use_ii_out) {
        int use_count = 0;
        size_t use_ii = 0;
        if (!reg) return 0;
        for (size_t jj = start_ii; jj < ilen; jj++) {
          yyjson_val* later = yyjson_arr_get(insts, jj);
          if (!inst_uses_reg(later, reg)) continue;
          use_count++;
          use_ii = jj;
          if (use_count > 1) return 0;
        }
        if (use_count != 1) return 0;
        if (use_ii_out) *use_ii_out = use_ii;
        return 1;
      }

      auto int has_direct_array_set_consumer(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long reg) {
        size_t use_ii = 0;
        if (!find_single_use_inst(ii + 1, ilen, insts, reg, &use_ii)) return 0;
        return is_string_concat_array_set_consumer_inst(yyjson_arr_get(insts, use_ii), reg);
      }

      auto int should_skip_string_const_box_for_concat_suffix(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long reg) {
        size_t use_ii = 0;
        yyjson_val* use_ins = NULL;
        const char* op = NULL;
        const char* kind = NULL;
        long long lhs = 0;
        long long rhs = 0;
        long long concat_dst = 0;
        if (!get_str_const(reg)) return 0;
        if (!find_single_use_inst(ii + 1, ilen, insts, reg, &use_ii)) return 0;
        use_ins = yyjson_arr_get(insts, use_ii);
        op = use_ins ? read_str(use_ins, "op") : NULL;
        if (!(op && !strcmp(op, "binop"))) return 0;
        kind = read_str(use_ins, "op_kind");
        if (!kind) kind = read_str(use_ins, "operation");
        if (!(kind && (!strcmp(kind, "Add") || !strcmp(kind, "+")))) return 0;
        lhs = read_int(use_ins, "lhs");
        rhs = read_int(use_ins, "rhs");
        if (rhs != reg) return 0;
        if (!future_reg_becomes_string_operand(insts, ii + 1, use_ii, 0, lhs)) return 0;
        concat_dst = read_int(use_ins, "dst");
        return has_direct_array_set_consumer(use_ii, ilen, insts, concat_dst);
      }

      auto int should_defer_string_concat_pair(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst) {
        size_t use_ii = 0;
        if (!dst) return 0;
        if (!find_single_use_inst(ii + 1, ilen, insts, dst, &use_ii)) return 0;
        return is_string_concat_chain_consumer_inst(
            yyjson_arr_get(insts, use_ii), insts, ii + 1, use_ii, dst);
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
          return 1;
        }
        if (find_string_concat_pair(rhs, &a, &b)) {
          append_string_concat_arg_ref(ab, sizeof(ab), lhs, 1);
          append_string_concat_arg_ref(ab, sizeof(ab), a, 0);
          append_string_concat_arg_ref(ab, sizeof(ab), b, 0);
          fprintf(f, "  %%r%lld = call i64 @\"nyash.string.concat3_hhh\"(%s)\n", dst, ab);
          finalize_string_concat_result(dst);
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
          } else {
            fprintf(f, "  call i64 @\"nyash.string.concat3_hhh\"(%s)\n", ab);
          }
          return 1;
        }

        return -1;
      }
