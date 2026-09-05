      auto int same_module_function_emit_call_instruction(
          FILE* f,
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        const char* op = read_str(ins, "op");
        const char* ctype = NULL;
        long long dst = read_int(ins, "dst");
        int rc = 0;
        if (!same_module_function_read_call_payload(ins, &cal, &args)) return 0;
        ctype = same_module_function_call_type(op, cal);
        {
          const char* active_function_name = fn0 ? read_str(fn0, "name") : NULL;
          const hako_llvmc_published_static_method_call_v1* typed_row =
              NULL;
          int typed_shape_rc = hako_llvmc_published_static_method_take_i64_global_row_v1(
              active_function_name, bid, ii,
              ctype, dst, args,
              &typed_row);
          if (typed_shape_rc < 0) return -1;
          if (typed_shape_rc > 0 &&
              typed_row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_BUILTIN_PRINT) {
            long long print_arg = 0;
            same_module_function_read_arg_reg(args, 0, &print_arg);
            return emit_global_mir_call(f, HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_PRINT, bid, ii, print_arg);
          }
          if (typed_shape_rc > 0 &&
              (typed_row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD ||
               typed_row->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION)) {
            uint32_t typed_arity = typed_row->arity;
            const char* typed_symbol = typed_row->target_symbol;
            {
              char typed_args[1024];
              typed_args[0] = '\0';
              for (size_t ai = 0; ai < (size_t)typed_arity; ai++) {
                long long arg_reg = 0;
                same_module_function_read_arg_reg(args, ai, &arg_reg);
                append_i64_arg_ref(
                    typed_args,
                    sizeof(typed_args),
                    arg_reg,
                    typed_args[0] == '\0');
              }
              same_module_function_emit_i64_call(dst, typed_symbol, typed_args);
              return 1;
            }
          }
        }
        rc = emit_extern_call_lowering_plan_mir_call(f, bid, ii, dst, args);
        if (rc != 0) {
          return rc;
        }
        if (ctype && !strcmp(ctype, "Extern")) {
          long long a0 = 0;
          if (classify_mir_call_string_extern_surface(read_str(cal, "name")) ==
                  HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_CONSOLE_LOG &&
              same_module_function_read_arg_reg(args, 0, &a0)) {
            rc = emit_global_mir_call(
                f,
                HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_PRINT,
                bid,
                ii,
                a0);
          } else {
            rc = 0;
          }
        } else if (ctype && !strcmp(ctype, "Global")) {
          rc = same_module_function_emit_global_call_mir_call(
              f,
              bid,
              ii,
              cal,
              args,
              dst);
        } else if (ctype && !strcmp(ctype, "Method")) {
          rc = same_module_function_emit_method_call_mir_call(
              f,
              bid,
              ii,
              op,
              cal,
              args,
              dst);
        } else {
          return 0;
        }
        return rc == 1;
      }
