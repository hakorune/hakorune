      /* StaticDataPlan reader/emitter.
       *
       * Owner boundary:
       * - MIR owns source_name/symbol/element/values/index.
       * - This shim only emits the readonly global and the load selected by
       *   static_data_load. It must not infer app-specific table semantics.
       */
      auto int hako_llvmc_static_data_plan_is_supported(yyjson_val* plan) {
        const char* symbol = read_str(plan, "symbol");
        const char* element = read_str(plan, "element");
        const char* linkage = read_str(plan, "linkage");
        yyjson_val* values = plan ? yyjson_obj_get(plan, "values") : NULL;
        long long align = plan ? read_int(plan, "align") : 0;
        if (!(symbol && llvm_quoted_symbol_name_is_safe(symbol))) return 0;
        if (linkage && linkage[0] && strcmp(linkage, "private") != 0) return 0;
        if (!(element && strcmp(element, "u16") == 0)) return 0;
        if (!(values && yyjson_is_arr(values))) return 0;
        if (yyjson_arr_size(values) < 1) return 0;
        return align > 0;
      }

      auto int hako_llvmc_emit_static_data_globals_for_root(FILE* out_f, yyjson_val* root_val) {
        yyjson_val* plans = root_val ? yyjson_obj_get(root_val, "static_data_plans") : NULL;
        if (!plans || yyjson_is_null(plans)) return 0;
        if (!yyjson_is_arr(plans)) return -1;
        size_t plan_count = yyjson_arr_size(plans);
        for (size_t i = 0; i < plan_count; i++) {
          yyjson_val* plan = yyjson_arr_get(plans, i);
          const char* symbol = read_str(plan, "symbol");
          const char* linkage = read_str(plan, "linkage");
          yyjson_val* unnamed_addr_val = plan ? yyjson_obj_get(plan, "unnamed_addr") : NULL;
          yyjson_val* values = plan ? yyjson_obj_get(plan, "values") : NULL;
          long long align = plan ? read_int(plan, "align") : 0;
          size_t len = values && yyjson_is_arr(values) ? yyjson_arr_size(values) : 0;
          int unnamed_addr =
              unnamed_addr_val && yyjson_is_bool(unnamed_addr_val)
                  ? yyjson_get_bool(unnamed_addr_val)
                  : 0;
          if (!hako_llvmc_static_data_plan_is_supported(plan)) return -1;
          fprintf(
              out_f,
              "@\"%s\" = %s%s constant [%zu x i16] [",
              symbol,
              (linkage && linkage[0]) ? linkage : "private",
              unnamed_addr ? " unnamed_addr" : "",
              len);
          for (size_t vi = 0; vi < len; vi++) {
            yyjson_val* value = yyjson_arr_get(values, vi);
            long long n = value && yyjson_is_num(value) ? (long long)yyjson_get_sint(value) : -1;
            if (n < 0 || n > 65535) return -1;
            fprintf(out_f, "%si16 %lld", vi ? ", " : "", n);
          }
          fprintf(out_f, "], align %lld\n", align);
        }
        return 0;
      }

      auto int hako_llvmc_emit_static_data_load_instruction(
          yyjson_val* ins,
          char* reason_out,
          size_t reason_out_len) {
        long long dst = read_int(ins, "dst");
        long long index_reg = read_int(ins, "index");
        long long len = read_int(ins, "len");
        long long align = read_int(ins, "align");
        const char* symbol = read_str(ins, "symbol");
        const char* element = read_str(ins, "element");
        long long index_base = resolve_copy_source_reg(index_reg);
        long long index_const = 0;
        char index_ref[64];

        if (reason_out && reason_out_len) reason_out[0] = '\0';
        if (!dst || !index_reg || len <= 0 || align <= 0) {
          if (reason_out && reason_out_len) {
            snprintf(reason_out, reason_out_len, "malformed_static_data_load");
          }
          return 0;
        }
        if (!(symbol && llvm_quoted_symbol_name_is_safe(symbol))) {
          if (reason_out && reason_out_len) {
            snprintf(reason_out, reason_out_len, "invalid_static_data_symbol");
          }
          return 0;
        }
        if (!(element && strcmp(element, "u16") == 0)) {
          if (reason_out && reason_out_len) {
            snprintf(reason_out, reason_out_len, "unsupported_static_data_element");
          }
          return 0;
        }

        if (has_const(index_base, &index_const)) {
          snprintf(index_ref, sizeof(index_ref), "%lld", index_const);
        } else {
          snprintf(index_ref, sizeof(index_ref), "%%r%lld", index_base);
        }
        EMIT(
            "  %%sdptr%lld = getelementptr inbounds [%lld x i16], ptr @\"%s\", i64 0, i64 %s\n",
            dst,
            len,
            symbol,
            index_ref);
        EMIT("  %%sdv%lld = load i16, ptr %%sdptr%lld, align %lld\n", dst, dst, align);
        EMIT("  %%r%lld = zext i16 %%sdv%lld to i64\n", dst, dst);
        set_type(dst, T_I64);
        return 1;
      }
