      auto void emit_hoisted_string_const_defs(FILE* f) {
        if (!hako_llvmc_fast_enabled()) return;
        for (size_t sci = 0; sci < str_consts_n; sci++) {
          size_t slen = strlen(str_consts[sci].val);
          fprintf(
              f,
              "  %%r%lld = call i64 @\"nyash.box.from_i8_string_const\"(ptr getelementptr inbounds ([%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0))\n",
              str_consts[sci].reg,
              slen + 1,
              str_consts[sci].reg);
          set_type(str_consts[sci].reg, T_I64);
          set_origin(str_consts[sci].reg, ORG_STRING);
          set_scan_origin(str_consts[sci].reg, ORG_STRING);
        }
      }
