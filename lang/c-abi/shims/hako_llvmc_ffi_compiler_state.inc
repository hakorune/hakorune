      auto long long get_copy_alias_src(long long r){ for(size_t i=0;i<copy_alias_n;i++){ if(copy_alias[i].reg==r) return copy_alias[i].src; } return 0; }
      auto void set_copy_alias(long long r,long long src){ if(!r || !src) return; for(size_t i=0;i<copy_alias_n;i++){ if(copy_alias[i].reg==r){ copy_alias[i].src=src; return; } } if(copy_alias_n<ARR_LEN(copy_alias)){ copy_alias[copy_alias_n].reg=r; copy_alias[copy_alias_n].src=src; copy_alias_n++; } else { note_generic_capacity_overflow("generic_copy_alias_inventory_full"); } }
      auto long long resolve_copy_source_reg(long long r){
        long long cur = r;
        for(int depth=0; depth<64 && cur; depth++){
          long long next = get_copy_alias_src(cur);
          if(!next || next==cur) break;
          cur = next;
        }
        return cur ? cur : r;
      }
      auto int get_scan_origin(long long r){
        for(size_t i=0;i<scan_origin_n;i++){ if(scan_origin[i].reg==r) return scan_origin[i].kind; }
        long long base = resolve_copy_source_reg(r);
        if(base!=r){ for(size_t i=0;i<scan_origin_n;i++){ if(scan_origin[i].reg==base) return scan_origin[i].kind; } }
        return ORG_NONE;
      }
      auto void set_scan_origin(long long r,int k){ for(size_t i=0;i<scan_origin_n;i++){ if(scan_origin[i].reg==r){ scan_origin[i].kind=k; return;} } if(scan_origin_n<ARR_LEN(scan_origin)){ scan_origin[scan_origin_n].reg=r; scan_origin[scan_origin_n].kind=k; scan_origin_n++; } else { note_generic_capacity_overflow("generic_scan_origin_inventory_full"); } }
      auto void promote_scan_origin_kind(int from, int to){ for(size_t i=0;i<scan_origin_n;i++){ if(scan_origin[i].kind==from) scan_origin[i].kind=to; } }
      auto int get_origin(long long r){
        for(size_t i=0;i<origin_n;i++){ if(origin[i].reg==r) return origin[i].kind; }
        long long base = resolve_copy_source_reg(r);
        if(base!=r){ for(size_t i=0;i<origin_n;i++){ if(origin[i].reg==base) return origin[i].kind; } }
        return ORG_NONE;
      }
      auto void set_origin(long long r,int k){ for(size_t i=0;i<origin_n;i++){ if(origin[i].reg==r){ origin[i].kind=k; return;} } if(origin_n<ARR_LEN(origin)){ origin[origin_n].reg=r; origin[origin_n].kind=k; origin_n++; } else { note_generic_capacity_overflow("generic_origin_inventory_full"); } }
      auto void promote_origin_kind(int from, int to){ for(size_t i=0;i<origin_n;i++){ if(origin[i].kind==from) origin[i].kind=to; } }
      auto int get_type(long long r) {
        for (size_t i=0;i<types_n;i++){ if (types[i].reg==r) return types[i].ty; }
        long long base = resolve_copy_source_reg(r);
        if (base != r) {
          for (size_t i=0;i<types_n;i++){ if (types[i].reg==base) return types[i].ty; }
        }
        return T_NONE;
      }
      auto void set_type(long long r, int t) { for (size_t i=0;i<types_n;i++){ if (types[i].reg==r){ types[i].ty=t; return; } } if (types_n<ARR_LEN(types)){ types[types_n].reg=r; types[types_n].ty=t; types_n++; } else { note_generic_capacity_overflow("generic_type_inventory_full"); } }
      auto int has_const(long long r, long long* out){
        for (size_t i=0;i<consts_n;i++){ if (consts[i].reg==r){ if(out) *out=consts[i].val; return 1; } }
        long long base = resolve_copy_source_reg(r);
        if (base != r) {
          for (size_t i=0;i<consts_n;i++){ if (consts[i].reg==base){ if(out) *out=consts[i].val; return 1; } }
        }
        return 0;
      }
      auto int has_bool_const(long long r, int* out){
        long long cv = 0;
        if (!has_const(r, &cv)) return 0;
        if (!(cv == 0 || cv == 1)) return 0;
        if (out) *out = cv ? 1 : 0;
        return 1;
      }
      auto int is_bool_like_phi_input(long long r){
        int bool_const = 0;
        if (!r) return 0;
        return get_type(r) == T_I1 || has_bool_const(r, &bool_const);
      }
      auto void put_const(long long r, long long v){ if (consts_n<ARR_LEN(consts)){ consts[consts_n].reg=r; consts[consts_n].val=v; consts_n++; } else { note_generic_capacity_overflow("generic_i64_const_inventory_full"); } }
      auto const char* get_str_const(long long r){
        for (size_t i=0;i<str_consts_n;i++){ if (str_consts[i].reg==r) return str_consts[i].val; }
        long long base = resolve_copy_source_reg(r);
        if (base != r) {
          for (size_t i=0;i<str_consts_n;i++){ if (str_consts[i].reg==base) return str_consts[i].val; }
        }
        return NULL;
      }
      auto int put_str_const(long long r, const char* v){
        if (str_consts_n<ARR_LEN(str_consts)){
          str_consts[str_consts_n].reg=r;
          str_consts[str_consts_n].val=v;
          str_consts_n++;
          return 1;
        }
        note_generic_capacity_overflow("generic_string_const_inventory_full");
        return 0;
      }
      auto int get_known_string_len(long long r, long long* out){
        for (size_t i=0;i<known_string_lens_n;i++){ if (known_string_lens[i].reg==r){ if(out) *out=known_string_lens[i].len; return 1; } }
        long long base = resolve_copy_source_reg(r);
        if (base != r) {
          for (size_t i=0;i<known_string_lens_n;i++){ if (known_string_lens[i].reg==base){ if(out) *out=known_string_lens[i].len; return 1; } }
        }
        return 0;
      }
      auto void put_known_string_len(long long r, long long len){
        if (!r || len < 0) return;
        for (size_t i=0;i<known_string_lens_n;i++){
          if (known_string_lens[i].reg==r){
            known_string_lens[i].len = len;
            return;
          }
        }
        if (known_string_lens_n<ARR_LEN(known_string_lens)){
          known_string_lens[known_string_lens_n].reg=r;
          known_string_lens[known_string_lens_n].len=len;
          known_string_lens_n++;
        } else {
          note_generic_capacity_overflow("generic_string_len_inventory_full");
        }
      }
      auto int known_string_window_len(long long start_reg, long long end_reg, long long* out){
        long long start = 0;
        long long end = 0;
        long long start_base = resolve_copy_source_reg(start_reg);
        long long end_base = resolve_copy_source_reg(end_reg);
        if (!(has_const(start_base, &start) && has_const(end_base, &end))) return 0;
        if (end < start) return 0;
        if (out) *out = end - start;
        return 1;
      }
      auto int should_skip_hoisted_string_const_box(long long r){
        for (size_t i=0;i<str_const_box_skip_n;i++){ if (str_const_box_skip[i].reg==r) return 1; }
        return 0;
      }
      auto void mark_hoisted_string_const_box_skip(long long r){
        if (!r || should_skip_hoisted_string_const_box(r)) return;
        if (str_const_box_skip_n < ARR_LEN(str_const_box_skip)) {
          str_const_box_skip[str_const_box_skip_n].reg = r;
          str_const_box_skip_n++;
        } else {
          note_generic_capacity_overflow("generic_string_const_skip_inventory_full");
        }
      }
      auto int string_const_box_was_emitted(long long r){
        long long base = resolve_copy_source_reg(r);
        for (size_t i=0;i<str_const_box_emitted_n;i++){
          if (str_const_box_emitted[i].reg==r || str_const_box_emitted[i].reg==base) return 1;
        }
        return 0;
      }
      auto void mark_string_const_box_emitted(long long r){
        long long base = resolve_copy_source_reg(r);
        if (!base || string_const_box_was_emitted(base)) return;
        if (str_const_box_emitted_n < ARR_LEN(str_const_box_emitted)) {
          str_const_box_emitted[str_const_box_emitted_n].reg = base;
          str_const_box_emitted_n++;
        } else {
          note_generic_capacity_overflow("generic_string_const_emitted_inventory_full");
        }
      }
      auto int is_plain_i64_value(long long r){
        if (!r) return 0;
        if (get_type(r) != T_I64) return 0;
        if (get_origin(r) != ORG_NONE) return 0;
        if (get_scan_origin(r) != ORG_NONE) return 0;
        return get_str_const(r) == NULL;
      }
      auto int inst_uses_reg(yyjson_val* ins, long long reg) {
        if (!ins || !reg) return 0;
        const char* op = read_str(ins, "op");
        if (!op) return 0;
        if (strcmp(op, "copy") == 0) return read_int(ins, "src") == reg;
        if (strcmp(op, "binop") == 0) {
          return read_int(ins, "lhs") == reg || read_int(ins, "rhs") == reg;
        }
        if (strcmp(op, "compare") == 0) {
          return read_int(ins, "lhs") == reg || read_int(ins, "left") == reg ||
                 read_int(ins, "rhs") == reg || read_int(ins, "right") == reg;
        }
        if (strcmp(op, "select") == 0) {
          return read_int(ins, "cond") == reg ||
                 read_int(ins, "then_val") == reg ||
                 read_int(ins, "else_val") == reg;
        }
        if (strcmp(op, "phi") == 0) {
          yyjson_val* vals = yyjson_obj_get(ins, "values");
          if (!vals) vals = yyjson_obj_get(ins, "incoming");
          if (vals && yyjson_is_arr(vals)) {
            size_t n = yyjson_arr_size(vals);
            for (size_t i = 0; i < n; i++) {
              yyjson_val* ent = yyjson_arr_get(vals, i);
              if (ent && yyjson_is_arr(ent) && yyjson_arr_size(ent) >= 1) {
                yyjson_val* ent0 = yyjson_arr_get(ent, 0);
                if (ent0 && (long long)yyjson_get_sint(ent0) == reg) return 1;
              } else if (read_int(ent, "value") == reg) {
                return 1;
              }
            }
          }
          return 0;
        }
        if (strcmp(op, "branch") == 0) return read_int(ins, "cond") == reg;
        if (strcmp(op, "ret") == 0) return read_int(ins, "value") == reg;
        if (strcmp(op, "mir_call") == 0) {
          yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
          yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
          if (cal && read_int(cal, "receiver") == reg) return 1;
          yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
          if (args && yyjson_is_arr(args)) {
            size_t n = yyjson_arr_size(args);
            for (size_t i = 0; i < n; i++) {
              yyjson_val* a = yyjson_arr_get(args, i);
              if (a && (long long)yyjson_get_sint(a) == reg) return 1;
            }
          }
        }
        return 0;
      }
      auto int find_single_effective_use_across_blocks(
          long long reg,
          size_t* use_ii_out,
          yyjson_val** use_insts_out,
          size_t* use_ilen_out,
          long long* effective_reg_out,
          int* dead_out) {
        long long cur = resolve_copy_source_reg(reg);
        int depth = 0;
        if (dead_out) *dead_out = 0;
        if (use_insts_out) *use_insts_out = NULL;
        if (use_ilen_out) *use_ilen_out = 0;
        if (!cur) return 0;
        while (depth < 16) {
          size_t use_count = 0;
          size_t use_ii = 0;
          yyjson_val* use_insts = NULL;
          size_t use_ilen = 0;
          yyjson_val* use_ins = NULL;
          const char* op = NULL;
          for (size_t bi = 0; bi < generic_pure_block_count; bi++) {
            struct GenericPureBlockView block;
            if (!read_generic_pure_block_view(bi, &block)) continue;
            yyjson_val* insts = block.instructions;
            size_t ilen = block.instruction_count;
            for (size_t ii = 0; ii < ilen; ii++) {
              yyjson_val* ins = yyjson_arr_get(insts, ii);
              if (!inst_uses_reg(ins, cur)) continue;
              use_count++;
              use_ii = ii;
              use_insts = insts;
              use_ilen = ilen;
              use_ins = ins;
              if (use_count > 1) return 0;
            }
          }
          if (use_count == 0) {
            if (dead_out) *dead_out = 1;
            return 0;
          }
          op = use_ins ? read_str(use_ins, "op") : NULL;
          if (!(op && strcmp(op, "copy") == 0)) {
            if (use_ii_out) *use_ii_out = use_ii;
            if (use_insts_out) *use_insts_out = use_insts;
            if (use_ilen_out) *use_ilen_out = use_ilen;
            if (effective_reg_out) *effective_reg_out = cur;
            return 1;
          }
          cur = read_int(use_ins, "dst");
          if (!cur) {
            if (dead_out) *dead_out = 1;
            return 0;
          }
          depth++;
        }
        return 0;
      }
