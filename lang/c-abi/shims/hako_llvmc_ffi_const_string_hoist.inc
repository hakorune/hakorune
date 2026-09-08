      auto void emit_hoisted_string_const_defs(FILE* f) {
        if (!hako_llvmc_fast_enabled() || invocation->static_v2.frame) return;
        for (size_t sci = 0; sci < str_consts_n; sci++) {
          if (should_skip_hoisted_string_const_box(str_consts[sci].reg)) continue;
          char global[64];
          snprintf(global, sizeof(global), "@.str_reg_%lld", str_consts[sci].reg);
          emit_string_const_handle(f, str_consts[sci].reg, global, str_consts[sci].byte_len);
          mark_string_const_box_emitted(str_consts[sci].reg);
          set_type(str_consts[sci].reg, T_I64);
          set_origin(str_consts[sci].reg, ORG_STRING);
          set_scan_origin(str_consts[sci].reg, ORG_STRING);
        }
      }
