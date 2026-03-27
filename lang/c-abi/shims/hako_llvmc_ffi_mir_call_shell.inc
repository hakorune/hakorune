      struct GenericMethodRouteState {
        int recv_org;
        int runtime_array_len;
        int runtime_array_get;
        int runtime_array_push;
        int runtime_array_has;
        int runtime_string;
        int runtime_array_string;
        int runtime_map_get;
        int runtime_map_size;
        int runtime_map_has;
        struct DeferredArrayStringIndexofSelect* deferred_array_string_indexof;
      };
      auto struct GenericMethodRouteState classify_generic_method_route(
          long long bid,
          size_t ii,
          const char* bname,
          const char* mname,
          long long recv) {
        struct GenericMethodRouteState route;
        memset(&route, 0, sizeof(route));
        route.recv_org = recv ? get_origin(recv) : ORG_NONE;
        if (route.recv_org == ORG_NONE && recv) {
          route.recv_org = get_scan_origin(recv);
        }
        route.runtime_array_len = bname && !strcmp(bname, "RuntimeDataBox") && route.recv_org == ORG_ARRAY_BIRTH;
        route.runtime_array_get = bname && !strcmp(bname, "RuntimeDataBox") && route.recv_org == ORG_ARRAY_BIRTH;
        route.runtime_array_push = bname && !strcmp(bname, "RuntimeDataBox") && route.recv_org == ORG_ARRAY_BIRTH;
        route.runtime_array_has = bname && !strcmp(bname, "RuntimeDataBox") && route.recv_org == ORG_ARRAY_BIRTH;
        route.runtime_string = bname && !strcmp(bname, "RuntimeDataBox") && route.recv_org == ORG_STRING;
        route.runtime_array_string = bname && !strcmp(bname, "RuntimeDataBox") && route.recv_org == ORG_ARRAY_STRING_BIRTH;
        route.runtime_map_get = bname && !strcmp(bname, "RuntimeDataBox") && route.recv_org == ORG_MAP_BIRTH;
        route.runtime_map_size = bname && !strcmp(bname, "RuntimeDataBox") && route.recv_org == ORG_MAP_BIRTH;
        route.runtime_map_has = bname && !strcmp(bname, "RuntimeDataBox") && route.recv_org == ORG_MAP_BIRTH;
        if (mname && !strcmp(mname, "indexOf") && route.runtime_string) {
          route.deferred_array_string_indexof = find_deferred_array_string_indexof_select(bid, ii);
        }
        return route;
      }
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
                 "ii=%zu dst=%lld recv=%lld recv_org=%d ctype=%s bname=%s mname=%s a0=%lld a1=%lld flags=len:%d get:%d push:%d has:%d string:%d array_string:%d map_get:%d map_size:%d map_has:%d",
                 ii, dst, recv, route->recv_org,
                 ctype ? ctype : "-",
                 bname ? bname : "-",
                 mname ? mname : "-",
                 a0, a1,
                 route->runtime_array_len,
                 route->runtime_array_get,
                 route->runtime_array_push,
                 route->runtime_array_has,
                 route->runtime_string,
                 route->runtime_array_string,
                 route->runtime_map_get,
                 route->runtime_map_size,
                 route->runtime_map_has);
        hako_llvmc_emit_route_trace("mir_call_method", "seen", mname ? mname : "unknown", extra);
      }
      auto int emit_constructor_mir_call(FILE* f, const char* bname, long long dst) {
        if (bname && !strcmp(bname, "MapBox")) {
          if (dst) {
            fprintf(f, "  %%r%lld = call i64 @\"nyash.map.birth_h\"()\n", dst);
            set_type(dst, T_I64);
            set_origin(dst, ORG_MAP_BIRTH);
          } else {
            fprintf(f, "  call i64 @\"nyash.map.birth_h\"()\n");
          }
          return 1;
        }
        if (bname && !strcmp(bname, "ArrayBox")) {
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
      auto int emit_global_mir_call(FILE* f, const char* mname, long long a0) {
        long long a0_base = resolve_copy_source_reg(a0);
        long long cv = 0;
        if (!(mname && !strcmp(mname, "print"))) return -1;
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
