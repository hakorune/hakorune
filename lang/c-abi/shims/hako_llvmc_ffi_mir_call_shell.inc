      auto void trace_generic_method_route_state(
          size_t ii,
          long long dst,
          long long recv,
          const char* ctype,
          const char* bname,
          const char* mname,
          long long a0,
          long long a1,
          const struct GenericMethodRouteState* route) {
        if (!route) return;
        if (!hako_llvmc_route_trace_enabled() || generic_method_trace_budget <= 0) return;
        generic_method_trace_budget--;
        char extra[384];
        snprintf(extra, sizeof(extra),
                 "ii=%zu dst=%lld recv=%lld recv_org=%d ctype=%s bname=%s mname=%s a0=%lld a1=%lld flags=len:%d get:%d push:%d string:%d string_indexof:%d array_string:%d map_get:%d map_size:%d map_has:%d",
                 ii, dst, recv, route->recv_org,
                 ctype ? ctype : "-",
                 bname ? bname : "-",
                 mname ? mname : "-",
                 a0, a1,
                 route->runtime_array_len,
                 route->runtime_array_get,
                 route->runtime_array_push,
                 route->runtime_string,
                 route->runtime_string_indexof,
                 route->runtime_array_string,
                 route->runtime_map_get,
                 route->runtime_map_size,
                 route->runtime_map_has);
        hako_llvmc_emit_route_trace("mir_call_method", "seen", mname ? mname : "unknown", extra);
      }
      auto int emit_constructor_mir_call(
          FILE* f,
          enum MirCallConstructorSurfaceKind kind,
          long long dst) {
        if (kind == HAKO_LLVMC_MIR_CALL_CONSTRUCTOR_SURFACE_MAP_BOX) {
          if (dst) {
            fprintf(f, "  %%r%lld = call i64 @\"nyash.map.birth_h\"()\n", dst);
            set_type(dst, T_I64);
            set_origin(dst, ORG_MAP_BIRTH);
          } else {
            fprintf(f, "  call i64 @\"nyash.map.birth_h\"()\n");
          }
          return 1;
        }
        if (kind == HAKO_LLVMC_MIR_CALL_CONSTRUCTOR_SURFACE_ARRAY_BOX) {
          if (dst) {
            fprintf(f, "  %%r%lld = call i64 @\"nyash.array.birth_h\"()\n", dst);
            set_type(dst, T_I64);
            set_origin(dst, ORG_ARRAY_BIRTH);
          } else {
            fprintf(f, "  call i64 @\"nyash.array.birth_h\"()\n");
          }
          return 1;
        }
        return -1;
      }
      auto int emit_method_birth_mir_call(
          enum MirCallMethodBirthSurfaceKind kind,
          long long dst,
          long long recv,
          yyjson_val* args) {
        int recv_org;
        size_t argc;
        if (kind == HAKO_LLVMC_MIR_CALL_METHOD_BIRTH_SURFACE_NONE) {
          return 0;
        }
        if (dst) {
          return -1;
        }
        argc = (args && yyjson_is_arr(args)) ? yyjson_arr_size(args) : 0;
        if (argc != 0) {
          return -1;
        }
        recv_org = recv ? get_origin(recv) : ORG_NONE;
        if (recv_org == ORG_NONE && recv) {
          recv_org = get_scan_origin(recv);
        }
        if (kind == HAKO_LLVMC_MIR_CALL_METHOD_BIRTH_SURFACE_ARRAY_BOX &&
            recv_org == ORG_ARRAY_BIRTH) {
          return 1;
        }
        if (kind == HAKO_LLVMC_MIR_CALL_METHOD_BIRTH_SURFACE_MAP_BOX &&
            recv_org == ORG_MAP_BIRTH) {
          return 1;
        }
        return -1;
      }
      auto int emit_global_mir_call(
          FILE* f,
          enum MirCallGlobalSurfaceKind kind,
          long long a0) {
        long long a0_base = resolve_copy_source_reg(a0);
        long long cv = 0;
        if (kind != HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_PRINT) return -1;
        if (!a0) return -1;
        if (has_const(a0_base, &cv)) {
          fprintf(f, "  %%print_call_%lld = call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([5 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %lld)\n",
                  a0, cv);
        } else {
          fprintf(f, "  %%print_call_%lld = call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([5 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %%r%lld)\n",
                  a0, a0_base ? a0_base : a0);
        }
        return 1;
      }
