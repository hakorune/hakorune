static int compile_json_compat_pure(const char* json_in, const char* obj_out, char** err_out) {
  // Phase 21.2: validate v1 JSON, try generic pure lowering (CFG/phi),
  // then fall back to a few pattern lowers, and finally to AOT helper.
  char* verr = NULL;
  if (hako_json_v1_validate_file(json_in, &verr) != 0) {
    return set_err_owned(err_out, verr ? verr : "invalid v1 json");
  }

  do {
    yyjson_read_err rerr_t; yyjson_doc* d = yyjson_read_file(json_in, 0, NULL, &rerr_t);
    if (!d) break;
    yyjson_val* root = yyjson_doc_get_root(d);
    yyjson_val* fns = yyjson_obj_get(root, "functions");
    yyjson_val* fn0 = hako_llvmc_find_entry_function(fns);
    hako_llvmc_log_selected_entry_runes(fn0);
    yyjson_doc_free(d);
  } while (0);

  if (hako_llvmc_match_substring_concat_loop_ascii_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (!hako_llvmc_skip_indexof_line_seed_enabled() &&
      hako_llvmc_match_indexof_line_ascii_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_runtime_data_length_hot_loop_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_string_length_hot_loop_seed(json_in, obj_out) == 0) {
    return 0;
  }

    // --- Generic CFG/PHI lowering (minimal i64 subset) ---
    // Supported ops: const/compare/branch/jump/ret/phi, mir_call (Array/Map minimal, Global print)
    do {
      yyjson_read_err rerr_g; yyjson_doc* d = yyjson_read_file(json_in, 0, NULL, &rerr_g);
      if (!d) break;
      yyjson_val* root = yyjson_doc_get_root(d);
      yyjson_val* fns = yyjson_obj_get(root, "functions");
      yyjson_val* fn0 = hako_llvmc_find_entry_function(fns);
      hako_llvmc_entry_rune_selection selection;
      yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
      if (!(blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) >= 1)) { yyjson_doc_free(d); break; }
      hako_llvmc_select_entry_runes(fn0, &selection);
      enum { T_NONE=0, T_I64=1, T_I1=2 };
      struct { long long reg; long long val; } consts[1024]; size_t consts_n = 0;
      struct { long long reg; const char* val; } str_consts[512]; size_t str_consts_n = 0;
      struct { long long reg; int ty; } types[2048]; size_t types_n = 0;
      struct { long long reg; int kind; } scan_origin[2048]; size_t scan_origin_n = 0;
      // Track simple origin kinds for selective unbox at ret
      enum { ORG_NONE=0, ORG_MAP_GET=1, ORG_MAP_BIRTH=2, ORG_ARRAY_BIRTH=3, ORG_STRING=4, ORG_ARRAY_STRING_BIRTH=5 };
      struct { long long reg; int kind; } origin[2048]; size_t origin_n = 0;
      auto int get_scan_origin(long long r){ for(size_t i=0;i<scan_origin_n;i++){ if(scan_origin[i].reg==r) return scan_origin[i].kind; } return ORG_NONE; }
      auto void set_scan_origin(long long r,int k){ for(size_t i=0;i<scan_origin_n;i++){ if(scan_origin[i].reg==r){ scan_origin[i].kind=k; return;} } if(scan_origin_n<2048){ scan_origin[scan_origin_n].reg=r; scan_origin[scan_origin_n].kind=k; scan_origin_n++; } }
      auto void promote_scan_origin_kind(int from, int to){ for(size_t i=0;i<scan_origin_n;i++){ if(scan_origin[i].kind==from) scan_origin[i].kind=to; } }
      auto int get_origin(long long r){ for(size_t i=0;i<origin_n;i++){ if(origin[i].reg==r) return origin[i].kind; } return ORG_NONE; }
      auto void set_origin(long long r,int k){ for(size_t i=0;i<origin_n;i++){ if(origin[i].reg==r){ origin[i].kind=k; return;} } if(origin_n<2048){ origin[origin_n].reg=r; origin[origin_n].kind=k; origin_n++; } }
      auto void promote_origin_kind(int from, int to){ for(size_t i=0;i<origin_n;i++){ if(origin[i].kind==from) origin[i].kind=to; } }
      struct Incoming { long long pred; long long val_reg; };
      struct PhiRec { long long dst; struct Incoming in[16]; int in_n; };
      struct BlockPhi { long long bid; struct PhiRec recs[16]; int rec_n; } phis[512]; int phi_n = 0;

      // small helpers
      #define ARR_LEN(a) ((int)(sizeof(a)/sizeof((a)[0])))
      auto int get_type(long long r) { for (size_t i=0;i<types_n;i++){ if (types[i].reg==r) return types[i].ty; } return T_NONE; }
      auto void set_type(long long r, int t) { for (size_t i=0;i<types_n;i++){ if (types[i].reg==r){ types[i].ty=t; return; } } if (types_n<2048){ types[types_n].reg=r; types[types_n].ty=t; types_n++; } }
      auto int has_const(long long r, long long* out){ for (size_t i=0;i<consts_n;i++){ if (consts[i].reg==r){ if(out) *out=consts[i].val; return 1; } } return 0; }
      auto void put_const(long long r, long long v){ if (consts_n<1024){ consts[consts_n].reg=r; consts[consts_n].val=v; consts_n++; } }
      auto const char* get_str_const(long long r){ for (size_t i=0;i<str_consts_n;i++){ if (str_consts[i].reg==r) return str_consts[i].val; } return NULL; }
      auto void put_str_const(long long r, const char* v){ if (str_consts_n<512){ str_consts[str_consts_n].reg=r; str_consts[str_consts_n].val=v; str_consts_n++; } }
      auto int is_plain_i64_value(long long r){
        if (!r) return 0;
        if (get_type(r) != T_I64) return 0;
        if (get_origin(r) != ORG_NONE) return 0;
        if (get_scan_origin(r) != ORG_NONE) return 0;
        return get_str_const(r) == NULL;
      }
      auto struct BlockPhi* ensure_phi_block(long long bid){ for (int i=0;i<phi_n;i++){ if (phis[i].bid==bid) return &phis[i]; } if (phi_n<512){ phis[phi_n].bid=bid; phis[phi_n].rec_n=0; return &phis[phi_n++]; } return NULL; }
      auto long long read_int(yyjson_val* obj, const char* key){ yyjson_val* v= yyjson_obj_get(obj,key); return v? (long long)yyjson_get_sint(v) : 0; }
      auto const char* read_str(yyjson_val* obj, const char* key){ yyjson_val* v= yyjson_obj_get(obj,key); return v? yyjson_get_str(v) : NULL; }
      int array_rmw_trace_budget = 8;
      int array_string_len_trace_budget = 8;
      int generic_walk_trace_budget = 24;
      int generic_call_trace_budget = 24;
      int generic_method_trace_budget = 24;
      struct ArrayRmwWindowMatch {
        int matched;
        size_t copy_iis[8];
        int copy_n;
        size_t const_ii;
        size_t binop_ii;
        size_t set_ii;
        long long input_reg;
        long long const_reg;
        long long result_reg;
      };
      struct ArrayStringLenWindowMatch {
        int matched;
        int keep_get_live;
        size_t copy_iis[8];
        int copy_n;
        size_t len_ii;
        long long len_dst;
      };
      auto int inst_is_array_string_len_safe_reuse(yyjson_val* ins, long long reg) {
        if (!ins || !reg) return 0;
        const char* op = read_str(ins, "op");
        if (!(op && strcmp(op, "mir_call") == 0)) return 0;
        yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
        yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        const char* ctype = cal ? read_str(cal, "type") : NULL;
        const char* bname = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
        const char* mname = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
        long long recv = cal ? read_int(cal, "receiver") : 0;
        if (!(ctype && !strcmp(ctype, "Method"))) return 0;
        if (recv != reg) return 0;
        if (!(bname && !strcmp(bname, "RuntimeDataBox"))) return 0;
        return mname && !strcmp(mname, "substring");
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
      #include "hako_llvmc_ffi_indexof_observer_state.inc"
      auto int analyze_array_rmw_window_candidate(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long cur_dst,
          int recv_org,
          int runtime_array_get,
          int runtime_array_string,
          struct ArrayRmwWindowMatch* out,
          const char** reason_out,
          char* extra,
          size_t extra_cap) {
        const char* reason = "shape_match";
        if (out) memset(out, 0, sizeof(*out));
        if (extra && extra_cap) extra[0] = '\0';
        if (!mname || strcmp(mname, "get") != 0) {
          reason = "not_get";
          goto done;
        }
        if (!((bname && !strcmp(bname, "ArrayBox")) ||
              runtime_array_get ||
              runtime_array_string ||
              (bname && !strcmp(bname, "RuntimeDataBox") &&
               (recv_org == ORG_ARRAY_BIRTH || recv_org == ORG_ARRAY_STRING_BIRTH)))) {
          reason = "not_array_get";
          goto done;
        }
        if (ii + 3 >= ilen) {
          reason = "lookahead_short";
          goto done;
        }

        size_t jj = ii + 1;
        long long carried = cur_dst;
        if (!carried) {
          reason = "missing_get_dst";
          goto done;
        }
        if (out) out->input_reg = cur_dst;
        while (jj < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, jj);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried)) break;
          if (out && out->copy_n < (int)(sizeof(out->copy_iis) / sizeof(out->copy_iis[0]))) {
            out->copy_iis[out->copy_n++] = jj;
          }
          carried = read_int(c, "dst");
          jj++;
        }

        if (jj + 2 >= ilen) {
          reason = "copy_chain_short";
          goto done;
        }

        yyjson_val* n_const = yyjson_arr_get(insts, jj);
        yyjson_val* n_bin = yyjson_arr_get(insts, jj + 1);
        yyjson_val* n_set = yyjson_arr_get(insts, jj + 2);
        const char* op_const = n_const ? read_str(n_const, "op") : NULL;
        const char* op_bin = n_bin ? read_str(n_bin, "op") : NULL;
        const char* op_set = n_set ? read_str(n_set, "op") : NULL;
        long long const_reg = 0, const_val = 0;
        long long bin_lhs = 0, bin_rhs = 0;
        const char* add_kind = NULL;

        if (!(op_const && strcmp(op_const, "const") == 0)) {
          reason = "next_noncopy_not_const";
          goto done;
        }
        const_reg = read_int(n_const, "dst");
        if (!has_const(const_reg, &const_val) || const_val != 1) {
          reason = "const_not_1";
          goto done;
        }
        if (!(op_bin && strcmp(op_bin, "binop") == 0)) {
          reason = "next_after_const_not_binop";
          goto done;
        }
        add_kind = read_str(n_bin, "op_kind");
        if (!add_kind) add_kind = read_str(n_bin, "operation");
        if (!(add_kind && (!strcmp(add_kind, "Add") || !strcmp(add_kind, "+")))) {
          reason = "binop_not_add";
          goto done;
        }
        bin_lhs = read_int(n_bin, "lhs");
        bin_rhs = read_int(n_bin, "rhs");
        if (!((bin_lhs == carried && bin_rhs == const_reg) ||
              (bin_rhs == carried && bin_lhs == const_reg))) {
          reason = "binop_not_copy_transparent_get_plus_1";
          goto done;
        }
        if (!(op_set && strcmp(op_set, "mir_call") == 0)) {
          reason = "next_after_binop_not_set";
          goto done;
        }
        {
          yyjson_val* mc = yyjson_obj_get(n_set, "mir_call");
          yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
          yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
          const char* ctype = cal ? read_str(cal, "type") : NULL;
          const char* bname2 = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
          const char* mname2 = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
          long long recv2 = cal ? read_int(cal, "receiver") : 0;
          long long idx2 = 0;
          long long value2 = 0;
          if (args && yyjson_is_arr(args)) {
            if (yyjson_arr_size(args) >= 1) idx2 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
            if (yyjson_arr_size(args) >= 2) value2 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
          }
          if (!(ctype && !strcmp(ctype, "Method") && mname2 && !strcmp(mname2, "set") &&
                ((bname2 && !strcmp(bname2, "ArrayBox")) ||
                 (bname2 && !strcmp(bname2, "RuntimeDataBox") &&
                  (recv_org == ORG_ARRAY_BIRTH || recv_org == ORG_ARRAY_STRING_BIRTH))))) {
            reason = "set_not_array_method";
            goto done;
          }
          if (recv && recv2 && recv != recv2) {
            reason = "set_recv_mismatch";
            goto done;
          }
          if (idx && idx2 && idx != idx2) {
            reason = "set_idx_mismatch";
            goto done;
          }
          if (value2 != read_int(n_bin, "dst")) {
            reason = "set_value_not_binop_dst";
            goto done;
          }
        }

        if (out) {
          for (size_t kk = jj + 3; kk < ilen; kk++) {
            yyjson_val* later = yyjson_arr_get(insts, kk);
            if (inst_uses_reg(later, cur_dst) || inst_uses_reg(later, carried)) {
              reason = "post_set_uses_consumed_get_value";
              goto done;
            }
          }
          out->matched = 1;
          out->const_ii = jj;
          out->const_reg = const_reg;
          out->binop_ii = jj + 1;
          out->result_reg = read_int(n_bin, "dst");
          out->set_ii = jj + 2;
        }

      done:
        if (extra && extra_cap) {
          long long result_reg = out && out->matched ? out->result_reg : 0;
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld copy_n=%d carried=%lld const_ii=%zu binop_ii=%zu set_ii=%zu result=%lld",
                   ii, cur_dst, recv, idx,
                   out ? out->copy_n : 0,
                   carried,
                   out ? out->const_ii : 0,
                   out ? out->binop_ii : 0,
                   out ? out->set_ii : 0,
                   result_reg);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
      auto int analyze_array_string_len_window_candidate(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long cur_dst,
          int recv_org,
          int runtime_array_string,
          struct ArrayStringLenWindowMatch* out,
          const char** reason_out,
          char* extra,
          size_t extra_cap) {
        const char* reason = "shape_match";
        if (out) memset(out, 0, sizeof(*out));
        if (extra && extra_cap) extra[0] = '\0';
        if (!mname || strcmp(mname, "get") != 0) {
          reason = "not_get";
          goto done;
        }
        if (!(runtime_array_string ||
              (bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_STRING_BIRTH))) {
          reason = "not_array_string_get";
          goto done;
        }
        if (ii + 2 >= ilen) {
          reason = "lookahead_short";
          goto done;
        }
        size_t jj = ii + 1;
        long long carried = cur_dst;
        if (!carried) {
          reason = "missing_get_dst";
          goto done;
        }
        while (jj < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, jj);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried)) break;
          if (out && out->copy_n < (int)(sizeof(out->copy_iis) / sizeof(out->copy_iis[0]))) {
            out->copy_iis[out->copy_n++] = jj;
          }
          carried = read_int(c, "dst");
          jj++;
        }
        if (jj >= ilen) {
          reason = "copy_chain_short";
          goto done;
        }
        yyjson_val* n_len = yyjson_arr_get(insts, jj);
        const char* op_len = n_len ? read_str(n_len, "op") : NULL;
        if (!(op_len && strcmp(op_len, "mir_call") == 0)) {
          reason = "next_noncopy_not_len";
          goto done;
        }
        {
          yyjson_val* mc = yyjson_obj_get(n_len, "mir_call");
          yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
          const char* ctype = cal ? read_str(cal, "type") : NULL;
          const char* bname2 = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
          const char* mname2 = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
          long long recv2 = cal ? read_int(cal, "receiver") : 0;
          if (!(ctype && !strcmp(ctype, "Method") &&
                mname2 && (!strcmp(mname2, "len") || !strcmp(mname2, "length") || !strcmp(mname2, "size")) &&
                bname2 && !strcmp(bname2, "RuntimeDataBox"))) {
            reason = "next_noncopy_not_len";
            goto done;
          }
          if (recv2 && carried && recv2 != carried) {
            reason = "len_recv_mismatch";
            goto done;
          }
          for (size_t kk = jj + 1; kk < ilen; kk++) {
            yyjson_val* later = yyjson_arr_get(insts, kk);
            if (!(inst_uses_reg(later, cur_dst) || inst_uses_reg(later, carried))) continue;
            if (inst_is_array_string_len_safe_reuse(later, cur_dst) ||
                inst_is_array_string_len_safe_reuse(later, carried)) {
              if (out) out->keep_get_live = 1;
              continue;
            }
            if (inst_uses_reg(later, cur_dst) || inst_uses_reg(later, carried)) {
              reason = "post_len_uses_consumed_get_value";
              goto done;
            }
          }
          if (out) {
            out->matched = 1;
            out->len_ii = jj;
            out->len_dst = read_int(n_len, "dst");
          }
        }
      done:
        if (extra && extra_cap) {
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld copy_n=%d len_ii=%zu len_dst=%lld keep_get_live=%d",
                   ii, cur_dst, recv, idx,
                   out ? out->copy_n : 0,
                   out ? out->len_ii : 0,
                   out ? out->len_dst : 0,
                   out ? out->keep_get_live : 0);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
      #include "hako_llvmc_ffi_indexof_observer_direct_match.inc"
      #include "hako_llvmc_ffi_indexof_observer_block_match.inc"
      #if 0
      auto int analyze_array_string_indexof_cross_block_select_window_candidate(
          yyjson_val* blocks,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long cur_dst,
          int recv_org,
          int runtime_array_string,
          struct ArrayStringIndexofCrossBlockSelectWindowMatch* out,
          const char** reason_out,
          char* extra,
          size_t extra_cap) {
        const char* reason = "shape_match";
        if (out) memset(out, 0, sizeof(*out));
        if (extra && extra_cap) extra[0] = '\0';
        if (!mname || strcmp(mname, "get") != 0) {
          reason = "not_get";
          goto done;
        }
        if (!(runtime_array_string ||
              (bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_STRING_BIRTH))) {
          reason = "not_array_string_get";
          goto done;
        }
        if (ii + 4 >= ilen) {
          reason = "lookahead_short";
          goto done;
        }

        size_t jj = ii + 1;
        long long carried_get = cur_dst;
        if (!carried_get) {
          reason = "missing_get_dst";
          goto done;
        }
        while (jj < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, jj);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_get)) break;
          if (out && out->get_copy_n < (int)(sizeof(out->get_copy_iis) / sizeof(out->get_copy_iis[0]))) {
            out->get_copy_iis[out->get_copy_n++] = jj;
          }
          carried_get = read_int(c, "dst");
          jj++;
        }
        if (jj + 3 >= ilen) {
          reason = "copy_chain_short";
          goto done;
        }

        yyjson_val* n_needle = yyjson_arr_get(insts, jj);
        const char* op_needle = n_needle ? read_str(n_needle, "op") : NULL;
        long long needle_reg = 0;
        const char* needle_const = NULL;
        if (!(op_needle && strcmp(op_needle, "const") == 0)) {
          reason = "next_noncopy_not_const_string";
          goto done;
        }
        needle_reg = read_int(n_needle, "dst");
        needle_const = get_str_const(needle_reg);
        if (!needle_const) needle_const = read_string_const_from_inst(n_needle);
        if (!needle_const) {
          reason = "needle_not_string_const";
          goto done;
        }

        size_t kk = jj + 1;
        long long carried_recv = carried_get;
        while (kk < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, kk);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_recv)) break;
          if (out && out->recv_copy_n < (int)(sizeof(out->recv_copy_iis) / sizeof(out->recv_copy_iis[0]))) {
            out->recv_copy_iis[out->recv_copy_n++] = kk;
          }
          carried_recv = read_int(c, "dst");
          kk++;
        }
        if (kk + 1 >= ilen) {
          reason = "receiver_copy_chain_short";
          goto done;
        }

        yyjson_val* n_indexof = yyjson_arr_get(insts, kk);
        const char* op_indexof = n_indexof ? read_str(n_indexof, "op") : NULL;
        if (!(op_indexof && strcmp(op_indexof, "mir_call") == 0)) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        yyjson_val* mc_indexof = yyjson_obj_get(n_indexof, "mir_call");
        yyjson_val* cal_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "callee") : NULL;
        yyjson_val* args_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "args") : NULL;
        const char* ctype_indexof = cal_indexof ? read_str(cal_indexof, "type") : NULL;
        const char* bname_indexof = cal_indexof ? (read_str(cal_indexof, "box_name") ? read_str(cal_indexof, "box_name") : read_str(cal_indexof, "box_type")) : NULL;
        const char* mname_indexof = cal_indexof ? (read_str(cal_indexof, "method") ? read_str(cal_indexof, "method") : read_str(cal_indexof, "name")) : NULL;
        long long recv_indexof = cal_indexof ? read_int(cal_indexof, "receiver") : 0;
        long long arg_indexof = 0;
        if (args_indexof && yyjson_is_arr(args_indexof) && yyjson_arr_size(args_indexof) >= 1) {
          arg_indexof = (long long)yyjson_get_sint(yyjson_arr_get(args_indexof, 0));
        }
        if (!(ctype_indexof && !strcmp(ctype_indexof, "Method") &&
              mname_indexof && !strcmp(mname_indexof, "indexOf"))) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        if (recv_indexof != carried_recv) {
          reason = "indexof_recv_mismatch";
          goto done;
        }
        if (arg_indexof != needle_reg) {
          reason = "indexof_needle_mismatch";
          goto done;
        }
        if (!((bname_indexof && !strcmp(bname_indexof, "RuntimeDataBox")) ||
              (bname_indexof && !strcmp(bname_indexof, "StringBox")))) {
          reason = "indexof_not_string_method";
          goto done;
        }

        size_t ll = kk + 1;
        long long carried_indexof = read_int(n_indexof, "dst");
        if (!carried_indexof) {
          reason = "indexof_missing_dst";
          goto done;
        }
        while (ll < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, ll);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_indexof)) break;
          if (out && out->indexof_copy_n < (int)(sizeof(out->indexof_copy_iis) / sizeof(out->indexof_copy_iis[0]))) {
            out->indexof_copy_iis[out->indexof_copy_n++] = ll;
          }
          carried_indexof = read_int(c, "dst");
          ll++;
        }
        if (ll + 0 >= ilen) {
          reason = "indexof_copy_chain_short";
          goto done;
        }
        {
          yyjson_val* n_jump = yyjson_arr_get(insts, ll);
          const char* op_jump = n_jump ? read_str(n_jump, "op") : NULL;
          long long succ_bid = 0;
          yyjson_val* succ_block = NULL;
          yyjson_val* succ_insts = NULL;
          size_t slen = 0;
          size_t mm = 0;
          size_t ss = 0;
          long long carried_succ = carried_indexof;
          yyjson_val* n_zero = NULL;
          yyjson_val* n_compare = NULL;
          yyjson_val* n_then = NULL;
          yyjson_val* n_else = NULL;
          yyjson_val* n_select = NULL;
          const char* op_compare = NULL;
          const char* op_select = NULL;
          long long zero_reg = 0;
          long long zero_val = 0;
          long long compare_dst = 0;
          long long then_reg = 0;
          long long else_reg = 0;
          long long then_val = 0;
          long long else_val = 0;

          if (!(op_jump && strcmp(op_jump, "jump") == 0)) {
            reason = "next_after_indexof_not_jump";
            goto done;
          }
          succ_bid = read_int(n_jump, "target");
          if (!succ_bid) {
            reason = "jump_target_missing";
            goto done;
          }
          for (mm = ll + 1; mm < ilen; mm++) {
            yyjson_val* later = yyjson_arr_get(insts, mm);
            if (inst_uses_reg(later, cur_dst) || inst_uses_reg(later, carried_get) ||
                inst_uses_reg(later, carried_recv) || inst_uses_reg(later, carried_indexof)) {
              reason = "post_jump_uses_consumed_value";
              goto done;
            }
          }
          succ_block = find_block_by_id(blocks, succ_bid);
          if (!succ_block) {
            reason = "successor_block_missing";
            goto done;
          }
          succ_insts = yyjson_obj_get(succ_block, "instructions");
          if (!(succ_insts && yyjson_is_arr(succ_insts))) {
            reason = "successor_insts_missing";
            goto done;
          }
          slen = yyjson_arr_size(succ_insts);
          while (ss < slen) {
            yyjson_val* c = yyjson_arr_get(succ_insts, ss);
            const char* cop = c ? read_str(c, "op") : NULL;
            if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_succ)) break;
            carried_succ = read_int(c, "dst");
            ss++;
          }
          if (ss + 4 >= slen) {
            reason = "successor_shape_short";
            goto done;
          }
          n_zero = yyjson_arr_get(succ_insts, ss);
          n_compare = yyjson_arr_get(succ_insts, ss + 1);
          n_then = yyjson_arr_get(succ_insts, ss + 2);
          n_else = yyjson_arr_get(succ_insts, ss + 3);
          n_select = yyjson_arr_get(succ_insts, ss + 4);
          if (!(n_zero && read_str(n_zero, "op") && !strcmp(read_str(n_zero, "op"), "const"))) {
            reason = "successor_zero_missing";
            goto done;
          }
          zero_reg = read_int(n_zero, "dst");
          if (!(has_const(zero_reg, &zero_val) || read_i64_const_from_inst(n_zero, &zero_val)) || zero_val != 0) {
            reason = "successor_zero_not_zero";
            goto done;
          }
          if (!(n_compare && read_str(n_compare, "op") && !strcmp(read_str(n_compare, "op"), "compare"))) {
            reason = "successor_compare_missing";
            goto done;
          }
          op_compare = read_str(n_compare, "operation");
          if (!op_compare) op_compare = read_str(n_compare, "cmp");
          if (!(op_compare && (!strcmp(op_compare, ">=") || !strcmp(op_compare, "Ge") || !strcmp(op_compare, "ge")))) {
            reason = "successor_compare_not_ge_zero";
            goto done;
          }
          {
            long long lhs = read_int(n_compare, "lhs");
            if (!lhs) lhs = read_int(n_compare, "left");
            long long rhs = read_int(n_compare, "rhs");
            if (!rhs) rhs = read_int(n_compare, "right");
            if (!(lhs == carried_succ && rhs == zero_reg)) {
              reason = "successor_compare_not_indexof_ge_zero";
              goto done;
            }
          }
          compare_dst = read_int(n_compare, "dst");
          if (!(n_then && read_str(n_then, "op") && !strcmp(read_str(n_then, "op"), "const"))) {
            reason = "successor_then_not_const";
            goto done;
          }
          then_reg = read_int(n_then, "dst");
          if (!(has_const(then_reg, &then_val) || read_i64_const_from_inst(n_then, &then_val))) {
            reason = "successor_then_not_i64_const";
            goto done;
          }
          if (!(n_else && read_str(n_else, "op") && !strcmp(read_str(n_else, "op"), "const"))) {
            reason = "successor_else_not_const";
            goto done;
          }
          else_reg = read_int(n_else, "dst");
          if (!(has_const(else_reg, &else_val) || read_i64_const_from_inst(n_else, &else_val))) {
            reason = "successor_else_not_i64_const";
            goto done;
          }
          op_select = n_select ? read_str(n_select, "op") : NULL;
          if (!(op_select && !strcmp(op_select, "select"))) {
            reason = "successor_select_missing";
            goto done;
          }
          if (!(read_int(n_select, "cond") == compare_dst &&
                read_int(n_select, "then_val") == then_reg &&
                read_int(n_select, "else_val") == else_reg)) {
            reason = "successor_select_not_compare_terminal";
            goto done;
          }
          if (out) {
            out->matched = 1;
            out->needle_const_ii = jj;
            out->needle_reg = needle_reg;
            out->indexof_ii = kk;
            out->indexof_dst = read_int(n_indexof, "dst");
            out->jump_ii = ll;
            out->succ_bid = succ_bid;
          }
        }
      done:
        if (extra && extra_cap) {
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld get_copy_n=%d recv_copy_n=%d indexof_copy_n=%d indexof_ii=%zu jump_ii=%zu succ_bid=%lld",
                   ii, cur_dst, recv, idx,
                   out ? out->get_copy_n : 0,
                   out ? out->recv_copy_n : 0,
                   out ? out->indexof_copy_n : 0,
                   out ? out->indexof_ii : 0,
                   out ? out->jump_ii : 0,
                   out ? out->succ_bid : 0);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
      auto int analyze_array_string_indexof_interleaved_branch_window_candidate(
          yyjson_val* blocks,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long cur_dst,
          int recv_org,
          int runtime_array_string,
          struct ArrayStringIndexofInterleavedBranchWindowMatch* out,
          const char** reason_out,
          char* extra,
          size_t extra_cap) {
        const char* reason = "shape_match";
        if (out) memset(out, 0, sizeof(*out));
        if (extra && extra_cap) extra[0] = '\0';
        if (!mname || strcmp(mname, "get") != 0) {
          reason = "not_get";
          goto done;
        }
        if (!(runtime_array_string ||
              (bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_STRING_BIRTH))) {
          reason = "not_array_string_get";
          goto done;
        }
        if (ii + 8 >= ilen) {
          reason = "lookahead_short";
          goto done;
        }

        size_t jj = ii + 1;
        long long carried_get = cur_dst;
        if (!carried_get) {
          reason = "missing_get_dst";
          goto done;
        }
        while (jj < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, jj);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_get)) break;
          if (out && out->get_copy_n < (int)(sizeof(out->get_copy_iis) / sizeof(out->get_copy_iis[0]))) {
            out->get_copy_iis[out->get_copy_n++] = jj;
          }
          carried_get = read_int(c, "dst");
          jj++;
        }
        if (jj + 7 >= ilen) {
          reason = "copy_chain_short";
          goto done;
        }

        yyjson_val* n_needle = yyjson_arr_get(insts, jj);
        const char* op_needle = n_needle ? read_str(n_needle, "op") : NULL;
        long long needle_reg = 0;
        const char* needle_const = NULL;
        if (!(op_needle && strcmp(op_needle, "const") == 0)) {
          reason = "next_noncopy_not_const_string";
          goto done;
        }
        needle_reg = read_int(n_needle, "dst");
        needle_const = get_str_const(needle_reg);
        if (!needle_const) needle_const = read_string_const_from_inst(n_needle);
        if (!needle_const) {
          reason = "needle_not_string_const";
          goto done;
        }

        size_t kk = jj + 1;
        long long carried_recv = carried_get;
        while (kk < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, kk);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_recv)) break;
          if (out && out->recv_copy_n < (int)(sizeof(out->recv_copy_iis) / sizeof(out->recv_copy_iis[0]))) {
            out->recv_copy_iis[out->recv_copy_n++] = kk;
          }
          carried_recv = read_int(c, "dst");
          kk++;
        }
        if (kk + 6 >= ilen) {
          reason = "receiver_copy_chain_short";
          goto done;
        }

        yyjson_val* n_indexof = yyjson_arr_get(insts, kk);
        const char* op_indexof = n_indexof ? read_str(n_indexof, "op") : NULL;
        if (!(op_indexof && strcmp(op_indexof, "mir_call") == 0)) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        yyjson_val* mc_indexof = yyjson_obj_get(n_indexof, "mir_call");
        yyjson_val* cal_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "callee") : NULL;
        yyjson_val* args_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "args") : NULL;
        const char* ctype_indexof = cal_indexof ? read_str(cal_indexof, "type") : NULL;
        const char* bname_indexof = cal_indexof ? (read_str(cal_indexof, "box_name") ? read_str(cal_indexof, "box_name") : read_str(cal_indexof, "box_type")) : NULL;
        const char* mname_indexof = cal_indexof ? (read_str(cal_indexof, "method") ? read_str(cal_indexof, "method") : read_str(cal_indexof, "name")) : NULL;
        long long recv_indexof = cal_indexof ? read_int(cal_indexof, "receiver") : 0;
        long long arg_indexof = 0;
        if (args_indexof && yyjson_is_arr(args_indexof) && yyjson_arr_size(args_indexof) >= 1) {
          arg_indexof = (long long)yyjson_get_sint(yyjson_arr_get(args_indexof, 0));
        }
        if (!(ctype_indexof && !strcmp(ctype_indexof, "Method") &&
              mname_indexof && !strcmp(mname_indexof, "indexOf"))) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        if (recv_indexof != carried_recv) {
          reason = "indexof_recv_mismatch";
          goto done;
        }
        if (arg_indexof != needle_reg) {
          reason = "indexof_needle_mismatch";
          goto done;
        }
        if (!((bname_indexof && !strcmp(bname_indexof, "RuntimeDataBox")) ||
              (bname_indexof && !strcmp(bname_indexof, "StringBox")))) {
          reason = "indexof_not_string_method";
          goto done;
        }

        size_t ll = kk + 1;
        long long carried_indexof = read_int(n_indexof, "dst");
        if (!carried_indexof) {
          reason = "indexof_missing_dst";
          goto done;
        }
        while (ll < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, ll);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_indexof)) break;
          if (out && out->indexof_copy_n < (int)(sizeof(out->indexof_copy_iis) / sizeof(out->indexof_copy_iis[0]))) {
            out->indexof_copy_iis[out->indexof_copy_n++] = ll;
          }
          carried_indexof = read_int(c, "dst");
          ll++;
        }
        if (ll + 5 >= ilen) {
          reason = "indexof_copy_chain_short";
          goto done;
        }

        yyjson_val* n_guard_copy = yyjson_arr_get(insts, ll);
        const char* op_guard_copy = n_guard_copy ? read_str(n_guard_copy, "op") : NULL;
        long long guard_carried = 0;
        if (!(op_guard_copy && !strcmp(op_guard_copy, "copy"))) {
          reason = "guard_copy_missing";
          goto done;
        }
        guard_carried = read_int(n_guard_copy, "dst");
        if (!guard_carried) {
          reason = "guard_copy_dst_missing";
          goto done;
        }
        if (!read_int(n_guard_copy, "src")) {
          reason = "guard_copy_src_missing";
          goto done;
        }

        yyjson_val* n_guard_mod_const = yyjson_arr_get(insts, ll + 1);
        yyjson_val* n_guard_mod = yyjson_arr_get(insts, ll + 2);
        yyjson_val* n_guard_zero = yyjson_arr_get(insts, ll + 3);
        yyjson_val* n_guard_compare = yyjson_arr_get(insts, ll + 4);
        yyjson_val* n_guard_branch = yyjson_arr_get(insts, ll + 5);
        long long mod_const_reg = 0;
        long long mod_const_val = 0;
        long long zero_reg = 0;
        long long zero_val = 0;
        long long guard_mod_dst = 0;
        long long guard_compare_dst = 0;
        long long consumer_bid = 0;
        yyjson_val* consumer_block = NULL;
        yyjson_val* consumer_insts = NULL;
        size_t slen = 0;
        size_t ss = 0;
        long long carried_consumer = carried_indexof;

        if (!(n_guard_mod_const && read_str(n_guard_mod_const, "op") && !strcmp(read_str(n_guard_mod_const, "op"), "const"))) {
          reason = "guard_mod_const_missing";
          goto done;
        }
        mod_const_reg = read_int(n_guard_mod_const, "dst");
        if (!(has_const(mod_const_reg, &mod_const_val) || read_i64_const_from_inst(n_guard_mod_const, &mod_const_val)) || mod_const_val != 16) {
          reason = "guard_mod_const_not_16";
          goto done;
        }
        if (!(n_guard_mod && read_str(n_guard_mod, "op") && !strcmp(read_str(n_guard_mod, "op"), "binop"))) {
          reason = "guard_mod_missing";
          goto done;
        }
        {
          const char* guard_mod_op = read_str(n_guard_mod, "operation");
          if (!guard_mod_op) guard_mod_op = read_str(n_guard_mod, "op_kind");
          if (!(guard_mod_op && (!strcmp(guard_mod_op, "%") || !strcmp(guard_mod_op, "Mod")))) {
            reason = "guard_mod_not_percent";
            goto done;
          }
          if (!((read_int(n_guard_mod, "lhs") == guard_carried && read_int(n_guard_mod, "rhs") == mod_const_reg) ||
                (read_int(n_guard_mod, "rhs") == guard_carried && read_int(n_guard_mod, "lhs") == mod_const_reg))) {
            reason = "guard_mod_not_copy_percent_16";
            goto done;
          }
        }
        guard_mod_dst = read_int(n_guard_mod, "dst");
        if (!(n_guard_zero && read_str(n_guard_zero, "op") && !strcmp(read_str(n_guard_zero, "op"), "const"))) {
          reason = "guard_zero_missing";
          goto done;
        }
        zero_reg = read_int(n_guard_zero, "dst");
        if (!(has_const(zero_reg, &zero_val) || read_i64_const_from_inst(n_guard_zero, &zero_val)) || zero_val != 0) {
          reason = "guard_zero_not_zero";
          goto done;
        }
        if (!(n_guard_compare && read_str(n_guard_compare, "op") && !strcmp(read_str(n_guard_compare, "op"), "compare"))) {
          reason = "guard_compare_missing";
          goto done;
        }
        {
          const char* guard_compare_op = read_str(n_guard_compare, "operation");
          if (!guard_compare_op) guard_compare_op = read_str(n_guard_compare, "cmp");
          if (!(guard_compare_op && (!strcmp(guard_compare_op, "==") || !strcmp(guard_compare_op, "Eq") || !strcmp(guard_compare_op, "eq")))) {
            reason = "guard_compare_not_eq_zero";
            goto done;
          }
          if (!((read_int(n_guard_compare, "lhs") == guard_mod_dst && read_int(n_guard_compare, "rhs") == zero_reg) ||
                (read_int(n_guard_compare, "right") == zero_reg && read_int(n_guard_compare, "left") == guard_mod_dst))) {
            reason = "guard_compare_not_mod_eq_zero";
            goto done;
          }
        }
        guard_compare_dst = read_int(n_guard_compare, "dst");
        if (!(n_guard_branch && read_str(n_guard_branch, "op") && !strcmp(read_str(n_guard_branch, "op"), "branch"))) {
          reason = "guard_branch_missing";
          goto done;
        }
        if (read_int(n_guard_branch, "cond") != guard_compare_dst) {
          reason = "guard_branch_cond_mismatch";
          goto done;
        }
        consumer_bid = read_int(n_guard_branch, "then");
        if (!consumer_bid) {
          reason = "guard_then_missing";
          goto done;
        }
        consumer_block = find_block_by_id(blocks, consumer_bid);
        if (!consumer_block) {
          reason = "consumer_block_missing";
          goto done;
        }
        consumer_insts = yyjson_obj_get(consumer_block, "instructions");
        if (!(consumer_insts && yyjson_is_arr(consumer_insts))) {
          reason = "consumer_insts_missing";
          goto done;
        }
        slen = yyjson_arr_size(consumer_insts);
        while (ss < slen) {
          yyjson_val* c = yyjson_arr_get(consumer_insts, ss);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_consumer)) break;
          carried_consumer = read_int(c, "dst");
          ss++;
        }
        if (ss + 2 >= slen) {
          reason = "consumer_shape_short";
          goto done;
        }
        {
          yyjson_val* n_consumer_zero = yyjson_arr_get(consumer_insts, ss);
          yyjson_val* n_consumer_compare = yyjson_arr_get(consumer_insts, ss + 1);
          yyjson_val* n_consumer_branch = yyjson_arr_get(consumer_insts, ss + 2);
          long long consumer_zero_reg = 0;
          long long consumer_zero_val = 0;
          if (!(n_consumer_zero && read_str(n_consumer_zero, "op") && !strcmp(read_str(n_consumer_zero, "op"), "const"))) {
            reason = "consumer_zero_missing";
            goto done;
          }
          consumer_zero_reg = read_int(n_consumer_zero, "dst");
          if (!(has_const(consumer_zero_reg, &consumer_zero_val) || read_i64_const_from_inst(n_consumer_zero, &consumer_zero_val)) || consumer_zero_val != 0) {
            reason = "consumer_zero_not_zero";
            goto done;
          }
          if (!(n_consumer_compare && read_str(n_consumer_compare, "op") && !strcmp(read_str(n_consumer_compare, "op"), "compare"))) {
            reason = "consumer_compare_missing";
            goto done;
          }
          {
            const char* consumer_compare_op = read_str(n_consumer_compare, "operation");
            if (!consumer_compare_op) consumer_compare_op = read_str(n_consumer_compare, "cmp");
            if (!(consumer_compare_op && (!strcmp(consumer_compare_op, ">=") || !strcmp(consumer_compare_op, "Ge") || !strcmp(consumer_compare_op, "ge")))) {
              reason = "consumer_compare_not_ge_zero";
              goto done;
            }
            if (!((read_int(n_consumer_compare, "lhs") == carried_consumer && read_int(n_consumer_compare, "rhs") == consumer_zero_reg) ||
                  (read_int(n_consumer_compare, "left") == carried_consumer && read_int(n_consumer_compare, "right") == consumer_zero_reg))) {
              reason = "consumer_compare_not_indexof_ge_zero";
              goto done;
            }
          }
          if (!(n_consumer_branch && read_str(n_consumer_branch, "op") && !strcmp(read_str(n_consumer_branch, "op"), "branch"))) {
            reason = "consumer_branch_missing";
            goto done;
          }
          if (read_int(n_consumer_branch, "cond") != read_int(n_consumer_compare, "dst")) {
            reason = "consumer_branch_cond_mismatch";
            goto done;
          }
        }

        if (out) {
          out->matched = 1;
          out->needle_const_ii = jj;
          out->needle_reg = needle_reg;
          out->indexof_ii = kk;
          out->indexof_dst = read_int(n_indexof, "dst");
          out->guard_compare_ii = ll + 4;
          out->guard_branch_ii = ll + 5;
          out->consumer_bid = consumer_bid;
        }
      done:
        if (extra && extra_cap) {
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld get_copy_n=%d recv_copy_n=%d indexof_copy_n=%d indexof_ii=%zu guard_branch_ii=%zu consumer_bid=%lld",
                   ii, cur_dst, recv, idx,
                   out ? out->get_copy_n : 0,
                   out ? out->recv_copy_n : 0,
                   out ? out->indexof_copy_n : 0,
                   out ? out->indexof_ii : 0,
                   out ? out->guard_branch_ii : 0,
                   out ? out->consumer_bid : 0);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
      auto int analyze_array_string_indexof_interleaved_select_window_candidate(
          yyjson_val* blocks,
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long cur_dst,
          int recv_org,
          int runtime_array_string,
          struct ArrayStringIndexofInterleavedSelectWindowMatch* out,
          const char** reason_out,
          char* extra,
          size_t extra_cap) {
        const char* reason = "shape_match";
        if (out) memset(out, 0, sizeof(*out));
        if (extra && extra_cap) extra[0] = '\0';
        if (!mname || strcmp(mname, "get") != 0) {
          reason = "not_get";
          goto done;
        }
        if (!(runtime_array_string ||
              (bname && !strcmp(bname, "RuntimeDataBox") && recv_org == ORG_ARRAY_STRING_BIRTH))) {
          reason = "not_array_string_get";
          goto done;
        }
        if (ii + 8 >= ilen) {
          reason = "lookahead_short";
          goto done;
        }

        size_t jj = ii + 1;
        long long carried_get = cur_dst;
        if (!carried_get) {
          reason = "missing_get_dst";
          goto done;
        }
        while (jj < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, jj);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_get)) break;
          if (out && out->get_copy_n < (int)(sizeof(out->get_copy_iis) / sizeof(out->get_copy_iis[0]))) {
            out->get_copy_iis[out->get_copy_n++] = jj;
          }
          carried_get = read_int(c, "dst");
          jj++;
        }
        if (jj + 7 >= ilen) {
          reason = "copy_chain_short";
          goto done;
        }

        yyjson_val* n_needle = yyjson_arr_get(insts, jj);
        const char* op_needle = n_needle ? read_str(n_needle, "op") : NULL;
        long long needle_reg = 0;
        const char* needle_const = NULL;
        if (!(op_needle && strcmp(op_needle, "const") == 0)) {
          reason = "next_noncopy_not_const_string";
          goto done;
        }
        needle_reg = read_int(n_needle, "dst");
        needle_const = get_str_const(needle_reg);
        if (!needle_const) needle_const = read_string_const_from_inst(n_needle);
        if (!needle_const) {
          reason = "needle_not_string_const";
          goto done;
        }

        size_t kk = jj + 1;
        long long carried_recv = carried_get;
        while (kk < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, kk);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_recv)) break;
          if (out && out->recv_copy_n < (int)(sizeof(out->recv_copy_iis) / sizeof(out->recv_copy_iis[0]))) {
            out->recv_copy_iis[out->recv_copy_n++] = kk;
          }
          carried_recv = read_int(c, "dst");
          kk++;
        }
        if (kk + 6 >= ilen) {
          reason = "receiver_copy_chain_short";
          goto done;
        }

        yyjson_val* n_indexof = yyjson_arr_get(insts, kk);
        const char* op_indexof = n_indexof ? read_str(n_indexof, "op") : NULL;
        if (!(op_indexof && strcmp(op_indexof, "mir_call") == 0)) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        yyjson_val* mc_indexof = yyjson_obj_get(n_indexof, "mir_call");
        yyjson_val* cal_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "callee") : NULL;
        yyjson_val* args_indexof = mc_indexof ? yyjson_obj_get(mc_indexof, "args") : NULL;
        const char* ctype_indexof = cal_indexof ? read_str(cal_indexof, "type") : NULL;
        const char* bname_indexof = cal_indexof ? (read_str(cal_indexof, "box_name") ? read_str(cal_indexof, "box_name") : read_str(cal_indexof, "box_type")) : NULL;
        const char* mname_indexof = cal_indexof ? (read_str(cal_indexof, "method") ? read_str(cal_indexof, "method") : read_str(cal_indexof, "name")) : NULL;
        long long recv_indexof = cal_indexof ? read_int(cal_indexof, "receiver") : 0;
        long long arg_indexof = 0;
        if (args_indexof && yyjson_is_arr(args_indexof) && yyjson_arr_size(args_indexof) >= 1) {
          arg_indexof = (long long)yyjson_get_sint(yyjson_arr_get(args_indexof, 0));
        }
        if (!(ctype_indexof && !strcmp(ctype_indexof, "Method") &&
              mname_indexof && !strcmp(mname_indexof, "indexOf"))) {
          reason = "next_after_const_not_indexof";
          goto done;
        }
        if (recv_indexof != carried_recv) {
          reason = "indexof_recv_mismatch";
          goto done;
        }
        if (arg_indexof != needle_reg) {
          reason = "indexof_needle_mismatch";
          goto done;
        }
        if (!((bname_indexof && !strcmp(bname_indexof, "RuntimeDataBox")) ||
              (bname_indexof && !strcmp(bname_indexof, "StringBox")))) {
          reason = "indexof_not_string_method";
          goto done;
        }

        size_t ll = kk + 1;
        long long carried_indexof = read_int(n_indexof, "dst");
        if (!carried_indexof) {
          reason = "indexof_missing_dst";
          goto done;
        }
        while (ll < ilen) {
          yyjson_val* c = yyjson_arr_get(insts, ll);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_indexof)) break;
          if (out && out->indexof_copy_n < (int)(sizeof(out->indexof_copy_iis) / sizeof(out->indexof_copy_iis[0]))) {
            out->indexof_copy_iis[out->indexof_copy_n++] = ll;
          }
          carried_indexof = read_int(c, "dst");
          ll++;
        }
        if (ll + 5 >= ilen) {
          reason = "indexof_copy_chain_short";
          goto done;
        }

        yyjson_val* n_guard_copy = yyjson_arr_get(insts, ll);
        const char* op_guard_copy = n_guard_copy ? read_str(n_guard_copy, "op") : NULL;
        long long guard_carried = 0;
        if (!(op_guard_copy && !strcmp(op_guard_copy, "copy"))) {
          reason = "guard_copy_missing";
          goto done;
        }
        guard_carried = read_int(n_guard_copy, "dst");
        if (!guard_carried) {
          reason = "guard_copy_dst_missing";
          goto done;
        }
        if (!read_int(n_guard_copy, "src")) {
          reason = "guard_copy_src_missing";
          goto done;
        }

        yyjson_val* n_guard_mod_const = yyjson_arr_get(insts, ll + 1);
        yyjson_val* n_guard_mod = yyjson_arr_get(insts, ll + 2);
        yyjson_val* n_guard_zero = yyjson_arr_get(insts, ll + 3);
        yyjson_val* n_guard_compare = yyjson_arr_get(insts, ll + 4);
        yyjson_val* n_guard_branch = yyjson_arr_get(insts, ll + 5);
        long long mod_const_reg = 0;
        long long mod_const_val = 0;
        long long zero_reg = 0;
        long long zero_val = 0;
        long long guard_mod_dst = 0;
        long long guard_compare_dst = 0;
        long long bridge_bid = 0;
        yyjson_val* bridge_block = NULL;
        yyjson_val* bridge_insts = NULL;
        yyjson_val* consumer_block = NULL;
        yyjson_val* consumer_insts = NULL;
        long long consumer_bid = 0;
        size_t slen = 0;
        size_t ss = 0;
        long long carried_consumer = carried_indexof;

        if (!(n_guard_mod_const && read_str(n_guard_mod_const, "op") && !strcmp(read_str(n_guard_mod_const, "op"), "const"))) {
          reason = "guard_mod_const_missing";
          goto done;
        }
        mod_const_reg = read_int(n_guard_mod_const, "dst");
        if (!(has_const(mod_const_reg, &mod_const_val) || read_i64_const_from_inst(n_guard_mod_const, &mod_const_val)) || mod_const_val != 16) {
          reason = "guard_mod_const_not_16";
          goto done;
        }
        if (!(n_guard_mod && read_str(n_guard_mod, "op") && !strcmp(read_str(n_guard_mod, "op"), "binop"))) {
          reason = "guard_mod_missing";
          goto done;
        }
        {
          const char* guard_mod_op = read_str(n_guard_mod, "operation");
          if (!guard_mod_op) guard_mod_op = read_str(n_guard_mod, "op_kind");
          if (!(guard_mod_op && (!strcmp(guard_mod_op, "%") || !strcmp(guard_mod_op, "Mod")))) {
            reason = "guard_mod_not_percent";
            goto done;
          }
          if (!((read_int(n_guard_mod, "lhs") == guard_carried && read_int(n_guard_mod, "rhs") == mod_const_reg) ||
                (read_int(n_guard_mod, "rhs") == guard_carried && read_int(n_guard_mod, "lhs") == mod_const_reg))) {
            reason = "guard_mod_not_copy_percent_16";
            goto done;
          }
        }
        guard_mod_dst = read_int(n_guard_mod, "dst");
        if (!(n_guard_zero && read_str(n_guard_zero, "op") && !strcmp(read_str(n_guard_zero, "op"), "const"))) {
          reason = "guard_zero_missing";
          goto done;
        }
        zero_reg = read_int(n_guard_zero, "dst");
        if (!(has_const(zero_reg, &zero_val) || read_i64_const_from_inst(n_guard_zero, &zero_val)) || zero_val != 0) {
          reason = "guard_zero_not_zero";
          goto done;
        }
        if (!(n_guard_compare && read_str(n_guard_compare, "op") && !strcmp(read_str(n_guard_compare, "op"), "compare"))) {
          reason = "guard_compare_missing";
          goto done;
        }
        {
          const char* guard_compare_op = read_str(n_guard_compare, "operation");
          if (!guard_compare_op) guard_compare_op = read_str(n_guard_compare, "cmp");
          if (!(guard_compare_op && (!strcmp(guard_compare_op, "==") || !strcmp(guard_compare_op, "Eq") || !strcmp(guard_compare_op, "eq")))) {
            reason = "guard_compare_not_eq_zero";
            goto done;
          }
          if (!((read_int(n_guard_compare, "lhs") == guard_mod_dst && read_int(n_guard_compare, "rhs") == zero_reg) ||
                (read_int(n_guard_compare, "right") == zero_reg && read_int(n_guard_compare, "left") == guard_mod_dst))) {
            reason = "guard_compare_not_mod_eq_zero";
            goto done;
          }
        }
        guard_compare_dst = read_int(n_guard_compare, "dst");
        if (!(n_guard_branch && read_str(n_guard_branch, "op") && !strcmp(read_str(n_guard_branch, "op"), "branch"))) {
          reason = "guard_branch_missing";
          goto done;
        }
        if (read_int(n_guard_branch, "cond") != guard_compare_dst) {
          reason = "guard_branch_cond_mismatch";
          goto done;
        }
        bridge_bid = read_int(n_guard_branch, "else");
        if (!bridge_bid) {
          reason = "guard_else_missing";
          goto done;
        }
        bridge_block = find_block_by_id(blocks, bridge_bid);
        if (!bridge_block) {
          reason = "bridge_block_missing";
          goto done;
        }
        bridge_insts = yyjson_obj_get(bridge_block, "instructions");
        if (!(bridge_insts && yyjson_is_arr(bridge_insts) && yyjson_arr_size(bridge_insts) == 1)) {
          reason = "bridge_shape_not_single_jump";
          goto done;
        }
        {
          yyjson_val* n_bridge_jump = yyjson_arr_get(bridge_insts, 0);
          if (!(n_bridge_jump && read_str(n_bridge_jump, "op") && !strcmp(read_str(n_bridge_jump, "op"), "jump"))) {
            reason = "bridge_not_jump";
            goto done;
          }
          consumer_bid = read_int(n_bridge_jump, "target");
        }
        if (!consumer_bid) {
          reason = "bridge_target_missing";
          goto done;
        }
        consumer_block = find_block_by_id(blocks, consumer_bid);
        if (!consumer_block) {
          reason = "consumer_block_missing";
          goto done;
        }
        consumer_insts = yyjson_obj_get(consumer_block, "instructions");
        if (!(consumer_insts && yyjson_is_arr(consumer_insts))) {
          reason = "consumer_insts_missing";
          goto done;
        }
        slen = yyjson_arr_size(consumer_insts);
        while (ss < slen) {
          yyjson_val* c = yyjson_arr_get(consumer_insts, ss);
          const char* cop = c ? read_str(c, "op") : NULL;
          if (!(cop && strcmp(cop, "copy") == 0 && read_int(c, "src") == carried_consumer)) break;
          carried_consumer = read_int(c, "dst");
          ss++;
        }
        if (ss + 5 >= slen) {
          reason = "consumer_shape_short";
          goto done;
        }
        {
          yyjson_val* n_consumer_zero = yyjson_arr_get(consumer_insts, ss);
          yyjson_val* n_consumer_compare = yyjson_arr_get(consumer_insts, ss + 1);
          yyjson_val* n_base_copy = yyjson_arr_get(consumer_insts, ss + 2);
          yyjson_val* n_add_one = yyjson_arr_get(consumer_insts, ss + 3);
          yyjson_val* n_add = yyjson_arr_get(consumer_insts, ss + 4);
          yyjson_val* n_select = yyjson_arr_get(consumer_insts, ss + 5);
          long long consumer_zero_reg = 0;
          long long consumer_zero_val = 0;
          long long base_reg = 0;
          long long add_one_reg = 0;
          long long add_one_val = 0;
          if (!(n_consumer_zero && read_str(n_consumer_zero, "op") && !strcmp(read_str(n_consumer_zero, "op"), "const"))) {
            reason = "consumer_zero_missing";
            goto done;
          }
          consumer_zero_reg = read_int(n_consumer_zero, "dst");
          if (!(has_const(consumer_zero_reg, &consumer_zero_val) || read_i64_const_from_inst(n_consumer_zero, &consumer_zero_val)) || consumer_zero_val != 0) {
            reason = "consumer_zero_not_zero";
            goto done;
          }
          if (!(n_consumer_compare && read_str(n_consumer_compare, "op") && !strcmp(read_str(n_consumer_compare, "op"), "compare"))) {
            reason = "consumer_compare_missing";
            goto done;
          }
          {
            const char* consumer_compare_op = read_str(n_consumer_compare, "operation");
            if (!consumer_compare_op) consumer_compare_op = read_str(n_consumer_compare, "cmp");
            if (!(consumer_compare_op && (!strcmp(consumer_compare_op, ">=") || !strcmp(consumer_compare_op, "Ge") || !strcmp(consumer_compare_op, "ge")))) {
              reason = "consumer_compare_not_ge_zero";
              goto done;
            }
            if (!((read_int(n_consumer_compare, "lhs") == carried_consumer && read_int(n_consumer_compare, "rhs") == consumer_zero_reg) ||
                  (read_int(n_consumer_compare, "left") == carried_consumer && read_int(n_consumer_compare, "right") == consumer_zero_reg))) {
              reason = "consumer_compare_not_indexof_ge_zero";
              goto done;
            }
          }
          if (!(n_base_copy && read_str(n_base_copy, "op") && !strcmp(read_str(n_base_copy, "op"), "copy"))) {
            reason = "consumer_base_copy_missing";
            goto done;
          }
          base_reg = read_int(n_base_copy, "dst");
          if (!(n_add_one && read_str(n_add_one, "op") && !strcmp(read_str(n_add_one, "op"), "const"))) {
            reason = "consumer_add_one_const_missing";
            goto done;
          }
          add_one_reg = read_int(n_add_one, "dst");
          if (!(has_const(add_one_reg, &add_one_val) || read_i64_const_from_inst(n_add_one, &add_one_val)) || add_one_val != 1) {
            reason = "consumer_add_one_not_1";
            goto done;
          }
          if (!(n_add && read_str(n_add, "op") && !strcmp(read_str(n_add, "op"), "binop"))) {
            reason = "consumer_add_missing";
            goto done;
          }
          {
            const char* add_op = read_str(n_add, "operation");
            if (!add_op) add_op = read_str(n_add, "op_kind");
            if (!(add_op && (!strcmp(add_op, "+") || !strcmp(add_op, "Add")))) {
              reason = "consumer_add_not_plus";
              goto done;
            }
            if (!((read_int(n_add, "lhs") == base_reg && read_int(n_add, "rhs") == add_one_reg) ||
                  (read_int(n_add, "rhs") == base_reg && read_int(n_add, "lhs") == add_one_reg))) {
              reason = "consumer_add_not_base_plus_1";
              goto done;
            }
          }
          if (!(n_select && read_str(n_select, "op") && !strcmp(read_str(n_select, "op"), "select"))) {
            reason = "consumer_select_missing";
            goto done;
          }
          if (!(read_int(n_select, "cond") == read_int(n_consumer_compare, "dst") &&
                read_int(n_select, "then_val") == read_int(n_add, "dst") &&
                read_int(n_select, "else_val") == base_reg)) {
            reason = "consumer_select_not_compare_terminal";
            goto done;
          }
        }

        if (out) {
          out->matched = 1;
          out->needle_const_ii = jj;
          out->needle_reg = needle_reg;
          out->indexof_ii = kk;
          out->indexof_dst = read_int(n_indexof, "dst");
          out->guard_compare_ii = ll + 4;
          out->guard_branch_ii = ll + 5;
          out->bridge_bid = bridge_bid;
          out->consumer_bid = consumer_bid;
        }
      done:
        if (extra && extra_cap) {
          snprintf(extra, extra_cap,
                   "ii=%zu dst=%lld recv=%lld idx=%lld get_copy_n=%d recv_copy_n=%d indexof_copy_n=%d indexof_ii=%zu guard_branch_ii=%zu bridge_bid=%lld consumer_bid=%lld",
                   ii, cur_dst, recv, idx,
                   out ? out->get_copy_n : 0,
                   out ? out->recv_copy_n : 0,
                   out ? out->indexof_copy_n : 0,
                   out ? out->indexof_ii : 0,
                   out ? out->guard_branch_ii : 0,
                   out ? out->bridge_bid : 0,
                   out ? out->consumer_bid : 0);
        }
        if (reason_out) *reason_out = reason;
        return out && out->matched;
      }
      #endif
      auto void trace_array_rmw_window_candidate(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long cur_dst,
          int recv_org,
          int runtime_array_get,
          int runtime_array_string) {
        if (!hako_llvmc_route_trace_enabled() || array_rmw_trace_budget <= 0) return;
        array_rmw_trace_budget--;
        struct ArrayRmwWindowMatch match;
        const char* reason = NULL;
        char extra[384];
        int matched = analyze_array_rmw_window_candidate(
            ii, ilen, insts, bname, mname, recv, idx, cur_dst, recv_org,
            runtime_array_get, runtime_array_string, &match, &reason, extra, sizeof(extra));
        hako_llvmc_emit_route_trace("array_rmw_window", matched ? "hit" : "miss", reason ? reason : "unknown", extra);
      }
      auto void trace_array_string_len_window_candidate(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          const char* bname,
          const char* mname,
          long long recv,
          long long idx,
          long long cur_dst,
          int recv_org,
          int runtime_array_string) {
        if (!hako_llvmc_route_trace_enabled() || array_string_len_trace_budget <= 0) return;
        array_string_len_trace_budget--;
        struct ArrayStringLenWindowMatch match;
        const char* reason = NULL;
        char extra[256];
        int matched = analyze_array_string_len_window_candidate(
            ii, ilen, insts, bname, mname, recv, idx, cur_dst, recv_org,
            runtime_array_string, &match, &reason, extra, sizeof(extra));
        hako_llvmc_emit_route_trace("array_string_len_window", matched ? "hit" : "miss", reason ? reason : "unknown", extra);
      }
      #include "hako_llvmc_ffi_indexof_observer_trace.inc"
      #include "hako_llvmc_ffi_indexof_observer_lowering.inc"
      #include "hako_llvmc_ffi_string_concat_lowering.inc"
      #include "hako_llvmc_ffi_const_string_hoist.inc"
      #include "hako_llvmc_ffi_mir_call_prepass.inc"
      #include "hako_llvmc_ffi_mir_call_shell.inc"
      #include "hako_llvmc_ffi_generic_method_lowering.inc"

      // Pre-scan: consts + phis + needed method decls
      struct GenericPureNeedFlags needs;
      memset(&needs, 0, sizeof(needs));
      size_t blen = yyjson_arr_size(blocks);
      for (size_t bi=0; bi<blen; bi++) {
        yyjson_val* b = yyjson_arr_get(blocks, bi);
        long long bid = read_int(b, "id");
        yyjson_val* insts = yyjson_obj_get(b, "instructions"); if (!insts || !yyjson_is_arr(insts)) continue;
        size_t ilen = yyjson_arr_size(insts);
        for (size_t ii=0; ii<ilen; ii++) {
          yyjson_val* ins = yyjson_arr_get(insts, ii);
          const char* op = read_str(ins, "op"); if (!op) continue;
          if (strcmp(op, "const")==0) {
            long long dst = read_int(ins, "dst");
            yyjson_val* vobj = yyjson_obj_get(ins, "value");
            yyjson_val* tyobj = vobj ? yyjson_obj_get(vobj, "type") : NULL;
            const char* box_type = tyobj ? read_str(tyobj, "box_type") : NULL;
            const char* sval = vobj ? read_str(vobj, "value") : NULL;
            if (box_type && strcmp(box_type, "StringBox") == 0 && sval) {
              put_str_const(dst, sval);
              set_type(dst, T_I64);
              set_scan_origin(dst, ORG_STRING);
              set_origin(dst, ORG_STRING);
              needs.string_const_box = 1;
            } else {
              long long v = vobj? (long long)yyjson_get_sint(yyjson_obj_get(vobj, "value")) : 0;
              put_const(dst, v); set_type(dst, T_I64);
            }
          } else if (strcmp(op, "newbox")==0) {
            long long dst = read_int(ins, "dst");
            const char* tname = read_str(ins, "type");
            if (tname && strcmp(tname, "ArrayBox") == 0) {
              needs.arr_birth = 1;
              if (dst) {
                set_scan_origin(dst, ORG_ARRAY_BIRTH);
                set_origin(dst, ORG_ARRAY_BIRTH);
              }
            } else if (tname && strcmp(tname, "StringBox") == 0) {
              yyjson_val* nb_args = yyjson_obj_get(ins, "args");
              yyjson_val* nb0 = nb_args && yyjson_is_arr(nb_args) ? yyjson_arr_get_first(nb_args) : NULL;
              long long arg0 = nb0 ? (long long)yyjson_get_sint(nb0) : 0;
              if (arg0 && get_scan_origin(arg0) == ORG_STRING) {
                if (dst) {
                  set_scan_origin(dst, ORG_STRING);
                  set_origin(dst, ORG_STRING);
                }
              }
            }
          } else if (strcmp(op, "copy")==0) {
            long long dst = read_int(ins, "dst");
            long long src = read_int(ins, "src");
            int scan_org = get_scan_origin(src);
            if (scan_org != ORG_NONE && dst) set_scan_origin(dst, scan_org);
          } else if (strcmp(op, "select")==0) {
            long long dst = read_int(ins, "dst");
            long long then_val = read_int(ins, "then_val");
            long long else_val = read_int(ins, "else_val");
            int then_ty = then_val ? get_type(then_val) : T_NONE;
            int else_ty = else_val ? get_type(else_val) : T_NONE;
            int then_scan = then_val ? get_scan_origin(then_val) : ORG_NONE;
            int else_scan = else_val ? get_scan_origin(else_val) : ORG_NONE;
            int then_org = then_val ? get_origin(then_val) : ORG_NONE;
            int else_org = else_val ? get_origin(else_val) : ORG_NONE;
            if (dst) {
              if (then_ty != T_NONE && then_ty == else_ty) set_type(dst, then_ty);
              else if (then_ty != T_NONE && else_ty == T_NONE) set_type(dst, then_ty);
              else if (else_ty != T_NONE && then_ty == T_NONE) set_type(dst, else_ty);
              else set_type(dst, T_I64);
              if (then_scan != ORG_NONE && then_scan == else_scan) set_scan_origin(dst, then_scan);
              if (then_org != ORG_NONE && then_org == else_org) set_origin(dst, then_org);
            }
          } else if (strcmp(op, "phi")==0) {
            struct BlockPhi* pb = ensure_phi_block(bid); if (!pb) { yyjson_doc_free(d); goto GEN_END; }
            struct PhiRec pr; pr.dst = read_int(ins, "dst"); pr.in_n=0;
            yyjson_val* vals = yyjson_obj_get(ins, "values"); if (!vals) vals = yyjson_obj_get(ins, "incoming");
            if (vals && yyjson_is_arr(vals)) {
              size_t vn = yyjson_arr_size(vals);
              for (size_t vi=0; vi<vn && pr.in_n<16; vi++) {
                yyjson_val* ent = yyjson_arr_get(vals, vi);
                long long pred = 0;
                long long vin = 0;
                if (ent && yyjson_is_arr(ent) && yyjson_arr_size(ent) >= 2) {
                  yyjson_val* ent0 = yyjson_arr_get(ent, 0);
                  yyjson_val* ent1 = yyjson_arr_get(ent, 1);
                  vin = ent0 ? (long long)yyjson_get_sint(ent0) : 0;
                  pred = ent1 ? (long long)yyjson_get_sint(ent1) : 0;
                } else {
                  pred = read_int(ent, "pred"); if (!pred) pred = read_int(ent, "block");
                  vin = read_int(ent, "value");
                }
                pr.in[pr.in_n].pred = pred; pr.in[pr.in_n].val_reg = vin; pr.in_n++;
              }
            } else {
              long long pred = read_int(ins, "pred"); long long vin = read_int(ins, "value");
              if (vin) { pr.in[pr.in_n].pred=pred; pr.in[pr.in_n].val_reg=vin; pr.in_n++; }
            }
            if (pb->rec_n < 16) {
              int scan_first = ORG_NONE;
              int scan_same = 1;
              int origin_first = ORG_NONE;
              int origin_same = 1;
              for (int i = 0; i < pr.in_n; i++) {
                int s = pr.in[i].val_reg ? get_scan_origin(pr.in[i].val_reg) : ORG_NONE;
                int o = pr.in[i].val_reg ? get_origin(pr.in[i].val_reg) : ORG_NONE;
                if (s != ORG_NONE) {
                  if (scan_first == ORG_NONE) scan_first = s;
                  else if (scan_first != s) scan_same = 0;
                }
                if (o != ORG_NONE) {
                  if (origin_first == ORG_NONE) origin_first = o;
                  else if (origin_first != o) origin_same = 0;
                }
              }
              pb->recs[pb->rec_n++] = pr;
              set_type(pr.dst, T_I64);
              if (scan_same && scan_first != ORG_NONE) {
                set_scan_origin(pr.dst, scan_first);
              }
              if (origin_same && origin_first != ORG_NONE) {
                set_origin(pr.dst, origin_first);
              }
            }
          } else if (strcmp(op, "mir_call")==0) {
            scan_mir_call_need_flags(
                blocks,
                bid,
                ii,
                ilen,
                insts,
                ins,
                &needs);
          } else if (strcmp(op, "binop")==0) {
            const char* bop = read_str(ins, "operation");
            if (!bop) bop = read_str(ins, "op_kind");
            long long dst = read_int(ins, "dst");
            long long lhs = read_int(ins, "lhs");
            long long rhs = read_int(ins, "rhs");
            if ((bop && (strcmp(bop, "+") == 0 || strcmp(bop, "Add") == 0)) &&
                get_scan_origin(lhs) == ORG_STRING && get_scan_origin(rhs) == ORG_STRING) {
              needs.string_concat = 1;
              if (dst) set_scan_origin(dst, ORG_STRING);
            }
          }
        }
      }

      // IR temp file
      char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_gen_%d.ll", "/tmp", (int)getpid());
      FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(d); break; }
      fprintf(f, "; nyash pure IR (generic)\n");
      fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
      if (needs.map_birth) fprintf(f, "declare i64 @\"nyash.map.birth_h\"()\n");
      if (needs.map_set)   fprintf(f, "declare i64 @\"nyash.map.slot_store_hhh\"(i64, i64, i64)\n");
      if (needs.map_get)   fprintf(f, "declare i64 @\"nyash.map.slot_load_hh\"(i64, i64)\n");
      if (needs.map_has)   fprintf(f, "declare i64 @\"nyash.map.probe_hh\"(i64, i64)\n");
      if (needs.map_size)  fprintf(f, "declare i64 @\"nyash.map.entry_count_h\"(i64)\n");
      if (needs.arr_birth) fprintf(f, "declare i64 @\"nyash.array.birth_h\"()\n");
      if (needs.arr_push)  fprintf(f, "declare i64 @\"nyash.array.slot_append_hh\"(i64, i64)\n");
      if (needs.arr_len)   fprintf(f, "declare i64 @\"nyash.array.slot_len_h\"(i64)\n");
      if (needs.arr_set)   fprintf(f, "declare i64 @\"nyash.array.set_hih\"(i64, i64, i64)\n");
      if (needs.arr_set)   fprintf(f, "declare i64 @\"nyash.array.set_his\"(i64, i64, i64)\n");
      if (needs.arr_set)   fprintf(f, "declare i64 @\"nyash.array.slot_store_hii\"(i64, i64, i64)\n");
      if (needs.arr_get && needs.arr_set) fprintf(f, "declare i64 @\"nyash.array.rmw_add1_hi\"(i64, i64)\n");
      if (needs.arr_get && needs.string_len) fprintf(f, "declare i64 @\"nyash.array.string_len_hi\"(i64, i64)\n");
      if (needs.arr_string_indexof) fprintf(f, "declare i64 @\"nyash.array.string_indexof_hih\"(i64, i64, i64)\n");
      if (needs.arr_get)   fprintf(f, "declare i64 @\"nyash.array.slot_load_hi\"(i64, i64)\n");
      if (needs.arr_has)   fprintf(f, "declare i64 @\"nyash.array.has_hi\"(i64, i64)\n");
      if (needs.string_const_box) fprintf(f, "declare i64 @\"nyash.box.from_i8_string_const\"(ptr)\n");
      if (needs.string_len) fprintf(f, "declare i64 @\"nyash.string.len_h\"(i64)\n");
      if (needs.string_substring) fprintf(f, "declare i64 @\"nyash.string.substring_hii\"(i64, i64, i64)\n");
      if (needs.string_indexof) fprintf(f, "declare i64 @\"nyash.string.indexOf_hh\"(i64, i64)\n");
      if (needs.string_concat) {
        fprintf(f, "declare i64 @\"nyash.string.concat_hh\"(i64, i64)\n");
        fprintf(f, "declare i64 @\"nyash.string.concat_hs\"(i64, ptr)\n");
        fprintf(f, "declare i64 @\"nyash.string.insert_hsi\"(i64, ptr, i64)\n");
        fprintf(f, "declare i64 @\"nyash.string.concat3_hhh\"(i64, i64, i64)\n");
      }
      if (needs.printf)    fprintf(f, "@.fmt_i64 = private unnamed_addr constant [5 x i8] c\"%%ld\\0A\\00\", align 1\n");
      for (size_t sci=0; sci<str_consts_n; sci++) {
        size_t slen = strlen(str_consts[sci].val);
        fprintf(f, "@.str_reg_%lld = private unnamed_addr constant [%zu x i8] c\"", str_consts[sci].reg, slen + 1);
        hako_llvmc_emit_llvm_byte_string(f, str_consts[sci].val, (long long)slen);
        fprintf(f, "\\00\", align 1\n");
      }
      // Unboxer (declare opportunistically; low cost)
      fprintf(f, "declare i64 @\"nyash.integer.get_h\"(i64)\n");
      if (needs.printf)    fprintf(f, "declare i32 @printf(ptr, ...)\n");
      fprintf(f, "\n");
      // Unboxer (declare opportunistically; low cost)
      hako_llvmc_emit_entry_header(f, &selection);
      // Emit blocks
      #define EMIT(...) do { fprintf(f, __VA_ARGS__); } while(0)
      auto void emit_block_label(long long bid){ EMIT("bb%lld:\n", bid); }
      auto void emit_phi(long long dst, struct Incoming* in, int in_n){ EMIT("  %%r%lld = phi i64 ", dst); for (int i=0;i<in_n;i++){ long long cv; int hc=has_const(in[i].val_reg,&cv); EMIT("[ %s%lld%s, %%bb%lld ]%s", hc?"":"%r", hc?cv:in[i].val_reg, hc?"":"", in[i].pred, (i+1<in_n)?", ":""); } EMIT("\n"); }
      auto void emit_icmp(long long dst, const char* pred, long long lhs_reg, long long rhs_reg){ long long vL,vR; int lc=has_const(lhs_reg,&vL), rc=has_const(rhs_reg,&vR); EMIT("  %%r%lld = icmp %s i64 %s%lld%s, %s%lld%s\n", dst, pred, lc?"":"%r", lc?vL:lhs_reg, lc?"":"", rc?"":"%r", rc?vR:rhs_reg, rc?"":""); set_type(dst,T_I1); }
      auto void emit_branch(long long cond_reg, long long then_id, long long else_id){ int ty=get_type(cond_reg); if (ty==T_I1) EMIT("  br i1 %%r%lld, label %%bb%lld, label %%bb%lld\n", cond_reg, then_id, else_id); else { long long cv; if (has_const(cond_reg,&cv)) { EMIT("  %%t%lld = icmp ne i64 %lld, 0\n", cond_reg, cv); EMIT("  br i1 %%t%lld, label %%bb%lld, label %%bb%lld\n", cond_reg, then_id, else_id);} else { EMIT("  %%t%lld = icmp ne i64 %%r%lld, 0\n", cond_reg, cond_reg); EMIT("  br i1 %%t%lld, label %%bb%lld, label %%bb%lld\n", cond_reg, then_id, else_id);} } }
      auto void emit_jump(long long target){ EMIT("  br label %%bb%lld\n", target); }
      auto void emit_ret(long long reg){ long long cv; if (has_const(reg,&cv)) EMIT("  ret i64 %lld\n", cv); else EMIT("  ret i64 %%r%lld\n", reg); }
      auto void emit_call_assign(long long dst, const char* sym, const char* args){ EMIT("  %%r%lld = call i64 @\"%s\"(%s)\n", dst, sym, args); set_type(dst, T_I64); }
      auto void emit_call_noret(const char* sym, const char* args){ EMIT("  call i64 @\"%s\"(%s)\n", sym, args); }

      for (size_t bi=0; bi<blen; bi++) {
        yyjson_val* b = yyjson_arr_get(blocks, bi);
        long long bid = read_int(b, "id");
        emit_block_label(bid);
        // phi first
        for (int pi=0; pi<phi_n; pi++) if (phis[pi].bid==bid){ for (int r=0;r<phis[pi].rec_n;r++){ emit_phi(phis[pi].recs[r].dst, phis[pi].recs[r].in, phis[pi].recs[r].in_n); } }
        if (bi == 0) {
          emit_hoisted_string_const_defs(f);
        }
        yyjson_val* insts = yyjson_obj_get(b, "instructions"); if (!insts || !yyjson_is_arr(insts)) continue;
        size_t ilen = yyjson_arr_size(insts);
        unsigned char skip_inst[ilen ? ilen : 1];
        memset(skip_inst, 0, sizeof(skip_inst));
        for (size_t ii=0; ii<ilen; ii++) {
          if (skip_inst[ii]) continue;
          yyjson_val* ins = yyjson_arr_get(insts, ii);
          const char* op = read_str(ins, "op"); if (!op) { yyjson_doc_free(d); goto GEN_ABORT; }
          if (hako_llvmc_route_trace_enabled() && generic_walk_trace_budget > 0) {
            char extra[256];
            long long dst_dbg = read_int(ins, "dst");
            snprintf(extra, sizeof(extra), "ii=%zu dst=%lld op=%s", ii, dst_dbg, op);
            hako_llvmc_emit_route_trace("generic_walk", "seen", op, extra);
            generic_walk_trace_budget--;
          }
          if (strcmp(op, "phi")==0) continue; // already emitted
          if (strcmp(op, "const")==0) {
            long long dst = read_int(ins, "dst");
            const char* sval = get_str_const(dst);
            if (!sval) continue; // numeric const stays inlined
            if (should_skip_string_const_box_for_concat_suffix(ii, ilen, insts, dst)) continue;
            if (hako_llvmc_fast_enabled()) continue;
            size_t slen = strlen(sval);
            EMIT("  %%r%lld = call i64 @\"nyash.box.from_i8_string_const\"(ptr getelementptr inbounds ([%zu x i8], ptr @.str_reg_%lld, i64 0, i64 0))\n", dst, slen + 1, dst);
            set_type(dst, T_I64);
            set_origin(dst, ORG_STRING);
            set_scan_origin(dst, ORG_STRING);
            continue;
          }
          if (strcmp(op, "release_strong")==0) {
            continue;
          }
          if (strcmp(op, "newbox")==0) {
            long long dst = read_int(ins, "dst");
            const char* tname = read_str(ins, "type");
            if (tname && strcmp(tname, "ArrayBox") == 0) {
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.array.birth_h\"()\n", dst);
                set_type(dst, T_I64);
                set_origin(dst, ORG_ARRAY_BIRTH);
                set_scan_origin(dst, ORG_ARRAY_BIRTH);
              } else {
                EMIT("  call i64 @\"nyash.array.birth_h\"()\n");
              }
              continue;
            }
            if (tname && strcmp(tname, "StringBox") == 0) {
              yyjson_val* nb_args = yyjson_obj_get(ins, "args");
              yyjson_val* nb0 = nb_args && yyjson_is_arr(nb_args) ? yyjson_arr_get_first(nb_args) : NULL;
              long long arg0 = nb0 ? (long long)yyjson_get_sint(nb0) : 0;
              if (dst && arg0) {
                long long cv = 0;
                if (has_const(arg0, &cv)) {
                  EMIT("  %%r%lld = add i64 0, %lld\n", dst, cv);
                } else {
                  EMIT("  %%r%lld = add i64 0, %%r%lld\n", dst, arg0);
                }
                set_type(dst, T_I64);
                set_origin(dst, ORG_STRING);
                set_scan_origin(dst, ORG_STRING);
                continue;
              }
            }
            yyjson_doc_free(d); goto GEN_ABORT;
          }
          if (strcmp(op, "copy")==0) {
            long long dst = read_int(ins, "dst");
            long long src = read_int(ins, "src");
            long long cv = 0;
            int src_ty = get_type(src);
            int src_origin = get_origin(src);
            int src_scan_origin = get_scan_origin(src);
            if (has_const(src, &cv)) {
              EMIT("  %%r%lld = add i64 0, %lld\n", dst, cv);
              put_const(dst, cv);
              set_type(dst, T_I64);
            } else if (src_ty == T_I1) {
              EMIT("  %%r%lld = or i1 %%r%lld, false\n", dst, src);
              set_type(dst, T_I1);
            } else {
              EMIT("  %%r%lld = add i64 0, %%r%lld\n", dst, src);
              set_type(dst, src_ty == T_NONE ? T_I64 : src_ty);
            }
            if (src_origin != ORG_NONE) set_origin(dst, src_origin);
            if (src_scan_origin != ORG_NONE) set_scan_origin(dst, src_scan_origin);
            continue;
          }
          if (strcmp(op, "compare")==0) {
            const char* pred = read_str(ins, "cmp"); if (!pred) pred = read_str(ins, "operation");
            const char* p2 = NULL; if (pred){ if (!strcmp(pred,"Lt")||!strcmp(pred,"lt")||!strcmp(pred,"<")) p2="slt"; else if (!strcmp(pred,"Le")||!strcmp(pred,"le")||!strcmp(pred,"<=")) p2="sle"; else if (!strcmp(pred,"Eq")||!strcmp(pred,"eq")||!strcmp(pred,"==")) p2="eq"; else if (!strcmp(pred,"Ne")||!strcmp(pred,"ne")||!strcmp(pred,"!=")) p2="ne"; else if (!strcmp(pred,"Ge")||!strcmp(pred,"ge")||!strcmp(pred,">=")) p2="sge"; else if (!strcmp(pred,"Gt")||!strcmp(pred,"gt")||!strcmp(pred,">")) p2="sgt"; }
            if (!p2) { yyjson_doc_free(d); goto GEN_ABORT; }
            long long dst = read_int(ins, "dst"); long long lhs = read_int(ins, "lhs"); if (!lhs) lhs=read_int(ins, "left"); long long rhs = read_int(ins, "rhs"); if (!rhs) rhs=read_int(ins, "right"); if (!lhs && !rhs){ yyjson_doc_free(d); goto GEN_ABORT; }
            // emit icmp
            long long vL,vR; int lc=has_const(lhs,&vL), rc2=has_const(rhs,&vR);
            EMIT("  %%r%lld = icmp %s i64 %s%lld%s, %s%lld%s\n", dst, p2, lc?"":"%r", lc?vL:lhs, lc?"":"", rc2?"":"%r", rc2?vR:rhs, rc2?"":""); set_type(dst, T_I1);
            continue;
          }
          if (strcmp(op, "select")==0) {
            long long dst = read_int(ins, "dst");
            long long cond = read_int(ins, "cond");
            long long then_val = read_int(ins, "then_val");
            long long else_val = read_int(ins, "else_val");
            long long cond_cv = 0, then_cv = 0, else_cv = 0;
            int cond_ty = get_type(cond);
            int then_ty = get_type(then_val);
            int else_ty = get_type(else_val);
            int val_ty = T_I64;
            int cond_const = has_const(cond, &cond_cv);
            int then_const = has_const(then_val, &then_cv);
            int else_const = has_const(else_val, &else_cv);
            int then_org = get_origin(then_val);
            int else_org = get_origin(else_val);
            int then_scan = get_scan_origin(then_val);
            int else_scan = get_scan_origin(else_val);
            char cond_ref[64];
            char then_ref[64];
            char else_ref[64];

            if (then_ty == T_I1 && else_ty == T_I1) val_ty = T_I1;
            else if (then_ty != T_NONE && then_ty == else_ty) val_ty = then_ty;
            else if (then_ty != T_NONE && else_ty == T_NONE) val_ty = then_ty;
            else if (else_ty != T_NONE && then_ty == T_NONE) val_ty = else_ty;

            if (cond_ty == T_I1) {
              if (cond_const) snprintf(cond_ref, sizeof(cond_ref), "%s", cond_cv ? "true" : "false");
              else snprintf(cond_ref, sizeof(cond_ref), "%%r%lld", cond);
            } else if (cond_const) {
              EMIT("  %%t%lld = icmp ne i64 %lld, 0\n", dst, cond_cv);
              snprintf(cond_ref, sizeof(cond_ref), "%%t%lld", dst);
            } else {
              EMIT("  %%t%lld = icmp ne i64 %%r%lld, 0\n", dst, cond);
              snprintf(cond_ref, sizeof(cond_ref), "%%t%lld", dst);
            }

            if (val_ty == T_I1) {
              if (then_const) snprintf(then_ref, sizeof(then_ref), "%s", then_cv ? "true" : "false");
              else snprintf(then_ref, sizeof(then_ref), "%%r%lld", then_val);
              if (else_const) snprintf(else_ref, sizeof(else_ref), "%s", else_cv ? "true" : "false");
              else snprintf(else_ref, sizeof(else_ref), "%%r%lld", else_val);
              EMIT("  %%r%lld = select i1 %s, i1 %s, i1 %s\n",
                   dst,
                   cond_ref,
                   then_ref,
                   else_ref);
              set_type(dst, T_I1);
            } else {
              if (then_const) snprintf(then_ref, sizeof(then_ref), "%lld", then_cv);
              else snprintf(then_ref, sizeof(then_ref), "%%r%lld", then_val);
              if (else_const) snprintf(else_ref, sizeof(else_ref), "%lld", else_cv);
              else snprintf(else_ref, sizeof(else_ref), "%%r%lld", else_val);
              EMIT("  %%r%lld = select i1 %s, i64 %s, i64 %s\n",
                   dst,
                   cond_ref,
                   then_ref,
                   else_ref);
              set_type(dst, T_I64);
            }
            if (then_org != ORG_NONE && then_org == else_org) set_origin(dst, then_org);
            if (then_scan != ORG_NONE && then_scan == else_scan) set_scan_origin(dst, then_scan);
            continue;
          }
          if (strcmp(op, "branch")==0) { long long cond=read_int(ins, "cond"); long long th=read_int(ins,"then"); long long el=read_int(ins,"else"); if(!el) el=read_int(ins,"else_id"); int ty=get_type(cond); if (ty==T_I1) EMIT("  br i1 %%r%lld, label %%bb%lld, label %%bb%lld\n", cond, th, el); else { long long cv; if (has_const(cond,&cv)) { EMIT("  %%t%lld = icmp ne i64 %lld, 0\n", cond, cv); EMIT("  br i1 %%t%lld, label %%bb%lld, label %%bb%lld\n", cond, th, el);} else { EMIT("  %%t%lld = icmp ne i64 %%r%lld, 0\n", cond, cond); EMIT("  br i1 %%t%lld, label %%bb%lld, label %%bb%lld\n", cond, th, el);} } continue; }
          if (strcmp(op, "jump")==0) { long long tgt=read_int(ins, "target"); EMIT("  br label %%bb%lld\n", tgt); continue; }
          if (strcmp(op, "ret")==0) {
            long long v=read_int(ins, "value"); long long cv;
            if (has_const(v,&cv)) { EMIT("  ret i64 %lld\n", cv); }
            else {
              int org = get_origin(v);
              if (org == ORG_MAP_GET) {
                // Auto-unbox map.get result as Integer (minimal MVP)
                EMIT("  %%map_get_unbox_%lld = call i64 @\"nyash.integer.get_h\"(i64 %%r%lld)\n", v, v);
                EMIT("  ret i64 %%map_get_unbox_%lld\n", v);
              } else {
                EMIT("  ret i64 %%r%lld\n", v);
              }
            }
            continue;
          }
          if (strcmp(op, "binop")==0) {
            const char* k = read_str(ins, "op_kind"); if (!k) k = read_str(ins, "operation");
            const char* irop = NULL;
            int lhs_org = get_origin(read_int(ins, "lhs"));
            int rhs_org = get_origin(read_int(ins, "rhs"));
            if (k) {
              if (!strcmp(k, "Add") || !strcmp(k, "+")) irop = "add";
              else if (!strcmp(k, "Sub") || !strcmp(k, "-")) irop = "sub";
              else if (!strcmp(k, "Mul") || !strcmp(k, "*")) irop = "mul";
              else if (!strcmp(k, "Div") || !strcmp(k, "/")) irop = "sdiv";
              else if (!strcmp(k, "Mod") || !strcmp(k, "%")) irop = "srem";
            }
            if (!irop) { yyjson_doc_free(d); goto GEN_ABORT; }
            long long dst = read_int(ins, "dst"); long long lhs = read_int(ins, "lhs"); long long rhs = read_int(ins, "rhs");
            if (!strcmp(irop, "add") && lhs_org == ORG_STRING && rhs_org == ORG_STRING) {
              if (should_defer_string_concat_pair(ii, ilen, insts, dst)) {
                remember_string_concat_pair(dst, lhs, rhs);
                finalize_string_concat_result(dst);
                continue;
              }
              if (try_emit_string_concat_const_suffix_call(f, ii, ilen, insts, dst, lhs, rhs)) {
                continue;
              }
              if (try_emit_string_insert_mid_const_call(f, ii, ilen, insts, dst, lhs, rhs)) {
                continue;
              }
              if (!try_emit_string_concat3_chain_call(f, dst, lhs, rhs)) {
                emit_string_concat_hh_call(f, dst, lhs, rhs);
              }
              continue;
            }
            long long vL, vR; int lc = has_const(lhs, &vL), rc2 = has_const(rhs, &vR);
            if (lc && rc2) {
              EMIT("  %%r%lld = %s i64 %lld, %lld\n", dst, irop, vL, vR);
            } else {
              EMIT("  %%r%lld = %s i64 %s%lld%s, %s%lld%s\n", dst, irop,
                   lc?"":"%r", lc?vL:lhs, lc?"":"",
                   rc2?"":"%r", rc2?vR:rhs, rc2?"":"");
            }
            set_type(dst, T_I64);
            continue;
          }
          if (strcmp(op, "mir_call")==0) {
            yyjson_val* mc = yyjson_obj_get(ins, "mir_call"); yyjson_val* cal = mc? yyjson_obj_get(mc, "callee") : NULL; const char* ctype = cal? read_str(cal, "type") : NULL; const char* bname = cal? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL; const char* mname = cal? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL; long long dst = read_int(ins, "dst"); long long recv = read_int(cal, "receiver"); yyjson_val* args = mc? yyjson_obj_get(mc, "args") : NULL; long long a0=0,a1=0; if (args && yyjson_is_arr(args)) { if (yyjson_arr_size(args)>=1) a0=(long long)yyjson_get_sint(yyjson_arr_get(args,0)); if (yyjson_arr_size(args)>=2) a1=(long long)yyjson_get_sint(yyjson_arr_get(args,1)); }
            char ab[256]; ab[0]='\0';
            if (hako_llvmc_route_trace_enabled() && generic_call_trace_budget > 0) {
              char extra[384];
              snprintf(extra, sizeof(extra),
                       "ii=%zu dst=%lld recv=%lld ctype=%s bname=%s mname=%s a0=%lld a1=%lld",
                       ii, dst, recv,
                       ctype ? ctype : "-",
                       bname ? bname : "-",
                       mname ? mname : "-",
                       a0, a1);
              hako_llvmc_emit_route_trace("mir_call", "seen", "enter", extra);
              generic_call_trace_budget--;
            }
            if (ctype && !strcmp(ctype, "Constructor")) {
              if (emit_constructor_mir_call(f, bname, dst) < 0) { yyjson_doc_free(d); goto GEN_ABORT; }
            } else if (ctype && !strcmp(ctype, "Method")) {
              struct GenericMethodRouteState route =
                  classify_generic_method_route(bid, ii, bname, mname, recv);
              trace_generic_method_route_state(
                  ii, dst, recv, ctype, bname, mname, a0, a1, &route);
              append_array_string_indexof_method_receiver_args(
                  route.deferred_array_string_indexof,
                  recv,
                  ab,
                  sizeof(ab));
              {
                int non_indexof_method_rc = try_emit_non_indexof_generic_method_call(
                    f,
                    blocks,
                    bid,
                    ii,
                    ilen,
                    insts,
                    bname,
                    mname,
                    dst,
                    recv,
                    a0,
                    a1,
                    &route,
                    ab,
                    sizeof(ab),
                    skip_inst);
                if (non_indexof_method_rc < 0) { yyjson_doc_free(d); goto GEN_ABORT; }
                if (non_indexof_method_rc > 0) continue;
              }
              if (mname && !strcmp(mname, "indexOf") && route.runtime_string) {
                if (!emit_deferred_array_string_indexof_call(f, route.deferred_array_string_indexof, dst, ab)) {
                  if (a0) append_i64_arg_ref(ab, sizeof(ab), a0, ab[0]=='\0');
                  if (dst) {
                    EMIT("  %%r%lld = call i64 @\"nyash.string.indexOf_hh\"(%s)\n", dst, ab);
                    set_type(dst, T_I64);
                  } else {
                    EMIT("  call i64 @\"nyash.string.indexOf_hh\"(%s)\n", ab);
                  }
                }
              }
              else { yyjson_doc_free(d); goto GEN_ABORT; }
            } else if (ctype && !strcmp(ctype, "Extern")) {
              if (emit_supported_string_extern_mir_call(f, mname, dst, args) < 0) {
                yyjson_doc_free(d);
                goto GEN_ABORT;
              }
            } else if (ctype && !strcmp(ctype, "Global")) {
              if (emit_global_mir_call(f, mname, a0) < 0) { yyjson_doc_free(d); goto GEN_ABORT; }
            } else { yyjson_doc_free(d); goto GEN_ABORT; }
            continue;
          }
          // any other op unsupported
          yyjson_doc_free(d); goto GEN_ABORT;
        }
      }
      fprintf(f, "}\n");
      hako_llvmc_emit_entry_alias_if_needed(f, &selection);
      fclose(f);

      // Optional: try LLVM TargetMachine emit (dlopen C-API) when HAKO_CAPI_TM=1
      int rc = -1;
#if !defined(_WIN32)
      {
        const char* tm = getenv("HAKO_CAPI_TM");
        if (tm && tm[0]=='1') {
          // Minimal dynamic loader for a subset of LLVM C-API
          typedef void* (*p_LLVMContextCreate)(void);
          typedef void  (*p_LLVMContextDispose)(void*);
          typedef void* (*p_LLVMCreateMemoryBufferWithContentsOfFile)(const char*, char**);
          typedef void  (*p_LLVMDisposeMemoryBuffer)(void*);
          typedef int   (*p_LLVMParseIRInContext)(void*, void*, void**, char**);
          typedef char* (*p_LLVMGetDefaultTargetTriple)(void);
          typedef void  (*p_LLVMDisposeMessage)(char*);
          typedef int   (*p_LLVMGetTargetFromTriple)(const char*, void**, char**);
          typedef void* (*p_LLVMCreateTargetMachine)(void*, const char*, const char*, const char*, int, int, int);
          typedef int   (*p_LLVMTargetMachineEmitToFile)(void*, void*, char*, int, char**);
          typedef void  (*p_LLVMDisposeTargetMachine)(void*);
          typedef void  (*p_LLVMDisposeModule)(void*);
          typedef void* (*p_LLVMModuleCreateWithNameInContext)(const char*, void*);
          // Target init (X86)
          typedef void (*p_LLVMInitializeX86TargetInfo)(void);
          typedef void (*p_LLVMInitializeX86Target)(void);
          typedef void (*p_LLVMInitializeX86TargetMC)(void);
          typedef void (*p_LLVMInitializeX86AsmPrinter)(void);

          const char* cand[] = { "libLLVM-18.so", "libLLVM.so.18", "libLLVM.so", NULL };
          void* h = NULL; for (int i=0;cand[i];i++){ h = dlopen(cand[i], RTLD_LAZY|RTLD_LOCAL); if (h) break; }
          if (h) {
            // Resolve required symbols
            p_LLVMContextCreate                 f_LLVMContextCreate                 = (p_LLVMContextCreate)dlsym(h, "LLVMContextCreate");
            p_LLVMContextDispose                f_LLVMContextDispose                = (p_LLVMContextDispose)dlsym(h, "LLVMContextDispose");
            p_LLVMCreateMemoryBufferWithContentsOfFile f_LLVMCreateMemoryBufferWithContentsOfFile = (p_LLVMCreateMemoryBufferWithContentsOfFile)dlsym(h, "LLVMCreateMemoryBufferWithContentsOfFile");
            p_LLVMDisposeMemoryBuffer           f_LLVMDisposeMemoryBuffer           = (p_LLVMDisposeMemoryBuffer)dlsym(h, "LLVMDisposeMemoryBuffer");
            p_LLVMParseIRInContext              f_LLVMParseIRInContext              = (p_LLVMParseIRInContext)dlsym(h, "LLVMParseIRInContext");
            p_LLVMGetDefaultTargetTriple        f_LLVMGetDefaultTargetTriple        = (p_LLVMGetDefaultTargetTriple)dlsym(h, "LLVMGetDefaultTargetTriple");
            p_LLVMDisposeMessage                f_LLVMDisposeMessage                = (p_LLVMDisposeMessage)dlsym(h, "LLVMDisposeMessage");
            p_LLVMGetTargetFromTriple           f_LLVMGetTargetFromTriple           = (p_LLVMGetTargetFromTriple)dlsym(h, "LLVMGetTargetFromTriple");
            p_LLVMCreateTargetMachine           f_LLVMCreateTargetMachine           = (p_LLVMCreateTargetMachine)dlsym(h, "LLVMCreateTargetMachine");
            p_LLVMTargetMachineEmitToFile       f_LLVMTargetMachineEmitToFile       = (p_LLVMTargetMachineEmitToFile)dlsym(h, "LLVMTargetMachineEmitToFile");
            p_LLVMDisposeTargetMachine          f_LLVMDisposeTargetMachine          = (p_LLVMDisposeTargetMachine)dlsym(h, "LLVMDisposeTargetMachine");
            p_LLVMDisposeModule                 f_LLVMDisposeModule                 = (p_LLVMDisposeModule)dlsym(h, "LLVMDisposeModule");
            p_LLVMModuleCreateWithNameInContext f_LLVMModuleCreateWithNameInContext = (p_LLVMModuleCreateWithNameInContext)dlsym(h, "LLVMModuleCreateWithNameInContext");
            // Target init
            p_LLVMInitializeX86TargetInfo       f_LLVMInitializeX86TargetInfo       = (p_LLVMInitializeX86TargetInfo)dlsym(h, "LLVMInitializeX86TargetInfo");
            p_LLVMInitializeX86Target           f_LLVMInitializeX86Target           = (p_LLVMInitializeX86Target)dlsym(h, "LLVMInitializeX86Target");
            p_LLVMInitializeX86TargetMC         f_LLVMInitializeX86TargetMC         = (p_LLVMInitializeX86TargetMC)dlsym(h, "LLVMInitializeX86TargetMC");
            p_LLVMInitializeX86AsmPrinter       f_LLVMInitializeX86AsmPrinter       = (p_LLVMInitializeX86AsmPrinter)dlsym(h, "LLVMInitializeX86AsmPrinter");

            int ok = f_LLVMContextCreate && f_LLVMCreateMemoryBufferWithContentsOfFile && f_LLVMParseIRInContext &&
                     f_LLVMGetDefaultTargetTriple && f_LLVMGetTargetFromTriple && f_LLVMCreateTargetMachine &&
                     f_LLVMTargetMachineEmitToFile && f_LLVMDisposeTargetMachine && f_LLVMDisposeMessage &&
                     f_LLVMDisposeMemoryBuffer && f_LLVMContextDispose && f_LLVMDisposeModule &&
                     f_LLVMInitializeX86TargetInfo && f_LLVMInitializeX86Target && f_LLVMInitializeX86TargetMC && f_LLVMInitializeX86AsmPrinter;
            if (ok) {
              // Init targets
              f_LLVMInitializeX86TargetInfo(); f_LLVMInitializeX86Target(); f_LLVMInitializeX86TargetMC(); f_LLVMInitializeX86AsmPrinter();
              void* ctx = f_LLVMContextCreate();
              char* emsg = NULL; void* buf = f_LLVMCreateMemoryBufferWithContentsOfFile(llpath, &emsg);
              if (!buf) { if (emsg) f_LLVMDisposeMessage(emsg); goto TM_END; }
              void* mod = NULL; if (f_LLVMParseIRInContext(ctx, buf, &mod, &emsg)) { if (emsg) f_LLVMDisposeMessage(emsg); f_LLVMDisposeMemoryBuffer(buf); goto TM_CTX; }
              char* triple = f_LLVMGetDefaultTargetTriple();
              void* tgt = NULL; if (f_LLVMGetTargetFromTriple(triple, &tgt, &emsg)) { if (emsg) f_LLVMDisposeMessage(emsg); goto TM_MOD; }
              // Opt level
              const char* ol = getenv("HAKO_LLVM_OPT_LEVEL"); if (!ol) ol = getenv("NYASH_LLVM_OPT_LEVEL"); if (!ol) ol = "0";
              int opt = (ol[0]=='3')?3:(ol[0]=='2')?2:(ol[0]=='1')?1:0; // LLVMCodeGenOptLevel
              void* tmachine = f_LLVMCreateTargetMachine(tgt, triple, "", "", opt, /*Reloc*/0, /*CodeModel*/0);
              if (!tmachine) { goto TM_MOD; }
              // Emit object (1 = LLVMObjectFile)
              if (f_LLVMTargetMachineEmitToFile(tmachine, mod, (char*)obj_out, /*Object*/1, &emsg)) {
                if (emsg) f_LLVMDisposeMessage(emsg);
              } else {
                rc = 0;
              }
              f_LLVMDisposeTargetMachine(tmachine);
            TM_MOD:
              if (mod) f_LLVMDisposeModule(mod);
              if (triple) f_LLVMDisposeMessage(triple);
              f_LLVMDisposeMemoryBuffer(buf);
            TM_CTX:
              f_LLVMContextDispose(ctx);
            TM_END: ;
            }
            dlclose(h);
          }
        }
      }
#endif

      hako_llvmc_maybe_dump_ir_file(llpath);
      if (rc != 0) {
        char cmd[2048]; snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
        rc = system(cmd);
      }
      remove(llpath);
      yyjson_doc_free(d);
      if (rc == 0) return 0;
    GEN_ABORT:;
      // fall through to pattern lowers
    GEN_END: ;
    } while(0);
    // Try minimal pure path #1: recognize simple Ret(Const)
    {
      yyjson_read_err rerr0; yyjson_doc* d0 = yyjson_read_file(json_in, 0, NULL, &rerr0);
      if (d0) {
        yyjson_val* root0 = yyjson_doc_get_root(d0);
        yyjson_val* fns0 = yyjson_obj_get(root0, "functions");
        yyjson_val* fn00 = hako_llvmc_find_entry_function(fns0);
        yyjson_val* blocks0 = fn00 && yyjson_is_obj(fn00) ? yyjson_obj_get(fn00, "blocks") : NULL;
        long long ret_const = 0; int have_ret = 0;
        if (blocks0 && yyjson_is_arr(blocks0) && yyjson_arr_size(blocks0) == 1) {
          // Accept only the honest single-block const/ret shape here.
          long long const_map_id[64]; long long const_map_val[64]; size_t const_n=0;
          int shape_ok = 1;
          yyjson_val* b = yyjson_arr_get_first(blocks0);
          yyjson_val* insts = b ? yyjson_obj_get(b, "instructions") : NULL;
          if (insts && yyjson_is_arr(insts)) {
            size_t ilen = yyjson_arr_size(insts);
            for (size_t ii=0; ii<ilen; ii++) {
              yyjson_val* ins = yyjson_arr_get(insts, ii);
              const char* op = yyjson_get_str(yyjson_obj_get(ins, "op")); if (!op) continue;
              if (ii + 1 == ilen) {
                if (strcmp(op, "ret") != 0) {
                  shape_ok = 0;
                  break;
                }
                yyjson_val* v = yyjson_obj_get(ins, "value");
                long long vid = v ? (long long)yyjson_get_sint(v) : 0;
                for (size_t k=0;k<const_n;k++){
                  if (const_map_id[k]==vid){
                    ret_const = const_map_val[k];
                    have_ret = 1;
                    break;
                  }
                }
                if (!have_ret) {
                  shape_ok = 0;
                }
                continue;
              }
              if (strcmp(op, "const")==0) {
                yyjson_val* dst = yyjson_obj_get(ins, "dst");
                yyjson_val* vv = yyjson_obj_get(yyjson_obj_get(ins, "value"), "value");
                if (!(dst && vv && yyjson_is_num(vv) && const_n < 64)) {
                  shape_ok = 0;
                  break;
                }
                const_map_id[const_n] = (long long)yyjson_get_sint(dst);
                const_map_val[const_n] = (long long)yyjson_get_sint(vv);
                const_n++;
              } else {
                shape_ok = 0;
                break;
              }
            }
          } else {
            shape_ok = 0;
          }
          if (!shape_ok) have_ret = 0;
        }
        if (have_ret) {
          hako_llvmc_entry_rune_selection selection;
          char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_ret_%d.ll", "/tmp", (int)getpid());
          FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(d0); return set_err_owned(err_out, "failed to open .ll"); }
          hako_llvmc_entry_rune_selection_from_json_file(json_in, &selection);
          fprintf(f, "; nyash minimal pure IR (ret const)\n");
          fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
          hako_llvmc_emit_entry_header(f, &selection);
          fprintf(f, "  ret i64 %lld\n", ret_const);
          fprintf(f, "}\n");
          hako_llvmc_emit_entry_alias_if_needed(f, &selection);
          fclose(f);
          char cmd[2048]; hako_llvmc_maybe_dump_ir_file(llpath); snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
          int rc = system(cmd); remove(llpath); yyjson_doc_free(d0);
          if (rc == 0) return 0;
          // else continue to try pattern #2 or fallback
        } else {
          yyjson_doc_free(d0);
        }
      }
    }

    // Try minimal pure path #2: recognize simple If (const/compare/branch + two const blocks + merge ret)
    // Parse JSON quickly via yyjson and synthesize IR when possible.
    yyjson_read_err rerr; yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
    if (!doc) {
      return set_err_owned(err_out, "json read failed");
    }
    yyjson_val* root = yyjson_doc_get_root(doc);
    yyjson_val* fns = yyjson_obj_get(root, "functions");
    yyjson_val* fn0 = hako_llvmc_find_entry_function(fns);
    yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
    if (blocks && yyjson_is_arr(blocks) && yyjson_arr_size(blocks) >= 3) {
      // Expect block0: const a, const b, compare Lt, branch then=t else=e
      yyjson_val* b0 = yyjson_arr_get_first(blocks);
      yyjson_val* i0 = b0 ? yyjson_obj_get(b0, "instructions") : NULL;
      if (i0 && yyjson_is_arr(i0) && yyjson_arr_size(i0) >= 4) {
        yyjson_val* ins0 = yyjson_arr_get(i0, 0);
        yyjson_val* ins1 = yyjson_arr_get(i0, 1);
        yyjson_val* ins2 = yyjson_arr_get(i0, 2);
        yyjson_val* ins3 = yyjson_arr_get(i0, 3);
        const char *op0 = yyjson_get_str(yyjson_obj_get(ins0, "op"));
        const char *op1 = yyjson_get_str(yyjson_obj_get(ins1, "op"));
        const char *op2 = yyjson_get_str(yyjson_obj_get(ins2, "op"));
        const char *op3 = yyjson_get_str(yyjson_obj_get(ins3, "op"));
        if (op0 && op1 && op2 && op3 &&
            strcmp(op0,"const")==0 &&
            strcmp(op1,"const")==0 &&
            strcmp(op2,"compare")==0 &&
            strcmp(op3,"branch")==0) {
          yyjson_val* v0 = yyjson_obj_get(yyjson_obj_get(ins0, "value"), "value");
          yyjson_val* v1 = yyjson_obj_get(yyjson_obj_get(ins1, "value"), "value");
          long long c0 = v0 ? (long long)yyjson_get_sint(v0) : 0;
          long long c1 = v1 ? (long long)yyjson_get_sint(v1) : 0;
          const char* cmp = yyjson_get_str(yyjson_obj_get(ins2, "cmp"));
          const char* pred = NULL;
          if (cmp) {
            if (strcmp(cmp,"Lt")==0 || strcmp(cmp,"lt")==0) pred = "slt";
            else if (strcmp(cmp,"Le")==0 || strcmp(cmp,"LE")==0 || strcmp(cmp,"le")==0) pred = "sle";
            else if (strcmp(cmp,"Eq")==0 || strcmp(cmp,"eq")==0) pred = "eq";
            else if (strcmp(cmp,"Ne")==0 || strcmp(cmp,"ne")==0) pred = "ne";
            else if (strcmp(cmp,"Ge")==0 || strcmp(cmp,"ge")==0) pred = "sge";
            else if (strcmp(cmp,"Gt")==0 || strcmp(cmp,"gt")==0) pred = "sgt";
          }
          if (pred) {
            int then_id = (int)yyjson_get_sint(yyjson_obj_get(ins3, "then"));
            int else_id = (int)yyjson_get_sint(yyjson_obj_get(ins3, "else"));
            yyjson_val* b_then = NULL;
            yyjson_val* b_else = NULL;
            yyjson_val* b_merge = NULL;
            size_t blen = yyjson_arr_size(blocks);
            for (size_t i=0;i<blen;i++) {
              yyjson_val* bi = yyjson_arr_get(blocks, i);
              int bid = (int)yyjson_get_sint(yyjson_obj_get(bi, "id"));
              if (bid == then_id) b_then = bi;
              else if (bid == else_id) b_else = bi;
            }
            for (size_t i=0;i<blen;i++) {
              yyjson_val* bi = yyjson_arr_get(blocks, i);
              yyjson_val* insts = yyjson_obj_get(bi, "instructions");
              size_t ilen = insts && yyjson_is_arr(insts) ? yyjson_arr_size(insts) : 0;
              for (size_t k=0;k<ilen;k++) {
                yyjson_val* ins = yyjson_arr_get(insts, k);
                const char* op = yyjson_get_str(yyjson_obj_get(ins, "op"));
                if (op && strcmp(op, "ret")==0) { b_merge = bi; break; }
              }
              if (b_merge) break;
            }
            if (b_then && b_else && b_merge) {
              yyjson_val* it = yyjson_obj_get(b_then, "instructions");
              yyjson_val* ie = yyjson_obj_get(b_else, "instructions");
              yyjson_val* t0 = it ? yyjson_arr_get(it, 0) : NULL;
              yyjson_val* e0 = ie ? yyjson_arr_get(ie, 0) : NULL;
              if (t0 && e0) {
                hako_llvmc_entry_rune_selection selection;
                yyjson_val* tv = yyjson_obj_get(yyjson_obj_get(t0, "value"), "value");
                yyjson_val* ev = yyjson_obj_get(yyjson_obj_get(e0, "value"), "value");
                long long then_const = tv ? (long long)yyjson_get_sint(tv) : 0;
                long long else_const = ev ? (long long)yyjson_get_sint(ev) : 0;
                char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_%d.ll", "/tmp", (int)getpid());
                FILE* f = fopen(llpath, "wb");
                if (!f) { yyjson_doc_free(doc); return set_err_owned(err_out, "failed to open .ll"); }
                hako_llvmc_select_entry_runes(fn0, &selection);
                fprintf(f, "; nyash minimal pure IR\n");
                fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
                hako_llvmc_emit_entry_header(f, &selection);
                fprintf(f, "bb0:\n");
                fprintf(f, "  %%cmp = icmp %s i64 %lld, %lld\n", pred, c0, c1);
                fprintf(f, "  br i1 %%cmp, label %%bb_then, label %%bb_else\n\n");
                fprintf(f, "bb_then:\n  br label %%bb_merge\n\n");
                fprintf(f, "bb_else:\n  br label %%bb_merge\n\n");
                fprintf(f, "bb_merge:\n  %%r = phi i64 [ %lld, %%bb_then ], [ %lld, %%bb_else ]\n  ret i64 %%r\n}\n", then_const, else_const);
                hako_llvmc_emit_entry_alias_if_needed(f, &selection);
                fclose(f);
                char cmd[2048]; hako_llvmc_maybe_dump_ir_file(llpath); snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
                int rc = system(cmd);
                remove(llpath);
                yyjson_doc_free(doc);
                if (rc != 0) {
                  if (compat_harness_replay_enabled()) {
                    return compile_json_via_explicit_compat_harness_replay(json_in, obj_out, err_out);
                  }
                  hako_llvmc_emit_route_replay("none", "unsupported_pure_shape");
                  return set_err_owned(err_out, "unsupported pure shape for current backend recipe");
                }
                return 0;
              }
            }
          }
        }
      }
    }
    yyjson_doc_free(doc);
    // Try minimal pure path #3: Map birth → set → size → ret
    {
      yyjson_read_err rerr; yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
      if (doc) {
        yyjson_val* root = yyjson_doc_get_root(doc);
        yyjson_val* fns = yyjson_obj_get(root, "functions");
        yyjson_val* fn0 = hako_llvmc_find_entry_function(fns);
        yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
        yyjson_val* b0 = blocks && yyjson_is_arr(blocks) ? yyjson_arr_get_first(blocks) : NULL;
        yyjson_val* insts = b0 ? yyjson_obj_get(b0, "instructions") : NULL;
        long long key_c = 0, val_c = 0; int have = 0;
        if (insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) >= 5) {
          // const, const, mir_call(Constructor:MapBox), mir_call(Method:set), mir_call(Method:size), ret
          yyjson_val* i0 = yyjson_arr_get(insts, 0);
          yyjson_val* i1 = yyjson_arr_get(insts, 1);
          yyjson_val* i2 = yyjson_arr_get(insts, 2);
          yyjson_val* i3 = yyjson_arr_get(insts, 3);
          yyjson_val* i4 = yyjson_arr_get(insts, 4);
          const char* op0 = yyjson_get_str(yyjson_obj_get(i0, "op"));
          const char* op1 = yyjson_get_str(yyjson_obj_get(i1, "op"));
          const char* op2 = yyjson_get_str(yyjson_obj_get(i2, "op"));
          const char* op3 = yyjson_get_str(yyjson_obj_get(i3, "op"));
          const char* op4 = yyjson_get_str(yyjson_obj_get(i4, "op"));
          if (op0 && op1 && op2 && op3 && op4 &&
              strcmp(op0, "const")==0 && strcmp(op1, "const")==0 &&
              strcmp(op2, "mir_call")==0 && strcmp(op3, "mir_call")==0 && strcmp(op4, "mir_call")==0) {
            yyjson_val* v0 = yyjson_obj_get(yyjson_obj_get(i0, "value"), "value");
            yyjson_val* v1 = yyjson_obj_get(yyjson_obj_get(i1, "value"), "value");
            key_c = v0 ? (long long)yyjson_get_sint(v0) : 0;
            val_c = v1 ? (long long)yyjson_get_sint(v1) : 0;
            // Constructor MapBox
            yyjson_val* mc2 = yyjson_obj_get(i2, "mir_call");
            yyjson_val* cal2 = mc2 ? yyjson_obj_get(mc2, "callee") : NULL;
            const char* ctype = cal2 ? yyjson_get_str(yyjson_obj_get(cal2, "type")) : NULL;
            const char* cname = cal2 ? yyjson_get_str(yyjson_obj_get(cal2, "name")) : NULL;
            // Method set/size
            yyjson_val* mc3 = yyjson_obj_get(i3, "mir_call");
            yyjson_val* cal3 = mc3 ? yyjson_obj_get(mc3, "callee") : NULL;
            const char* m3 = cal3 ? yyjson_get_str(yyjson_obj_get(cal3, "name")) : NULL;
            yyjson_val* mc4 = yyjson_obj_get(i4, "mir_call");
            yyjson_val* cal4 = mc4 ? yyjson_obj_get(mc4, "callee") : NULL;
            const char* m4 = cal4 ? yyjson_get_str(yyjson_obj_get(cal4, "name")) : NULL;
            if (ctype && cname && strcmp(ctype, "Constructor")==0 && strcmp(cname, "MapBox")==0 &&
                m3 && strcmp(m3, "set")==0 && m4 && (strcmp(m4, "size")==0 || strcmp(m4, "len")==0)) {
              have = 1;
            }
          }
        }
        if (have) {
          hako_llvmc_entry_rune_selection selection;
          char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_map_%d.ll", "/tmp", (int)getpid());
          FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(doc); return set_err_owned(err_out, "failed to open .ll"); }
          hako_llvmc_select_entry_runes(fn0, &selection);
          fprintf(f, "; nyash minimal pure IR (map set->size)\n");
          fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
          fprintf(f, "declare i64 @\"nyash.map.birth_h\"()\n");
          fprintf(f, "declare i64 @\"nyash.map.slot_store_hhh\"(i64, i64, i64)\n");
          fprintf(f, "declare i64 @\"nyash.map.entry_count_h\"(i64)\n\n");
          hako_llvmc_emit_entry_header(f, &selection);
          fprintf(f, "  %%h = call i64 @\"nyash.map.birth_h\"()\n");
          fprintf(f, "  %%_s = call i64 @\"nyash.map.slot_store_hhh\"(i64 %%h, i64 %lld, i64 %lld)\n", key_c, val_c);
          fprintf(f, "  %%sz = call i64 @\"nyash.map.entry_count_h\"(i64 %%h)\n");
          fprintf(f, "  ret i64 %%sz\n}\n");
          hako_llvmc_emit_entry_alias_if_needed(f, &selection);
          fclose(f);
          char cmd[2048]; hako_llvmc_maybe_dump_ir_file(llpath); snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
          int rc = system(cmd); remove(llpath); yyjson_doc_free(doc);
          if (rc == 0) return 0;
        } else {
          yyjson_doc_free(doc);
        }
      }
    }

    // Try minimal pure path #4: Array birth → push → len → ret
    {
      yyjson_read_err rerr; yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
      if (doc) {
        yyjson_val* root = yyjson_doc_get_root(doc);
        yyjson_val* fns = yyjson_obj_get(root, "functions");
        yyjson_val* fn0 = hako_llvmc_find_entry_function(fns);
        yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
        yyjson_val* b0 = blocks && yyjson_is_arr(blocks) ? yyjson_arr_get_first(blocks) : NULL;
        yyjson_val* insts = b0 ? yyjson_obj_get(b0, "instructions") : NULL;
        long long val_c = 0; int have = 0;
        if (insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) >= 4) {
          // const, mir_call(Constructor:ArrayBox), mir_call(Method:push), mir_call(Method:len/length/size), ret
          yyjson_val* i0 = yyjson_arr_get(insts, 0);
          yyjson_val* i1 = yyjson_arr_get(insts, 1);
          yyjson_val* i2 = yyjson_arr_get(insts, 2);
          yyjson_val* i3 = yyjson_arr_get(insts, 3);
          const char* op0 = yyjson_get_str(yyjson_obj_get(i0, "op"));
          const char* op1 = yyjson_get_str(yyjson_obj_get(i1, "op"));
          const char* op2 = yyjson_get_str(yyjson_obj_get(i2, "op"));
          const char* op3 = yyjson_get_str(yyjson_obj_get(i3, "op"));
          if (op0 && op1 && op2 && op3 && strcmp(op0, "const")==0 && strcmp(op1, "mir_call")==0 && strcmp(op2, "mir_call")==0 && strcmp(op3, "mir_call")==0) {
            yyjson_val* v0 = yyjson_obj_get(yyjson_obj_get(i0, "value"), "value");
            val_c = v0 ? (long long)yyjson_get_sint(v0) : 0;
            // Constructor ArrayBox
            yyjson_val* mc1 = yyjson_obj_get(i1, "mir_call");
            yyjson_val* cal1 = mc1 ? yyjson_obj_get(mc1, "callee") : NULL;
            const char* ctype = cal1 ? yyjson_get_str(yyjson_obj_get(cal1, "type")) : NULL;
            const char* cname = cal1 ? yyjson_get_str(yyjson_obj_get(cal1, "name")) : NULL;
            // Method push/len
            yyjson_val* mc2 = yyjson_obj_get(i2, "mir_call");
            yyjson_val* cal2 = mc2 ? yyjson_obj_get(mc2, "callee") : NULL;
            const char* m2 = cal2 ? yyjson_get_str(yyjson_obj_get(cal2, "name")) : NULL;
            yyjson_val* mc3 = yyjson_obj_get(i3, "mir_call");
            yyjson_val* cal3 = mc3 ? yyjson_obj_get(mc3, "callee") : NULL;
            const char* m3 = cal3 ? yyjson_get_str(yyjson_obj_get(cal3, "name")) : NULL;
            if (ctype && cname && strcmp(ctype, "Constructor")==0 && strcmp(cname, "ArrayBox")==0 &&
                m2 && strcmp(m2, "push")==0 && m3 && (strcmp(m3, "len")==0 || strcmp(m3, "length")==0 || strcmp(m3, "size")==0)) {
              have = 1;
            }
          }
        }
        if (have) {
          hako_llvmc_entry_rune_selection selection;
          char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_array_%d.ll", "/tmp", (int)getpid());
          FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(doc); return set_err_owned(err_out, "failed to open .ll"); }
          hako_llvmc_select_entry_runes(fn0, &selection);
          fprintf(f, "; nyash minimal pure IR (array push->len)\n");
          fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
          fprintf(f, "declare i64 @\"nyash.array.birth_h\"()\n");
          fprintf(f, "declare i64 @\"nyash.array.slot_append_hh\"(i64, i64)\n");
          fprintf(f, "declare i64 @\"nyash.array.slot_len_h\"(i64)\n\n");
          hako_llvmc_emit_entry_header(f, &selection);
          fprintf(f, "  %%h = call i64 @\"nyash.array.birth_h\"()\n");
          fprintf(f, "  %%_p = call i64 @\"nyash.array.slot_append_hh\"(i64 %%h, i64 %lld)\n", val_c);
          fprintf(f, "  %%len = call i64 @\"nyash.array.slot_len_h\"(i64 %%h)\n");
          fprintf(f, "  ret i64 %%len\n}\n");
          hako_llvmc_emit_entry_alias_if_needed(f, &selection);
          fclose(f);
          char cmd[2048]; hako_llvmc_maybe_dump_ir_file(llpath); snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
          int rc = system(cmd); remove(llpath); yyjson_doc_free(doc);
          if (rc == 0) return 0;
        } else {
          yyjson_doc_free(doc);
        }
      }
    }

    // Try minimal pure path #5: const ASCII string handle -> newbox StringBox ->
    // StringBox/RuntimeDataBox length/size -> ret
    {
      yyjson_read_err rerr; yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
      if (doc) {
        yyjson_val* root = yyjson_doc_get_root(doc);
        yyjson_val* fns = yyjson_obj_get(root, "functions");
        yyjson_val* fn0 = hako_llvmc_find_entry_function(fns);
        yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
        yyjson_val* b0 = blocks && yyjson_is_arr(blocks) ? yyjson_arr_get_first(blocks) : NULL;
        yyjson_val* insts = b0 ? yyjson_obj_get(b0, "instructions") : NULL;
        long long lit_len = 0;
        int have = 0;
        if (insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) >= 4) {
          yyjson_val* i0 = yyjson_arr_get(insts, 0);
          yyjson_val* i1 = yyjson_arr_get(insts, 1);
          yyjson_val* i2 = yyjson_arr_get(insts, 2);
          yyjson_val* i3 = yyjson_arr_get(insts, 3);
          const char* op0 = yyjson_get_str(yyjson_obj_get(i0, "op"));
          const char* op1 = yyjson_get_str(yyjson_obj_get(i1, "op"));
          const char* op2 = yyjson_get_str(yyjson_obj_get(i2, "op"));
          const char* op3 = yyjson_get_str(yyjson_obj_get(i3, "op"));
          if (op0 && op1 && op2 && op3 &&
              strcmp(op0, "const") == 0 &&
              strcmp(op1, "newbox") == 0 &&
              strcmp(op2, "mir_call") == 0 &&
              strcmp(op3, "ret") == 0) {
            long long lit_dst = (long long)yyjson_get_sint(yyjson_obj_get(i0, "dst"));
            long long box_dst = (long long)yyjson_get_sint(yyjson_obj_get(i1, "dst"));
            long long call_dst = (long long)yyjson_get_sint(yyjson_obj_get(i2, "dst"));
            long long ret_val = (long long)yyjson_get_sint(yyjson_obj_get(i3, "value"));
            const char* box_type = yyjson_get_str(yyjson_obj_get(i1, "type"));
            yyjson_val* val0 = yyjson_obj_get(yyjson_obj_get(i0, "value"), "value");
            const char* lit = val0 ? yyjson_get_str(val0) : NULL;
            yyjson_val* nb_args = yyjson_obj_get(i1, "args");
            yyjson_val* nb0 = nb_args && yyjson_is_arr(nb_args) ? yyjson_arr_get_first(nb_args) : NULL;
            long long arg0 = nb0 ? (long long)yyjson_get_sint(nb0) : 0;
            yyjson_val* mc = yyjson_obj_get(i2, "mir_call");
            yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
            const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
            const char* bname = cal ? (yyjson_get_str(yyjson_obj_get(cal, "box_name")) ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : yyjson_get_str(yyjson_obj_get(cal, "box_type"))) : NULL;
            const char* mname = cal ? (yyjson_get_str(yyjson_obj_get(cal, "method")) ? yyjson_get_str(yyjson_obj_get(cal, "method")) : yyjson_get_str(yyjson_obj_get(cal, "name"))) : NULL;
            long long recv = cal ? (long long)yyjson_get_sint(yyjson_obj_get(cal, "receiver")) : 0;
            yyjson_val* call_args = mc ? yyjson_obj_get(mc, "args") : NULL;
            size_t argc = call_args && yyjson_is_arr(call_args) ? yyjson_arr_size(call_args) : 0;
            if (lit &&
                hako_llvmc_ascii_strlen(lit, &lit_len) &&
                box_type && strcmp(box_type, "StringBox") == 0 &&
                lit_dst != 0 &&
                arg0 == lit_dst &&
                ctype && strcmp(ctype, "Method") == 0 &&
                bname && (strcmp(bname, "StringBox") == 0 || strcmp(bname, "RuntimeDataBox") == 0) &&
                mname && (strcmp(mname, "length") == 0 || strcmp(mname, "size") == 0) &&
                recv == box_dst &&
                argc == 0 &&
                call_dst != 0 &&
                ret_val == call_dst) {
              have = 1;
            }
          }
        }
        if (have) {
          hako_llvmc_entry_rune_selection selection;
          char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_string_len_%d.ll", "/tmp", (int)getpid());
          FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(doc); return set_err_owned(err_out, "failed to open .ll"); }
          hako_llvmc_select_entry_runes(fn0, &selection);
          fprintf(f, "; nyash minimal pure IR (string length const)\n");
          fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
          hako_llvmc_emit_entry_header(f, &selection);
          fprintf(f, "  ret i64 %lld\n", lit_len);
          fprintf(f, "}\n");
          hako_llvmc_emit_entry_alias_if_needed(f, &selection);
          fclose(f);
          char cmd[2048]; hako_llvmc_maybe_dump_ir_file(llpath); snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
          int rc = system(cmd); remove(llpath); yyjson_doc_free(doc);
          if (rc == 0) return 0;
        } else {
          yyjson_doc_free(doc);
        }
      }
    }

    // Try minimal pure path #6: const ASCII string handle -> newbox StringBox ->
    // const ASCII needle -> newbox StringBox -> StringBox.indexOf(needle) -> ret
    {
      yyjson_read_err rerr; yyjson_doc* doc = yyjson_read_file(json_in, 0, NULL, &rerr);
      if (doc) {
        yyjson_val* root = yyjson_doc_get_root(doc);
        yyjson_val* fns = yyjson_obj_get(root, "functions");
        yyjson_val* fn0 = hako_llvmc_find_entry_function(fns);
        yyjson_val* blocks = fn0 && yyjson_is_obj(fn0) ? yyjson_obj_get(fn0, "blocks") : NULL;
        yyjson_val* b0 = blocks && yyjson_is_arr(blocks) ? yyjson_arr_get_first(blocks) : NULL;
        yyjson_val* insts = b0 ? yyjson_obj_get(b0, "instructions") : NULL;
        long long found_idx = -1;
        int have = 0;
        if (insts && yyjson_is_arr(insts) && yyjson_arr_size(insts) >= 6) {
          yyjson_val* i0 = yyjson_arr_get(insts, 0);
          yyjson_val* i1 = yyjson_arr_get(insts, 1);
          yyjson_val* i2 = yyjson_arr_get(insts, 2);
          yyjson_val* i3 = yyjson_arr_get(insts, 3);
          yyjson_val* i4 = yyjson_arr_get(insts, 4);
          yyjson_val* i5 = yyjson_arr_get(insts, 5);
          const char* op0 = yyjson_get_str(yyjson_obj_get(i0, "op"));
          const char* op1 = yyjson_get_str(yyjson_obj_get(i1, "op"));
          const char* op2 = yyjson_get_str(yyjson_obj_get(i2, "op"));
          const char* op3 = yyjson_get_str(yyjson_obj_get(i3, "op"));
          const char* op4 = yyjson_get_str(yyjson_obj_get(i4, "op"));
          const char* op5 = yyjson_get_str(yyjson_obj_get(i5, "op"));
          if (op0 && op1 && op2 && op3 && op4 && op5 &&
              strcmp(op0, "const") == 0 &&
              strcmp(op1, "newbox") == 0 &&
              strcmp(op2, "const") == 0 &&
              strcmp(op3, "newbox") == 0 &&
              strcmp(op4, "mir_call") == 0 &&
              strcmp(op5, "ret") == 0) {
            long long hay_dst = (long long)yyjson_get_sint(yyjson_obj_get(i0, "dst"));
            long long hay_box_dst = (long long)yyjson_get_sint(yyjson_obj_get(i1, "dst"));
            long long needle_dst = (long long)yyjson_get_sint(yyjson_obj_get(i2, "dst"));
            long long needle_box_dst = (long long)yyjson_get_sint(yyjson_obj_get(i3, "dst"));
            long long call_dst = (long long)yyjson_get_sint(yyjson_obj_get(i4, "dst"));
            long long ret_val = (long long)yyjson_get_sint(yyjson_obj_get(i5, "value"));
            const char* box_type1 = yyjson_get_str(yyjson_obj_get(i1, "type"));
            const char* box_type3 = yyjson_get_str(yyjson_obj_get(i3, "type"));
            yyjson_val* hay_val = yyjson_obj_get(yyjson_obj_get(i0, "value"), "value");
            yyjson_val* needle_val = yyjson_obj_get(yyjson_obj_get(i2, "value"), "value");
            const char* hay = hay_val ? yyjson_get_str(hay_val) : NULL;
            const char* needle = needle_val ? yyjson_get_str(needle_val) : NULL;
            yyjson_val* nb1_args = yyjson_obj_get(i1, "args");
            yyjson_val* nb3_args = yyjson_obj_get(i3, "args");
            yyjson_val* nb1_0 = nb1_args && yyjson_is_arr(nb1_args) ? yyjson_arr_get_first(nb1_args) : NULL;
            yyjson_val* nb3_0 = nb3_args && yyjson_is_arr(nb3_args) ? yyjson_arr_get_first(nb3_args) : NULL;
            long long arg1_0 = nb1_0 ? (long long)yyjson_get_sint(nb1_0) : 0;
            long long arg3_0 = nb3_0 ? (long long)yyjson_get_sint(nb3_0) : 0;
            yyjson_val* mc = yyjson_obj_get(i4, "mir_call");
            yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
            const char* ctype = cal ? yyjson_get_str(yyjson_obj_get(cal, "type")) : NULL;
            const char* bname = cal ? (yyjson_get_str(yyjson_obj_get(cal, "box_name")) ? yyjson_get_str(yyjson_obj_get(cal, "box_name")) : yyjson_get_str(yyjson_obj_get(cal, "box_type"))) : NULL;
            const char* mname = cal ? (yyjson_get_str(yyjson_obj_get(cal, "method")) ? yyjson_get_str(yyjson_obj_get(cal, "method")) : yyjson_get_str(yyjson_obj_get(cal, "name"))) : NULL;
            long long recv = cal ? (long long)yyjson_get_sint(yyjson_obj_get(cal, "receiver")) : 0;
            yyjson_val* call_args = mc ? yyjson_obj_get(mc, "args") : NULL;
            yyjson_val* call_arg0 = call_args && yyjson_is_arr(call_args) ? yyjson_arr_get_first(call_args) : NULL;
            size_t argc = call_args && yyjson_is_arr(call_args) ? yyjson_arr_size(call_args) : 0;
            long long call_arg0_v = call_arg0 ? (long long)yyjson_get_sint(call_arg0) : 0;
            if (hay &&
                needle &&
                hako_llvmc_ascii_index_of(hay, needle, &found_idx) &&
                box_type1 && strcmp(box_type1, "StringBox") == 0 &&
                box_type3 && strcmp(box_type3, "StringBox") == 0 &&
                hay_dst != 0 &&
                needle_dst != 0 &&
                arg1_0 == hay_dst &&
                arg3_0 == needle_dst &&
                ctype && strcmp(ctype, "Method") == 0 &&
                bname && strcmp(bname, "StringBox") == 0 &&
                mname && strcmp(mname, "indexOf") == 0 &&
                recv == hay_box_dst &&
                argc == 1 &&
                call_arg0_v == needle_box_dst &&
                call_dst != 0 &&
                ret_val == call_dst) {
              have = 1;
            }
          }
        }
        if (have) {
          hako_llvmc_entry_rune_selection selection;
          char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_string_indexof_%d.ll", "/tmp", (int)getpid());
          FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(doc); return set_err_owned(err_out, "failed to open .ll"); }
          hako_llvmc_select_entry_runes(fn0, &selection);
          fprintf(f, "; nyash minimal pure IR (string indexOf const)\n");
          fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
          hako_llvmc_emit_entry_header(f, &selection);
          fprintf(f, "  ret i64 %lld\n", found_idx);
          fprintf(f, "}\n");
          hako_llvmc_emit_entry_alias_if_needed(f, &selection);
          fclose(f);
          char cmd[2048]; hako_llvmc_maybe_dump_ir_file(llpath); snprintf(cmd, sizeof(cmd), "llc -filetype=obj -o \"%s\" \"%s\" 2>/dev/null", obj_out, llpath);
          int rc = system(cmd); remove(llpath); yyjson_doc_free(doc);
          if (rc == 0) return 0;
        } else {
          yyjson_doc_free(doc);
        }
      }
  }
  if (compat_harness_replay_enabled()) {
    return compile_json_via_explicit_compat_harness_replay(json_in, obj_out, err_out);
  }
  hako_llvmc_emit_route_replay("none", "unsupported_pure_shape");
  return set_err_owned(err_out, "unsupported pure shape for current backend recipe");
}

__attribute__((visibility("default")))
int hako_llvmc_link_obj(const char* obj_in, const char* exe_out, const char* extra_ldflags, char** err_out) {
  return forward_link_obj_to_aot(obj_in, exe_out, extra_ldflags, err_out);
}
