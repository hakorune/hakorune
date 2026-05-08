      /*
       * Same-module value metadata reader.
       *
       * This file only publishes backend-local register type/origin facts from
       * MIR-owned metadata and TypedObjectPlan. It must not infer user-box
       * semantics from raw declarations or app-specific box/field names.
       */
      auto void same_module_function_publish_origin(
          long long reg,
          int origin_kind) {
        set_origin(reg, origin_kind);
        set_scan_origin(reg, origin_kind);
      }

      auto void same_module_function_publish_i64_origin(
          long long reg,
          int origin_kind) {
        set_type(reg, T_I64);
        same_module_function_publish_origin(reg, origin_kind);
      }

      auto int same_module_function_publish_typed_object_if_plan(
          long long reg,
          const char* box_name) {
        int typed_plan_index = -1;
        if (!box_name) return 0;
        if (!find_typed_object_plan_index(box_name, &typed_plan_index)) return 0;
        same_module_function_publish_i64_origin(reg, ORG_TYPED_OBJECT);
        set_typed_object_binding(reg, typed_plan_index);
        return 1;
      }

      auto int same_module_function_publish_result_origin_box(
          long long reg,
          const char* box_name) {
        if (!box_name) return 1;
        if (!reg) return 1;
        if (!strcmp(box_name, "StringBox")) {
          same_module_function_publish_i64_origin(reg, ORG_STRING);
          return 1;
        }
        if (!strcmp(box_name, "ArrayBox")) {
          same_module_function_publish_i64_origin(reg, ORG_ARRAY_BIRTH);
          return 1;
        }
        if (!strcmp(box_name, "MapBox")) {
          same_module_function_publish_i64_origin(reg, ORG_MAP_BIRTH);
          return 1;
        }
        return same_module_function_publish_typed_object_if_plan(reg, box_name);
      }

      auto int same_module_function_publish_handle_value_metadata(
          long long reg,
          yyjson_val* value_type) {
        const char* kind = value_type ? read_str(value_type, "kind") : NULL;
        const char* box_type = value_type ? read_str(value_type, "box_type") : NULL;
        if (!(kind && !strcmp(kind, "handle"))) return 0;
        if (box_type && !strcmp(box_type, "StringBox")) {
          same_module_function_publish_i64_origin(reg, ORG_STRING);
          return 1;
        }
        if (box_type && !strcmp(box_type, "ArrayBox")) {
          same_module_function_publish_i64_origin(reg, ORG_ARRAY_BIRTH);
          return 1;
        }
        if (box_type && !strcmp(box_type, "MapBox")) {
          same_module_function_publish_i64_origin(reg, ORG_MAP_BIRTH);
          return 1;
        }
        if (same_module_function_publish_typed_object_if_plan(reg, box_type)) {
          return 1;
        }
        set_type(reg, T_I64);
        return 1;
      }

      auto int same_module_function_publish_reg_metadata(long long reg) {
        char key[32];
        yyjson_val* value_types = fn_metadata ? yyjson_obj_get(fn_metadata, "value_types") : NULL;
        yyjson_val* value_type = NULL;
        const char* scalar_type = NULL;
        const char* kind = NULL;
        if (!(value_types && yyjson_is_obj(value_types))) {
          return 0;
        }
        snprintf(key, sizeof(key), "%lld", reg);
        value_type = yyjson_obj_get(value_types, key);
        if (!value_type) return 0;
        if (yyjson_is_str(value_type)) {
          scalar_type = yyjson_get_str(value_type);
          if (scalar_type && !strcmp(scalar_type, "i1")) {
            set_type(reg, T_I1);
            return 1;
          }
          if (scalar_type &&
              (!strcmp(scalar_type, "i64") ||
               !strcmp(scalar_type, "void"))) {
            set_type(reg, T_I64);
            return 1;
          }
          return 0;
        }
        kind = read_str(value_type, "kind");
        if (kind && !strcmp(kind, "string")) {
          same_module_function_publish_i64_origin(reg, ORG_STRING);
          return 1;
        }
        return same_module_function_publish_handle_value_metadata(reg, value_type);
      }
