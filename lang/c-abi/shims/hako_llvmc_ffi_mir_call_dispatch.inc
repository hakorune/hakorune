      auto int hako_llvmc_mir_call_arg_reg_at(
          yyjson_val* args,
          size_t index,
          long long* out) {
        yyjson_val* val = args && yyjson_is_arr(args) && yyjson_arr_size(args) > index
                              ? yyjson_arr_get(args, index)
                              : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto void hako_llvmc_normalize_generic_method_logical_args(
          yyjson_val* entry,
          yyjson_val* args,
          long long recv,
          long long* a0,
          long long* a1) {
        long long arity = -1;
        size_t argc = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        size_t start = 0;
        long long raw0 = 0;
        const char* source = entry ? read_str(entry, "source") : NULL;
        const char* source_route_id = entry ? read_str(entry, "source_route_id") : NULL;
        struct LoweringPlanGenericMethodView view;
        memset(&view, 0, sizeof(view));
        if (!(source && !strcmp(source, "generic_method_routes"))) return;
        if (!lowering_plan_read_i64_field(entry, "arity", &arity)) return;
        if (arity < 0) return;
        read_lowering_plan_generic_method_view(entry, &view);
        if (recv &&
            argc == (size_t)(arity + 1) &&
            hako_llvmc_mir_call_arg_reg_at(args, 0, &raw0)) {
          if (resolve_copy_source_reg(raw0) == resolve_copy_source_reg(recv)) {
            start = 1;
          } else if (source_route_id &&
                     !strcmp(source_route_id, "generic_method.substring") &&
                     lowering_plan_generic_method_view_receiver_origin_is_string(&view) &&
                     lowering_plan_generic_method_view_arg0_origin_is_string(&view)) {
            start = 1;
          }
        }
        if (a0) {
          *a0 = 0;
          hako_llvmc_mir_call_arg_reg_at(args, start, a0);
        }
        if (a1) {
          *a1 = 0;
          hako_llvmc_mir_call_arg_reg_at(args, start + 1, a1);
        }
      }

      auto void hako_llvmc_normalize_receiver_method_args(
          yyjson_val* args,
          long long recv,
          long long* a0,
          long long* a1) {
        size_t argc = args && yyjson_is_arr(args) ? yyjson_arr_size(args) : 0;
        size_t start = 0;
        long long raw0 = 0;
        if (recv &&
            argc > 0 &&
            hako_llvmc_mir_call_arg_reg_at(args, 0, &raw0) &&
            resolve_copy_source_reg(raw0) == resolve_copy_source_reg(recv)) {
          start = 1;
        }
        if (a0) {
          *a0 = 0;
          hako_llvmc_mir_call_arg_reg_at(args, start, a0);
        }
        if (a1) {
          *a1 = 0;
          hako_llvmc_mir_call_arg_reg_at(args, start + 1, a1);
        }
      }

      auto int emit_filebox_method_mir_call(
          FILE* f,
          const char* bname,
          const char* mname,
          long long dst,
          long long recv,
          yyjson_val* args) {
        long long recv_base = recv ? resolve_copy_source_reg(recv) : 0;
        long long file_a0 = 0;
        long long file_a1 = 0;
        int recv_is_filebox =
            recv_base &&
            (get_origin(recv_base) == ORG_FILEBOX ||
             get_scan_origin(recv_base) == ORG_FILEBOX ||
             get_origin(recv) == ORG_FILEBOX ||
             get_scan_origin(recv) == ORG_FILEBOX);
        int callee_is_filebox =
            bname &&
            (!strcmp(bname, "FileBox") ||
             (!strcmp(bname, "RuntimeDataBox") && recv_is_filebox));
        if (!(callee_is_filebox && mname && recv_base)) {
          return 0;
        }
        hako_llvmc_normalize_receiver_method_args(args, recv, &file_a0, &file_a1);
        file_a0 = file_a0 ? resolve_copy_source_reg(file_a0) : 0;
        file_a1 = file_a1 ? resolve_copy_source_reg(file_a1) : 0;
        if (!strcmp(mname, "open")) {
          if (!(file_a0 && file_a1)) return -1;
          if (dst) {
            EMIT("  %%r%lld = call i64 @\"nyash.file.open_hhh\"(i64 %%r%lld, i64 %%r%lld, i64 %%r%lld)\n",
                 dst,
                 recv_base,
                 file_a0,
                 file_a1);
            set_type(dst, T_I64);
          } else {
            EMIT("  call i64 @\"nyash.file.open_hhh\"(i64 %%r%lld, i64 %%r%lld, i64 %%r%lld)\n",
                 recv_base,
                 file_a0,
                 file_a1);
          }
          return 1;
        }
        if (!strcmp(mname, "read")) {
          if (dst) {
            EMIT("  %%r%lld = call i64 @\"nyash.file.read_h\"(i64 %%r%lld)\n", dst, recv_base);
            set_type(dst, T_I64);
            set_origin(dst, ORG_STRING);
            set_scan_origin(dst, ORG_STRING);
          } else {
            EMIT("  call i64 @\"nyash.file.read_h\"(i64 %%r%lld)\n", recv_base);
          }
          return 1;
        }
        if (!strcmp(mname, "close")) {
          if (dst) {
            EMIT("  %%r%lld = call i64 @\"nyash.file.close_h\"(i64 %%r%lld)\n", dst, recv_base);
            set_type(dst, T_I64);
          } else {
            EMIT("  call i64 @\"nyash.file.close_h\"(i64 %%r%lld)\n", recv_base);
          }
          return 1;
        }
        return 0;
      }

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

        {
          int plan_extern_rc =
              emit_extern_call_lowering_plan_mir_call(f, bid, ii, dst, args);
          if (plan_extern_rc != 0) {
            return plan_extern_rc;
          }
        }

        if (ctype && !strcmp(ctype, "Constructor")) {
          enum MirCallConstructorSurfaceKind kind =
              classify_mir_call_constructor_surface(bname);
          return emit_constructor_mir_call(f, kind, dst);
        }

        if (ctype && !strcmp(ctype, "Method")) {
          int user_box_method_rc =
              emit_user_box_method_lowering_plan_mir_call(f, bid, ii, dst, recv, args);
          if (user_box_method_rc != 0) {
            return user_box_method_rc;
          }

          enum MirCallMethodBirthSurfaceKind birth_kind =
              classify_mir_call_method_birth_surface(bname, mname);
          int birth_rc = emit_method_birth_mir_call(birth_kind, dst, recv, args);
          if (birth_rc != 0) {
            return birth_rc;
          }

          {
            int filebox_rc = emit_filebox_method_mir_call(f, bname, mname, dst, recv, args);
            if (filebox_rc != 0) {
              return filebox_rc;
            }
          }

          struct GenericMethodRouteState route =
              classify_generic_method_route(bid, ii, recv);
          yyjson_val* generic_method_entry = find_lowering_plan_entry_for_site(bid, ii);
          long long generic_a0 = a0;
          long long generic_a1 = a1;
          hako_llvmc_normalize_generic_method_logical_args(
              generic_method_entry,
              args,
              recv,
              &generic_a0,
              &generic_a1);
          trace_generic_method_route_state(
              ii, dst, recv, ctype, bname, mname, generic_a0, generic_a1, &route);
          enum GenericMethodEmitKind emit_kind =
              classify_generic_method_emit_kind_from_site_lowering_plan(bid, ii);
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
                generic_a0,
                generic_a1,
                emit_kind,
                &route,
                ab,
                sizeof(ab),
                skip_inst);
            if (non_indexof_method_rc != 0) {
              return non_indexof_method_rc;
            }
          }
          if (route.runtime_string_indexof) {
            if (!emit_deferred_array_string_indexof_call(
                    f, route.deferred_array_string_indexof, dst, ab)) {
              if (generic_a0) {
                append_i64_arg_ref(ab, sizeof(ab), generic_a0, ab[0] == '\0');
              }
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.string.indexOf_hh\"(%s)\n", dst, ab);
                set_type(dst, T_I64);
              } else {
                EMIT("  call i64 @\"nyash.string.indexOf_hh\"(%s)\n", ab);
              }
            }
            return 1;
          }
          if (route.runtime_string_lastindexof) {
            if (generic_a0) {
              append_i64_arg_ref(ab, sizeof(ab), generic_a0, ab[0] == '\0');
            }
            if (dst) {
              EMIT("  %%r%lld = call i64 @\"nyash.string.lastIndexOf_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              EMIT("  call i64 @\"nyash.string.lastIndexOf_hh\"(%s)\n", ab);
            }
            return 1;
          }
          if (route.runtime_string_contains) {
            if (generic_a0) {
              append_i64_arg_ref(ab, sizeof(ab), generic_a0, ab[0] == '\0');
            }
            if (dst) {
              EMIT("  %%r%lld = call i64 @\"nyash.string.contains_hh\"(%s)\n", dst, ab);
              set_type(dst, T_I64);
            } else {
              EMIT("  call i64 @\"nyash.string.contains_hh\"(%s)\n", ab);
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
          int plan_global_rc =
              emit_global_call_lowering_plan_mir_call(f, bid, ii, dst, args);
          if (plan_global_rc != 0) {
            return plan_global_rc;
          }
          enum MirCallGlobalSurfaceKind kind =
              classify_mir_call_global_surface(mname);
          return emit_global_mir_call(f, kind, bid, ii, a0);
        }

        return -1;
      }
