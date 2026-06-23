      /*
       * BoxedSumAbiPlanV1 reader.
       *
       * Backends consume MIR-owned enum ABI rows. They must not infer runtime
       * enum layout from enum names, variant names, or payload spellings.
       */
      auto yyjson_val* boxed_sum_abi_plans_array(void) {
        yyjson_val* plans = root ? yyjson_obj_get(root, "boxed_sum_abi_plans") : NULL;
        return (plans && yyjson_is_arr(plans)) ? plans : NULL;
      }

      auto yyjson_val* boxed_sum_abi_plan_at_index(int plan_index) {
        yyjson_val* plans = boxed_sum_abi_plans_array();
        if (!plans || plan_index < 0) return NULL;
        return yyjson_arr_get(plans, (size_t)plan_index);
      }

      auto size_t boxed_sum_abi_plan_count(void) {
        yyjson_val* plans = boxed_sum_abi_plans_array();
        return plans ? yyjson_arr_size(plans) : 0;
      }

      auto int find_boxed_sum_abi_plan_index(
          const char* enum_name,
          int* out_plan_index) {
        yyjson_val* plans = boxed_sum_abi_plans_array();
        size_t count = boxed_sum_abi_plan_count();
        if (!(enum_name && plans)) return 0;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* plan = yyjson_arr_get(plans, i);
          const char* plan_enum = plan ? read_str(plan, "enum_name") : NULL;
          const char* version = plan ? read_str(plan, "version") : NULL;
          if (plan_enum &&
              version &&
              strcmp(version, "boxed_runtime_v1") == 0 &&
              strcmp(plan_enum, enum_name) == 0) {
            if (out_plan_index) *out_plan_index = (int)i;
            return 1;
          }
        }
        return 0;
      }

      auto long long boxed_sum_abi_runtime_type_id(int plan_index) {
        yyjson_val* plan = boxed_sum_abi_plan_at_index(plan_index);
        return plan ? read_int(plan, "runtime_type_id") : 0;
      }

      auto long long boxed_sum_abi_variant_count(int plan_index) {
        yyjson_val* plan = boxed_sum_abi_plan_at_index(plan_index);
        yyjson_val* variants = plan ? yyjson_obj_get(plan, "variants") : NULL;
        return (variants && yyjson_is_arr(variants))
                   ? (long long)yyjson_arr_size(variants)
                   : 0;
      }

      auto int boxed_sum_abi_variant_payload_storage_is_none(
          int plan_index,
          long long tag) {
        yyjson_val* plan = boxed_sum_abi_plan_at_index(plan_index);
        yyjson_val* variants = plan ? yyjson_obj_get(plan, "variants") : NULL;
        yyjson_val* variant = NULL;
        const char* storage = NULL;
        if (!(variants && yyjson_is_arr(variants) && tag >= 0)) return 0;
        variant = yyjson_arr_get(variants, (size_t)tag);
        storage = variant ? read_str(variant, "payload_storage") : NULL;
        return storage && strcmp(storage, "none") == 0;
      }

      auto int emit_boxed_sum_abi_layout_registrations(FILE* f) {
        size_t count = boxed_sum_abi_plan_count();
        for (size_t pi = 0; pi < count; pi++) {
          long long type_id = boxed_sum_abi_runtime_type_id((int)pi);
          long long variant_count = boxed_sum_abi_variant_count((int)pi);
          if (!(type_id > 0 && variant_count > 0)) return 0;
          fprintf(
              f,
              "  %%boxed_sum_layout_%lld = call i64 @\"nyash.object.register_typed_layout_hi\"(i64 %lld, i64 2)\n",
              type_id,
              type_id);
          fprintf(
              f,
              "  %%boxed_sum_layout_%lld_tag = call i64 @\"nyash.object.register_typed_layout_slot_iii\"(i64 %lld, i64 0, i64 1)\n",
              type_id,
              type_id);
          fprintf(
              f,
              "  %%boxed_sum_layout_%lld_payload = call i64 @\"nyash.object.register_typed_layout_slot_iii\"(i64 %lld, i64 1, i64 2)\n",
              type_id,
              type_id);
        }
        return 1;
      }
