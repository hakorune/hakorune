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

      auto int same_module_function_emit_typed_object_field_set(yyjson_val* ins) {
        long long box_reg = read_int(ins, "box");
        long long value_reg = read_int(ins, "value");
        const char* field = read_str(ins, "field");
        int typed_plan_index = -1;
        long long slot = -1;
        char ab[256];
        if (!(box_reg && value_reg)) return 0;
        if (!get_typed_object_binding(box_reg, &typed_plan_index)) return 0;
        if (!typed_object_plan_field_runtime_slot(typed_plan_index, field, &slot)) return 0;
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), box_reg, 1);
        {
          size_t used = strlen(ab);
          if (used >= sizeof(ab)) return 0;
          snprintf(ab + used, sizeof(ab) - used, ", i64 %lld", slot);
        }
        append_i64_arg_ref(ab, sizeof(ab), value_reg, 0);
        EMIT("  call void @\"nyash.object.field_set_hii\"(%s)\n", ab);
        return 1;
      }

      auto int same_module_function_emit_typed_object_field_get(yyjson_val* ins) {
        long long dst = read_int(ins, "dst");
        long long box_reg = read_int(ins, "box");
        const char* field = read_str(ins, "field");
        int typed_plan_index = -1;
        long long slot = -1;
        char ab[256];
        if (!(dst && box_reg)) return 0;
        if (!get_typed_object_binding(box_reg, &typed_plan_index)) return 0;
        if (!typed_object_plan_field_runtime_slot(typed_plan_index, field, &slot)) return 0;
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), box_reg, 1);
        {
          size_t used = strlen(ab);
          if (used >= sizeof(ab)) return 0;
          snprintf(ab + used, sizeof(ab) - used, ", i64 %lld", slot);
        }
        EMIT("  %%r%lld = call i64 @\"nyash.object.field_get_hii\"(%s)\n", dst, ab);
        if (!same_module_function_publish_reg_metadata(dst)) {
          set_type(dst, T_I64);
        }
        return 1;
      }
