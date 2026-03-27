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

      auto int is_string_concat_chain_consumer_inst(yyjson_val* ins, long long reg) {
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
        return get_origin(other) == ORG_STRING;
      }

      auto int should_defer_string_concat_pair(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long dst) {
        size_t use_ii = 0;
        int use_count = 0;
        if (!dst) return 0;
        for (size_t jj = ii + 1; jj < ilen; jj++) {
          yyjson_val* later = yyjson_arr_get(insts, jj);
          if (!inst_uses_reg(later, dst)) continue;
          use_count++;
          use_ii = jj;
          if (use_count > 1) return 0;
        }
        if (use_count != 1) return 0;
        return is_string_concat_chain_consumer_inst(yyjson_arr_get(insts, use_ii), dst);
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
