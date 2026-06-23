      /*
       * ObjectStoragePlan reader.
       *
       * This consumes MIR JSON representation metadata only. It must not infer
       * flattened object representation from benchmark names, helper names,
       * method names, source file names, or raw user-box declarations.
       */
      auto yyjson_val* object_storage_plans_array(void) {
        yyjson_val* plans = root ? yyjson_obj_get(root, "object_storage_plans") : NULL;
        return (plans && yyjson_is_arr(plans)) ? plans : NULL;
      }

      auto size_t object_storage_plan_count(void) {
        yyjson_val* plans = object_storage_plans_array();
        return plans ? yyjson_arr_size(plans) : 0;
      }

      auto int object_storage_plan_str_eq(const char* lhs, const char* rhs) {
        return lhs && rhs && strcmp(lhs, rhs) == 0;
      }

      auto yyjson_val* find_flattened_nested_object_storage_plan(
          const char* owner_box,
          const char* owner_field,
          const char* nested_box) {
        yyjson_val* plans = object_storage_plans_array();
        size_t count = object_storage_plan_count();
        if (!(plans && owner_box && owner_field && nested_box)) return NULL;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* plan = yyjson_arr_get(plans, i);
          const char* representation = plan ? read_str(plan, "representation") : NULL;
          const char* plan_owner_box = plan ? read_str(plan, "owner_box") : NULL;
          const char* plan_owner_field = plan ? read_str(plan, "owner_field") : NULL;
          const char* plan_nested_box = plan ? read_str(plan, "nested_box") : NULL;
          if (object_storage_plan_str_eq(representation, "flattened_nested_fields") &&
              object_storage_plan_str_eq(plan_owner_box, owner_box) &&
              object_storage_plan_str_eq(plan_owner_field, owner_field) &&
              object_storage_plan_str_eq(plan_nested_box, nested_box)) {
            return plan;
          }
        }
        return NULL;
      }

      auto yyjson_val* object_storage_plan_field_row(
          yyjson_val* plan,
          const char* flattened_field) {
        yyjson_val* fields = plan ? yyjson_obj_get(plan, "fields") : NULL;
        size_t count = (fields && yyjson_is_arr(fields)) ? yyjson_arr_size(fields) : 0;
        if (!(fields && flattened_field)) return NULL;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* field = yyjson_arr_get(fields, i);
          const char* name = field ? read_str(field, "flattened_field") : NULL;
          if (object_storage_plan_str_eq(name, flattened_field)) return field;
        }
        return NULL;
      }

      auto yyjson_val* object_storage_plan_method_row(
          yyjson_val* plan,
          const char* method_name) {
        yyjson_val* methods = plan ? yyjson_obj_get(plan, "methods") : NULL;
        size_t count = (methods && yyjson_is_arr(methods)) ? yyjson_arr_size(methods) : 0;
        if (!(methods && method_name)) return NULL;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* method = yyjson_arr_get(methods, i);
          const char* name = method ? read_str(method, "method") : NULL;
          if (object_storage_plan_str_eq(name, method_name)) return method;
        }
        return NULL;
      }

      auto int object_storage_plan_selected_alignment_result_ready(void) {
        yyjson_val* plan = find_flattened_nested_object_storage_plan(
            "HakoAllocObjectLifecycleFacade",
            "alignment_result",
            "HakoAllocObjectLifecycleAlignmentResult");
        yyjson_val* fields = plan ? yyjson_obj_get(plan, "fields") : NULL;
        yyjson_val* methods = plan ? yyjson_obj_get(plan, "methods") : NULL;
        size_t field_count = (fields && yyjson_is_arr(fields)) ? yyjson_arr_size(fields) : 0;
        if (!plan) return 0;
        if (!(field_count > 0 && methods && yyjson_is_arr(methods))) return 0;
        for (size_t i = 0; i < field_count; i++) {
          yyjson_val* field = yyjson_arr_get(fields, i);
          if (!(read_str(field, "flattened_field") &&
                read_str(field, "key_global") &&
                object_storage_plan_str_eq(read_str(field, "scalar_type"), "i64"))) {
            return 0;
          }
        }
        return 1;
      }

      auto int object_storage_plan_alignment_result_owner_receiver_ready(
          long long owner_reg) {
        int bound_plan_index = -1;
        yyjson_val* plan = find_flattened_nested_object_storage_plan(
            "HakoAllocObjectLifecycleFacade",
            "alignment_result",
            "HakoAllocObjectLifecycleAlignmentResult");
        long long owner_layout_id = plan ? read_int(plan, "owner_layout_id") : 0;
        if (!object_storage_plan_selected_alignment_result_ready()) return 0;
        if (!get_typed_object_binding(owner_reg, &bound_plan_index)) return 0;
        return owner_layout_id > 0 &&
               typed_object_plan_type_id(bound_plan_index) == owner_layout_id;
      }

      auto size_t object_storage_plan_alignment_result_key_len(
          const char* flattened_field) {
        return flattened_field ? strlen(flattened_field) + 1 : 0;
      }

      auto int emit_object_storage_plan_alignment_result_key_box(
          FILE* f,
          const char* local_name,
          const char* flattened_field) {
        yyjson_val* plan = find_flattened_nested_object_storage_plan(
            "HakoAllocObjectLifecycleFacade",
            "alignment_result",
            "HakoAllocObjectLifecycleAlignmentResult");
        yyjson_val* field = object_storage_plan_field_row(plan, flattened_field);
        const char* global_name = field ? read_str(field, "key_global") : NULL;
        size_t key_len =
            object_storage_plan_alignment_result_key_len(flattened_field);
        if (!(f && local_name && global_name && key_len > 0)) return 0;
        fprintf(
            f,
            "  %%%s = call i64 @\"nyash.box.from_i8_string_const\"(ptr getelementptr inbounds ([%zu x i8], ptr %s, i64 0, i64 0))\n",
            local_name,
            key_len,
            global_name);
        return 1;
      }

      auto int emit_object_storage_plan_key_globals(FILE* f) {
        yyjson_val* plan = find_flattened_nested_object_storage_plan(
            "HakoAllocObjectLifecycleFacade",
            "alignment_result",
            "HakoAllocObjectLifecycleAlignmentResult");
        yyjson_val* fields = plan ? yyjson_obj_get(plan, "fields") : NULL;
        size_t count = (fields && yyjson_is_arr(fields)) ? yyjson_arr_size(fields) : 0;
        if (!object_storage_plan_selected_alignment_result_ready()) return 0;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* field = yyjson_arr_get(fields, i);
          const char* flattened_field = field ? read_str(field, "flattened_field") : NULL;
          const char* global_name = field ? read_str(field, "key_global") : NULL;
          size_t key_len = object_storage_plan_alignment_result_key_len(flattened_field);
          if (!(flattened_field && global_name && key_len > 0)) return -1;
          fprintf(
              f,
              "%s = private unnamed_addr constant [%zu x i8] c\"%s\\00\", align 1\n",
              global_name,
              key_len,
              flattened_field);
        }
        return 1;
      }

      auto void set_flattened_nested_binding(long long reg, long long owner_reg) {
        long long owner_base = resolve_copy_source_reg(owner_reg);
        if (!(reg && owner_base)) return;
        for (size_t i = 0; i < flattened_nested_bindings_n; i++) {
          if (flattened_nested_bindings[i].reg == reg) {
            flattened_nested_bindings[i].owner_reg = owner_base;
            return;
          }
        }
        if (flattened_nested_bindings_n < ARR_LEN(flattened_nested_bindings)) {
          flattened_nested_bindings[flattened_nested_bindings_n].reg = reg;
          flattened_nested_bindings[flattened_nested_bindings_n].owner_reg = owner_base;
          flattened_nested_bindings_n++;
        } else {
          note_generic_capacity_overflow("generic_flattened_nested_binding_inventory_full");
        }
      }

      auto int get_flattened_nested_binding(
          long long reg,
          long long* out_owner_reg) {
        long long base = resolve_copy_source_reg(reg);
        for (size_t i = 0; i < flattened_nested_bindings_n; i++) {
          if (flattened_nested_bindings[i].reg == reg ||
              (base != reg && flattened_nested_bindings[i].reg == base)) {
            if (out_owner_reg) {
              *out_owner_reg = flattened_nested_bindings[i].owner_reg;
            }
            return 1;
          }
        }
        return 0;
      }

      auto int emit_flattened_nested_field_load(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          long long owner_reg,
          const char* flattened_field) {
        char key_name[96];
        if (!(dst && owner_reg && flattened_field)) return 0;
        if (!object_storage_plan_selected_alignment_result_ready()) return 0;
        snprintf(key_name, sizeof(key_name), "objstore_key_%lld_%zu_%lld", bid, ii, dst);
        if (!emit_object_storage_plan_alignment_result_key_box(
                f,
                key_name,
                flattened_field)) {
          return 0;
        }
        fprintf(
            f,
            "  %%r%lld = call i64 @\"nyash.runtime_data.get_hh\"(i64 %%r%lld, i64 %%%s)\n",
            dst,
            resolve_copy_source_reg(owner_reg),
            key_name);
        set_type(dst, T_I64);
        return 1;
      }

      auto int emit_flattened_nested_field_store_const(
          FILE* f,
          long long bid,
          size_t ii,
          long long owner_reg,
          const char* flattened_field,
          long long value,
          int ordinal) {
        char key_name[96];
        char status_name[96];
        if (!(owner_reg && flattened_field)) return 0;
        if (!object_storage_plan_selected_alignment_result_ready()) return 0;
        snprintf(key_name, sizeof(key_name), "objstore_key_%lld_%zu_%s", bid, ii, flattened_field);
        for (char* p = key_name; *p; p++) {
          if (*p == '.') *p = '_';
        }
        snprintf(status_name, sizeof(status_name), "objstore_set_%lld_%zu_%d", bid, ii, ordinal);
        if (!emit_object_storage_plan_alignment_result_key_box(
                f,
                key_name,
                flattened_field)) {
          return 0;
        }
        fprintf(
            f,
            "  %%%s = call i64 @\"nyash.runtime_data.set_hhh\"(i64 %%r%lld, i64 %%%s, i64 %lld)\n",
            status_name,
            resolve_copy_source_reg(owner_reg),
            key_name,
            value);
        return 1;
      }

      auto int emit_flattened_nested_field_store_reg(
          FILE* f,
          long long bid,
          size_t ii,
          long long owner_reg,
          const char* flattened_field,
          long long value_reg,
          int ordinal) {
        char key_name[96];
        char status_name[96];
        if (!(owner_reg && flattened_field && value_reg)) return 0;
        if (!object_storage_plan_selected_alignment_result_ready()) return 0;
        snprintf(
            key_name,
            sizeof(key_name),
            "objstore_key_%lld_%zu_%d",
            bid,
            ii,
            ordinal);
        snprintf(
            status_name,
            sizeof(status_name),
            "objstore_set_%lld_%zu_%d",
            bid,
            ii,
            ordinal);
        if (!emit_object_storage_plan_alignment_result_key_box(
                f,
                key_name,
                flattened_field)) {
          return 0;
        }
        fprintf(
            f,
            "  %%%s = call i64 @\"nyash.runtime_data.set_hhh\"(i64 %%r%lld, i64 %%%s, i64 %%r%lld)\n",
            status_name,
            resolve_copy_source_reg(owner_reg),
            key_name,
            resolve_copy_source_reg(value_reg));
        return 1;
      }

      auto int emit_flattened_nested_alignment_result_reset(
          FILE* f,
          long long bid,
          size_t ii,
          long long owner_reg,
          yyjson_val* method_row) {
        yyjson_val* fields = method_row ? yyjson_obj_get(method_row, "fields") : NULL;
        size_t count = (fields && yyjson_is_arr(fields)) ? yyjson_arr_size(fields) : 0;
        if (count == 0) return 0;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* field = yyjson_arr_get(fields, i);
          const char* flattened_field = field && yyjson_is_str(field) ? yyjson_get_str(field) : NULL;
          if (!emit_flattened_nested_field_store_const(
                  f,
                  bid,
                  ii,
                  owner_reg,
                  flattened_field,
                  0,
                  (int)i)) {
            return 0;
          }
        }
        return 1;
      }

      auto int emit_flattened_nested_alignment_result_method_call(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          long long recv,
          const char* method_name,
          yyjson_val* args) {
        long long owner_reg = 0;
        yyjson_val* plan = find_flattened_nested_object_storage_plan(
            "HakoAllocObjectLifecycleFacade",
            "alignment_result",
            "HakoAllocObjectLifecycleAlignmentResult");
        yyjson_val* method_row = object_storage_plan_method_row(plan, method_name);
        const char* method_kind = method_row ? read_str(method_row, "kind") : NULL;
        size_t argc = (args && yyjson_is_arr(args)) ? yyjson_arr_size(args) : 0;
        if (!get_flattened_nested_binding(recv, &owner_reg)) return 0;
        if (!object_storage_plan_alignment_result_owner_receiver_ready(owner_reg)) return 0;
        if (object_storage_plan_str_eq(method_kind, "read_i64")) {
          const char* read_field = read_str(method_row, "field");
          if (argc != 0) return -1;
          return emit_flattened_nested_field_load(
              f,
              bid,
              ii,
              dst,
              owner_reg,
              read_field)
              ? 1
              : -1;
        }
        if (object_storage_plan_str_eq(method_kind, "reset_all_i64_zero")) {
          if (argc != 0) return -1;
          if (!emit_flattened_nested_alignment_result_reset(
                  f,
                  bid,
                  ii,
                  owner_reg,
                  method_row)) {
            return -1;
          }
          if (dst) {
            fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
            set_type(dst, T_I64);
          }
          return 1;
        }
        if (object_storage_plan_str_eq(method_kind, "record_pair_with_const_fields")) {
          yyjson_val* arg0 = NULL;
          yyjson_val* arg1 = NULL;
          yyjson_val* const_fields = yyjson_obj_get(method_row, "const_fields");
          long long requested_reg = 0;
          long long normalized_reg = 0;
          const char* arg0_field = read_str(method_row, "arg0_field");
          const char* arg1_field = read_str(method_row, "arg1_field");
          size_t const_count =
              (const_fields && yyjson_is_arr(const_fields)) ? yyjson_arr_size(const_fields) : 0;
          if (argc != 2) return -1;
          arg0 = yyjson_arr_get(args, 0);
          arg1 = yyjson_arr_get(args, 1);
          if (!(arg0 && yyjson_is_num(arg0) && arg1 && yyjson_is_num(arg1))) return -1;
          requested_reg = (long long)yyjson_get_sint(arg0);
          normalized_reg = (long long)yyjson_get_sint(arg1);
          if (!emit_flattened_nested_field_store_reg(
                  f,
                  bid,
                  ii,
                  owner_reg,
                  arg0_field,
                  requested_reg,
                  0) ||
              !emit_flattened_nested_field_store_reg(
                  f,
                  bid,
                  ii,
                  owner_reg,
                  arg1_field,
                  normalized_reg,
                  1)) {
            return -1;
          }
          for (size_t ci = 0; ci < const_count; ci++) {
            yyjson_val* const_row = yyjson_arr_get(const_fields, ci);
            const char* const_field = read_str(const_row, "field");
            long long const_value = read_int(const_row, "value");
            if (!emit_flattened_nested_field_store_const(
                    f,
                    bid,
                    ii,
                    owner_reg,
                    const_field,
                    const_value,
                    (int)(ci + 2))) {
              return -1;
            }
          }
          if (dst) {
            fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
            set_type(dst, T_I64);
          }
          return 1;
        }
        return 0;
      }
