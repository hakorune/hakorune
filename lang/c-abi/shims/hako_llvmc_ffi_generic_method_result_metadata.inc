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
