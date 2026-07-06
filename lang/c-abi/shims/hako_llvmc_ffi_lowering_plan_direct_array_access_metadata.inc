      struct LoweringPlanDirectArrayAccessView {
        yyjson_val* entry;
        const char* route_id;
        const char* op;
        const char* array_kind;
        const char* element_type;
        const char* route;
        const char* bounds_policy;
        const char* proof_kind;
        yyjson_val* proof_ids;
        const char* fallback_policy;
        const char* cfg_shape;
        const char* store_semantics;
        long long receiver_value;
        long long index_value;
        long long value_value;
        long long result_value;
        int has_value_value;
        int has_result_value;
      };

      auto int read_lowering_plan_direct_array_access_view(
          yyjson_val* entry,
          struct LoweringPlanDirectArrayAccessView* out) {
        long long receiver_value = 0;
        long long index_value = 0;
        long long value_value = 0;
        long long result_value = 0;
        if (!(entry && out)) return 0;
        memset(out, 0, sizeof(*out));
        out->entry = entry;
        out->route_id = read_str(entry, "route_id");
        out->op = read_str(entry, "op");
        out->array_kind = read_str(entry, "array_kind");
        out->element_type = read_str(entry, "element_type");
        out->route = read_str(entry, "route");
        out->bounds_policy = read_str(entry, "bounds_policy");
        out->proof_kind = read_str(entry, "proof_kind");
        out->proof_ids = yyjson_obj_get(entry, "proof_ids");
        out->fallback_policy = read_str(entry, "fallback_policy");
        out->cfg_shape = read_str(entry, "cfg_shape");
        out->store_semantics = read_str(entry, "store_semantics");
        if (!lowering_plan_read_i64_field(entry, "receiver_value", &receiver_value) ||
            !lowering_plan_read_i64_field(entry, "index_value", &index_value)) {
          return 0;
        }
        out->receiver_value = receiver_value;
        out->index_value = index_value;
        if (lowering_plan_read_i64_field(entry, "value_value", &value_value)) {
          out->value_value = value_value;
          out->has_value_value = 1;
        }
        if (lowering_plan_read_i64_field(entry, "result_value", &result_value)) {
          out->result_value = result_value;
          out->has_result_value = 1;
        }
        return out->route_id &&
               !strcmp(out->route_id, "direct_array.access") &&
               out->op &&
               out->array_kind &&
               !strcmp(out->array_kind, "DirectArrayI64") &&
               out->element_type &&
               !strcmp(out->element_type, "i64") &&
               out->route &&
               out->bounds_policy &&
               out->proof_kind &&
               out->proof_ids &&
               yyjson_is_arr(out->proof_ids) &&
               out->fallback_policy &&
               out->cfg_shape &&
               out->store_semantics;
      }

      auto int lowering_plan_direct_array_access_view_has_proof_id(
          const struct LoweringPlanDirectArrayAccessView* view,
          const char* proof_id) {
        size_t pi = 0;
        if (!(view && view->proof_ids && yyjson_is_arr(view->proof_ids) && proof_id)) return 0;
        for (pi = 0; pi < yyjson_arr_size(view->proof_ids); pi++) {
          const char* actual = yyjson_get_str(yyjson_arr_get(view->proof_ids, pi));
          if (actual && !strcmp(actual, proof_id)) return 1;
        }
        return 0;
      }

      auto int lowering_plan_direct_array_access_view_is_checked_i64(
          const struct LoweringPlanDirectArrayAccessView* view,
          const char* op,
          const char* route) {
        return view &&
               view->op &&
               !strcmp(view->op, op) &&
               view->route &&
               !strcmp(view->route, route) &&
               view->bounds_policy &&
               !strcmp(view->bounds_policy, "checked") &&
               view->proof_kind &&
               !strcmp(view->proof_kind, "exact_front_contract") &&
               lowering_plan_direct_array_access_view_has_proof_id(view, "exact_front_contract") &&
               view->fallback_policy &&
               !strcmp(view->fallback_policy, "allow_checked") &&
               view->cfg_shape &&
               !strcmp(view->cfg_shape, "checked_branching") &&
               view->store_semantics &&
               ((!strcmp(op, "store") &&
                 !strcmp(view->store_semantics, "append_or_overwrite")) ||
                (!strcmp(op, "load") &&
                 !strcmp(view->store_semantics, "not_store")));
      }

      auto int lowering_plan_direct_array_access_view_is_proved_unchecked_range_index_store_i64(
          const struct LoweringPlanDirectArrayAccessView* view,
          const char* route) {
        return view &&
               view->op &&
               !strcmp(view->op, "store") &&
               view->route &&
               !strcmp(view->route, route) &&
               view->bounds_policy &&
               !strcmp(view->bounds_policy, "proved_unchecked") &&
               view->proof_kind &&
               !strcmp(view->proof_kind, "range_index") &&
               lowering_plan_direct_array_access_view_has_proof_id(view, "range_index") &&
               view->fallback_policy &&
               !strcmp(view->fallback_policy, "fail_fast") &&
               view->cfg_shape &&
               !strcmp(view->cfg_shape, "branchless") &&
               view->store_semantics &&
               !strcmp(view->store_semantics, "append_or_overwrite");
      }

      auto int lowering_plan_direct_array_access_view_is_proved_unchecked_stack_top_pop_i64(
          const struct LoweringPlanDirectArrayAccessView* view,
          const char* op,
          const char* route) {
        return view &&
               view->op &&
               !strcmp(view->op, op) &&
               view->route &&
               !strcmp(view->route, route) &&
               view->bounds_policy &&
               !strcmp(view->bounds_policy, "proved_unchecked") &&
               view->proof_kind &&
               !strcmp(view->proof_kind, "stack_top_pop") &&
               lowering_plan_direct_array_access_view_has_proof_id(view, "stack_top_pop") &&
               view->fallback_policy &&
               !strcmp(view->fallback_policy, "fail_fast") &&
               view->cfg_shape &&
               !strcmp(view->cfg_shape, "branchless") &&
               view->store_semantics &&
               ((!strcmp(op, "store") &&
                 !strcmp(view->store_semantics, "overwrite_existing")) ||
                (!strcmp(op, "load") &&
                 !strcmp(view->store_semantics, "not_store")));
      }

      auto int lowering_plan_direct_array_access_view_is_proved_unchecked_caller_precondition_store_i64(
          const struct LoweringPlanDirectArrayAccessView* view,
          const char* route) {
        return view &&
               view->op &&
               !strcmp(view->op, "store") &&
               view->route &&
               !strcmp(view->route, route) &&
               view->bounds_policy &&
               !strcmp(view->bounds_policy, "proved_unchecked") &&
               view->proof_kind &&
               !strcmp(view->proof_kind, "caller_precondition") &&
               lowering_plan_direct_array_access_view_has_proof_id(view, "caller_precondition") &&
               view->fallback_policy &&
               !strcmp(view->fallback_policy, "fail_fast") &&
               view->cfg_shape &&
               !strcmp(view->cfg_shape, "branchless") &&
               view->store_semantics &&
               !strcmp(view->store_semantics, "overwrite_existing");
      }

      auto int lowering_plan_direct_array_access_site_matches_checked_i64(
          long long bid,
          size_t ii,
          const char* op,
          const char* route,
          long long receiver_value,
          long long index_value,
          long long value_value,
          int require_value,
          long long result_value,
          int require_result) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        if (!lowering_plan_direct_array_access_view_is_checked_i64(&view, op, route)) return 0;
        if (resolve_copy_source_reg(view.receiver_value) != resolve_copy_source_reg(receiver_value) ||
            resolve_copy_source_reg(view.index_value) != resolve_copy_source_reg(index_value)) return 0;
        if (require_value &&
            (!view.has_value_value ||
             resolve_copy_source_reg(view.value_value) != resolve_copy_source_reg(value_value))) return 0;
        if (require_result &&
            (!view.has_result_value ||
             resolve_copy_source_reg(view.result_value) != resolve_copy_source_reg(result_value))) return 0;
        return 1;
      }

      auto int lowering_plan_direct_array_access_site_matches_proved_unchecked_range_index_store_i64(
          long long bid,
          size_t ii,
          const char* route,
          long long receiver_value,
          long long index_value,
          long long value_value,
          long long result_value,
          int require_result) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        if (!lowering_plan_direct_array_access_view_is_proved_unchecked_range_index_store_i64(&view, route)) return 0;
        if (resolve_copy_source_reg(view.receiver_value) != resolve_copy_source_reg(receiver_value) ||
            resolve_copy_source_reg(view.index_value) != resolve_copy_source_reg(index_value)) return 0;
        if (!view.has_value_value ||
            resolve_copy_source_reg(view.value_value) != resolve_copy_source_reg(value_value)) return 0;
        if (require_result &&
            (!view.has_result_value ||
             resolve_copy_source_reg(view.result_value) != resolve_copy_source_reg(result_value))) return 0;
        return 1;
      }

      auto int lowering_plan_direct_array_access_site_has_proved_unchecked_range_index_store_i64(
          long long bid,
          size_t ii,
          const char* route) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        return lowering_plan_direct_array_access_view_is_proved_unchecked_range_index_store_i64(
            &view,
            route);
      }

      auto int lowering_plan_direct_array_access_site_matches_proved_unchecked_stack_top_pop_load_i64(
          long long bid,
          size_t ii,
          const char* route,
          long long receiver_value,
          long long index_value,
          long long result_value,
          int require_result) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        if (!lowering_plan_direct_array_access_view_is_proved_unchecked_stack_top_pop_i64(&view, "load", route)) return 0;
        if (resolve_copy_source_reg(view.receiver_value) != resolve_copy_source_reg(receiver_value) ||
            resolve_copy_source_reg(view.index_value) != resolve_copy_source_reg(index_value)) return 0;
        if (require_result &&
            (!view.has_result_value ||
             resolve_copy_source_reg(view.result_value) != resolve_copy_source_reg(result_value))) return 0;
        return 1;
      }

      auto int lowering_plan_direct_array_access_site_matches_proved_unchecked_stack_top_pop_store_i64(
          long long bid,
          size_t ii,
          const char* route,
          long long receiver_value,
          long long index_value,
          long long value_value,
          long long result_value,
          int require_result) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        if (!lowering_plan_direct_array_access_view_is_proved_unchecked_stack_top_pop_i64(&view, "store", route)) return 0;
        if (resolve_copy_source_reg(view.receiver_value) != resolve_copy_source_reg(receiver_value) ||
            resolve_copy_source_reg(view.index_value) != resolve_copy_source_reg(index_value)) return 0;
        if (!view.has_value_value ||
            resolve_copy_source_reg(view.value_value) != resolve_copy_source_reg(value_value)) return 0;
        if (require_result &&
            (!view.has_result_value ||
             resolve_copy_source_reg(view.result_value) != resolve_copy_source_reg(result_value))) return 0;
        return 1;
      }

      auto int lowering_plan_direct_array_access_site_matches_proved_unchecked_caller_precondition_store_i64(
          long long bid,
          size_t ii,
          const char* route,
          long long receiver_value,
          long long index_value,
          long long value_value,
          long long result_value,
          int require_result) {
        struct LoweringPlanDirectArrayAccessView view;
        yyjson_val* entry = find_direct_array_access_plan_for_site(bid, ii);
        if (!read_lowering_plan_direct_array_access_view(entry, &view)) return 0;
        if (!lowering_plan_direct_array_access_view_is_proved_unchecked_caller_precondition_store_i64(&view, route)) return 0;
        if (resolve_copy_source_reg(view.receiver_value) != resolve_copy_source_reg(receiver_value) ||
            resolve_copy_source_reg(view.index_value) != resolve_copy_source_reg(index_value)) return 0;
        if (!view.has_value_value ||
            resolve_copy_source_reg(view.value_value) != resolve_copy_source_reg(value_value)) return 0;
        if (require_result &&
            (!view.has_result_value ||
             resolve_copy_source_reg(view.result_value) != resolve_copy_source_reg(result_value))) return 0;
        return 1;
      }

