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
          fprintf(f, "declare i64 @\"nyash.map.entry_count_i64\"(i64)%s\n\n", HAKO_LLVM_READONLY_ATTRS);
          hako_llvmc_emit_entry_header(f, &selection);
          fprintf(f, "  %%h = call i64 @\"nyash.map.birth_h\"()\n");
          fprintf(f, "  %%_s = call i64 @\"nyash.map.slot_store_hhh\"(i64 %%h, i64 %lld, i64 %lld)\n", key_c, val_c);
          fprintf(f, "  %%sz = call i64 @\"nyash.map.entry_count_i64\"(i64 %%h)\n");
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
