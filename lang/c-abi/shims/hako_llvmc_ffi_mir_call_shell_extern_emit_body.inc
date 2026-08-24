      auto int emit_extern_call_lowering_plan_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          yyjson_val* args) {
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        const char* source = entry ? read_str(entry, "source") : NULL;
        struct LoweringPlanExternCallView view;
        const struct MirCallExternEmitRule* rule = NULL;
        long long arity = 0;
        long long key_value = 0;
        long long value_value = 0;
        long long result_value = 0;
        long long actual_value = 0;
        long long actual_args[5] = {0, 0, 0, 0, 0};
        size_t argc = 0;
        size_t ai = 0;
        char ab[160];

        if (!entry) return 0;
        if (!(source && !strcmp(source, "extern_call_routes"))) return 0;
        if (!read_lowering_plan_extern_call_view(entry, &view)) return -1;
        rule = find_mir_call_extern_emit_rule(&view);
        if (!rule) return -1;
        if (!lowering_plan_read_i64_field(entry, "key_value", &key_value)) {
          return -1;
        }
        if (view.has_result_value) {
          if (!dst || view.result_value != dst) {
            return -1;
          }
          result_value = view.result_value;
        } else if (dst) {
          return -1;
        }
        if (!lowering_plan_read_i64_field(entry, "arity", &arity) ||
            arity != rule->arity ||
            arity < 0 ||
            arity > (long long)ARR_LEN(actual_args)) {
          return -1;
        }
        if (!(args && yyjson_is_arr(args))) {
          return -1;
        }
        argc = yyjson_arr_size(args);
        if ((long long)argc != arity) {
          return -1;
        }
        for (ai = 0; ai < argc; ai++) {
          yyjson_val* arg = yyjson_arr_get(args, ai);
          if (!(arg && yyjson_is_num(arg))) return -1;
          actual_args[ai] = (long long)yyjson_get_sint(arg);
        }
        if (argc > 0 && actual_args[0] != key_value) {
          return -1;
        }
        if (rule->read_value_value) {
          if (!lowering_plan_read_i64_field(entry, "value_value", &value_value)) {
            return -1;
          }
          actual_value = actual_args[argc - 1];
          if (actual_value != value_value) {
            return -1;
          }
        }
        if (rule->require_nonzero_method_arg && (!actual_args[1])) {
          return -1;
        }
        if (rule->behavior == HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_TRAP_SENTINEL) {
          fprintf(f, "  call void @llvm.trap()\n");
          if (dst) {
            fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
            set_type(dst, T_I64);
            if (rule->set_string_origin) {
              set_origin(dst, ORG_STRING);
              set_scan_origin(dst, ORG_STRING);
            }
          }
          trace_lowering_plan_entry_hit(entry, rule->trace_consumer);
          return 1;
        }
        if (rule->behavior ==
            HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR_RETURN_I64) {
          char size_ref[64];
          if (argc != 1) return -1;
          if (!materialize_direct_global_call_arg_if_needed(f, actual_args[0])) {
            return -1;
          }
          format_extern_i64_value_ref(size_ref, sizeof(size_ref), actual_args[0]);
          fprintf(
              f,
              "  %%hako_mem_alloc_ptr_b%lld_i%zu = call ptr @\"%s\"(i64 %s)\n",
              bid,
              ii,
              rule->symbol,
              size_ref);
          fprintf(
              f,
              "  %%r%lld = ptrtoint ptr %%hako_mem_alloc_ptr_b%lld_i%zu to i64\n",
              dst,
              bid,
              ii);
          set_type(dst, T_I64);
          trace_lowering_plan_entry_hit(entry, rule->trace_consumer);
          return 1;
        }
        if (rule->behavior ==
            HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR_ARG_VOID) {
          char ptr_ref[64];
          if (dst || view.has_result_value || argc != 1) return -1;
          if (!materialize_direct_global_call_arg_if_needed(f, actual_args[0])) {
            return -1;
          }
          format_extern_i64_value_ref(ptr_ref, sizeof(ptr_ref), actual_args[0]);
          fprintf(
              f,
              "  %%hako_mem_free_ptr_b%lld_i%zu = inttoptr i64 %s to ptr\n",
              bid,
              ii,
              ptr_ref);
          fprintf(
              f,
              "  call void @\"%s\"(ptr %%hako_mem_free_ptr_b%lld_i%zu)\n",
              rule->symbol,
              bid,
              ii);
          trace_lowering_plan_entry_hit(entry, rule->trace_consumer);
          return 1;
        }
        if (rule->behavior ==
            HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR3_I64_I64_ARG_RETURN_I64) {
          char cell_ref[64];
          char expected_ref[64];
          char desired_ref[64];
          char success_order_ref[64];
          char failure_order_ref[64];
          if (argc != 5) return -1;
          for (ai = 0; ai < argc; ai++) {
            if (!materialize_direct_global_call_arg_if_needed(f, actual_args[ai])) {
              return -1;
            }
          }
          format_extern_i64_value_ref(cell_ref, sizeof(cell_ref), actual_args[0]);
          format_extern_i64_value_ref(expected_ref, sizeof(expected_ref), actual_args[1]);
          format_extern_i64_value_ref(desired_ref, sizeof(desired_ref), actual_args[2]);
          format_extern_i64_value_ref(success_order_ref, sizeof(success_order_ref), actual_args[3]);
          format_extern_i64_value_ref(failure_order_ref, sizeof(failure_order_ref), actual_args[4]);
          fprintf(
              f,
              "  %%hako_atomic_ptr_cas_cell_b%lld_i%zu = inttoptr i64 %s to ptr\n",
              bid,
              ii,
              cell_ref);
          fprintf(
              f,
              "  %%hako_atomic_ptr_cas_expected_b%lld_i%zu = inttoptr i64 %s to ptr\n",
              bid,
              ii,
              expected_ref);
          fprintf(
              f,
              "  %%hako_atomic_ptr_cas_desired_b%lld_i%zu = inttoptr i64 %s to ptr\n",
              bid,
              ii,
              desired_ref);
          if (dst) {
            fprintf(
                f,
                "  %%hako_atomic_ptr_cas_ptr_b%lld_i%zu = call ptr @\"%s\"(ptr %%hako_atomic_ptr_cas_cell_b%lld_i%zu, ptr %%hako_atomic_ptr_cas_expected_b%lld_i%zu, ptr %%hako_atomic_ptr_cas_desired_b%lld_i%zu, i64 %s, i64 %s)\n",
                bid,
                ii,
                rule->symbol,
                bid,
                ii,
                bid,
                ii,
                bid,
                ii,
                success_order_ref,
                failure_order_ref);
            fprintf(
                f,
                "  %%r%lld = ptrtoint ptr %%hako_atomic_ptr_cas_ptr_b%lld_i%zu to i64\n",
                dst,
                bid,
                ii);
            set_type(dst, T_I64);
          } else {
            fprintf(
                f,
                "  call ptr @\"%s\"(ptr %%hako_atomic_ptr_cas_cell_b%lld_i%zu, ptr %%hako_atomic_ptr_cas_expected_b%lld_i%zu, ptr %%hako_atomic_ptr_cas_desired_b%lld_i%zu, i64 %s, i64 %s)\n",
                rule->symbol,
                bid,
                ii,
                bid,
                ii,
                bid,
                ii,
                success_order_ref,
                failure_order_ref);
          }
          trace_lowering_plan_entry_hit(entry, rule->trace_consumer);
          return 1;
        }
        if (rule->behavior ==
            HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR1_I64_ARG_RETURN_I64) {
          char cell_ref[64];
          char order_ref[64];
          if (argc != 2) return -1;
          for (ai = 0; ai < argc; ai++) {
            if (!materialize_direct_global_call_arg_if_needed(f, actual_args[ai])) {
              return -1;
            }
          }
          format_extern_i64_value_ref(cell_ref, sizeof(cell_ref), actual_args[0]);
          format_extern_i64_value_ref(order_ref, sizeof(order_ref), actual_args[1]);
          fprintf(
              f,
              "  %%hako_atomic_ptr_load_cell_b%lld_i%zu = inttoptr i64 %s to ptr\n",
              bid,
              ii,
              cell_ref);
          fprintf(
              f,
              "  %%hako_atomic_ptr_load_ptr_b%lld_i%zu = call ptr @\"%s\"(ptr %%hako_atomic_ptr_load_cell_b%lld_i%zu, i64 %s)\n",
              bid,
              ii,
              rule->symbol,
              bid,
              ii,
              order_ref);
          fprintf(
              f,
              "  %%r%lld = ptrtoint ptr %%hako_atomic_ptr_load_ptr_b%lld_i%zu to i64\n",
              dst,
              bid,
              ii);
          set_type(dst, T_I64);
          trace_lowering_plan_entry_hit(entry, rule->trace_consumer);
          return 1;
        }
        if (rule->behavior ==
            HAKO_LLVMC_MIR_CALL_EXTERN_EMIT_NATIVE_PTR2_I64_ARG_CALL) {
          char cell_ref[64];
          char value_ref[64];
          char order_ref[64];
          long long planned_value = 0;
          if (argc != 3) return -1;
          if (!lowering_plan_read_i64_field(entry, "value_value", &planned_value) ||
              planned_value != actual_args[1]) {
            return -1;
          }
          for (ai = 0; ai < argc; ai++) {
            if (!materialize_direct_global_call_arg_if_needed(f, actual_args[ai])) {
              return -1;
            }
          }
          format_extern_i64_value_ref(cell_ref, sizeof(cell_ref), actual_args[0]);
          format_extern_i64_value_ref(value_ref, sizeof(value_ref), actual_args[1]);
          format_extern_i64_value_ref(order_ref, sizeof(order_ref), actual_args[2]);
          fprintf(
              f,
              "  %%hako_atomic_ptr_store_cell_b%lld_i%zu = inttoptr i64 %s to ptr\n",
              bid,
              ii,
              cell_ref);
          fprintf(
              f,
              "  %%hako_atomic_ptr_store_value_b%lld_i%zu = inttoptr i64 %s to ptr\n",
              bid,
              ii,
              value_ref);
          if (dst) {
            fprintf(
                f,
                "  %%r%lld = call i64 @\"%s\"(ptr %%hako_atomic_ptr_store_cell_b%lld_i%zu, ptr %%hako_atomic_ptr_store_value_b%lld_i%zu, i64 %s)\n",
                dst,
                rule->symbol,
                bid,
                ii,
                bid,
                ii,
                order_ref);
            set_type(dst, T_I64);
          } else {
            fprintf(
                f,
                "  call i64 @\"%s\"(ptr %%hako_atomic_ptr_store_cell_b%lld_i%zu, ptr %%hako_atomic_ptr_store_value_b%lld_i%zu, i64 %s)\n",
                rule->symbol,
                bid,
                ii,
                bid,
                ii,
                order_ref);
          }
          trace_lowering_plan_entry_hit(entry, rule->trace_consumer);
          return 1;
        }
        ab[0] = '\0';
        for (ai = 0; ai < argc; ai++) {
          append_i64_arg_ref(ab, sizeof(ab), actual_args[ai], ai == 0);
        }
        fprintf(f, "  %%r%lld = call i64 @\"%s\"(%s)\n", dst, rule->symbol, ab);
        set_type(dst, T_I64);
        if (rule->set_string_origin) {
          set_origin(dst, ORG_STRING);
          set_scan_origin(dst, ORG_STRING);
        }
        trace_lowering_plan_entry_hit(entry, rule->trace_consumer);
        return 1;
      }
