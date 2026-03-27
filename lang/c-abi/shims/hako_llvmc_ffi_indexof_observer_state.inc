      struct ArrayStringIndexofSelectWindowMatch {
        int matched;
        size_t get_copy_iis[8];
        int get_copy_n;
        size_t recv_copy_iis[8];
        int recv_copy_n;
        size_t indexof_copy_iis[8];
        int indexof_copy_n;
        size_t needle_const_ii;
        size_t indexof_ii;
        size_t zero_const_ii;
        size_t compare_ii;
        size_t then_const_ii;
        size_t else_const_ii;
        size_t select_ii;
        long long needle_reg;
        long long indexof_dst;
      };
      struct ArrayStringIndexofBranchWindowMatch {
        int matched;
        size_t get_copy_iis[8];
        int get_copy_n;
        size_t recv_copy_iis[8];
        int recv_copy_n;
        size_t indexof_copy_iis[8];
        int indexof_copy_n;
        size_t needle_const_ii;
        size_t indexof_ii;
        size_t zero_const_ii;
        size_t compare_ii;
        size_t branch_ii;
        long long needle_reg;
        long long indexof_dst;
      };
      struct ArrayStringIndexofCrossBlockSelectWindowMatch {
        int matched;
        size_t get_copy_iis[8];
        int get_copy_n;
        size_t recv_copy_iis[8];
        int recv_copy_n;
        size_t indexof_copy_iis[8];
        int indexof_copy_n;
        size_t needle_const_ii;
        size_t indexof_ii;
        size_t jump_ii;
        long long needle_reg;
        long long indexof_dst;
        long long succ_bid;
      };
      struct ArrayStringIndexofInterleavedBranchWindowMatch {
        int matched;
        size_t get_copy_iis[8];
        int get_copy_n;
        size_t recv_copy_iis[8];
        int recv_copy_n;
        size_t indexof_copy_iis[8];
        int indexof_copy_n;
        size_t needle_const_ii;
        size_t indexof_ii;
        size_t guard_compare_ii;
        size_t guard_branch_ii;
        long long needle_reg;
        long long indexof_dst;
        long long consumer_bid;
      };
      struct ArrayStringIndexofInterleavedSelectWindowMatch {
        int matched;
        size_t get_copy_iis[8];
        int get_copy_n;
        size_t recv_copy_iis[8];
        int recv_copy_n;
        size_t indexof_copy_iis[8];
        int indexof_copy_n;
        size_t needle_const_ii;
        size_t indexof_ii;
        size_t guard_compare_ii;
        size_t guard_branch_ii;
        long long needle_reg;
        long long indexof_dst;
        long long bridge_bid;
        long long consumer_bid;
      };
      struct DeferredArrayStringIndexofSelect {
        long long bid;
        size_t indexof_ii;
        long long recv;
        long long idx;
        long long needle_reg;
      } deferred_array_string_indexof_select[128];
      int deferred_array_string_indexof_select_n = 0;
      auto struct DeferredArrayStringIndexofSelect* find_deferred_array_string_indexof_select(long long bid, size_t indexof_ii) {
        for (int i = 0; i < deferred_array_string_indexof_select_n; i++) {
          if (deferred_array_string_indexof_select[i].bid == bid &&
              deferred_array_string_indexof_select[i].indexof_ii == indexof_ii) {
            return &deferred_array_string_indexof_select[i];
          }
        }
        return NULL;
      }
      auto void remember_deferred_array_string_indexof_select(
          long long bid,
          size_t indexof_ii,
          long long recv,
          long long idx,
          long long needle_reg) {
        struct DeferredArrayStringIndexofSelect* existing =
            find_deferred_array_string_indexof_select(bid, indexof_ii);
        if (existing) {
          existing->recv = recv;
          existing->idx = idx;
          existing->needle_reg = needle_reg;
          return;
        }
        if (deferred_array_string_indexof_select_n >= (int)(sizeof(deferred_array_string_indexof_select) / sizeof(deferred_array_string_indexof_select[0]))) return;
        struct DeferredArrayStringIndexofSelect* slot =
            &deferred_array_string_indexof_select[deferred_array_string_indexof_select_n++];
        slot->bid = bid;
        slot->indexof_ii = indexof_ii;
        slot->recv = recv;
        slot->idx = idx;
        slot->needle_reg = needle_reg;
      }
      auto const char* read_string_const_from_inst(yyjson_val* ins) {
        if (!ins) return NULL;
        const char* op = read_str(ins, "op");
        if (!(op && strcmp(op, "const") == 0)) return NULL;
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* tyobj = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        const char* box_type = tyobj ? read_str(tyobj, "box_type") : NULL;
        if (!(box_type && strcmp(box_type, "StringBox") == 0)) return NULL;
        return vobj ? read_str(vobj, "value") : NULL;
      }
      auto int read_i64_const_from_inst(yyjson_val* ins, long long* out) {
        if (out) *out = 0;
        if (!ins) return 0;
        const char* op = read_str(ins, "op");
        if (!(op && strcmp(op, "const") == 0)) return 0;
        yyjson_val* vobj = yyjson_obj_get(ins, "value");
        yyjson_val* tyobj = vobj ? yyjson_obj_get(vobj, "type") : NULL;
        const char* box_type = tyobj ? read_str(tyobj, "box_type") : NULL;
        if (box_type && strcmp(box_type, "StringBox") == 0) return 0;
        yyjson_val* value = vobj ? yyjson_obj_get(vobj, "value") : NULL;
        if (!value) return 0;
        if (out) *out = (long long)yyjson_get_sint(value);
        return 1;
      }
      auto yyjson_val* find_block_by_id(yyjson_val* blocks, long long bid) {
        if (!(blocks && yyjson_is_arr(blocks))) return NULL;
        size_t n = yyjson_arr_size(blocks);
        for (size_t i = 0; i < n; i++) {
          yyjson_val* block = yyjson_arr_get(blocks, i);
          if (block && read_int(block, "id") == bid) return block;
        }
        return NULL;
      }
