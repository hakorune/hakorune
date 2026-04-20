      struct DeferredArrayStringIndexofSelect {
        long long bid;
        size_t indexof_ii;
        long long recv;
        long long idx;
        long long needle_reg;
        int needle_const_utf8;
        long long needle_byte_len;
      } deferred_array_string_indexof_select[128];
      int deferred_array_string_indexof_select_n = 0;
      struct DeferredArrayStringGetSource {
        long long dst;
        long long recv;
        long long idx;
      } deferred_array_string_get_source[128];
      int deferred_array_string_get_source_n = 0;
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
          long long needle_reg,
          int needle_const_utf8,
          long long needle_byte_len) {
        struct DeferredArrayStringIndexofSelect* existing =
            find_deferred_array_string_indexof_select(bid, indexof_ii);
        if (existing) {
          existing->recv = recv;
          existing->idx = idx;
          existing->needle_reg = needle_reg;
          existing->needle_const_utf8 = needle_const_utf8;
          existing->needle_byte_len = needle_byte_len;
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
        slot->needle_const_utf8 = needle_const_utf8;
        slot->needle_byte_len = needle_byte_len;
      }
      auto void remember_array_string_get_source(long long dst, long long recv, long long idx) {
        dst = resolve_copy_source_reg(dst);
        recv = resolve_copy_source_reg(recv);
        idx = resolve_copy_source_reg(idx);
        if (!(dst && recv && idx)) return;
        for (int i = 0; i < deferred_array_string_get_source_n; i++) {
          if (deferred_array_string_get_source[i].dst != dst) continue;
          deferred_array_string_get_source[i].recv = recv;
          deferred_array_string_get_source[i].idx = idx;
          return;
        }
        if (deferred_array_string_get_source_n >=
            (int)(sizeof(deferred_array_string_get_source) / sizeof(deferred_array_string_get_source[0]))) return;
        deferred_array_string_get_source[deferred_array_string_get_source_n].dst = dst;
        deferred_array_string_get_source[deferred_array_string_get_source_n].recv = recv;
        deferred_array_string_get_source[deferred_array_string_get_source_n].idx = idx;
        deferred_array_string_get_source_n++;
      }
      auto int find_array_string_get_source(
          long long reg,
          long long* recv_out,
          long long* idx_out) {
        reg = resolve_copy_source_reg(reg);
        if (!reg) return 0;
        for (int i = 0; i < deferred_array_string_get_source_n; i++) {
          if (deferred_array_string_get_source[i].dst != reg) continue;
          if (recv_out) *recv_out = deferred_array_string_get_source[i].recv;
          if (idx_out) *idx_out = deferred_array_string_get_source[i].idx;
          return 1;
        }
        return 0;
      }
