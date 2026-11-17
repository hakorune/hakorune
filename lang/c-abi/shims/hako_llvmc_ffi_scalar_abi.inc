      /* One scalar boundary: logical Bool is i1, the existing ABI carries i64.
       * T_I64 alone is not evidence of a scalar: handles share that storage. */
      unsigned long long scalar_abi_serial = 0;
      auto int scalar_abi_is_bool(long long reg) {
        long long base = resolve_copy_source_reg(reg);
        if (get_type(base) == T_I1) return 1;
        yyjson_val* type_rows = fn_metadata ? yyjson_obj_get(fn_metadata, "value_types") : NULL;
        long long candidates[2] = {reg, base};
        for (size_t i = 0; i < 2; i++) {
          char key[32];
          snprintf(key, sizeof(key), "%lld", candidates[i]);
          yyjson_val* type = type_rows ? yyjson_obj_get(type_rows, key) : NULL;
          if (type && yyjson_is_str(type) && !strcmp(yyjson_get_str(type), "i1")) return 1;
        }
        return 0;
      }
      /* Call before starting an LLVM instruction: this may emit a conversion. */
      auto void scalar_abi_format_i64_ref(char* out, size_t cap, long long reg) {
        long long base = resolve_copy_source_reg(reg), value;
        if (has_const(base, &value)) {
          snprintf(out, cap, "%lld", value);
        } else if (get_type(base) == T_I1) {
          snprintf(out, cap, "%%scalar_abi_%llu", scalar_abi_serial++);
          fprintf(f, "  %s = zext i1 %%r%lld to i64\n", out, base);
        } else {
          snprintf(out, cap, "%%r%lld", base);
        }
      }
      auto void scalar_abi_restore_bool(FILE* output, long long dst, const char* bits) {
        fprintf(output, "  %%r%lld = icmp ne i64 %s, 0\n", dst, bits);
        set_type(dst, T_I1);
      }
