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
        int keep_get_live;
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
      auto int block_uses_reg(yyjson_val* block, long long reg) {
        if (!(block && reg)) return 0;
        yyjson_val* insts = yyjson_obj_get(block, "instructions");
        if (!(insts && yyjson_is_arr(insts))) return 0;
        size_t n = yyjson_arr_size(insts);
        for (size_t i = 0; i < n; i++) {
          yyjson_val* ins = yyjson_arr_get(insts, i);
          if (inst_uses_reg(ins, reg)) return 1;
        }
        return 0;
      }
      auto int branch_targets_use_reg(
          yyjson_val* blocks,
          yyjson_val* insts,
          size_t branch_ii,
          long long reg) {
        yyjson_val* branch = NULL;
        long long then_bid = 0;
        long long else_bid = 0;
        if (!(blocks && insts && reg)) return 0;
        branch = yyjson_arr_get(insts, branch_ii);
        if (!(branch && read_str(branch, "op") && strcmp(read_str(branch, "op"), "branch") == 0)) return 0;
        then_bid = read_int(branch, "then");
        else_bid = read_int(branch, "else");
        if (then_bid && block_uses_reg(find_block_by_id(blocks, then_bid), reg)) return 1;
        if (else_bid && block_uses_reg(find_block_by_id(blocks, else_bid), reg)) return 1;
        return 0;
      }
      auto int reg_in_list(long long* regs, int n, long long reg) {
        reg = resolve_copy_source_reg(reg);
        if (!reg) return 0;
        for (int i = 0; i < n; i++) {
          if (regs[i] == reg) return 1;
        }
        return 0;
      }
      auto void add_reg_to_list(long long* regs, int* n, long long reg, int cap) {
        reg = resolve_copy_source_reg(reg);
        if (!(regs && n && reg) || *n >= cap) return;
        if (reg_in_list(regs, *n, reg)) return;
        regs[*n] = reg;
        *n = *n + 1;
      }
      auto int block_uses_reg_only_as_const_suffix_store(
          yyjson_val* block,
          long long reg,
          long long recv,
          long long idx) {
        long long aliases[16];
        int alias_n = 0;
        long long concat_dst = 0;
        int saw_concat = 0;
        int saw_store = 0;
        yyjson_val* insts = NULL;
        size_t n = 0;
        if (!(block && reg && recv && idx)) return 0;
        add_reg_to_list(aliases, &alias_n, reg, 16);
        insts = yyjson_obj_get(block, "instructions");
        if (!(insts && yyjson_is_arr(insts))) return 0;
        n = yyjson_arr_size(insts);
        for (size_t i = 0; i < n; i++) {
          yyjson_val* ins = yyjson_arr_get(insts, i);
          const char* op = ins ? read_str(ins, "op") : NULL;
          if (!op) continue;
          if (strcmp(op, "copy") == 0) {
            long long src = read_int(ins, "src");
            long long dst = read_int(ins, "dst");
            if (reg_in_list(aliases, alias_n, src)) {
              add_reg_to_list(aliases, &alias_n, dst, 16);
              continue;
            }
            if (inst_uses_reg(ins, reg)) return 0;
            continue;
          }
          if (strcmp(op, "binop") == 0) {
            const char* k = read_str(ins, "op_kind");
            long long lhs = read_int(ins, "lhs");
            long long rhs = read_int(ins, "rhs");
            int lhs_is_alias = reg_in_list(aliases, alias_n, lhs);
            int rhs_is_alias = reg_in_list(aliases, alias_n, rhs);
            if (!k) k = read_str(ins, "operation");
            if (!(k && (!strcmp(k, "Add") || !strcmp(k, "+")))) {
              if (lhs_is_alias || rhs_is_alias) return 0;
              continue;
            }
            if (lhs_is_alias && !rhs_is_alias && get_str_const(rhs) != NULL && !saw_concat) {
              concat_dst = read_int(ins, "dst");
              saw_concat = concat_dst != 0;
              continue;
            }
            if (lhs_is_alias || rhs_is_alias) return 0;
            continue;
          }
          if (strcmp(op, "mir_call") == 0) {
            yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
            yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
            yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
            const char* ctype = cal ? read_str(cal, "type") : NULL;
            const char* mname =
                cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
            long long call_recv = cal ? resolve_copy_source_reg(read_int(cal, "receiver")) : 0;
            long long arg_idx = 0;
            long long arg_value = 0;
            if (args && yyjson_is_arr(args) && yyjson_arr_size(args) >= 2) {
              arg_idx = resolve_copy_source_reg((long long)yyjson_get_sint(yyjson_arr_get(args, 0)));
              arg_value = resolve_copy_source_reg((long long)yyjson_get_sint(yyjson_arr_get(args, 1)));
            }
            if (ctype && !strcmp(ctype, "Method") && mname && !strcmp(mname, "set") &&
                call_recv == resolve_copy_source_reg(recv) &&
                arg_idx == resolve_copy_source_reg(idx) &&
                concat_dst && arg_value == resolve_copy_source_reg(concat_dst) &&
                !saw_store) {
              saw_store = 1;
              continue;
            }
            for (int ai = 0; ai < alias_n; ai++) {
              if (inst_uses_reg(ins, aliases[ai])) return 0;
            }
            if (concat_dst && inst_uses_reg(ins, concat_dst)) return 0;
            continue;
          }
          for (int ai = 0; ai < alias_n; ai++) {
            if (inst_uses_reg(ins, aliases[ai])) return 0;
          }
          if (concat_dst && inst_uses_reg(ins, concat_dst)) return 0;
        }
        return saw_concat && saw_store;
      }
      auto int branch_targets_use_reg_only_as_const_suffix_store(
          yyjson_val* blocks,
          yyjson_val* insts,
          size_t branch_ii,
          long long reg,
          long long recv,
          long long idx) {
        yyjson_val* branch = NULL;
        long long then_bid = 0;
        long long else_bid = 0;
        int matched = 0;
        if (!(blocks && insts && reg && recv && idx)) return 0;
        branch = yyjson_arr_get(insts, branch_ii);
        if (!(branch && read_str(branch, "op") && strcmp(read_str(branch, "op"), "branch") == 0)) return 0;
        then_bid = read_int(branch, "then");
        else_bid = read_int(branch, "else");
        if (then_bid) {
          yyjson_val* then_block = find_block_by_id(blocks, then_bid);
          if (block_uses_reg(then_block, reg)) {
            if (!block_uses_reg_only_as_const_suffix_store(then_block, reg, recv, idx)) return 0;
            matched++;
          }
        }
        if (else_bid) {
          yyjson_val* else_block = find_block_by_id(blocks, else_bid);
          if (block_uses_reg(else_block, reg)) {
            if (!block_uses_reg_only_as_const_suffix_store(else_block, reg, recv, idx)) return 0;
            matched++;
          }
        }
        return matched == 1;
      }
