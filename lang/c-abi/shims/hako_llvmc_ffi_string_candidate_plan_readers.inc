      #ifndef HAKO_LLVMC_STRING_CANDIDATE_PLAN_READERS_INC
      #define HAKO_LLVMC_STRING_CANDIDATE_PLAN_READERS_INC

      auto yyjson_val* hako_llvmc_string_corridor_value_candidates(
          long long value) {
        char key[32];
        yyjson_val* candidates = NULL;
        if (!(fn_metadata && value)) return NULL;
        value = resolve_copy_source_reg(value);
        snprintf(key, sizeof(key), "%lld", value);
        candidates = yyjson_obj_get(fn_metadata, "string_corridor_candidates");
        if (!(candidates && yyjson_is_obj(candidates))) return NULL;
        candidates = yyjson_obj_get(candidates, key);
        if (!(candidates && yyjson_is_arr(candidates))) return NULL;
        return candidates;
      }

      auto yyjson_val* hako_llvmc_string_corridor_candidate_plan_by_kind(
          long long value,
          const char* kind) {
        yyjson_val* candidates = hako_llvmc_string_corridor_value_candidates(value);
        size_t count = candidates ? yyjson_arr_size(candidates) : 0;
        if (!(candidates && kind)) return NULL;
        for (size_t i = 0; i < count; i++) {
          yyjson_val* candidate = yyjson_arr_get(candidates, i);
          const char* candidate_kind =
              candidate ? yyjson_get_str(yyjson_obj_get(candidate, "kind")) : NULL;
          yyjson_val* plan = candidate ? yyjson_obj_get(candidate, "plan") : NULL;
          if (candidate_kind && !strcmp(candidate_kind, kind) && plan &&
              yyjson_is_obj(plan)) {
            return plan;
          }
        }
        return NULL;
      }

      auto int hako_llvmc_string_corridor_read_plan_window(
          long long value,
          const char* kind,
          long long* start,
          long long* end) {
        yyjson_val* plan =
            hako_llvmc_string_corridor_candidate_plan_by_kind(value, kind);
        yyjson_val* start_val = plan ? yyjson_obj_get(plan, "start") : NULL;
        yyjson_val* end_val = plan ? yyjson_obj_get(plan, "end") : NULL;
        if (!(start_val && end_val && yyjson_is_num(start_val) && yyjson_is_num(end_val))) {
          return 0;
        }
        if (start) *start = (long long)yyjson_get_sint(start_val);
        if (end) *end = (long long)yyjson_get_sint(end_val);
        return 1;
      }

      auto int hako_llvmc_string_corridor_read_plan_source_window(
          long long value,
          const char* kind,
          long long* source_root,
          long long* start,
          long long* end) {
        yyjson_val* plan =
            hako_llvmc_string_corridor_candidate_plan_by_kind(value, kind);
        yyjson_val* source_val = plan ? yyjson_obj_get(plan, "source_root") : NULL;
        yyjson_val* start_val = plan ? yyjson_obj_get(plan, "start") : NULL;
        yyjson_val* end_val = plan ? yyjson_obj_get(plan, "end") : NULL;
        if (!(source_val && start_val && end_val && yyjson_is_num(source_val) &&
              yyjson_is_num(start_val) && yyjson_is_num(end_val))) {
          return 0;
        }
        if (source_root) *source_root = (long long)yyjson_get_sint(source_val);
        if (start) *start = (long long)yyjson_get_sint(start_val);
        if (end) *end = (long long)yyjson_get_sint(end_val);
        return 1;
      }

      auto int hako_llvmc_string_corridor_plan_has_publication_contract(
          long long value,
          const char* kind,
          const char* contract_name) {
        yyjson_val* plan =
            hako_llvmc_string_corridor_candidate_plan_by_kind(value, kind);
        yyjson_val* contract_val =
            plan ? yyjson_obj_get(plan, "publication_contract") : NULL;
        if (!(contract_name && contract_val && yyjson_is_str(contract_val))) {
          return 0;
        }
        return strcmp(yyjson_get_str(contract_val), contract_name) == 0;
      }

      auto int hako_llvmc_string_corridor_read_route_window_from_placement_effect_routes(
          long long value,
          long long* start,
          long long* end) {
        yyjson_val* routes;
        size_t i;
        long long resolved_value = resolve_copy_source_reg(value);

        if (!(fn_metadata && resolved_value)) return 0;
        routes = yyjson_obj_get(fn_metadata, "placement_effect_routes");
        if (!(routes && yyjson_is_arr(routes))) return 0;

        for (i = 0; i < yyjson_arr_size(routes); i++) {
          yyjson_val* row = yyjson_arr_get(routes, i);
          yyjson_val* source = row ? yyjson_obj_get(row, "source") : NULL;
          yyjson_val* decision = row ? yyjson_obj_get(row, "decision") : NULL;
          yyjson_val* route_value = row ? yyjson_obj_get(row, "value") : NULL;
          yyjson_val* route_start = row ? yyjson_obj_get(row, "window_start") : NULL;
          yyjson_val* route_end = row ? yyjson_obj_get(row, "window_end") : NULL;

          if (!(source && decision && route_value && route_start && route_end &&
                yyjson_is_str(source) && yyjson_is_str(decision) &&
                yyjson_is_num(route_value) && yyjson_is_num(route_start) &&
                yyjson_is_num(route_end))) {
            continue;
          }
          if (strcmp(yyjson_get_str(source), "string_corridor") != 0) {
            continue;
          }
          if (strcmp(yyjson_get_str(decision), "direct_kernel_entry") != 0) {
            continue;
          }
          if (resolve_copy_source_reg((long long)yyjson_get_sint(route_value)) != resolved_value) {
            continue;
          }

          if (start) *start = (long long)yyjson_get_sint(route_start);
          if (end) *end = (long long)yyjson_get_sint(route_end);
          return 1;
        }

        return 0;
      }

      auto int hako_llvmc_string_corridor_read_concat_triplet_plan_values(
          long long value,
          const char* kind,
          long long* left_value,
          long long* middle,
          long long* right_value) {
        yyjson_val* plan =
            hako_llvmc_string_corridor_candidate_plan_by_kind(value, kind);
        yyjson_val* proof = plan ? yyjson_obj_get(plan, "proof") : NULL;
        yyjson_val* proof_kind = proof ? yyjson_obj_get(proof, "kind") : NULL;
        yyjson_val* left_val = proof ? yyjson_obj_get(proof, "left_value") : NULL;
        yyjson_val* middle_val = proof ? yyjson_obj_get(proof, "middle") : NULL;
        yyjson_val* right_val = proof ? yyjson_obj_get(proof, "right_value") : NULL;
        if (!(proof && yyjson_is_obj(proof) && proof_kind && yyjson_is_str(proof_kind) &&
              !strcmp(yyjson_get_str(proof_kind), "concat_triplet") && left_val &&
              middle_val && right_val && yyjson_is_num(left_val) &&
              yyjson_is_num(middle_val) && yyjson_is_num(right_val))) {
          return 0;
        }
        if (left_value) *left_value = (long long)yyjson_get_sint(left_val);
        if (middle) *middle = (long long)yyjson_get_sint(middle_val);
        if (right_value) *right_value = (long long)yyjson_get_sint(right_val);
        return 1;
      }

      auto int hako_llvmc_string_corridor_read_any_plan_source_window(
          long long value,
          long long* source_root,
          long long* start,
          long long* end) {
        return hako_llvmc_string_corridor_read_plan_source_window(
                   value,
                   "direct_kernel_entry",
                   source_root,
                   start,
                   end) ||
               hako_llvmc_string_corridor_read_plan_source_window(
                   value,
                   "publication_sink",
                   source_root,
                   start,
                   end) ||
               hako_llvmc_string_corridor_read_plan_source_window(
                   value,
                   "materialization_sink",
                   source_root,
                   start,
                   end) ||
               hako_llvmc_string_corridor_read_plan_source_window(
                   value,
                   "borrowed_corridor_fusion",
                    source_root,
                    start,
                    end);
      }

      auto yyjson_val* hako_llvmc_string_kernel_plan_by_value(long long value) {
        char key[32];
        yyjson_val* plans = NULL;
        if (!(fn_metadata && value)) return NULL;
        value = resolve_copy_source_reg(value);
        snprintf(key, sizeof(key), "%lld", value);
        plans = yyjson_obj_get(fn_metadata, "string_kernel_plans");
        if (!(plans && yyjson_is_obj(plans))) return NULL;
        plans = yyjson_obj_get(plans, key);
        if (!(plans && yyjson_is_obj(plans))) return NULL;
        return plans;
      }

      auto int hako_llvmc_string_kernel_plan_read_alias_fact(
          long long value,
          const char* fact,
          int* out) {
        yyjson_val* plan = hako_llvmc_string_kernel_plan_by_value(value);
        yyjson_val* read_alias = plan ? yyjson_obj_get(plan, "read_alias") : NULL;
        yyjson_val* fact_val = read_alias && fact ? yyjson_obj_get(read_alias, fact) : NULL;
        if (!(fact_val && yyjson_is_bool(fact_val))) return 0;
        if (out) *out = yyjson_get_bool(fact_val) ? 1 : 0;
        return 1;
      }

      auto int hako_llvmc_string_kernel_plan_has_publication_contract(
          long long value,
          const char* contract_name) {
        yyjson_val* plan = hako_llvmc_string_kernel_plan_by_value(value);
        yyjson_val* contract_val =
            plan ? yyjson_obj_get(plan, "publication_contract") : NULL;
        if (!(contract_name && contract_val && yyjson_is_str(contract_val))) {
          return 0;
        }
        return strcmp(yyjson_get_str(contract_val), contract_name) == 0;
      }

      auto int hako_llvmc_string_kernel_plan_read_source_window(
          long long value,
          long long* source_root,
          long long* start,
          long long* end) {
        yyjson_val* plan = hako_llvmc_string_kernel_plan_by_value(value);
        yyjson_val* parts = plan ? yyjson_obj_get(plan, "parts") : NULL;
        yyjson_val* legality = plan ? yyjson_obj_get(plan, "legality") : NULL;
        yyjson_val* read_alias = plan ? yyjson_obj_get(plan, "read_alias") : NULL;
        yyjson_val* part = parts ? yyjson_arr_get(parts, 0) : NULL;
        yyjson_val* family_val = plan ? yyjson_obj_get(plan, "family") : NULL;
        yyjson_val* byte_exact_val = legality ? yyjson_obj_get(legality, "byte_exact") : NULL;
        yyjson_val* source_window_val =
            read_alias ? yyjson_obj_get(read_alias, "source_window") : NULL;
        yyjson_val* kind_val = part ? yyjson_obj_get(part, "kind") : NULL;
        yyjson_val* source_val = part ? yyjson_obj_get(part, "source") : NULL;
        yyjson_val* start_val = part ? yyjson_obj_get(part, "start") : NULL;
        yyjson_val* end_val = part ? yyjson_obj_get(part, "end") : NULL;
        if (!(plan && parts && yyjson_is_arr(parts) && yyjson_arr_size(parts) == 1 &&
              family_val && yyjson_is_str(family_val) &&
              !strcmp(yyjson_get_str(family_val), "borrowed_slice_window") &&
              byte_exact_val && yyjson_is_bool(byte_exact_val) &&
              yyjson_get_bool(byte_exact_val) && source_window_val &&
              yyjson_is_bool(source_window_val) && yyjson_get_bool(source_window_val) &&
              kind_val && yyjson_is_str(kind_val) &&
              !strcmp(yyjson_get_str(kind_val), "slice") && source_val &&
              yyjson_is_num(source_val) && start_val && yyjson_is_num(start_val) &&
              end_val && yyjson_is_num(end_val))) {
          return 0;
        }
        if (source_root) *source_root = (long long)yyjson_get_sint(source_val);
        if (start) *start = (long long)yyjson_get_sint(start_val);
        if (end) *end = (long long)yyjson_get_sint(end_val);
        return 1;
      }

      auto int hako_llvmc_string_kernel_plan_read_publication_boundary_window(
          long long value,
          const char* contract_name,
          long long* source_root,
          long long* start,
          long long* end) {
        if (!(contract_name &&
              hako_llvmc_string_kernel_plan_has_publication_contract(value, contract_name))) {
          return 0;
        }
        if (hako_llvmc_string_kernel_plan_read_source_window(value, source_root, start, end)) {
          return 1;
        }
        return hako_llvmc_string_corridor_plan_has_publication_contract(
                   value, "direct_kernel_entry", contract_name) &&
               hako_llvmc_string_corridor_read_plan_source_window(
                   value, "direct_kernel_entry", source_root, start, end);
      }

      auto int hako_llvmc_string_kernel_plan_read_insert_mid_window_values(
          long long value,
          long long* source_root,
          long long* middle_value,
          long long* split) {
        yyjson_val* plan = hako_llvmc_string_kernel_plan_by_value(value);
        yyjson_val* parts = plan ? yyjson_obj_get(plan, "parts") : NULL;
        yyjson_val* legality = plan ? yyjson_obj_get(plan, "legality") : NULL;
        yyjson_val* read_alias = plan ? yyjson_obj_get(plan, "read_alias") : NULL;
        yyjson_val* first_part = parts ? yyjson_arr_get(parts, 0) : NULL;
        yyjson_val* middle_part = parts ? yyjson_arr_get(parts, 1) : NULL;
        yyjson_val* last_part = parts ? yyjson_arr_get(parts, 2) : NULL;
        yyjson_val* plan_source_root = plan ? yyjson_obj_get(plan, "source_root") : NULL;
        yyjson_val* family_val = plan ? yyjson_obj_get(plan, "family") : NULL;
        yyjson_val* consumer_val = plan ? yyjson_obj_get(plan, "consumer") : NULL;
        yyjson_val* retained_form_val = plan ? yyjson_obj_get(plan, "retained_form") : NULL;
        yyjson_val* byte_exact_val = legality ? yyjson_obj_get(legality, "byte_exact") : NULL;
        yyjson_val* source_window_val =
            read_alias ? yyjson_obj_get(read_alias, "source_window") : NULL;
        yyjson_val* piecewise_val =
            read_alias ? yyjson_obj_get(read_alias, "piecewise_subrange") : NULL;
        yyjson_val* first_kind = first_part ? yyjson_obj_get(first_part, "kind") : NULL;
        yyjson_val* middle_kind = middle_part ? yyjson_obj_get(middle_part, "kind") : NULL;
        yyjson_val* last_kind = last_part ? yyjson_obj_get(last_part, "kind") : NULL;
        yyjson_val* first_source = first_part ? yyjson_obj_get(first_part, "source") : NULL;
        yyjson_val* last_source = last_part ? yyjson_obj_get(last_part, "source") : NULL;
        yyjson_val* first_start = first_part ? yyjson_obj_get(first_part, "start") : NULL;
        yyjson_val* first_end = first_part ? yyjson_obj_get(first_part, "end") : NULL;
        yyjson_val* last_start = last_part ? yyjson_obj_get(last_part, "start") : NULL;
        yyjson_val* middle_value_val = middle_part ? yyjson_obj_get(middle_part, "value") : NULL;

        if (!(plan && parts && yyjson_is_arr(parts) && yyjson_arr_size(parts) == 3 &&
              plan_source_root && yyjson_is_num(plan_source_root) && family_val &&
              yyjson_is_str(family_val) && consumer_val &&
              yyjson_is_str(consumer_val) && retained_form_val &&
              yyjson_is_str(retained_form_val) && byte_exact_val &&
              yyjson_is_bool(byte_exact_val) && yyjson_get_bool(byte_exact_val) &&
              source_window_val && yyjson_is_bool(source_window_val) &&
              yyjson_get_bool(source_window_val) && piecewise_val &&
              yyjson_is_bool(piecewise_val) && yyjson_get_bool(piecewise_val) &&
              first_kind && yyjson_is_str(first_kind) && middle_kind &&
              yyjson_is_str(middle_kind) && last_kind && yyjson_is_str(last_kind) &&
              !strcmp(yyjson_get_str(family_val), "concat_triplet_window") &&
              !strcmp(yyjson_get_str(consumer_val), "direct_kernel_entry") &&
              !strcmp(yyjson_get_str(retained_form_val), "borrowed_text") &&
              !strcmp(yyjson_get_str(first_kind), "slice") &&
              !strcmp(yyjson_get_str(middle_kind), "const") &&
              !strcmp(yyjson_get_str(last_kind), "slice") && first_source &&
              yyjson_is_num(first_source) && last_source && yyjson_is_num(last_source) &&
              first_start && yyjson_is_num(first_start) && first_end &&
              yyjson_is_num(first_end) && last_start && yyjson_is_num(last_start) &&
              middle_value_val && yyjson_is_num(middle_value_val))) {
          return 0;
        }

        if ((long long)yyjson_get_sint(first_source) !=
                (long long)yyjson_get_sint(plan_source_root) ||
            (long long)yyjson_get_sint(last_source) !=
                (long long)yyjson_get_sint(plan_source_root) ||
            (long long)yyjson_get_sint(first_start) != 0 ||
            (long long)yyjson_get_sint(first_end) !=
                (long long)yyjson_get_sint(last_start)) {
          return 0;
        }

        if (source_root) *source_root = (long long)yyjson_get_sint(plan_source_root);
        if (middle_value) *middle_value = (long long)yyjson_get_sint(middle_value_val);
        if (split) *split = (long long)yyjson_get_sint(first_end);
        return 1;
      }

      auto int hako_llvmc_string_kernel_plan_is_slot_text_carrier(long long value) {
        yyjson_val* plan = hako_llvmc_string_kernel_plan_by_value(value);
        yyjson_val* legality = plan ? yyjson_obj_get(plan, "legality") : NULL;
        yyjson_val* consumer_val = plan ? yyjson_obj_get(plan, "consumer") : NULL;
        yyjson_val* text_consumer_val =
            plan ? yyjson_obj_get(plan, "text_consumer") : NULL;
        yyjson_val* carrier_val = plan ? yyjson_obj_get(plan, "carrier") : NULL;
        yyjson_val* retained_form_val =
            plan ? yyjson_obj_get(plan, "retained_form") : NULL;
        yyjson_val* boundary_val =
            plan ? yyjson_obj_get(plan, "publication_boundary") : NULL;
        yyjson_val* requires_slot_val =
            legality ? yyjson_obj_get(legality, "requires_kernel_text_slot") : NULL;
        if (!(consumer_val && yyjson_is_str(consumer_val) && text_consumer_val &&
              yyjson_is_str(text_consumer_val) && carrier_val &&
              yyjson_is_str(carrier_val) && retained_form_val &&
              yyjson_is_str(retained_form_val) && boundary_val &&
              yyjson_is_str(boundary_val) && requires_slot_val &&
              yyjson_is_bool(requires_slot_val) &&
              yyjson_get_bool(requires_slot_val))) {
          return 0;
        }
        return !strcmp(yyjson_get_str(consumer_val), "direct_kernel_entry") &&
               !strcmp(yyjson_get_str(text_consumer_val), "slot_text") &&
               !strcmp(yyjson_get_str(carrier_val), "kernel_text_slot") &&
               !strcmp(yyjson_get_str(retained_form_val), "borrowed_text") &&
               !strcmp(yyjson_get_str(boundary_val), "first_external_boundary");
      }

      #endif
