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
                 "ii=%zu dst=%lld recv=%lld recv_org=%d ctype=%s bname=%s mname=%s a0=%lld a1=%lld flags=len:%d get:%d push:%d string:%d string_indexof:%d string_lastindexof:%d string_contains:%d array_string:%d map_get:%d map_size:%d map_has:%d",
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
                 route->runtime_string_lastindexof,
                 route->runtime_string_contains,
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
          long long bid,
          size_t ii,
          long long a0) {
        long long a0_base = resolve_copy_source_reg(a0);
        long long cv = 0;
        int a0_is_string =
            get_origin(a0_base) == ORG_STRING ||
            get_scan_origin(a0_base) == ORG_STRING ||
            get_origin(a0) == ORG_STRING ||
            get_scan_origin(a0) == ORG_STRING;
        if (kind != HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_PRINT) return -1;
        if (a0_is_string) {
          fprintf(f, "  %%print_call_b%lld_i%zu = call i64 @\"nyash.console.log_handle\"(i64 %%r%lld)\n",
                  bid, ii, a0_base ? a0_base : a0);
        } else if (has_const(a0_base, &cv)) {
          fprintf(f, "  %%print_call_b%lld_i%zu = call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([5 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %lld)\n",
                  bid, ii, cv);
        } else {
          fprintf(f, "  %%print_call_b%lld_i%zu = call i32 (ptr, ...) @printf(ptr getelementptr inbounds ([5 x i8], ptr @.fmt_i64, i64 0, i64 0), i64 %%r%lld)\n",
                  bid, ii, a0_base ? a0_base : a0);
        }
        return 1;
      }
      auto int materialize_direct_global_call_arg_if_needed(FILE* f, long long reg) {
        long long base = resolve_copy_source_reg(reg);
        if (!base) return 1;
        if (!get_str_const(base)) return 1;
        if (string_const_box_was_emitted(base)) return 1;
        return emit_materialized_string_const_box_result(f, base);
      }
      auto int global_call_definition_target_is_ready(
          const struct LoweringPlanGlobalCallView* view) {
        if (!view) return 0;
        if (lowering_plan_global_call_view_uses_leaf_i64_definition(view)) {
          return module_leaf_symbol_was_emitted(view->target_symbol);
        }
        if (lowering_plan_global_call_view_uses_generic_i64_or_leaf_definition(view)) {
          return same_module_function_symbol_will_be_defined(view->target_symbol) ||
                 module_leaf_symbol_was_emitted(view->target_symbol);
        }
        if (lowering_plan_global_call_view_requires_same_module_function_definition(view)) {
          return same_module_function_symbol_will_be_defined(view->target_symbol);
        }
        return 0;
      }
      auto int emit_global_call_lowering_plan_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          yyjson_val* args) {
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        const char* source = entry ? read_str(entry, "source") : NULL;
        struct LoweringPlanGlobalCallView view;
        size_t argc = 0;
        int is_void_sentinel_i64_zero = 0;
        char ab[2048];
        if (!entry) return 0;
        if (!(source && !strcmp(source, "global_call_routes"))) return 0;
        if (!read_lowering_plan_global_call_view(entry, &view)) return -1;
        is_void_sentinel_i64_zero =
            lowering_plan_global_call_view_is_direct_void_sentinel_i64_zero(&view);
        if (is_void_sentinel_i64_zero && !dst && !view.has_result_value) {
          /* no result register: consume the void-sentinel return directly */
        } else if (!view.has_result_value || !dst || view.result_value != dst) {
          return -1;
        }
        if (!(args && yyjson_is_arr(args))) return -1;
        argc = yyjson_arr_size(args);
        if ((long long)argc != view.arity || argc > 64) return -1;
        if (!global_call_definition_target_is_ready(&view)) return -1;
        ab[0] = '\0';
        for (size_t ai = 0; ai < argc; ai++) {
          yyjson_val* arg = yyjson_arr_get(args, ai);
          long long arg_reg = 0;
          if (!(arg && yyjson_is_num(arg))) return -1;
          arg_reg = (long long)yyjson_get_sint(arg);
          if (!materialize_direct_global_call_arg_if_needed(f, arg_reg)) return -1;
          append_i64_arg_ref(
              ab,
              sizeof(ab),
              arg_reg,
              ai == 0);
        }
        if (dst) {
          fprintf(f, "  %%r%lld = call i64 @\"%s\"(%s)\n", dst, view.target_symbol, ab);
          set_type(dst, T_I64);
        } else {
          fprintf(f, "  call i64 @\"%s\"(%s)\n", view.target_symbol, ab);
        }
        mark_global_call_result_origin_from_view(&view, dst);
        trace_lowering_plan_entry_hit(
            entry,
            lowering_plan_global_call_view_emit_trace_consumer(&view));
        return 1;
      }
      auto int lowering_plan_extern_call_view_is_valid_with(
          const struct LoweringPlanExternCallView* view,
          const char* route_id,
          const char* core_op,
          const char* route_kind,
          const char* symbol,
          const char* return_shape,
          const char* value_demand) {
        return view &&
               view->route_id &&
               route_id &&
               !strcmp(view->route_id, route_id) &&
               view->core_op &&
               core_op &&
               !strcmp(view->core_op, core_op) &&
               view->route_kind &&
               route_kind &&
               !strcmp(view->route_kind, route_kind) &&
               view->emit_kind &&
               !strcmp(view->emit_kind, "runtime_call") &&
               view->symbol &&
               symbol &&
               !strcmp(view->symbol, symbol) &&
               view->proof &&
               !strcmp(view->proof, "extern_registry") &&
               view->route_proof &&
               !strcmp(view->route_proof, "extern_registry") &&
               view->return_shape &&
               return_shape &&
               !strcmp(view->return_shape, return_shape) &&
               view->value_demand &&
               value_demand &&
               !strcmp(view->value_demand, value_demand) &&
               view->tier == HAKO_LLVMC_LOWERING_PLAN_TIER_COLD_RUNTIME;
      }

      auto int lowering_plan_env_get_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_call_view_is_valid_with(
            view,
            "extern.env.get",
            "EnvGet",
            "extern.env.get",
            "nyash.env.get",
            "string_handle_or_null",
            "runtime_i64_or_handle");
      }

      auto int lowering_plan_env_set_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_call_view_is_valid_with(
            view,
            "extern.env.set",
            "EnvSet",
            "extern.env.set",
            "nyash.env.set",
            "scalar_i64",
            "runtime_i64");
      }

      auto int lowering_plan_hostbridge_extern_invoke_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_call_view_is_valid_with(
            view,
            "extern.hostbridge.extern_invoke",
            "HostBridgeExternInvoke",
            "extern.hostbridge.extern_invoke",
            "nyash.hostbridge.extern_invoke",
            "string_handle_or_null",
            "runtime_i64_or_handle");
      }

      auto int lowering_plan_stage1_emit_program_json_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_call_view_is_valid_with(
            view,
            "extern.stage1.emit_program_json_v0",
            "Stage1EmitProgramJson",
            "extern.stage1.emit_program_json_v0",
            "nyash.stage1.emit_program_json_v0_h",
            "string_handle",
            "runtime_i64_or_handle");
      }

      auto int lowering_plan_stage1_emit_mir_from_source_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_call_view_is_valid_with(
            view,
            "extern.stage1.emit_mir_from_source_v0",
            "Stage1EmitMirFromSource",
            "extern.stage1.emit_mir_from_source_v0",
            "nyash.stage1.emit_mir_from_source_v0_h",
            "string_handle",
            "runtime_i64_or_handle");
      }

      auto int lowering_plan_stage1_emit_mir_from_program_json_view_is_valid(
          const struct LoweringPlanExternCallView* view) {
        return lowering_plan_extern_call_view_is_valid_with(
            view,
            "extern.stage1.emit_mir_from_program_json_v0",
            "Stage1EmitMirFromProgramJson",
            "extern.stage1.emit_mir_from_program_json_v0",
            "nyash.stage1.emit_mir_from_program_json_v0_h",
            "string_handle",
            "runtime_i64_or_handle");
      }

      auto int emit_extern_call_lowering_plan_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          yyjson_val* args) {
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        const char* source = entry ? read_str(entry, "source") : NULL;
        struct LoweringPlanExternCallView view;
        long long arity = 0;
        long long key_value = 0;
        long long value_value = 0;
        long long result_value = 0;
        long long actual_key = 0;
        long long actual_value = 0;
        char ab[160];

        if (!entry) return 0;
        if (!(source && !strcmp(source, "extern_call_routes"))) return 0;
        if (!read_lowering_plan_extern_call_view(entry, &view)) return -1;
        if (!lowering_plan_read_i64_field(entry, "key_value", &key_value)) {
          return -1;
        }
        if (!lowering_plan_read_i64_field(entry, "result_value", &result_value) ||
            result_value != dst) {
          return -1;
        }
        if (lowering_plan_env_get_view_is_valid(&view)) {
          if (!lowering_plan_read_i64_field(entry, "arity", &arity) || arity != 1) {
            return -1;
          }
          if (!(args && yyjson_is_arr(args)) || yyjson_arr_size(args) != 1) {
            return -1;
          }
          actual_key = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
          if (actual_key != key_value) {
            return -1;
          }

          ab[0] = '\0';
          append_i64_arg_ref(ab, sizeof(ab), key_value, 1);
          fprintf(f, "  %%r%lld = call i64 @\"nyash.env.get\"(%s)\n", dst, ab);
          set_type(dst, T_I64);
          set_origin(dst, ORG_STRING);
          set_scan_origin(dst, ORG_STRING);
          trace_lowering_plan_entry_hit(entry, "mir_call_extern_emit");
          return 1;
        }
        if (lowering_plan_env_set_view_is_valid(&view)) {
          if (!lowering_plan_read_i64_field(entry, "arity", &arity) || arity != 2) {
            return -1;
          }
          if (!lowering_plan_read_i64_field(entry, "value_value", &value_value)) {
            return -1;
          }
          if (!(args && yyjson_is_arr(args)) || yyjson_arr_size(args) != 2) {
            return -1;
          }
          actual_key = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
          actual_value = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
          if (actual_key != key_value || actual_value != value_value) {
            return -1;
          }

          ab[0] = '\0';
          append_i64_arg_ref(ab, sizeof(ab), key_value, 1);
          append_i64_arg_ref(ab, sizeof(ab), value_value, 0);
          fprintf(f, "  %%r%lld = call i64 @\"nyash.env.set\"(%s)\n", dst, ab);
          set_type(dst, T_I64);
          trace_lowering_plan_entry_hit(entry, "mir_call_extern_emit");
          return 1;
        }
        if (lowering_plan_hostbridge_extern_invoke_view_is_valid(&view)) {
          long long actual_method = 0;
          if (!lowering_plan_read_i64_field(entry, "arity", &arity) || arity != 3) {
            return -1;
          }
          if (!lowering_plan_read_i64_field(entry, "value_value", &value_value)) {
            return -1;
          }
          if (!(args && yyjson_is_arr(args)) || yyjson_arr_size(args) != 3) {
            return -1;
          }
          actual_key = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
          actual_method = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
          actual_value = (long long)yyjson_get_sint(yyjson_arr_get(args, 2));
          if (actual_key != key_value || !actual_method || actual_value != value_value) {
            return -1;
          }

          fprintf(f, "  call void @llvm.trap()\n");
          if (dst) {
            fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
            set_type(dst, T_I64);
            set_origin(dst, ORG_STRING);
            set_scan_origin(dst, ORG_STRING);
          }
          trace_lowering_plan_entry_hit(entry, "mir_call_hostbridge_extern_invoke_trap_emit");
          return 1;
        }
        if (lowering_plan_stage1_emit_program_json_view_is_valid(&view)) {
          if (!lowering_plan_read_i64_field(entry, "arity", &arity) || arity != 1) {
            return -1;
          }
          if (!(args && yyjson_is_arr(args)) || yyjson_arr_size(args) != 1) {
            return -1;
          }
          actual_key = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
          if (actual_key != key_value) {
            return -1;
          }

          ab[0] = '\0';
          append_i64_arg_ref(ab, sizeof(ab), key_value, 1);
          fprintf(f, "  %%r%lld = call i64 @\"nyash.stage1.emit_program_json_v0_h\"(%s)\n", dst, ab);
          set_type(dst, T_I64);
          set_origin(dst, ORG_STRING);
          set_scan_origin(dst, ORG_STRING);
          trace_lowering_plan_entry_hit(entry, "mir_call_stage1_emit_program_json_emit");
          return 1;
        }
        if (lowering_plan_stage1_emit_mir_from_source_view_is_valid(&view)) {
          if (!lowering_plan_read_i64_field(entry, "arity", &arity) || arity != 1) {
            return -1;
          }
          if (!(args && yyjson_is_arr(args)) || yyjson_arr_size(args) != 1) {
            return -1;
          }
          actual_key = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
          if (actual_key != key_value) {
            return -1;
          }

          ab[0] = '\0';
          append_i64_arg_ref(ab, sizeof(ab), key_value, 1);
          fprintf(f, "  %%r%lld = call i64 @\"nyash.stage1.emit_mir_from_source_v0_h\"(%s)\n", dst, ab);
          set_type(dst, T_I64);
          set_origin(dst, ORG_STRING);
          set_scan_origin(dst, ORG_STRING);
          trace_lowering_plan_entry_hit(entry, "mir_call_stage1_emit_mir_from_source_emit");
          return 1;
        }
        if (lowering_plan_stage1_emit_mir_from_program_json_view_is_valid(&view)) {
          if (!lowering_plan_read_i64_field(entry, "arity", &arity) || arity != 1) {
            return -1;
          }
          if (!(args && yyjson_is_arr(args)) || yyjson_arr_size(args) != 1) {
            return -1;
          }
          actual_key = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
          if (actual_key != key_value) {
            return -1;
          }

          ab[0] = '\0';
          append_i64_arg_ref(ab, sizeof(ab), key_value, 1);
          fprintf(f, "  %%r%lld = call i64 @\"nyash.stage1.emit_mir_from_program_json_v0_h\"(%s)\n", dst, ab);
          set_type(dst, T_I64);
          set_origin(dst, ORG_STRING);
          set_scan_origin(dst, ORG_STRING);
          trace_lowering_plan_entry_hit(entry, "mir_call_stage1_emit_mir_from_program_json_emit");
          return 1;
        }
        return -1;
      }
