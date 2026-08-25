      auto int generic_pure_publish_value_type_metadata(
          long long reg,
          yyjson_val* value_type) {
        const char* scalar_type = NULL;
        const char* kind = NULL;
        const char* box_type = NULL;
        if (!reg || !value_type) return 0;
        if (yyjson_is_str(value_type)) {
          scalar_type = yyjson_get_str(value_type);
          if (scalar_type && !strcmp(scalar_type, "i1")) {
            set_type(reg, T_I1);
            return 1;
          }
          if (scalar_type && !strcmp(scalar_type, "i64")) {
            publish_plain_i64_value(reg);
            return 1;
          }
          if (scalar_type && !strcmp(scalar_type, "void")) {
            set_type(reg, T_I64);
            return 1;
          }
          return 0;
        }
        if (!yyjson_is_obj(value_type)) return 0;
        kind = read_str(value_type, "kind");
        box_type = read_str(value_type, "box_type");
        if (kind && !strcmp(kind, "string")) {
          generic_pure_publish_i64_origin(reg, ORG_STRING);
          return 1;
        }
        if (!(kind && !strcmp(kind, "handle"))) return 0;
        if (box_type && !strcmp(box_type, "StringBox")) {
          generic_pure_publish_i64_origin(reg, ORG_STRING);
          return 1;
        }
        if (box_type && (!strcmp(box_type, "ArrayBox") ||
                         !strcmp(box_type, "DirectArrayI64"))) {
          generic_pure_publish_i64_origin(reg, ORG_ARRAY_BIRTH);
          return 1;
        }
        if (box_type && !strcmp(box_type, "MapBox")) {
          generic_pure_publish_i64_origin(reg, ORG_MAP_BIRTH);
          return 1;
        }
        if (box_type && !strcmp(box_type, "FileBox")) {
          generic_pure_publish_i64_origin(reg, ORG_FILEBOX);
          return 1;
        }
        if (generic_pure_publish_typed_object_if_plan(reg, box_type)) {
          return 1;
        }
        set_type(reg, T_I64);
        return 1;
      }
