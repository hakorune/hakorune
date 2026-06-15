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

      auto int object_storage_plan_field_has_flattened_name(
          yyjson_val* plan,
          const char* flattened_field) {
        yyjson_val* fields = plan ? yyjson_obj_get(plan, "fields") : NULL;
        size_t count = (fields && yyjson_is_arr(fields)) ? yyjson_arr_size(fields) : 0;
        if (!(fields && flattened_field)) return 0;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* field = yyjson_arr_get(fields, i);
          const char* name = field ? read_str(field, "flattened_field") : NULL;
          if (object_storage_plan_str_eq(name, flattened_field)) return 1;
        }
        return 0;
      }

      auto int object_storage_plan_selected_alignment_result_ready(void) {
        yyjson_val* plan = find_flattened_nested_object_storage_plan(
            "HakoAllocObjectLifecycleFacade",
            "alignment_result",
            "HakoAllocObjectLifecycleAlignmentResult");
        if (!plan) return 0;
        return object_storage_plan_field_has_flattened_name(
                   plan,
                   "alignment_result.last_requested") &&
               object_storage_plan_field_has_flattened_name(
                   plan,
                   "alignment_result.last_normalized") &&
               object_storage_plan_field_has_flattened_name(
                   plan,
                   "alignment_result.last_reason") &&
               object_storage_plan_field_has_flattened_name(
                   plan,
                   "alignment_result.last_supported");
      }

      auto int object_storage_plan_alignment_result_owner_receiver_ready(
          long long owner_reg) {
        int bound_plan_index = -1;
        int facade_plan_index = -1;
        if (!object_storage_plan_selected_alignment_result_ready()) return 0;
        if (!get_typed_object_binding(owner_reg, &bound_plan_index)) return 0;
        if (!find_typed_object_plan_index(
                "HakoAllocObjectLifecycleFacade",
                &facade_plan_index)) {
          return 0;
        }
        return bound_plan_index == facade_plan_index;
      }

      auto const char* object_storage_plan_alignment_result_field_for_method(
          const char* method_name) {
        if (!method_name) return NULL;
        if (object_storage_plan_str_eq(method_name, "requested")) {
          return "alignment_result.last_requested";
        }
        if (object_storage_plan_str_eq(method_name, "normalized")) {
          return "alignment_result.last_normalized";
        }
        if (object_storage_plan_str_eq(method_name, "reason")) {
          return "alignment_result.last_reason";
        }
        if (object_storage_plan_str_eq(method_name, "supported")) {
          return "alignment_result.last_supported";
        }
        return NULL;
      }

      auto const char* object_storage_plan_alignment_result_key_global(
          const char* flattened_field) {
        if (object_storage_plan_str_eq(
                flattened_field,
                "alignment_result.last_requested")) {
          return "@.objstore_alignment_result_last_requested";
        }
        if (object_storage_plan_str_eq(
                flattened_field,
                "alignment_result.last_normalized")) {
          return "@.objstore_alignment_result_last_normalized";
        }
        if (object_storage_plan_str_eq(
                flattened_field,
                "alignment_result.last_reason")) {
          return "@.objstore_alignment_result_last_reason";
        }
        if (object_storage_plan_str_eq(
                flattened_field,
                "alignment_result.last_supported")) {
          return "@.objstore_alignment_result_last_supported";
        }
        return NULL;
      }

      auto size_t object_storage_plan_alignment_result_key_len(
          const char* flattened_field) {
        return flattened_field ? strlen(flattened_field) + 1 : 0;
      }

      auto int emit_object_storage_plan_alignment_result_key_box(
          FILE* f,
          const char* local_name,
          const char* flattened_field) {
        const char* global_name =
            object_storage_plan_alignment_result_key_global(flattened_field);
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
          long long owner_reg) {
        return emit_flattened_nested_field_store_const(
                   f,
                   bid,
                   ii,
                   owner_reg,
                   "alignment_result.last_requested",
                   0,
                   0) &&
               emit_flattened_nested_field_store_const(
                   f,
                   bid,
                   ii,
                   owner_reg,
                   "alignment_result.last_normalized",
                   0,
                   1) &&
               emit_flattened_nested_field_store_const(
                   f,
                   bid,
                   ii,
                   owner_reg,
                   "alignment_result.last_reason",
                   0,
                   2) &&
               emit_flattened_nested_field_store_const(
                   f,
                   bid,
                   ii,
                   owner_reg,
                   "alignment_result.last_supported",
                   0,
                   3);
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
        const char* read_field =
            object_storage_plan_alignment_result_field_for_method(method_name);
        size_t argc = (args && yyjson_is_arr(args)) ? yyjson_arr_size(args) : 0;
        if (!get_flattened_nested_binding(recv, &owner_reg)) return 0;
        if (!object_storage_plan_alignment_result_owner_receiver_ready(owner_reg)) return 0;
        if (read_field) {
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
        if (object_storage_plan_str_eq(method_name, "reset")) {
          if (argc != 0) return -1;
          if (!emit_flattened_nested_alignment_result_reset(f, bid, ii, owner_reg)) {
            return -1;
          }
          if (dst) {
            fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
            set_type(dst, T_I64);
          }
          return 1;
        }
        if (object_storage_plan_str_eq(method_name, "recordFailure") ||
            object_storage_plan_str_eq(method_name, "recordSuccess")) {
          yyjson_val* arg0 = NULL;
          yyjson_val* arg1 = NULL;
          long long requested_reg = 0;
          long long normalized_reg = 0;
          long long reason = object_storage_plan_str_eq(method_name, "recordFailure") ? 1 : 0;
          long long supported = object_storage_plan_str_eq(method_name, "recordSuccess") ? 1 : 0;
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
                  "alignment_result.last_requested",
                  requested_reg,
                  0) ||
              !emit_flattened_nested_field_store_reg(
                  f,
                  bid,
                  ii,
                  owner_reg,
                  "alignment_result.last_normalized",
                  normalized_reg,
                  1) ||
              !emit_flattened_nested_field_store_const(
                  f,
                  bid,
                  ii,
                  owner_reg,
                  "alignment_result.last_reason",
                  reason,
                  2) ||
              !emit_flattened_nested_field_store_const(
                  f,
                  bid,
                  ii,
                  owner_reg,
                  "alignment_result.last_supported",
                  supported,
                  3)) {
            return -1;
          }
          if (dst) {
            fprintf(f, "  %%r%lld = add i64 0, 0\n", dst);
            set_type(dst, T_I64);
          }
          return 1;
        }
        return 0;
      }
