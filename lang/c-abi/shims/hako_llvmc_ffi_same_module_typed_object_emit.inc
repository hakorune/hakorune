      /*
       * Same-module typed-object body emit.
       *
       * This file emits typed-object newbox and field access bodies from
       * MIR-owned TypedObjectPlan bindings only. It must not infer layout or
       * slots from raw declaration names.
       */
      auto int same_module_function_emit_newbox(yyjson_val* ins) {
        long long dst = read_int(ins, "dst");
        const char* tname = read_str(ins, "type");
        int typed_plan_index = -1;
        if (tname && !strcmp(tname, "ArrayBox")) {
          same_module_function_emit_i64_birth_call(
              dst,
              "nyash.array.birth_h",
              ORG_ARRAY_BIRTH);
          return 1;
        }
        if (tname && !strcmp(tname, "MapBox")) {
          same_module_function_emit_i64_birth_call(
              dst,
              "nyash.map.birth_h",
              ORG_MAP_BIRTH);
          return 1;
        }
        if (tname && find_typed_object_plan_index(tname, &typed_plan_index)) {
          long long type_id = typed_object_plan_type_id(typed_plan_index);
          long long field_count = typed_object_plan_field_count(typed_plan_index);
          if (dst && type_id > 0 && field_count >= 0) {
            EMIT("  %%r%lld = call i64 @\"nyash.object.new_typed_hi\"(i64 %lld, i64 %lld)\n",
                 dst,
                 type_id,
                 field_count);
            same_module_function_publish_i64_origin(dst, ORG_TYPED_OBJECT);
            set_typed_object_binding(dst, typed_plan_index);
            return 1;
          }
        }
        return 0;
      }

      auto void same_module_function_emit_exact_status_trap(
          const char* status_name,
          long long bid,
          size_t ii) {
        EMIT("  %%exact_status_ok_%lld_%zu = icmp ne i64 %%%s, 0\n", bid, ii, status_name);
        EMIT("  br i1 %%exact_status_ok_%lld_%zu, label %%exact_status_continue_%lld_%zu, label %%exact_status_trap_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("exact_status_trap_%lld_%zu:\n", bid, ii);
        EMIT("  call void @llvm.trap()\n");
        EMIT("  unreachable\n");
        EMIT("exact_status_continue_%lld_%zu:\n", bid, ii);
      }

      auto void same_module_function_emit_unsigned_nonnegative_trap(
          long long value_reg,
          long long bid,
          size_t ii) {
        EMIT("  %%exact_unsigned_nonnegative_%lld_%zu = icmp sge i64 %%r%lld, 0\n",
             bid,
             ii,
             value_reg);
        EMIT("  br i1 %%exact_unsigned_nonnegative_%lld_%zu, label %%exact_unsigned_continue_%lld_%zu, label %%exact_unsigned_trap_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("exact_unsigned_trap_%lld_%zu:\n", bid, ii);
        EMIT("  call void @llvm.trap()\n");
        EMIT("  unreachable\n");
        EMIT("exact_unsigned_continue_%lld_%zu:\n", bid, ii);
      }

      auto int same_module_function_emit_typed_object_field_set(
          yyjson_val* ins,
          long long bid,
          size_t ii) {
        long long box_reg = read_int(ins, "box");
        long long value_reg = read_int(ins, "value");
        const char* field = read_str(ins, "field");
        int typed_plan_index = -1;
        long long slot = -1;
        const char* storage = NULL;
        long long value_base = resolve_copy_source_reg(value_reg);
        long long value_const = 0;
        int value_is_const = has_const(value_base, &value_const);
        char ab[256];
        if (!(box_reg && value_reg)) return 0;
        if (!get_typed_object_binding(box_reg, &typed_plan_index)) return 0;
        if (!typed_object_plan_field_runtime_slot_with_storage(
                typed_plan_index,
                field,
                &slot,
                &storage)) return 0;
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), box_reg, 1);
        {
          size_t used = strlen(ab);
          if (used >= sizeof(ab)) return 0;
          snprintf(ab + used, sizeof(ab) - used, ", i64 %lld", slot);
        }
        append_i64_arg_ref(ab, sizeof(ab), value_reg, 0);
        if (typed_object_storage_is_unsigned_exact(storage)) {
          char status_name[96];
          if (value_is_const && value_const < 0) return 0;
          if (!value_is_const) {
            same_module_function_emit_unsigned_nonnegative_trap(
                value_base,
                bid,
                ii);
          }
          snprintf(status_name, sizeof(status_name), "exact_field_set_u64_%lld_%zu", bid, ii);
          EMIT("  %%%s = call i64 @\"nyash.object.field_set_u64_hiu\"(%s)\n",
               status_name,
               ab);
          same_module_function_emit_exact_status_trap(status_name, bid, ii);
          return 1;
        }
        if (typed_object_storage_is_signed_exact(storage)) {
          char status_name[96];
          snprintf(status_name, sizeof(status_name), "exact_field_set_i64_%lld_%zu", bid, ii);
          EMIT("  %%%s = call i64 @\"nyash.object.field_set_i64_hii\"(%s)\n",
               status_name,
               ab);
          same_module_function_emit_exact_status_trap(status_name, bid, ii);
          return 1;
        }
        EMIT("  call void @\"nyash.object.field_set_hii\"(%s)\n", ab);
        return 1;
      }

      auto int same_module_function_emit_typed_object_field_get(yyjson_val* ins) {
        long long dst = read_int(ins, "dst");
        long long box_reg = read_int(ins, "box");
        const char* field = read_str(ins, "field");
        int typed_plan_index = -1;
        long long slot = -1;
        const char* storage = NULL;
        char ab[256];
        if (!(dst && box_reg)) return 0;
        if (!get_typed_object_binding(box_reg, &typed_plan_index)) return 0;
        if (!typed_object_plan_field_runtime_slot_with_storage(
                typed_plan_index,
                field,
                &slot,
                &storage)) return 0;
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), box_reg, 1);
        {
          size_t used = strlen(ab);
          if (used >= sizeof(ab)) return 0;
          snprintf(ab + used, sizeof(ab) - used, ", i64 %lld", slot);
        }
        if (typed_object_storage_is_unsigned_exact(storage)) {
          EMIT("  %%r%lld = call i64 @\"nyash.object.field_get_u64_hii\"(%s)\n", dst, ab);
        } else if (typed_object_storage_is_signed_exact(storage)) {
          EMIT("  %%r%lld = call i64 @\"nyash.object.field_get_i64_hii\"(%s)\n", dst, ab);
        } else {
          EMIT("  %%r%lld = call i64 @\"nyash.object.field_get_hii\"(%s)\n", dst, ab);
        }
        if (!same_module_function_publish_reg_metadata(dst)) {
          set_type(dst, T_I64);
        }
        return 1;
      }
