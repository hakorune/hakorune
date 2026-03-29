      auto void emit_branch(long long cond_reg, long long then_id, long long else_id){ long long cond_base=resolve_copy_source_reg(cond_reg); int ty=get_type(cond_reg); if (ty==T_I1) EMIT("  br i1 %%r%lld, label %%bb%lld, label %%bb%lld\n", cond_base, then_id, else_id); else { long long cv; if (has_const(cond_base,&cv)) { EMIT("  %%t%lld = icmp ne i64 %lld, 0\n", cond_reg, cv); EMIT("  br i1 %%t%lld, label %%bb%lld, label %%bb%lld\n", cond_reg, then_id, else_id);} else { EMIT("  %%t%lld = icmp ne i64 %%r%lld, 0\n", cond_reg, cond_base); EMIT("  br i1 %%t%lld, label %%bb%lld, label %%bb%lld\n", cond_reg, then_id, else_id);} } }

      auto void emit_jump(long long target){ EMIT("  br label %%bb%lld\n", target); }

      auto void emit_ret(long long reg){ long long cv; long long base=resolve_copy_source_reg(reg); if (has_const(base,&cv)) EMIT("  ret i64 %lld\n", cv); else EMIT("  ret i64 %%r%lld\n", base); }

      auto void emit_call_assign(long long dst, const char* sym, const char* args){ EMIT("  %%r%lld = call i64 @\"%s\"(%s)\n", dst, sym, args); set_type(dst, T_I64); }

      auto void emit_call_noret(const char* sym, const char* args){ EMIT("  call i64 @\"%s\"(%s)\n", sym, args); }
