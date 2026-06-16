      struct LocalI64MapEntryTableMetadata {
        int invalid;
        int entry_n;
        long long keys[32];
        long long values[32];
      };

      auto int local_i64_map_entry_table_has_direct_storage_plan(long long recv) {
        yyjson_val* plans = NULL;
        size_t pi = 0;
        if (!fn_metadata) return 0;
        plans = yyjson_obj_get(fn_metadata, "local_i64_map_direct_storage_plans");
        if (!(plans && yyjson_is_arr(plans))) return 0;
        for (pi = 0; pi < yyjson_arr_size(plans); pi++) {
          yyjson_val* plan = yyjson_arr_get(plans, pi);
          const char* representation = plan ? read_str(plan, "representation") : NULL;
          long long receiver_value = 0;
          if (!hako_llvmc_json_read_i64_field(
                  plan,
                  "receiver_value",
                  &receiver_value)) {
            return 0;
          }
          if (receiver_value == recv &&
              representation &&
              !strcmp(representation, "closed_world_i64_key_value_table")) {
            return 1;
          }
        }
        return 0;
      }

      auto int match_local_i64_map_entry_table_metadata(
          long long recv,
          struct LocalI64MapEntryTableMetadata* out) {
        yyjson_val* rows = NULL;
        size_t ri = 0;
        if (out) memset(out, 0, sizeof(*out));
        if (!local_i64_map_entry_table_has_direct_storage_plan(recv)) return 0;
        rows = yyjson_obj_get(fn_metadata, "local_i64_map_entry_value_tracking_plans");
        if (!(rows && yyjson_is_arr(rows))) return 0;
        for (ri = 0; ri < yyjson_arr_size(rows); ri++) {
          yyjson_val* row = yyjson_arr_get(rows, ri);
          long long receiver_value = 0;
          long long key_const = 0;
          long long value_const = 0;
          if (!hako_llvmc_json_read_i64_field(
                  row,
                  "receiver_value",
                  &receiver_value)) {
            if (out) out->invalid = 1;
            return 0;
          }
          if (receiver_value != recv) continue;
          if (!hako_llvmc_json_read_i64_field(
                  row,
                  "key_const_if_known",
                  &key_const) ||
              !hako_llvmc_json_read_i64_field(
                  row,
                  "value_const_if_known",
                  &value_const)) {
            if (out) out->invalid = 1;
            return 0;
          }
          if (out && out->entry_n <
                         (int)(sizeof(out->keys) / sizeof(out->keys[0]))) {
            out->keys[out->entry_n] = key_const;
            out->values[out->entry_n] = value_const;
            out->entry_n++;
          }
        }
        return out && out->entry_n > 0;
      }
