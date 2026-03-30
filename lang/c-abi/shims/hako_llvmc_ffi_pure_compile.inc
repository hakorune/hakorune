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
  if (hako_llvmc_match_indexof_leaf_ascii_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (!hako_llvmc_skip_indexof_line_seed_enabled() &&
      hako_llvmc_match_indexof_line_ascii_seed(json_in, obj_out) == 0) {
    return 0;
  }
  if (hako_llvmc_match_array_getset_micro_seed(json_in, obj_out) == 0) {
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
      struct { long long reg; } str_const_box_skip[512]; size_t str_const_box_skip_n = 0;
      struct { long long reg; int ty; } types[2048]; size_t types_n = 0;
      struct { long long reg; long long src; } copy_alias[2048]; size_t copy_alias_n = 0;
      struct { long long reg; int kind; } scan_origin[2048]; size_t scan_origin_n = 0;
      // Track simple origin kinds for selective unbox at ret
      enum { ORG_NONE=0, ORG_MAP_GET=1, ORG_MAP_BIRTH=2, ORG_ARRAY_BIRTH=3, ORG_STRING=4, ORG_ARRAY_STRING_BIRTH=5 };
      struct { long long reg; int kind; } origin[2048]; size_t origin_n = 0;
      struct Incoming { long long pred; long long val_reg; };
      struct PhiRec { long long dst; struct Incoming in[16]; int in_n; };
      struct BlockPhi { long long bid; struct PhiRec recs[16]; int rec_n; } phis[512]; int phi_n = 0;

      // small helpers
      #define ARR_LEN(a) ((int)(sizeof(a)/sizeof((a)[0])))
      auto long long read_int(yyjson_val* obj, const char* key);
      auto const char* read_str(yyjson_val* obj, const char* key);
      #include "hako_llvmc_ffi_compiler_state.inc"
      auto struct BlockPhi* ensure_phi_block(long long bid){ for (int i=0;i<phi_n;i++){ if (phis[i].bid==bid) return &phis[i]; } if (phi_n<512){ phis[phi_n].bid=bid; phis[phi_n].rec_n=0; return &phis[phi_n++]; } return NULL; }
      auto long long read_int(yyjson_val* obj, const char* key){ yyjson_val* v= yyjson_obj_get(obj,key); return v? (long long)yyjson_get_sint(v) : 0; }
      auto const char* read_str(yyjson_val* obj, const char* key){ yyjson_val* v= yyjson_obj_get(obj,key); return v? yyjson_get_str(v) : NULL; }
      int array_rmw_trace_budget = 8;
      int array_string_len_trace_budget = 8;
      int generic_walk_trace_budget = 24;
      int generic_call_trace_budget = 24;
      int generic_method_trace_budget = 24;
      #include "hako_llvmc_ffi_indexof_observer_state.inc"
      #include "hako_llvmc_ffi_indexof_observer_direct_match.inc"
      #include "hako_llvmc_ffi_indexof_observer_block_match.inc"
      #include "hako_llvmc_ffi_indexof_observer_trace.inc"
      #include "hako_llvmc_ffi_indexof_observer_lowering.inc"
      #include "hako_llvmc_ffi_string_chain_terms.inc"
      #include "hako_llvmc_ffi_mir_call_surface_policy.inc"
      #include "hako_llvmc_ffi_string_concat_lowering.inc"
      #include "hako_llvmc_ffi_string_chain_policy.inc"
      #include "hako_llvmc_ffi_const_string_hoist.inc"
      #include "hako_llvmc_ffi_mir_call_route_policy.inc"
      #include "hako_llvmc_ffi_mir_call_need_policy.inc"
      #include "hako_llvmc_ffi_mir_call_prepass.inc"
      #include "hako_llvmc_ffi_mir_call_shell.inc"
      #define HAKO_LLVM_READONLY_ATTRS " nounwind readonly willreturn"

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
              if (should_skip_string_const_box_for_concat_suffix(ii, ilen, insts, dst)) {
                mark_hoisted_string_const_box_skip(dst);
              } else {
                needs.string_const_box = 1;
              }
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
            } else if (tname && strcmp(tname, "MapBox") == 0) {
              needs.map_birth = 1;
              if (dst) {
                set_scan_origin(dst, ORG_MAP_BIRTH);
                set_origin(dst, ORG_MAP_BIRTH);
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
            int src_ty = get_type(src);
            int src_org = get_origin(src);
            int scan_org = get_scan_origin(src);
            if (dst && src) set_copy_alias(dst, resolve_copy_source_reg(src));
            if (dst && src_ty != T_NONE) set_type(dst, src_ty);
            if (dst && src_org != ORG_NONE) set_origin(dst, src_org);
            if (scan_org != ORG_NONE && dst) set_scan_origin(dst, scan_org);
          } else if (strcmp(op, "compare")==0) {
            long long dst = read_int(ins, "dst");
            if (dst) set_type(dst, T_I1);
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
              int phi_bool_ok = 1;
              int phi_bool_seen = 0;
              int scan_first = ORG_NONE;
              int scan_same = 1;
              int origin_first = ORG_NONE;
              int origin_same = 1;
              for (int i = 0; i < pr.in_n; i++) {
                int bool_const = 0;
                int t = pr.in[i].val_reg ? get_type(pr.in[i].val_reg) : T_NONE;
                int s = pr.in[i].val_reg ? get_scan_origin(pr.in[i].val_reg) : ORG_NONE;
                int o = pr.in[i].val_reg ? get_origin(pr.in[i].val_reg) : ORG_NONE;
                if (t == T_I1) {
                  phi_bool_seen = 1;
                } else if (pr.in[i].val_reg && has_bool_const(pr.in[i].val_reg, &bool_const)) {
                  phi_bool_seen = 1;
                } else {
                  phi_bool_ok = 0;
                }
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
              set_type(pr.dst, (phi_bool_ok && phi_bool_seen) ? T_I1 : T_I64);
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
      if (needs.map_get)   fprintf(f, "declare i64 @\"nyash.map.slot_load_hh\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.map_has)   fprintf(f, "declare i64 @\"nyash.map.probe_hh\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.map_size)  fprintf(f, "declare i64 @\"nyash.map.entry_count_h\"(i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      fprintf(f, "declare i64 @\"nyash.runtime_data.get_hh\"(i64, i64)\n");
      fprintf(f, "declare i64 @\"nyash.runtime_data.set_hhh\"(i64, i64, i64)\n");
      fprintf(f, "declare i64 @\"nyash.runtime_data.has_hh\"(i64, i64)\n");
      fprintf(f, "declare i64 @\"nyash.runtime_data.push_hh\"(i64, i64)\n");
      if (needs.arr_birth) fprintf(f, "declare i64 @\"nyash.array.birth_h\"()\n");
      if (needs.arr_push)  fprintf(f, "declare i64 @\"nyash.array.slot_append_hh\"(i64, i64)\n");
      if (needs.arr_len)   fprintf(f, "declare i64 @\"nyash.array.slot_len_h\"(i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.arr_set)   fprintf(f, "declare i64 @\"nyash.array.slot_store_hih\"(i64, i64, i64)\n");
      if (needs.arr_set)   fprintf(f, "declare i64 @\"nyash.array.set_his\"(i64, i64, i64)\n");
      if (needs.arr_set)   fprintf(f, "declare i64 @\"nyash.array.slot_store_hii\"(i64, i64, i64)\n");
      if (needs.arr_get && needs.arr_set) fprintf(f, "declare i64 @\"nyash.array.rmw_add1_hi\"(i64, i64)\n");
      if (needs.arr_get || needs.arr_string_indexof || needs.string_len) fprintf(f, "declare i64 @\"nyash.array.string_len_hi\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.arr_get || needs.arr_string_indexof) fprintf(f, "declare i64 @\"nyash.array.string_indexof_hih\"(i64, i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.arr_get)   fprintf(f, "declare i64 @\"nyash.array.slot_load_hi\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.string_const_box) fprintf(f, "declare i64 @\"nyash.box.from_i8_string_const\"(ptr)\n");
      if (needs.string_len) fprintf(f, "declare i64 @\"nyash.string.len_h\"(i64)\n");
      if (needs.string_substring) fprintf(f, "declare i64 @\"nyash.string.substring_hii\"(i64, i64, i64)\n");
      if (needs.string_indexof) fprintf(f, "declare i64 @\"nyash.string.indexOf_hh\"(i64, i64)\n");
      if (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof) {
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
      #include "hako_llvmc_ffi_emit_seam.inc"
      #include "hako_llvmc_ffi_generic_method_lowering.inc"
      #include "hako_llvmc_ffi_mir_call_dispatch.inc"
      auto void emit_block_label(long long bid){ EMIT("bb%lld:\n", bid); }
      auto void emit_phi(long long dst, struct Incoming* in, int in_n){
        int phi_ty = get_type(dst);
        EMIT("  %%r%lld = phi %s ", dst, phi_ty == T_I1 ? "i1" : "i64");
        for (int i=0;i<in_n;i++){
          long long cv;
          long long val_reg = resolve_copy_source_reg(in[i].val_reg);
          int hc=has_const(val_reg,&cv);
          if (phi_ty == T_I1) {
            if (hc) EMIT("[ %s, %%bb%lld ]%s", cv ? "true" : "false", in[i].pred, (i+1<in_n)?", ":"");
            else EMIT("[ %%r%lld, %%bb%lld ]%s", val_reg, in[i].pred, (i+1<in_n)?", ":"");
          } else {
            EMIT("[ %s%lld%s, %%bb%lld ]%s", hc?"":"%r", hc?cv:val_reg, hc?"":"", in[i].pred, (i+1<in_n)?", ":"");
          }
        }
        EMIT("\n");
      }
      auto void emit_icmp(long long dst, const char* pred, long long lhs_reg, long long rhs_reg){ long long vL,vR; long long lhs_base=resolve_copy_source_reg(lhs_reg), rhs_base=resolve_copy_source_reg(rhs_reg); int lc=has_const(lhs_base,&vL), rc=has_const(rhs_base,&vR); EMIT("  %%r%lld = icmp %s i64 %s%lld%s, %s%lld%s\n", dst, pred, lc?"":"%r", lc?vL:lhs_base, lc?"":"", rc?"":"%r", rc?vR:rhs_base, rc?"":""); set_type(dst,T_I1); }

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
            if (tname && strcmp(tname, "MapBox") == 0) {
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.map.birth_h\"()\n", dst);
                set_type(dst, T_I64);
                set_origin(dst, ORG_MAP_BIRTH);
                set_scan_origin(dst, ORG_MAP_BIRTH);
              } else {
                EMIT("  call i64 @\"nyash.map.birth_h\"()\n");
              }
              continue;
            }
            if (tname && strcmp(tname, "StringBox") == 0) {
              yyjson_val* nb_args = yyjson_obj_get(ins, "args");
              yyjson_val* nb0 = nb_args && yyjson_is_arr(nb_args) ? yyjson_arr_get_first(nb_args) : NULL;
              long long arg0 = nb0 ? (long long)yyjson_get_sint(nb0) : 0;
              if (dst && arg0) {
                set_copy_alias(dst, resolve_copy_source_reg(arg0));
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
            set_copy_alias(dst, resolve_copy_source_reg(src));
            if (has_const(src, &cv)) {
              put_const(dst, cv);
              set_type(dst, T_I64);
            } else if (src_ty == T_I1) {
              set_type(dst, T_I1);
            } else {
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
            emit_icmp(dst, p2, lhs, rhs);
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
            long long cond_base = resolve_copy_source_reg(cond);
            long long then_base = resolve_copy_source_reg(then_val);
            long long else_base = resolve_copy_source_reg(else_val);
            int cond_const = has_const(cond_base, &cond_cv);
            int then_const = has_const(then_base, &then_cv);
            int else_const = has_const(else_base, &else_cv);
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
              else snprintf(cond_ref, sizeof(cond_ref), "%%r%lld", cond_base);
            } else if (cond_const) {
              EMIT("  %%t%lld = icmp ne i64 %lld, 0\n", dst, cond_cv);
              snprintf(cond_ref, sizeof(cond_ref), "%%t%lld", dst);
            } else {
              EMIT("  %%t%lld = icmp ne i64 %%r%lld, 0\n", dst, cond_base);
              snprintf(cond_ref, sizeof(cond_ref), "%%t%lld", dst);
            }

            if (val_ty == T_I1) {
              if (then_const) snprintf(then_ref, sizeof(then_ref), "%s", then_cv ? "true" : "false");
              else snprintf(then_ref, sizeof(then_ref), "%%r%lld", then_base);
              if (else_const) snprintf(else_ref, sizeof(else_ref), "%s", else_cv ? "true" : "false");
              else snprintf(else_ref, sizeof(else_ref), "%%r%lld", else_base);
              EMIT("  %%r%lld = select i1 %s, i1 %s, i1 %s\n",
                   dst,
                   cond_ref,
                   then_ref,
                   else_ref);
              set_type(dst, T_I1);
            } else {
              if (then_const) snprintf(then_ref, sizeof(then_ref), "%lld", then_cv);
              else snprintf(then_ref, sizeof(then_ref), "%%r%lld", then_base);
              if (else_const) snprintf(else_ref, sizeof(else_ref), "%lld", else_cv);
              else snprintf(else_ref, sizeof(else_ref), "%%r%lld", else_base);
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
          if (strcmp(op, "branch")==0) { long long cond=read_int(ins, "cond"); long long th=read_int(ins,"then"); long long el=read_int(ins,"else"); if(!el) el=read_int(ins,"else_id"); emit_branch(cond, th, el); continue; }
          if (strcmp(op, "jump")==0) { long long tgt=read_int(ins, "target"); EMIT("  br label %%bb%lld\n", tgt); continue; }
          if (strcmp(op, "ret")==0) {
            long long v=read_int(ins, "value"); long long cv;
            if (has_const(v,&cv)) { EMIT("  ret i64 %lld\n", cv); }
            else {
              int org = get_origin(v);
              long long base = resolve_copy_source_reg(v);
              if (org == ORG_MAP_GET) {
                // Auto-unbox map.get result as Integer (minimal MVP)
                EMIT("  %%map_get_unbox_%lld = call i64 @\"nyash.integer.get_h\"(i64 %%r%lld)\n", v, base);
                EMIT("  ret i64 %%map_get_unbox_%lld\n", v);
              } else {
                EMIT("  ret i64 %%r%lld\n", base);
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
              emit_string_concat_pair_by_policy(f, ii, ilen, insts, dst, lhs, rhs);
              continue;
            }
            long long lhs_base = resolve_copy_source_reg(lhs);
            long long rhs_base = resolve_copy_source_reg(rhs);
            long long vL, vR; int lc = has_const(lhs_base, &vL), rc2 = has_const(rhs_base, &vR);
            if (lc && rc2) {
              EMIT("  %%r%lld = %s i64 %lld, %lld\n", dst, irop, vL, vR);
            } else {
              EMIT("  %%r%lld = %s i64 %s%lld%s, %s%lld%s\n", dst, irop,
                   lc?"":"%r", lc?vL:lhs_base, lc?"":"",
                   rc2?"":"%r", rc2?vR:rhs_base, rc2?"":"");
            }
            set_type(dst, T_I64);
            continue;
          }
          if (strcmp(op, "mir_call")==0) {
            yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
            yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
            const char* ctype = cal ? read_str(cal, "type") : NULL;
            const char* bname = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
            const char* mname = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
            long long dst = read_int(ins, "dst");
            long long recv = read_int(cal, "receiver");
            yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
            long long a0 = 0, a1 = 0;
            if (args && yyjson_is_arr(args)) {
              if (yyjson_arr_size(args) >= 1) a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
              if (yyjson_arr_size(args) >= 2) a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
            }
            if (emit_mir_call_dispatch(
                    f, blocks, bid, ii, ilen, insts, ctype, bname, mname,
                    dst, recv, args, a0, a1, skip_inst) < 0) {
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
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

      if (rc != 0) {
        rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
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
          int rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out); remove(llpath); yyjson_doc_free(d0);
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
                int rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out);
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
          fprintf(f, "declare i64 @\"nyash.map.entry_count_h\"(i64)%s\n\n", HAKO_LLVM_READONLY_ATTRS);
          hako_llvmc_emit_entry_header(f, &selection);
          fprintf(f, "  %%h = call i64 @\"nyash.map.birth_h\"()\n");
          fprintf(f, "  %%_s = call i64 @\"nyash.map.slot_store_hhh\"(i64 %%h, i64 %lld, i64 %lld)\n", key_c, val_c);
          fprintf(f, "  %%sz = call i64 @\"nyash.map.entry_count_h\"(i64 %%h)\n");
          fprintf(f, "  ret i64 %%sz\n}\n");
          hako_llvmc_emit_entry_alias_if_needed(f, &selection);
          fclose(f);
          int rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out); remove(llpath); yyjson_doc_free(doc);
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
          fprintf(f, "declare i64 @\"nyash.array.slot_len_h\"(i64)%s\n\n", HAKO_LLVM_READONLY_ATTRS);
          hako_llvmc_emit_entry_header(f, &selection);
          fprintf(f, "  %%h = call i64 @\"nyash.array.birth_h\"()\n");
          fprintf(f, "  %%_p = call i64 @\"nyash.array.slot_append_hh\"(i64 %%h, i64 %lld)\n", val_c);
          fprintf(f, "  %%len = call i64 @\"nyash.array.slot_len_h\"(i64 %%h)\n");
          fprintf(f, "  ret i64 %%len\n}\n");
          hako_llvmc_emit_entry_alias_if_needed(f, &selection);
          fclose(f);
          int rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out); remove(llpath); yyjson_doc_free(doc);
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
          int rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out); remove(llpath); yyjson_doc_free(doc);
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
          int rc = hako_llvmc_mem2reg_canonicalize_and_llc(llpath, obj_out); remove(llpath); yyjson_doc_free(doc);
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
