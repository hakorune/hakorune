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

      auto int find_typed_object_plan_index(const char* box_name, int* out_plan_index) {
        yyjson_val* plans = typed_object_plans_array();
        size_t count = plans ? yyjson_arr_size(plans) : 0;
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

      auto int typed_object_plan_field_i64_slot(
          int plan_index,
          const char* field_name,
          long long* out_slot) {
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
              strcmp(storage, "i64") == 0 && slot >= 0) {
            if (out_slot) *out_slot = slot;
            return 1;
          }
        }
        return 0;
      }

      auto void set_typed_object_binding(long long reg, int plan_index) {
        if (!reg || plan_index < 0) return;
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
