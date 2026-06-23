      /*
       * Explicit per-value runtime representation metadata.
       *
       * This reader consumes MIR-owned representation facts. It must not infer
       * boxed-sum handles from enum names, box spellings, or raw i64 sign.
       */
      auto void set_boxed_sum_binding(long long reg, int plan_index) {
        if (!reg || plan_index < 0) return;
        for (size_t i = 0; i < boxed_sum_bindings_n; i++) {
          if (boxed_sum_bindings[i].reg == reg) {
            boxed_sum_bindings[i].plan_index = plan_index;
            return;
          }
        }
        if (boxed_sum_bindings_n < ARR_LEN(boxed_sum_bindings)) {
          boxed_sum_bindings[boxed_sum_bindings_n].reg = reg;
          boxed_sum_bindings[boxed_sum_bindings_n].plan_index = plan_index;
          boxed_sum_bindings_n++;
        } else {
          note_generic_capacity_overflow("generic_boxed_sum_binding_inventory_full");
        }
      }

      auto int get_boxed_sum_binding(long long reg, int* out_plan_index) {
        long long base = resolve_copy_source_reg(reg);
        for (size_t i = 0; i < boxed_sum_bindings_n; i++) {
          if (boxed_sum_bindings[i].reg == reg ||
              (base != reg && boxed_sum_bindings[i].reg == base)) {
            if (out_plan_index) *out_plan_index = boxed_sum_bindings[i].plan_index;
            return 1;
          }
        }
        return 0;
      }

      auto yyjson_val* value_representation_row(long long reg) {
        char key[32];
        yyjson_val* rows = fn_metadata ? yyjson_obj_get(fn_metadata, "value_representations") : NULL;
        if (!reg || !(rows && yyjson_is_obj(rows))) return NULL;
        snprintf(key, sizeof(key), "%lld", reg);
        return yyjson_obj_get(rows, key);
      }

      auto int publish_value_representation_fact(long long reg) {
        yyjson_val* row = value_representation_row(reg);
        const char* kind = row ? read_str(row, "kind") : NULL;
        long long plan_id = 0;
        int plan_index = -1;
        if (!(kind && !strcmp(kind, "boxed_sum_handle"))) return 0;
        plan_id = read_int(row, "abi_plan_id");
        if (!boxed_sum_abi_plan_index_from_plan_id(plan_id, &plan_index)) {
          note_generic_capacity_overflow("boxed_sum_value_representation_invalid_plan");
          return 0;
        }
        set_type(reg, T_I64);
        set_boxed_sum_binding(reg, plan_index);
        return 1;
      }

