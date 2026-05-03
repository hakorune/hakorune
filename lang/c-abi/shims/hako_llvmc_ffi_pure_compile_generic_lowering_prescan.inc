      // Pre-scan: consts + phis + needed method decls
      struct GenericPureNeedFlags needs;
      memset(&needs, 0, sizeof(needs));
      size_t blen = generic_pure_block_count;
      for (size_t bi=0; bi<blen; bi++) {
        struct GenericPureBlockView block;
        if (!read_generic_pure_block_view(bi, &block)) continue;
        long long bid = block.id;
        yyjson_val* insts = block.instructions;
        size_t ilen = block.instruction_count;
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
              put_known_string_len(dst, (long long)strlen(sval));
              set_type(dst, T_I64);
              set_scan_origin(dst, ORG_STRING);
              set_origin(dst, ORG_STRING);
              if (should_skip_string_const_box_for_concat_suffix(ii, ilen, insts, dst)) {
                mark_hoisted_string_const_box_skip(dst);
              } else if (array_text_observer_const_arg_can_skip(dst)) {
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
                  {
                    long long known_len = 0;
                    if (get_known_string_len(arg0, &known_len)) {
                      put_known_string_len(dst, known_len);
                    }
                  }
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
            if (dst && src) {
              long long known_len = 0;
              if (get_known_string_len(src, &known_len)) {
                put_known_string_len(dst, known_len);
              }
            }
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
              if ((then_scan == ORG_STRING && then_scan == else_scan) ||
                  (then_org == ORG_STRING && then_org == else_org)) {
                long long then_len = 0;
                long long else_len = 0;
                if (get_known_string_len(then_val, &then_len) &&
                    get_known_string_len(else_val, &else_len) &&
                    then_len == else_len) {
                  put_known_string_len(dst, then_len);
                }
              }
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
            yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
            yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
            yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
            const char* ctype = cal ? read_str(cal, "type") : NULL;
            const char* bname = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
            const char* mname = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
            long long dst = read_int(ins, "dst");
            long long recv = cal ? read_int(cal, "receiver") : 0;
            long long a0 = 0;
            long long a1 = 0;
            long long known_len = 0;
            if (args && yyjson_is_arr(args)) {
              if (yyjson_arr_size(args) >= 1) a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
              if (yyjson_arr_size(args) >= 2) a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
            }
            if (dst && ctype && !strcmp(ctype, "Method") && recv &&
                get_scan_origin(recv) == ORG_STRING) {
              if (mname && !strcmp(mname, "substring") &&
                  known_string_window_len(a0, a1, &known_len)) {
                set_type(dst, T_I64);
                set_scan_origin(dst, ORG_STRING);
                set_origin(dst, ORG_STRING);
                put_known_string_len(dst, known_len);
              } else if (mname &&
                         (!strcmp(mname, "length") || !strcmp(mname, "len") ||
                          !strcmp(mname, "size")) &&
                         get_known_string_len(recv, &known_len)) {
                put_const(dst, known_len);
                set_type(dst, T_I64);
              }
            } else if (dst && ctype && !strcmp(ctype, "Extern") && mname) {
              if ((!strcmp(mname, "nyash.string.substring_hii") ||
                   !strcmp(mname, "nyash.string.substring_concat_hhii") ||
                   !strcmp(mname, "nyash.string.substring_concat3_hhhii") ||
                   !strcmp(mname, "nyash.string.piecewise_subrange_hsiii")) &&
                  known_string_window_len(a0, a1, &known_len)) {
                set_type(dst, T_I64);
                set_scan_origin(dst, ORG_STRING);
                set_origin(dst, ORG_STRING);
                put_known_string_len(dst, known_len);
              }
            }
            scan_mir_call_need_flags(
                blocks,
                bid,
                ii,
                ilen,
                insts,
                ins,
                &needs);
            if (ctype && !strcmp(ctype, "Method") &&
                mname && !strcmp(mname, "substring") &&
                args && yyjson_is_arr(args) && yyjson_arr_size(args) == 1) {
              needs.string_len = 1;
            }
          } else if (strcmp(op, "binop")==0) {
            const char* bop = read_str(ins, "operation");
            if (!bop) bop = read_str(ins, "op_kind");
            long long dst = read_int(ins, "dst");
            long long lhs = read_int(ins, "lhs");
            long long rhs = read_int(ins, "rhs");
            long long lhs_const_val = 0;
            long long rhs_const_val = 0;
            if ((bop && (strcmp(bop, "+") == 0 || strcmp(bop, "Add") == 0)) &&
                get_scan_origin(lhs) == ORG_STRING && get_scan_origin(rhs) == ORG_STRING) {
              const char* lhs_const = get_str_const(lhs);
              const char* rhs_const = get_str_const(rhs);
              long long lhs_len = 0;
              long long rhs_len = 0;
              if (lhs_const && rhs_const) {
                char* folded = concat_owned_string_consts(lhs_const, rhs_const);
                if (folded && put_str_const_owned(dst, folded)) {
                  if (dst) {
                    set_type(dst, T_I64);
                    set_scan_origin(dst, ORG_STRING);
                    set_origin(dst, ORG_STRING);
                    put_known_string_len(dst, (long long)strlen(folded));
                  }
                  needs.string_const_box = 1;
                  continue;
                }
              }
              needs.string_concat = 1;
              if (dst) {
                set_scan_origin(dst, ORG_STRING);
                set_origin(dst, ORG_STRING);
                if (get_known_string_len(lhs, &lhs_len) &&
                    get_known_string_len(rhs, &rhs_len)) {
                  put_known_string_len(dst, lhs_len + rhs_len);
                }
              }
            } else if (dst && bop &&
                       has_const(lhs, &lhs_const_val) &&
                       has_const(rhs, &rhs_const_val)) {
              int matched = 1;
              long long folded = 0;
              if (strcmp(bop, "+") == 0 || strcmp(bop, "Add") == 0) {
                folded = lhs_const_val + rhs_const_val;
              } else if (strcmp(bop, "-") == 0 || strcmp(bop, "Sub") == 0) {
                folded = lhs_const_val - rhs_const_val;
              } else if (strcmp(bop, "*") == 0 || strcmp(bop, "Mul") == 0) {
                folded = lhs_const_val * rhs_const_val;
              } else if (strcmp(bop, "/") == 0 || strcmp(bop, "Div") == 0) {
                if (rhs_const_val == 0) matched = 0;
                else folded = lhs_const_val / rhs_const_val;
              } else if (strcmp(bop, "%") == 0 || strcmp(bop, "Mod") == 0) {
                if (rhs_const_val == 0) matched = 0;
                else folded = lhs_const_val % rhs_const_val;
              } else {
                matched = 0;
              }
              if (matched) {
                put_const(dst, folded);
                set_type(dst, T_I64);
              }
            }
          }
        }
      }

      for (int pass = 0; pass < 8; pass++) {
        int changed = 0;
        for (int pi = 0; pi < phi_n; pi++) {
          for (int ri = 0; ri < phis[pi].rec_n; ri++) {
            struct PhiRec* pr = &phis[pi].recs[ri];
            long long same_len = -1;
            int all_known = 1;
            int is_string_phi = 0;
            if (!pr->dst) continue;
            if (!(get_scan_origin(pr->dst) == ORG_STRING || get_origin(pr->dst) == ORG_STRING)) {
              continue;
            }
            for (int ii = 0; ii < pr->in_n; ii++) {
              long long in_len = 0;
              long long in_reg = pr->in[ii].val_reg;
              if (!in_reg) {
                all_known = 0;
                break;
              }
              if (!(get_scan_origin(in_reg) == ORG_STRING || get_origin(in_reg) == ORG_STRING)) {
                all_known = 0;
                break;
              }
              is_string_phi = 1;
              if (!get_known_string_len(in_reg, &in_len)) {
                all_known = 0;
                break;
              }
              if (same_len < 0) same_len = in_len;
              else if (same_len != in_len) {
                all_known = 0;
                break;
              }
            }
            if (is_string_phi && all_known && same_len >= 0) {
              long long cur_len = 0;
              if (!get_known_string_len(pr->dst, &cur_len) || cur_len != same_len) {
                put_known_string_len(pr->dst, same_len);
                changed = 1;
              }
            }
          }
        }
        if (!changed) break;
      }

      if (array_text_observer_has_const_utf8_route()) {
        needs.arr_string_indexof_const_utf8 = 1;
      }

      // Cross-block refinement for string literal hoists.
      // Some literals are defined in an entry block but only consumed by a
      // single later concat/set window. The per-block pre-scan above cannot see
      // those uses, so rescan the function and suppress the hoisted box when the
      // literal has exactly one use and that use is a raw-ptr concat window.
      for (size_t sci = 0; sci < str_consts_n; sci++) {
        long long reg = str_consts[sci].reg;
        size_t use_ii = 0;
        yyjson_val* use_insts = NULL;
        size_t use_ilen = 0;
        long long effective_reg = reg;
        int dead_copy_chain = 0;
        if (!reg || should_skip_hoisted_string_const_box(reg)) continue;
        if (!find_single_effective_use_across_blocks(
                reg,
                &use_ii,
                &use_insts,
                &use_ilen,
                &effective_reg,
                &dead_copy_chain)) {
          if (dead_copy_chain) {
            mark_hoisted_string_const_box_skip(reg);
          }
          continue;
        }
        if (!effective_reg) {
          mark_hoisted_string_const_box_skip(reg);
          continue;
        }
        if (!use_insts || !yyjson_is_arr(use_insts)) continue;
        if (should_skip_string_const_box_for_known_len_use(use_ii, use_ilen, use_insts, effective_reg)) {
          mark_hoisted_string_const_box_skip(reg);
          continue;
        }
        if (should_skip_string_const_box_for_concat_suffix((size_t)-1, use_ilen, use_insts, effective_reg) ||
            should_skip_string_const_box_for_insert_mid_window((size_t)-1, use_ilen, use_insts, effective_reg)) {
          mark_hoisted_string_const_box_skip(reg);
        }
      }

      if (plan_module_generic_string_definitions() < 0) {
        yyjson_doc_free(d);
        goto GEN_ABORT;
      }

      // IR temp file
      char llpath[1024]; snprintf(llpath, sizeof(llpath), "%s/hako_pure_gen_%d.ll", "/tmp", (int)getpid());
      FILE* f = fopen(llpath, "wb"); if (!f) { yyjson_doc_free(d); break; }
      fprintf(f, "; nyash pure IR (generic)\n");
      fprintf(f, "target triple = \"x86_64-pc-linux-gnu\"\n\n");
      emit_module_function_declarations(f);
      if (needs.map_birth || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.map.birth_h\"()\n");
      if (needs.map_set || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.map.slot_store_hhh\"(i64, i64, i64)\n");
      if (needs.map_get || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.map.slot_load_hh\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.map_has)   fprintf(f, "declare i64 @\"nyash.map.probe_hh\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.map_has)   fprintf(f, "declare i64 @\"nyash.map.probe_hi\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.map_size)  fprintf(f, "declare i64 @\"nyash.map.entry_count_i64\"(i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.map_keys || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.map.keys_h\"(i64)\n");
      fprintf(f, "declare i64 @\"nyash.runtime_data.get_hh\"(i64, i64)\n");
      fprintf(f, "declare i64 @\"nyash.runtime_data.set_hhh\"(i64, i64, i64)\n");
      if (needs.runtime_data_has) fprintf(f, "declare i64 @\"nyash.runtime_data.has_hh\"(i64, i64)\n");
      fprintf(f, "declare i64 @\"nyash.runtime_data.push_hh\"(i64, i64)\n");
      fprintf(f, "declare ptr @llvm.stacksave()\n");
      fprintf(f, "declare void @llvm.stackrestore(ptr)\n");
      if (needs.arr_birth || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.array.birth_h\"()\n");
      if (needs.arr_push || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.array.slot_append_hh\"(i64, i64)\n");
      if (needs.arr_len || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.array.slot_len_h\"(i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.arr_has)   fprintf(f, "declare i64 @\"nyash.array.has_hh\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.arr_set)   fprintf(f, "declare i64 @\"nyash.array.slot_store_hih\"(i64, i64, i64)\n");
      if (needs.arr_set)   fprintf(f, "declare i64 @\"nyash.array.set_his\"(i64, i64, i64)\n");
      if (needs.arr_set)   fprintf(f, "declare i64 @\"nyash.array.slot_store_hii\"(i64, i64, i64)\n");
      if (needs.arr_set && (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof || needs.arr_kernel_slot_concat_by_index))
        fprintf(f, "declare i64 @\"nyash.array.kernel_slot_store_hi\"(i64, i64, ptr)\n");
      if (needs.arr_set && (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof || needs.arr_kernel_slot_concat_by_index))
        fprintf(f, "declare i64 @\"nyash.array.kernel_slot_concat_his\"(ptr, i64, i64, ptr)\n");
      if (needs.arr_set && (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof || needs.arr_kernel_slot_concat_by_index))
        fprintf(f, "declare i64 @\"nyash.array.string_suffix_store_his\"(i64, i64, ptr)\n");
      if (needs.arr_set && (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof || needs.arr_kernel_slot_concat_by_index))
        fprintf(f, "declare i64 @\"nyash.array.string_suffix_store_hisi\"(i64, i64, ptr, i64)\n");
      if (needs.arr_set && (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof || needs.arr_kernel_slot_concat_by_index))
        fprintf(f, "declare i64 @\"nyash.array.kernel_slot_insert_hisi\"(ptr, i64, i64, ptr, i64)\n");
      if (needs.arr_set && (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof || needs.arr_kernel_slot_concat_by_index))
        fprintf(f, "declare i64 @\"nyash.array.string_insert_mid_store_hisi\"(i64, i64, ptr, i64)\n");
      if (needs.arr_set && (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof || needs.arr_kernel_slot_concat_by_index))
        fprintf(f, "declare i64 @\"nyash.array.string_insert_mid_store_hisii\"(i64, i64, ptr, i64, i64)\n");
      if (needs.arr_get && needs.arr_set)
        fprintf(f, "declare i64 @\"nyash.array.string_insert_mid_lenhalf_store_hisi\"(i64, i64, ptr, i64)\n");
      if (needs.arr_set && (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof || needs.arr_kernel_slot_concat_by_index))
        fprintf(f, "declare i64 @\"nyash.array.string_insert_mid_subrange_store_hisiii\"(i64, i64, ptr, i64, i64, i64)\n");
      if (needs.arr_set && (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof || needs.arr_kernel_slot_concat_by_index))
        fprintf(f, "declare i64 @\"nyash.array.string_insert_mid_subrange_store_hisiiii\"(i64, i64, ptr, i64, i64, i64, i64)\n");
      if (needs.arr_set && (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof || needs.arr_kernel_slot_concat_by_index))
        fprintf(f, "declare i64 @\"nyash.array.string_insert_mid_subrange_len_store_hisi\"(i64, i64, ptr, i64)\n");
      if (needs.arr_set && (needs.string_concat || needs.string_substring || needs.arr_get || needs.arr_string_indexof || needs.arr_kernel_slot_concat_by_index))
        fprintf(f, "declare i64 @\"nyash.array.string_insert_mid_subrange_len_store_region_hiisi\"(i64, i64, i64, ptr, i64)\n");
      if (array_text_combined_region_has_route())
        fprintf(f, "declare i64 @\"nyash.array.string_lenhalf_insert_mid_periodic_indexof_suffix_region_hiisiiisisi\"(i64, i64, i64, ptr, i64, i64, i64, ptr, i64, ptr, i64)\n");
      if (array_text_combined_region_has_route())
        fprintf(f, "declare i64 @\"nyash.array.string_lenhalf_insert_mid_periodic_indexof_suffix_region_ascii_hiisiiisisi\"(i64, i64, i64, ptr, i64, i64, i64, ptr, i64, ptr, i64)\n");
      if (array_text_observer_has_store_region_route())
        fprintf(f, "declare i64 @\"nyash.array.string_indexof_suffix_store_region_hisisi\"(i64, i64, ptr, i64, ptr, i64)\n");
      if (needs.arr_get && needs.arr_set) fprintf(f, "declare i64 @\"nyash.array.rmw_add1_hi\"(i64, i64)\n");
      if (needs.arr_get || needs.arr_string_indexof || needs.string_len) fprintf(f, "declare i64 @\"nyash.array.string_len_hi\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.arr_get || needs.arr_string_indexof) fprintf(f, "declare i64 @\"nyash.array.string_indexof_hih\"(i64, i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.arr_string_indexof_const_utf8) fprintf(f, "declare i64 @\"nyash.array.string_indexof_hisi\"(i64, i64, ptr, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.arr_slot_load || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.array.slot_load_hi\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.string_const_box || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.box.from_i8_string_const\"(ptr)\n");
      if (needs.string_len || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.string.len_h\"(i64)\n");
      fprintf(f, "declare i64 @\"nyash.string.eq_hh\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      fprintf(f, "declare i64 @\"nyash.string.lt_hh\"(i64, i64)%s\n", HAKO_LLVM_READONLY_ATTRS);
      if (needs.string_substring || needs.arr_set || needs.arr_get || needs.arr_string_indexof || needs.string_indexof || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.string.substring_hii\"(i64, i64, i64)\n");
      if (needs.string_substring) fprintf(f, "declare i64 @\"nyash.string.substring_concat_hhii\"(i64, i64, i64, i64)\n");
      if (needs.string_substring) fprintf(f, "declare i64 @\"nyash.string.substring_concat3_hhhii\"(i64, i64, i64, i64, i64)\n");
      if (needs.string_substring || needs.arr_set || needs.arr_get || needs.arr_string_indexof || needs.string_indexof)
        fprintf(f, "declare i64 @\"nyash.string.piecewise_subrange_hsiii\"(i64, ptr, i64, i64, i64)\n");
      if (needs.string_substring || needs.arr_set || needs.arr_get || needs.arr_string_indexof || needs.string_indexof)
        fprintf(f, "declare i64 @\"nyash.string.kernel_slot_piecewise_subrange_hsiii\"(ptr, i64, ptr, i64, i64, i64)\n");
      if (needs.string_substring || needs.arr_set || needs.arr_get || needs.arr_string_indexof || needs.string_indexof)
        fprintf(f, "declare i64 @\"nyash.string.kernel_slot_substring_hii_in_place\"(ptr, i64, i64)\n");
      if (needs.string_substring || needs.arr_set || needs.arr_get || needs.arr_string_indexof || needs.string_indexof)
        fprintf(f, "declare i64 @\"nyash.string.kernel_slot_publish_h\"(ptr)\n");
      if (needs.string_indexof || needs.arr_string_indexof || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.string.indexOf_hh\"(i64, i64)\n");
      if (needs.string_indexof || needs.arr_string_indexof || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.string.lastIndexOf_hh\"(i64, i64)\n");
      if (needs.string_contains || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.string.contains_hh\"(i64, i64)\n");
      if (needs.env_get || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.env.get\"(i64)\n");
      if (needs.env_set || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.env.set\"(i64, i64)\n");
      if (module_has_planned_parser_program_json_definition())
        fprintf(f, "declare i64 @\"nyash.stage1.emit_program_json_v0_h\"(i64)\n");
      if (needs.string_concat || needs.string_substring || needs.arr_get ||
          needs.arr_string_indexof || module_has_planned_generic_string_definition()) {
        fprintf(f, "declare i64 @\"nyash.string.concat_hh\"(i64, i64)\n");
        fprintf(f, "declare i64 @\"nyash.string.concat_hs\"(i64, ptr)\n");
        fprintf(f, "declare i64 @\"nyash.string.kernel_slot_concat_hs\"(ptr, i64, ptr)\n");
        fprintf(f, "declare i64 @\"nyash.string.kernel_slot_insert_hsi\"(ptr, i64, ptr, i64)\n");
        fprintf(f, "declare i64 @\"nyash.string.insert_hsi\"(i64, ptr, i64)\n");
        fprintf(f, "declare i64 @\"nyash.string.concat3_hhh\"(i64, i64, i64)\n");
      }
      if (needs.printf || module_has_planned_generic_string_definition())
        fprintf(f, "@.fmt_i64 = private unnamed_addr constant [5 x i8] c\"%%ld\\0A\\00\", align 1\n");
      for (size_t sci=0; sci<str_consts_n; sci++) {
        size_t slen = strlen(str_consts[sci].val);
        fprintf(f, "@.str_reg_%lld = private unnamed_addr constant [%zu x i8] c\"", str_consts[sci].reg, slen + 1);
        hako_llvmc_emit_llvm_byte_string(f, str_consts[sci].val, (long long)slen);
        fprintf(f, "\\00\", align 1\n");
      }
      // Unboxer (declare opportunistically; low cost)
      fprintf(f, "declare i64 @\"nyash.integer.get_h\"(i64)\n");
      if (needs.hostbridge_extern_invoke_trap)
        fprintf(f, "declare void @llvm.trap()\n");
      if (needs.printf || module_has_planned_generic_string_definition())
        fprintf(f, "declare i32 @printf(ptr, ...)\n");
      if (needs.printf || module_has_planned_generic_string_definition())
        fprintf(f, "declare i64 @\"nyash.console.log_handle\"(i64)\n");
      if (needs.string_substring_len) fprintf(f, "declare i64 @\"nyash.string.substring_len_hii\"(i64, i64, i64)\n");
      fprintf(f, "\n");
      if (emit_module_function_leaf_definitions(f) < 0) {
        fclose(f);
        yyjson_doc_free(d);
        goto GEN_ABORT;
      }
      // Emit blocks
      #define EMIT(...) do { fprintf(f, __VA_ARGS__); } while(0)
