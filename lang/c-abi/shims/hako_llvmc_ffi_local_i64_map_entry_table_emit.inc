      auto void emit_local_i64_map_entry_table_get(
          FILE* f,
          long long dst,
          long long key_reg,
          const struct LocalI64MapEntryTableMetadata* table) {
        char prev[96];
        snprintf(prev, sizeof(prev), "0");
        for (int i = 0; table && i < table->entry_n; i++) {
          fprintf(
              f,
              "  %%local_i64_map_entry_table_hit_%lld_%d = icmp eq i64 %%r%lld, %lld\n",
              dst,
              i,
              key_reg,
              table->keys[i]);
          fprintf(
              f,
              "  %%local_i64_map_entry_table_value_%lld_%d = select i1 %%local_i64_map_entry_table_hit_%lld_%d, i64 %lld, i64 %s\n",
              dst,
              i,
              dst,
              i,
              table->values[i],
              prev);
          snprintf(
              prev,
              sizeof(prev),
              "%%local_i64_map_entry_table_value_%lld_%d",
              dst,
              i);
        }
        fprintf(f, "  %%r%lld = add i64 0, %s\n", dst, prev);
      }
