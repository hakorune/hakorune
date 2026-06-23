      /*
       * Constructor call route metadata reader.
       *
       * This file consumes MIR-owned constructor_call_routes lowering rows.
       * It must not classify constructor names or select routes.
       */
      struct LoweringPlanConstructorCallView {
        yyjson_val* entry;
        const char* route_id;
        const char* box_type;
        const char* core_op;
        const char* route_kind;
        const char* emit_kind;
        const char* symbol;
        const char* proof;
        const char* route_proof;
        const char* return_shape;
        const char* value_demand;
        const char* result_origin;
        const char* need_kind;
        long long result_value;
        int has_result_value;
        enum LoweringPlanTierMetadata tier;
      };

      auto int read_lowering_plan_constructor_call_view(
          yyjson_val* entry,
          struct LoweringPlanConstructorCallView* out) {
        const char* source = entry ? read_str(entry, "source") : NULL;
        int invalid = 0;
        enum LoweringPlanTierMetadata tier =
            read_lowering_plan_tier_metadata(entry, &invalid);
        long long result_value = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!(source && !strcmp(source, "constructor_call_routes")) || invalid) {
          return 0;
        }
        if (out) {
          out->entry = entry;
          out->route_id = read_str(entry, "source_route_id");
          out->box_type = read_str(entry, "box_type");
          out->core_op = read_str(entry, "core_op");
          out->route_kind = read_str(entry, "route_kind");
          out->emit_kind = read_str(entry, "emit_kind");
          out->symbol = read_str(entry, "symbol");
          out->proof = read_str(entry, "proof");
          out->route_proof = read_str(entry, "route_proof");
          out->return_shape = read_str(entry, "return_shape");
          out->value_demand = read_str(entry, "value_demand");
          out->result_origin = read_str(entry, "result_origin");
          out->need_kind = read_str(entry, "need_kind");
          if (lowering_plan_read_i64_field(entry, "result_value", &result_value)) {
            out->result_value = result_value;
            out->has_result_value = 1;
          }
          out->tier = tier;
        }
        return out &&
               out->route_id &&
               out->box_type &&
               out->core_op &&
               out->route_kind &&
               out->emit_kind &&
               out->symbol &&
               out->proof &&
               out->route_proof &&
               out->return_shape &&
               out->value_demand &&
               out->result_origin &&
               out->need_kind;
      }

      auto int lowering_plan_constructor_call_view_result_origin_kind(
          const struct LoweringPlanConstructorCallView* view) {
        if (!(view && view->result_origin)) return ORG_NONE;
        if (!strcmp(view->result_origin, "map_birth")) return ORG_MAP_BIRTH;
        if (!strcmp(view->result_origin, "array_birth")) return ORG_ARRAY_BIRTH;
        return ORG_NONE;
      }
