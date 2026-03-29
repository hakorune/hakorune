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
