      auto int find_single_use_inst(
          size_t start_ii,
          size_t ilen,
          yyjson_val* insts,
          long long reg,
          size_t* use_ii_out) {
        int use_count = 0;
        size_t use_ii = 0;
        if (!reg) return 0;
        for (size_t jj = start_ii; jj < ilen; jj++) {
          yyjson_val* later = yyjson_arr_get(insts, jj);
          if (!inst_uses_reg(later, reg)) continue;
          use_count++;
          use_ii = jj;
          if (use_count > 1) return 0;
        }
        if (use_count != 1) return 0;
        if (use_ii_out) *use_ii_out = use_ii;
        return 1;
      }

      auto long long resolve_copy_chain_source_reg(
          yyjson_val* insts,
          size_t search_end_ii,
          long long reg) {
        long long cursor = reg;
        int depth = 0;
        if (!cursor) return 0;
        while (depth < 8) {
          int advanced = 0;
          for (size_t jj = search_end_ii; jj-- > 0;) {
            yyjson_val* ins = yyjson_arr_get(insts, jj);
            const char* op = ins ? read_str(ins, "op") : NULL;
            if (read_int(ins, "dst") != cursor) continue;
            if (!(op && !strcmp(op, "copy"))) return cursor;
            cursor = read_int(ins, "src");
            advanced = 1;
            break;
          }
          if (!advanced) break;
          depth++;
        }
        return cursor;
      }

      auto int find_def_inst_index(
          size_t search_end_ii,
          yyjson_val* insts,
          long long reg,
          size_t* def_ii_out) {
        if (!reg) return 0;
        for (size_t jj = search_end_ii; jj-- > 0;) {
          yyjson_val* ins = yyjson_arr_get(insts, jj);
          if (read_int(ins, "dst") != reg) continue;
          if (def_ii_out) *def_ii_out = jj;
          return 1;
        }
        return 0;
      }

      auto int follow_single_use_copy_chain_to_use(
          size_t start_ii,
          size_t ilen,
          yyjson_val* insts,
          long long seed_reg,
          size_t* use_ii_out,
          long long* final_reg_out,
          size_t* copy_iis_out,
          int* copy_n_out) {
        long long cursor = seed_reg;
        int copy_n = 0;
        if (!cursor) return 0;
        while (1) {
          size_t use_ii = 0;
          yyjson_val* use_ins = NULL;
          const char* op = NULL;
          if (!find_single_use_inst(start_ii, ilen, insts, cursor, &use_ii)) return 0;
          use_ins = yyjson_arr_get(insts, use_ii);
          op = use_ins ? read_str(use_ins, "op") : NULL;
          if (op && !strcmp(op, "copy") && read_int(use_ins, "src") == cursor) {
            if (copy_iis_out && copy_n < 8) copy_iis_out[copy_n] = use_ii;
            copy_n++;
            cursor = read_int(use_ins, "dst");
            start_ii = use_ii + 1;
            continue;
          }
          if (use_ii_out) *use_ii_out = use_ii;
          if (final_reg_out) *final_reg_out = cursor;
          if (copy_n_out) *copy_n_out = copy_n;
          return 1;
        }
      }
