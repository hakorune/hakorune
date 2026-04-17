      auto int emit_mir_call_dispatch(
          FILE* f,
          yyjson_val* blocks,
          long long bid,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* ctype,
          const char* bname,
          const char* mname,
          long long dst,
          long long recv,
          yyjson_val* args,
          long long a0,
          long long a1,
          unsigned char* skip_inst) {
        char ab[256];
        ab[0] = '\0';
        if (hako_llvmc_route_trace_enabled() && generic_call_trace_budget > 0) {
          char extra[384];
          snprintf(extra, sizeof(extra),
                   "ii=%zu dst=%lld recv=%lld ctype=%s bname=%s mname=%s a0=%lld a1=%lld",
                   ii, dst, recv,
                   ctype ? ctype : "-",
                   bname ? bname : "-",
                   mname ? mname : "-",
                   a0, a1);
          hako_llvmc_emit_route_trace("mir_call", "seen", "enter", extra);
          generic_call_trace_budget--;
        }

        if (ctype && !strcmp(ctype, "Constructor")) {
          enum MirCallConstructorSurfaceKind kind =
              classify_mir_call_constructor_surface(bname);
          return emit_constructor_mir_call(f, kind, dst);
        }

        if (ctype && !strcmp(ctype, "Method")) {
          struct GenericMethodRouteState route =
              classify_generic_method_route(bid, ii, bname, mname, recv);
          trace_generic_method_route_state(
              ii, dst, recv, ctype, bname, mname, a0, a1, &route);
          enum GenericMethodEmitKind emit_kind =
              classify_generic_method_emit_kind(bname, mname, &route, a0, a1);
          append_array_string_indexof_method_receiver_args(
              route.deferred_array_string_indexof,
              recv,
              ab,
              sizeof(ab));
          {
            int non_indexof_method_rc = try_emit_non_indexof_generic_method_call(
                f,
                blocks,
                bid,
                ii,
                ilen,
                insts,
                bname,
                mname,
                dst,
                recv,
                a0,
                a1,
                emit_kind,
                &route,
                ab,
                sizeof(ab),
                skip_inst);
            if (non_indexof_method_rc != 0) {
              return non_indexof_method_rc;
            }
          }
          if (mname && !strcmp(mname, "indexOf") && route.runtime_string) {
            if (!emit_deferred_array_string_indexof_call(
                    f, route.deferred_array_string_indexof, dst, ab)) {
              if (a0) append_i64_arg_ref(ab, sizeof(ab), a0, ab[0] == '\0');
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.string.indexOf_hh\"(%s)\n", dst, ab);
                set_type(dst, T_I64);
              } else {
                EMIT("  call i64 @\"nyash.string.indexOf_hh\"(%s)\n", ab);
              }
            }
            return 1;
          }
          return -1;
        }

        if (ctype && !strcmp(ctype, "Extern")) {
          enum MirCallStringExternSurfaceKind kind =
              classify_mir_call_string_extern_surface(mname);
          return emit_supported_string_extern_mir_call(
              f, ii, ilen, insts, kind, dst, args, skip_inst);
        }

        if (ctype && !strcmp(ctype, "Global")) {
          enum MirCallGlobalSurfaceKind kind =
              classify_mir_call_global_surface(mname);
          return emit_global_mir_call(f, kind, a0);
        }

        return -1;
      }
