      auto long long get_copy_alias_src(long long r){ for(size_t i=0;i<copy_alias_n;i++){ if(copy_alias[i].reg==r) return copy_alias[i].src; } return 0; }
      auto void set_copy_alias(long long r,long long src){ if(!r || !src) return; for(size_t i=0;i<copy_alias_n;i++){ if(copy_alias[i].reg==r){ copy_alias[i].src=src; return; } } if(copy_alias_n<2048){ copy_alias[copy_alias_n].reg=r; copy_alias[copy_alias_n].src=src; copy_alias_n++; } }
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
      auto void set_scan_origin(long long r,int k){ for(size_t i=0;i<scan_origin_n;i++){ if(scan_origin[i].reg==r){ scan_origin[i].kind=k; return;} } if(scan_origin_n<2048){ scan_origin[scan_origin_n].reg=r; scan_origin[scan_origin_n].kind=k; scan_origin_n++; } }
      auto void promote_scan_origin_kind(int from, int to){ for(size_t i=0;i<scan_origin_n;i++){ if(scan_origin[i].kind==from) scan_origin[i].kind=to; } }
      auto int get_origin(long long r){
        for(size_t i=0;i<origin_n;i++){ if(origin[i].reg==r) return origin[i].kind; }
        long long base = resolve_copy_source_reg(r);
        if(base!=r){ for(size_t i=0;i<origin_n;i++){ if(origin[i].reg==base) return origin[i].kind; } }
        return ORG_NONE;
      }
      auto void set_origin(long long r,int k){ for(size_t i=0;i<origin_n;i++){ if(origin[i].reg==r){ origin[i].kind=k; return;} } if(origin_n<2048){ origin[origin_n].reg=r; origin[origin_n].kind=k; origin_n++; } }
      auto void promote_origin_kind(int from, int to){ for(size_t i=0;i<origin_n;i++){ if(origin[i].kind==from) origin[i].kind=to; } }
      auto int get_type(long long r) {
        for (size_t i=0;i<types_n;i++){ if (types[i].reg==r) return types[i].ty; }
        long long base = resolve_copy_source_reg(r);
        if (base != r) {
          for (size_t i=0;i<types_n;i++){ if (types[i].reg==base) return types[i].ty; }
        }
        return T_NONE;
      }
      auto void set_type(long long r, int t) { for (size_t i=0;i<types_n;i++){ if (types[i].reg==r){ types[i].ty=t; return; } } if (types_n<2048){ types[types_n].reg=r; types[types_n].ty=t; types_n++; } }
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
      auto void put_const(long long r, long long v){ if (consts_n<1024){ consts[consts_n].reg=r; consts[consts_n].val=v; consts_n++; } }
      auto const char* get_str_const(long long r){
        for (size_t i=0;i<str_consts_n;i++){ if (str_consts[i].reg==r) return str_consts[i].val; }
        long long base = resolve_copy_source_reg(r);
        if (base != r) {
          for (size_t i=0;i<str_consts_n;i++){ if (str_consts[i].reg==base) return str_consts[i].val; }
        }
        return NULL;
      }
      auto void put_str_const(long long r, const char* v){ if (str_consts_n<512){ str_consts[str_consts_n].reg=r; str_consts[str_consts_n].val=v; str_consts_n++; } }
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
        if (known_string_lens_n<2048){
          known_string_lens[known_string_lens_n].reg=r;
          known_string_lens[known_string_lens_n].len=len;
          known_string_lens_n++;
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
        if (str_const_box_skip_n < 512) {
          str_const_box_skip[str_const_box_skip_n].reg = r;
          str_const_box_skip_n++;
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
          yyjson_val* blocks,
          size_t blen,
          long long reg,
          size_t* use_bi_out,
          size_t* use_ii_out,
          long long* effective_reg_out,
          int* dead_out) {
        long long cur = resolve_copy_source_reg(reg);
        int depth = 0;
        if (dead_out) *dead_out = 0;
        if (!cur) return 0;
        while (depth < 16) {
          size_t use_count = 0;
          size_t use_bi = 0;
          size_t use_ii = 0;
          yyjson_val* use_ins = NULL;
          const char* op = NULL;
          for (size_t bi = 0; bi < blen; bi++) {
            yyjson_val* b = yyjson_arr_get(blocks, bi);
            yyjson_val* insts = b ? yyjson_obj_get(b, "instructions") : NULL;
            if (!insts || !yyjson_is_arr(insts)) continue;
            size_t ilen = yyjson_arr_size(insts);
            for (size_t ii = 0; ii < ilen; ii++) {
              yyjson_val* ins = yyjson_arr_get(insts, ii);
              if (!inst_uses_reg(ins, cur)) continue;
              use_count++;
              use_bi = bi;
              use_ii = ii;
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
            if (use_bi_out) *use_bi_out = use_bi;
            if (use_ii_out) *use_ii_out = use_ii;
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
