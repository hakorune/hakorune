      struct ArrayTextEditLenHalfRouteMetadata {
        int matched;
        size_t skip_iis[32];
        int skip_n;
        long long block;
        long long get_instruction_index;
        long long array_value;
        long long index_value;
        long long source_value;
        long long middle_reg;
        long long middle_len;
        long long result_value;
      };

      auto int array_text_edit_metadata_read_i64(
          yyjson_val* obj,
          const char* key,
          long long* out) {
        yyjson_val* val = obj ? yyjson_obj_get(obj, key) : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto int array_text_edit_metadata_string_array_eq2(
          yyjson_val* vals,
          const char* first,
          const char* second) {
        yyjson_val* v0 = NULL;
        yyjson_val* v1 = NULL;
        const char* s0 = NULL;
        const char* s1 = NULL;
        if (!(vals && yyjson_is_arr(vals) && yyjson_arr_size(vals) == 2)) return 0;
        v0 = yyjson_arr_get(vals, 0);
        v1 = yyjson_arr_get(vals, 1);
        if (!(v0 && v1 && yyjson_is_str(v0) && yyjson_is_str(v1))) return 0;
        s0 = yyjson_get_str(v0);
        s1 = yyjson_get_str(v1);
        return s0 && s1 && !strcmp(s0, first) && !strcmp(s1, second);
      }

      auto int array_text_edit_metadata_string_array_eq1(
          yyjson_val* vals,
          const char* only) {
        yyjson_val* v0 = NULL;
        const char* s0 = NULL;
        if (!(vals && yyjson_is_arr(vals) && yyjson_arr_size(vals) == 1)) return 0;
        v0 = yyjson_arr_get(vals, 0);
        if (!(v0 && yyjson_is_str(v0))) return 0;
        s0 = yyjson_get_str(v0);
        return s0 && !strcmp(s0, only);
      }

      auto int array_text_edit_lenhalf_route_valid(yyjson_val* route) {
        const char* edit_kind = route ? read_str(route, "edit_kind") : NULL;
        const char* split_policy = route ? read_str(route, "split_policy") : NULL;
        const char* proof = route ? read_str(route, "proof") : NULL;
        const char* carrier = route ? read_str(route, "carrier") : NULL;
        const char* publication_boundary =
            route ? read_str(route, "publication_boundary") : NULL;
        const char* materialization_policy =
            route ? read_str(route, "materialization_policy") : NULL;
        yyjson_val* effects = route ? yyjson_obj_get(route, "effects") : NULL;
        yyjson_val* capabilities =
            route ? yyjson_obj_get(route, "consumer_capabilities") : NULL;
        long long middle_len = 0;
        long long get_instruction_index = 0;
        long long set_instruction_index = 0;
        return route &&
               array_text_edit_metadata_read_i64(
                   route,
                   "get_instruction_index",
                   &get_instruction_index) &&
               array_text_edit_metadata_read_i64(
                   route,
                   "set_instruction_index",
                   &set_instruction_index) &&
               set_instruction_index > get_instruction_index &&
               array_text_edit_metadata_read_i64(route, "middle_byte_len", &middle_len) &&
               middle_len >= 0 &&
               edit_kind && !strcmp(edit_kind, "insert_mid_const") &&
               split_policy && !strcmp(split_policy, "source_len_div_const(2)") &&
               proof && !strcmp(proof, "array_get_lenhalf_insert_mid_same_slot") &&
               carrier && !strcmp(carrier, "array_lane_text_cell") &&
               publication_boundary && !strcmp(publication_boundary, "none") &&
               materialization_policy &&
                   !strcmp(materialization_policy, "text_resident_or_stringlike_slot") &&
               array_text_edit_metadata_string_array_eq2(effects, "load.ref", "store.cell") &&
               array_text_edit_metadata_string_array_eq1(capabilities, "sink_store");
      }

      auto void array_text_edit_lenhalf_route_fill(
          yyjson_val* route,
          struct ArrayTextEditLenHalfRouteMetadata* out) {
        yyjson_val* skip_vals = NULL;
        size_t si = 0;
        long long tmp = 0;
        if (!route || !out) return;
        memset(out, 0, sizeof(*out));
        out->matched = 1;
        if (array_text_edit_metadata_read_i64(route, "block", &tmp)) out->block = tmp;
        if (array_text_edit_metadata_read_i64(route, "get_instruction_index", &tmp)) {
          out->get_instruction_index = tmp;
        }
        if (array_text_edit_metadata_read_i64(route, "array_value", &tmp)) out->array_value = tmp;
        if (array_text_edit_metadata_read_i64(route, "index_value", &tmp)) out->index_value = tmp;
        if (array_text_edit_metadata_read_i64(route, "source_value", &tmp)) out->source_value = tmp;
        if (array_text_edit_metadata_read_i64(route, "middle_value", &tmp)) out->middle_reg = tmp;
        if (array_text_edit_metadata_read_i64(route, "middle_byte_len", &tmp)) {
          out->middle_len = tmp;
        }
        if (array_text_edit_metadata_read_i64(route, "result_value", &tmp)) out->result_value = tmp;
        skip_vals = yyjson_obj_get(route, "skip_instruction_indices");
        if (skip_vals && yyjson_is_arr(skip_vals)) {
          for (si = 0; si < yyjson_arr_size(skip_vals); si++) {
            yyjson_val* item = yyjson_arr_get(skip_vals, si);
            if (!(item && yyjson_is_num(item))) continue;
            if (out->skip_n >= (int)(sizeof(out->skip_iis) / sizeof(out->skip_iis[0]))) break;
            out->skip_iis[out->skip_n++] = (size_t)yyjson_get_sint(item);
          }
        }
      }

      auto int match_array_text_edit_lenhalf_route_metadata(
          long long bid,
          size_t ii,
          struct ArrayTextEditLenHalfRouteMetadata* out) {
        yyjson_val* routes = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!fn_metadata) return 0;
        routes = yyjson_obj_get(fn_metadata, "array_text_edit_routes");
        if (!(routes && yyjson_is_arr(routes))) return 0;
        for (ri = 0; ri < yyjson_arr_size(routes); ri++) {
          yyjson_val* route = yyjson_arr_get(routes, ri);
          long long block = 0;
          long long get_instruction_index = 0;
          if (!array_text_edit_lenhalf_route_valid(route)) continue;
          if (!array_text_edit_metadata_read_i64(route, "block", &block)) continue;
          if (!array_text_edit_metadata_read_i64(
                  route,
                  "get_instruction_index",
                  &get_instruction_index)) {
            continue;
          }
          if (block != bid || get_instruction_index != (long long)ii) continue;
          array_text_edit_lenhalf_route_fill(route, out);
          return out && out->matched;
        }
        return 0;
      }
