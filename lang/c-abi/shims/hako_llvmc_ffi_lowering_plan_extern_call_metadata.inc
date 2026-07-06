      auto int read_lowering_plan_extern_call_view(
          yyjson_val* entry,
          struct LoweringPlanExternCallView* out) {
        const char* source = entry ? read_str(entry, "source") : NULL;
        int invalid = 0;
        enum LoweringPlanTierMetadata tier =
            read_lowering_plan_tier_metadata(entry, &invalid);
        long long result_value = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!(source && !strcmp(source, "extern_call_routes")) ||
            invalid ||
            !lowering_plan_entry_is_extern_registry(entry)) {
          return 0;
        }
        if (out) {
          out->entry = entry;
          out->route_id = read_str(entry, "source_route_id");
          out->source_symbol = read_str(entry, "source_symbol");
          out->core_op = read_str(entry, "core_op");
          out->route_kind = read_str(entry, "route_kind");
          out->emit_kind = read_str(entry, "emit_kind");
          out->symbol = read_str(entry, "symbol");
          out->proof = read_str(entry, "proof");
          out->route_proof = read_str(entry, "route_proof");
          out->return_shape = read_str(entry, "return_shape");
          out->value_demand = read_str(entry, "value_demand");
          if (lowering_plan_read_i64_field(entry, "result_value", &result_value)) {
            out->result_value = result_value;
            out->has_result_value = 1;
          }
          out->tier = tier;
        }
        return out &&
               out->route_id &&
               out->core_op &&
               out->route_kind &&
               out->emit_kind &&
               out->symbol &&
               out->proof;
      }

      auto int lowering_plan_extern_call_view_matches_runtime_route(
          const struct LoweringPlanExternCallView* view,
          const char* route_id,
          const char* core_op,
          const char* route_kind,
          const char* symbol,
          enum LoweringPlanTierMetadata tier) {
        return view &&
               view->route_id &&
               route_id &&
               !strcmp(view->route_id, route_id) &&
               view->core_op &&
               core_op &&
               !strcmp(view->core_op, core_op) &&
               view->route_kind &&
               route_kind &&
               !strcmp(view->route_kind, route_kind) &&
               view->emit_kind &&
               !strcmp(view->emit_kind, "runtime_call") &&
               view->symbol &&
               symbol &&
               !strcmp(view->symbol, symbol) &&
               view->proof &&
               !strcmp(view->proof, "extern_registry") &&
               view->route_proof &&
                !strcmp(view->route_proof, "extern_registry") &&
                view->tier == tier;
      }

      auto int lowering_plan_extern_call_view_result_origin_kind(
          const struct LoweringPlanExternCallView* view) {
        if (!(view && view->return_shape && view->value_demand)) return ORG_NONE;
        if (strcmp(view->value_demand, "runtime_i64_or_handle")) return ORG_NONE;
        if (!strcmp(view->return_shape, "string_handle") ||
            !strcmp(view->return_shape, "string_handle_or_null")) {
          return ORG_STRING;
        }
        return ORG_NONE;
      }

      auto void mark_extern_call_result_origin_from_view(
          const struct LoweringPlanExternCallView* view,
          long long dst) {
        int origin_kind = ORG_NONE;
        if (!dst || !view) return;
        origin_kind = lowering_plan_extern_call_view_result_origin_kind(view);
        if (origin_kind == ORG_NONE) return;
        set_scan_origin(dst, origin_kind);
      }

      auto void mark_extern_call_result_origin_from_lowering_plan(
          long long bid,
          size_t ii,
          long long dst) {
        yyjson_val* entry = NULL;
        struct LoweringPlanExternCallView view;
        if (!dst) return;
        entry = find_lowering_plan_entry_for_site(bid, ii);
        if (!read_lowering_plan_extern_call_view(entry, &view)) return;
        mark_extern_call_result_origin_from_view(&view, dst);
      }

      auto int lowering_plan_extern_call_view_returns_scalar_i64(
          const struct LoweringPlanExternCallView* view) {
        return view &&
               view->return_shape &&
               !strcmp(view->return_shape, "scalar_i64") &&
               view->value_demand;
      }

      auto int lowering_plan_extern_call_view_returns_native_ptr_nullable_transport_i64(
          const struct LoweringPlanExternCallView* view) {
        return view &&
               view->return_shape &&
               !strcmp(view->return_shape, "native_ptr_nullable") &&
               view->value_demand &&
               !strcmp(view->value_demand, "native_ptr_nullable");
      }

      auto int lowering_plan_extern_call_view_returns_void_sentinel_i64_zero(
          const struct LoweringPlanExternCallView* view) {
        return view &&
               view->return_shape &&
               !strcmp(view->return_shape, "void_sentinel_i64_zero") &&
               view->value_demand &&
               !strcmp(view->value_demand, "scalar_i64");
      }

