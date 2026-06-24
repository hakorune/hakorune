      /*
       * Generic method result metadata consumer.
       *
       * This file consumes already-selected LoweringPlan generic-method rows.
       * It does not choose routes from box or method spelling.
       */
      auto int lowering_plan_generic_method_result_origin_kind(
          const struct LoweringPlanGenericMethodView* view) {
        if (!(view && view->result_origin_box)) return ORG_NONE;
        if (!strcmp(view->result_origin_box, "StringBox")) return ORG_STRING;
        if (!strcmp(view->result_origin_box, "ArrayBox") ||
            !strcmp(view->result_origin_box, "DirectArrayI64")) {
          return ORG_ARRAY_BIRTH;
        }
        if (!strcmp(view->result_origin_box, "MapBox")) return ORG_MAP_BIRTH;
        return ORG_NONE;
      }

      auto int publish_generic_method_result_origin_box(
          long long reg,
          const char* box_name) {
        int typed_plan_index = -1;
        int origin_kind = ORG_NONE;
        if (!box_name || !reg) return 1;
        if (!strcmp(box_name, "StringBox")) {
          origin_kind = ORG_STRING;
        } else if (!strcmp(box_name, "ArrayBox") ||
                   !strcmp(box_name, "DirectArrayI64")) {
          origin_kind = ORG_ARRAY_BIRTH;
        } else if (!strcmp(box_name, "MapBox")) {
          origin_kind = ORG_MAP_BIRTH;
        } else {
          if (!find_typed_object_plan_index(box_name, &typed_plan_index)) {
            return 0;
          }
          set_type(reg, T_I64);
          set_origin(reg, ORG_TYPED_OBJECT);
          set_scan_origin(reg, ORG_TYPED_OBJECT);
          set_typed_object_binding(reg, typed_plan_index);
          return 1;
        }
        set_type(reg, T_I64);
        set_origin(reg, origin_kind);
        set_scan_origin(reg, origin_kind);
        return 1;
      }

      auto int publish_generic_method_result_metadata(
          long long reg,
          const char* box_name) {
        if (publish_value_representation_fact(reg)) return 1;
        return publish_generic_method_result_origin_box(reg, box_name);
      }

      auto int mark_generic_method_result_origin_from_lowering_plan(
          long long bid,
          size_t ii,
          long long dst) {
        yyjson_val* entry = NULL;
        const char* source = NULL;
        struct LoweringPlanGenericMethodView view;
        int origin_kind = ORG_NONE;
        if (!dst) return 0;
        entry = find_lowering_plan_entry_for_site(bid, ii);
        source = entry ? read_str(entry, "source") : NULL;
        if (!(source && !strcmp(source, "generic_method_routes"))) return 0;
        if (!read_lowering_plan_generic_method_view(entry, &view)) return 0;
        origin_kind = lowering_plan_generic_method_result_origin_kind(&view);
        if (origin_kind == ORG_NONE) return 0;
        set_scan_origin(dst, origin_kind);
        set_origin(dst, origin_kind);
        return 1;
      }
