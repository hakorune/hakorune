      /*
       * TypedObjectPlan reader.
       *
       * The backend consumes MIR-owned layout facts only. It must not infer
       * user-box slots from names or clone VM InstanceBox behavior.
       */
      auto yyjson_val* typed_object_plans_array(void) {
        yyjson_val* plans = root ? yyjson_obj_get(root, "typed_object_plans") : NULL;
        return (plans && yyjson_is_arr(plans)) ? plans : NULL;
      }

      auto yyjson_val* typed_object_plan_at_index(int plan_index) {
        yyjson_val* plans = typed_object_plans_array();
        if (!plans || plan_index < 0) return NULL;
        return yyjson_arr_get(plans, (size_t)plan_index);
      }

      auto size_t typed_object_plan_count(void) {
        yyjson_val* plans = typed_object_plans_array();
        return plans ? yyjson_arr_size(plans) : 0;
      }

      auto int find_typed_object_plan_index(const char* box_name, int* out_plan_index) {
        yyjson_val* plans = typed_object_plans_array();
        size_t count = typed_object_plan_count();
        if (!(box_name && plans)) return 0;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* plan = yyjson_arr_get(plans, i);
          const char* plan_box = plan ? read_str(plan, "box_name") : NULL;
          if (plan_box && strcmp(plan_box, box_name) == 0) {
            if (out_plan_index) *out_plan_index = (int)i;
            return 1;
          }
        }
        return 0;
      }

      auto long long typed_object_plan_type_id(int plan_index) {
        yyjson_val* plan = typed_object_plan_at_index(plan_index);
        return plan ? read_int(plan, "type_id") : 0;
      }

      auto long long typed_object_plan_field_count(int plan_index) {
        yyjson_val* plan = typed_object_plan_at_index(plan_index);
        return plan ? read_int(plan, "field_count") : 0;
      }

      auto long long typed_object_storage_tag(const char* storage) {
        if (!storage) return 0;
        if (!strcmp(storage, "i64")) return 1;
        if (!strcmp(storage, "handle")) return 2;
        if (!strcmp(storage, "isize")) return 3;
        if (!strcmp(storage, "usize")) return 4;
        if (!strcmp(storage, "i8")) return 5;
        if (!strcmp(storage, "i16")) return 6;
        if (!strcmp(storage, "i32")) return 7;
        if (!strcmp(storage, "u8")) return 8;
        if (!strcmp(storage, "u16")) return 9;
        if (!strcmp(storage, "u32")) return 10;
        if (!strcmp(storage, "u64")) return 11;
        return 0;
      }

      auto int typed_object_storage_is_handle(const char* storage) {
        return storage && !strcmp(storage, "handle");
      }

      auto int typed_object_storage_is_legacy_i64(const char* storage) {
        return storage && !strcmp(storage, "i64");
      }

      auto int typed_object_storage_is_unsigned_exact(const char* storage) {
        return storage &&
               (!strcmp(storage, "usize") ||
                !strcmp(storage, "u8") ||
                !strcmp(storage, "u16") ||
                !strcmp(storage, "u32") ||
                !strcmp(storage, "u64"));
      }

      auto int typed_object_storage_is_signed_exact(const char* storage) {
        return storage &&
               (!strcmp(storage, "isize") ||
                !strcmp(storage, "i8") ||
                !strcmp(storage, "i16") ||
                !strcmp(storage, "i32"));
      }

      auto int typed_object_storage_is_runtime_slot_supported(const char* storage) {
        return typed_object_storage_is_handle(storage) ||
               typed_object_storage_is_legacy_i64(storage) ||
               typed_object_storage_is_unsigned_exact(storage) ||
               typed_object_storage_is_signed_exact(storage);
      }

      auto int typed_object_plan_field_runtime_slot_with_storage(
          int plan_index,
          const char* field_name,
          long long* out_slot,
          const char** out_storage) {
        yyjson_val* plan = typed_object_plan_at_index(plan_index);
        yyjson_val* fields = plan ? yyjson_obj_get(plan, "fields") : NULL;
        size_t count = (fields && yyjson_is_arr(fields)) ? yyjson_arr_size(fields) : 0;
        if (!(field_name && fields)) return 0;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* field = yyjson_arr_get(fields, i);
          const char* name = field ? read_str(field, "name") : NULL;
          const char* storage = field ? read_str(field, "storage") : NULL;
          long long slot = field ? read_int(field, "slot") : -1;
          if (name && storage && strcmp(name, field_name) == 0 &&
              typed_object_storage_is_runtime_slot_supported(storage) &&
              slot >= 0) {
            if (out_slot) *out_slot = slot;
            if (out_storage) *out_storage = storage;
            return 1;
          }
        }
        return 0;
      }

      auto int typed_object_plan_field_runtime_slot(
          int plan_index,
          const char* field_name,
          long long* out_slot) {
        return typed_object_plan_field_runtime_slot_with_storage(
            plan_index,
            field_name,
            out_slot,
            NULL);
      }

      auto int emit_typed_object_layout_registrations(FILE* f) {
        yyjson_val* plans = typed_object_plans_array();
        size_t count = typed_object_plan_count();
        for (size_t pi = 0; pi < count; pi++) {
          yyjson_val* plan = yyjson_arr_get(plans, pi);
          yyjson_val* fields = plan ? yyjson_obj_get(plan, "fields") : NULL;
          size_t field_count = (fields && yyjson_is_arr(fields)) ? yyjson_arr_size(fields) : 0;
          long long type_id = plan ? read_int(plan, "type_id") : 0;
          long long declared_count = plan ? read_int(plan, "field_count") : 0;
          if (!(type_id > 0 && declared_count >= 0)) return 0;
          fprintf(
              f,
              "  %%typed_layout_%lld = call i64 @\"nyash.object.register_typed_layout_hi\"(i64 %lld, i64 %lld)\n",
              type_id,
              type_id,
              declared_count);
          for (size_t fi = 0; fi < field_count; fi++) {
            yyjson_val* field = yyjson_arr_get(fields, fi);
            const char* storage = field ? read_str(field, "storage") : NULL;
            long long slot = field ? read_int(field, "slot") : -1;
            long long tag = typed_object_storage_tag(storage);
            if (!(slot >= 0 && tag > 0)) return 0;
            fprintf(
                f,
                "  %%typed_layout_%lld_%lld = call i64 @\"nyash.object.register_typed_layout_slot_iii\"(i64 %lld, i64 %lld, i64 %lld)\n",
                type_id,
                slot,
                type_id,
                slot,
                tag);
          }
        }
        return 1;
      }

      auto void set_typed_object_binding(long long reg, int plan_index) {
        if (plan_index < 0) return;
        for (size_t i = 0; i < typed_object_bindings_n; i++) {
          if (typed_object_bindings[i].reg == reg) {
            typed_object_bindings[i].plan_index = plan_index;
            return;
          }
        }
        if (typed_object_bindings_n < ARR_LEN(typed_object_bindings)) {
          typed_object_bindings[typed_object_bindings_n].reg = reg;
          typed_object_bindings[typed_object_bindings_n].plan_index = plan_index;
          typed_object_bindings_n++;
        } else {
          note_generic_capacity_overflow("generic_typed_object_binding_inventory_full");
        }
      }

      auto int get_typed_object_binding(long long reg, int* out_plan_index) {
        long long base = resolve_copy_source_reg(reg);
        for (size_t i = 0; i < typed_object_bindings_n; i++) {
          if (typed_object_bindings[i].reg == reg ||
              (base != reg && typed_object_bindings[i].reg == base)) {
            if (out_plan_index) *out_plan_index = typed_object_bindings[i].plan_index;
            return 1;
          }
        }
        return 0;
      }
