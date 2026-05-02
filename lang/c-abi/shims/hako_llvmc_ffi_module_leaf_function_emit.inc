      /*
       * Module leaf function emitter.
       *
       * This include owns the P118 emitter-side validation for the narrow
       * `numeric_i64_leaf` target shape. MIR/LoweringPlan owns the semantic
       * permission to call a same-module function; this file only proves that
       * the target body was emitted as a definition before a direct call uses
       * the symbol.
       */
      auto int generic_pure_read_i64_field(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* v = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(v && yyjson_is_num(v))) return 0;
        if (out) *out = (long long)yyjson_get_sint(v);
        return 1;
      }
      auto int module_function_i64_leaf_const_is_numeric(yyjson_val* ins) {
        yyjson_val* value = ins ? yyjson_obj_get(ins, "value") : NULL;
        yyjson_val* ty = value ? yyjson_obj_get(value, "type") : NULL;
        yyjson_val* literal = value ? yyjson_obj_get(value, "value") : NULL;
        return ty &&
               yyjson_is_str(ty) &&
               !strcmp(yyjson_get_str(ty), "i64") &&
               literal &&
               yyjson_is_num(literal);
      }
      auto int module_function_i64_leaf_binop_supported(const char* op) {
        return op &&
               (!strcmp(op, "+") ||
                !strcmp(op, "-") ||
                !strcmp(op, "*") ||
                !strcmp(op, "/") ||
                !strcmp(op, "%") ||
                !strcmp(op, "Add") ||
                !strcmp(op, "Sub") ||
                !strcmp(op, "Mul") ||
                !strcmp(op, "Div") ||
                !strcmp(op, "Mod"));
      }
      auto int module_function_is_numeric_i64_leaf(yyjson_val* fn) {
        hako_llvmc_generic_pure_function_view fn_view;
        struct GenericPureBlockView block_view;
        yyjson_val* params = fn ? yyjson_obj_get(fn, "params") : NULL;
        yyjson_val* insts = NULL;
        size_t inst_count = 0;
        const char* name = fn ? read_str(fn, "name") : NULL;
        if (!name || !llvm_quoted_symbol_name_is_safe(name)) return 0;
        if (!(params && yyjson_is_arr(params))) return 0;
        for (size_t pi = 0; pi < yyjson_arr_size(params); pi++) {
          yyjson_val* param = yyjson_arr_get(params, pi);
          if (!(param && yyjson_is_num(param))) return 0;
        }
        if (!hako_llvmc_read_generic_pure_function_view(fn, 0, 0, &fn_view) ||
            fn_view.block_count != 1 ||
            !read_generic_pure_function_block_view(&fn_view, 0, &block_view)) {
          return 0;
        }
        insts = block_view.instructions;
        inst_count = block_view.instruction_count;
        if (inst_count < 1 || inst_count > 1024) return 0;
        for (size_t ii = 0; ii < inst_count; ii++) {
          yyjson_val* ins = yyjson_arr_get(insts, ii);
          const char* op = ins ? read_str(ins, "op") : NULL;
          long long tmp = 0;
          if (!op) return 0;
          if (ii + 1 == inst_count) {
            return !strcmp(op, "ret") &&
                   generic_pure_read_i64_field(ins, "value", &tmp);
          }
          if (!strcmp(op, "const")) {
            if (!generic_pure_read_i64_field(ins, "dst", &tmp)) return 0;
            if (!module_function_i64_leaf_const_is_numeric(ins)) return 0;
            continue;
          }
          if (!strcmp(op, "copy")) {
            if (!generic_pure_read_i64_field(ins, "dst", &tmp)) return 0;
            if (!generic_pure_read_i64_field(ins, "src", &tmp)) return 0;
            continue;
          }
          if (!strcmp(op, "binop")) {
            const char* kind = read_str(ins, "op_kind");
            if (!kind) kind = read_str(ins, "operation");
            if (!module_function_i64_leaf_binop_supported(kind)) return 0;
            if (!generic_pure_read_i64_field(ins, "dst", &tmp)) return 0;
            if (!generic_pure_read_i64_field(ins, "lhs", &tmp)) return 0;
            if (!generic_pure_read_i64_field(ins, "rhs", &tmp)) return 0;
            continue;
          }
          return 0;
        }
        return 0;
      }
      const char* emitted_module_leaf_symbols[256];
      size_t emitted_module_leaf_symbols_n = 0;
      auto int remember_emitted_module_leaf_symbol(const char* name) {
        if (!name) return 0;
        for (size_t i = 0; i < emitted_module_leaf_symbols_n; i++) {
          if (!strcmp(emitted_module_leaf_symbols[i], name)) return 1;
        }
        if (emitted_module_leaf_symbols_n >= ARR_LEN(emitted_module_leaf_symbols)) return 0;
        emitted_module_leaf_symbols[emitted_module_leaf_symbols_n++] = name;
        return 1;
      }
      auto int module_leaf_symbol_was_emitted(const char* name) {
        if (!name) return 0;
        for (size_t i = 0; i < emitted_module_leaf_symbols_n; i++) {
          if (!strcmp(emitted_module_leaf_symbols[i], name)) return 1;
        }
        return 0;
      }
      struct ModuleFunctionLeafConst { long long reg; long long val; };
      struct ModuleFunctionLeafCopy { long long reg; long long src; };
      auto const char* module_function_i64_leaf_irop(const char* op) {
        if (!op) return NULL;
        if (!strcmp(op, "+") || !strcmp(op, "Add")) return "add";
        if (!strcmp(op, "-") || !strcmp(op, "Sub")) return "sub";
        if (!strcmp(op, "*") || !strcmp(op, "Mul")) return "mul";
        if (!strcmp(op, "/") || !strcmp(op, "Div")) return "sdiv";
        if (!strcmp(op, "%") || !strcmp(op, "Mod")) return "srem";
        return NULL;
      }
      auto long long module_leaf_resolve_copy_source_reg(
          struct ModuleFunctionLeafCopy* aliases,
          size_t aliases_n,
          long long reg) {
        for (int depth = 0; depth < 64; depth++) {
          int changed = 0;
          for (size_t i = 0; i < aliases_n; i++) {
            if (aliases[i].reg == reg) {
              reg = aliases[i].src;
              changed = 1;
              break;
            }
          }
          if (!changed) return reg;
        }
        return reg;
      }
      auto int module_leaf_has_const(
          struct ModuleFunctionLeafConst* leaf_consts,
          size_t leaf_consts_n,
          long long reg,
          long long* out) {
        for (size_t i = 0; i < leaf_consts_n; i++) {
          if (leaf_consts[i].reg == reg) {
            if (out) *out = leaf_consts[i].val;
            return 1;
          }
        }
        return 0;
      }
      auto int module_leaf_put_const(
          struct ModuleFunctionLeafConst* leaf_consts,
          size_t* leaf_consts_n,
          long long reg,
          long long val) {
        for (size_t i = 0; i < *leaf_consts_n; i++) {
          if (leaf_consts[i].reg == reg) {
            leaf_consts[i].val = val;
            return 1;
          }
        }
        if (*leaf_consts_n >= 1024) return 0;
        leaf_consts[*leaf_consts_n].reg = reg;
        leaf_consts[*leaf_consts_n].val = val;
        (*leaf_consts_n)++;
        return 1;
      }
      auto int module_leaf_put_copy(
          struct ModuleFunctionLeafCopy* aliases,
          size_t* aliases_n,
          long long reg,
          long long src) {
        for (size_t i = 0; i < *aliases_n; i++) {
          if (aliases[i].reg == reg) {
            aliases[i].src = src;
            return 1;
          }
        }
        if (*aliases_n >= 1024) return 0;
        aliases[*aliases_n].reg = reg;
        aliases[*aliases_n].src = src;
        (*aliases_n)++;
        return 1;
      }
      auto void module_leaf_format_i64_ref(
          char* out,
          size_t out_cap,
          struct ModuleFunctionLeafConst* leaf_consts,
          size_t leaf_consts_n,
          struct ModuleFunctionLeafCopy* aliases,
          size_t aliases_n,
          long long reg) {
        long long base = module_leaf_resolve_copy_source_reg(aliases, aliases_n, reg);
        long long cv = 0;
        if (module_leaf_has_const(leaf_consts, leaf_consts_n, base, &cv)) {
          snprintf(out, out_cap, "%lld", cv);
        } else {
          snprintf(out, out_cap, "%%r%lld", base);
        }
      }
      auto int emit_numeric_i64_leaf_function_definition(FILE* f, yyjson_val* fn) {
        hako_llvmc_generic_pure_function_view fn_view;
        struct GenericPureBlockView block_view;
        struct ModuleFunctionLeafConst leaf_consts[1024];
        struct ModuleFunctionLeafCopy aliases[1024];
        size_t leaf_consts_n = 0;
        size_t aliases_n = 0;
        yyjson_val* params = fn ? yyjson_obj_get(fn, "params") : NULL;
        yyjson_val* insts = NULL;
        const char* name = fn ? read_str(fn, "name") : NULL;
        size_t argc = generic_pure_function_param_count(fn);
        size_t inst_count = 0;
        int emitted_ret = 0;
        if (!module_function_is_numeric_i64_leaf(fn)) return 0;
        if (!hako_llvmc_read_generic_pure_function_view(fn, 0, 0, &fn_view) ||
            fn_view.block_count != 1 ||
            !read_generic_pure_function_block_view(&fn_view, 0, &block_view)) {
          return -1;
        }
        insts = block_view.instructions;
        inst_count = block_view.instruction_count;
        fprintf(f, "define i64 @\"%s\"(", name);
        for (size_t i = 0; i < argc; i++) {
          long long param = (long long)yyjson_get_sint(yyjson_arr_get(params, i));
          fprintf(f, "%si64 %%r%lld", i ? ", " : "", param);
        }
        fprintf(f, ") {\nentry:\n");
        for (size_t ii = 0; ii < inst_count; ii++) {
          yyjson_val* ins = yyjson_arr_get(insts, ii);
          const char* op = ins ? read_str(ins, "op") : NULL;
          if (!strcmp(op, "const")) {
            yyjson_val* value = yyjson_obj_get(ins, "value");
            yyjson_val* literal = value ? yyjson_obj_get(value, "value") : NULL;
            long long dst = 0;
            if (!generic_pure_read_i64_field(ins, "dst", &dst) ||
                !module_leaf_put_const(
                    leaf_consts,
                    &leaf_consts_n,
                    dst,
                    (long long)yyjson_get_sint(literal))) {
              return -1;
            }
            continue;
          }
          if (!strcmp(op, "copy")) {
            long long dst = 0;
            long long src = 0;
            if (!generic_pure_read_i64_field(ins, "dst", &dst) ||
                !generic_pure_read_i64_field(ins, "src", &src) ||
                !module_leaf_put_copy(
                    aliases,
                    &aliases_n,
                    dst,
                    module_leaf_resolve_copy_source_reg(aliases, aliases_n, src))) {
              return -1;
            }
            continue;
          }
          if (!strcmp(op, "binop")) {
            const char* kind = read_str(ins, "op_kind");
            const char* irop = NULL;
            long long dst = 0;
            long long lhs = 0;
            long long rhs = 0;
            char lhs_ref[64];
            char rhs_ref[64];
            if (!kind) kind = read_str(ins, "operation");
            irop = module_function_i64_leaf_irop(kind);
            if (!irop ||
                !generic_pure_read_i64_field(ins, "dst", &dst) ||
                !generic_pure_read_i64_field(ins, "lhs", &lhs) ||
                !generic_pure_read_i64_field(ins, "rhs", &rhs)) {
              return -1;
            }
            module_leaf_format_i64_ref(
                lhs_ref,
                sizeof(lhs_ref),
                leaf_consts,
                leaf_consts_n,
                aliases,
                aliases_n,
                lhs);
            module_leaf_format_i64_ref(
                rhs_ref,
                sizeof(rhs_ref),
                leaf_consts,
                leaf_consts_n,
                aliases,
                aliases_n,
                rhs);
            fprintf(f, "  %%r%lld = %s i64 %s, %s\n", dst, irop, lhs_ref, rhs_ref);
            continue;
          }
          if (!strcmp(op, "ret")) {
            long long value = 0;
            char value_ref[64];
            if (!generic_pure_read_i64_field(ins, "value", &value)) return -1;
            module_leaf_format_i64_ref(
                value_ref,
                sizeof(value_ref),
                leaf_consts,
                leaf_consts_n,
                aliases,
                aliases_n,
                value);
            fprintf(f, "  ret i64 %s\n", value_ref);
            emitted_ret = 1;
            continue;
          }
          return -1;
        }
        if (!emitted_ret) return -1;
        fprintf(f, "}\n\n");
        return remember_emitted_module_leaf_symbol(name) ? 1 : -1;
      }
      auto int emit_module_function_leaf_definitions(FILE* f) {
        int emitted = 0;
        for (size_t i = 0; i < program.function_count; i++) {
          yyjson_val* fn = yyjson_arr_get(program.functions, i);
          if (!fn || fn == fn0) continue;
          int rc = emit_numeric_i64_leaf_function_definition(f, fn);
          if (rc < 0) return -1;
          if (rc > 0) emitted++;
        }
        return emitted;
      }
