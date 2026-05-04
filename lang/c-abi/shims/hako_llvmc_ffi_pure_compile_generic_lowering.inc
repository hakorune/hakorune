      #include "hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc"
      if (!generic_capacity_ok()) {
        fclose(f);
        yyjson_doc_free(d);
        goto GEN_ABORT;
      }
      #include "hako_llvmc_ffi_emit_seam.inc"
      #include "hako_llvmc_ffi_generic_method_lowering.inc"
      #include "hako_llvmc_ffi_mir_call_dispatch.inc"
      auto int note_global_call_lowering_plan_unsupported_shape(
          long long bid,
          size_t ii,
          const char* op) {
        yyjson_val* plan_entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanGlobalCallView global_view;
        if (read_lowering_plan_global_call_view(plan_entry, &global_view) &&
            global_view.tier == HAKO_LLVMC_LOWERING_PLAN_TIER_UNSUPPORTED) {
          if (lowering_plan_global_call_view_has_direct_target(&global_view)) {
            trace_lowering_plan_entry_hit(plan_entry, "global_call_direct_target_pending");
          }
          note_pure_unsupported_shape_detail(
              bid,
              ii,
              op,
              (global_view.reason && global_view.reason[0])
                  ? global_view.reason
                  : "lowering_plan_unsupported_global_call",
              global_view.target_return_type,
              global_view.target_shape_reason,
              global_view.target_shape_blocker_symbol,
              global_view.target_shape_blocker_reason);
          return 1;
        }
        return 0;
      }
      struct ArrayTextResidenceSessionRouteMetadata active_region_executor;
      int active_region_executor_matched = 0;
      struct ArrayTextCombinedRegionRouteMetadata active_combined_region;
      int active_combined_region_available = 0;
      int active_combined_region_emitted = 0;
      struct ArrayTextObserverStoreRegionRouteMetadata active_observer_store_region;
      int active_observer_store_region_available = 0;
      int active_observer_store_region_emitted = 0;
      memset(&active_region_executor, 0, sizeof(active_region_executor));
      memset(&active_combined_region, 0, sizeof(active_combined_region));
      memset(&active_observer_store_region, 0, sizeof(active_observer_store_region));
      active_combined_region_available =
          match_array_text_combined_region_metadata_any(&active_combined_region);
      active_observer_store_region_available =
          match_array_text_observer_store_region_metadata_any(
              &active_observer_store_region);
      auto void emit_array_text_region_executor_call(
          const struct ArrayTextResidenceSessionRouteMetadata* region) {
        if (!region) return;
        fprintf(
            f,
            "  %%r%lld = call i64 @\"nyash.array.string_insert_mid_subrange_len_store_region_hiisi\"(i64 %%r%lld, i64 %lld, i64 %lld, ptr getelementptr inbounds ([%lld x i8], ptr @.str_reg_%lld, i64 0, i64 0), i64 %lld)\n",
            region->region_result_dst,
            region->array_root_value,
            region->loop_bound_const,
            region->row_modulus_const,
            region->middle_len + 1,
            region->middle_reg,
            region->middle_len);
        set_type(region->region_result_dst, T_I64);
      }
      auto void emit_array_text_observer_store_region_call(
          const struct ArrayTextObserverStoreRegionRouteMetadata* region) {
        if (!region) return;
        fprintf(
            f,
            "  call i64 @\"nyash.array.string_indexof_suffix_store_region_hisisi\"(i64 %%r%lld, i64 %lld, ptr getelementptr inbounds ([%lld x i8], ptr @.str_reg_%lld, i64 0, i64 0), i64 %lld, ptr getelementptr inbounds ([%lld x i8], ptr @.str_reg_%lld, i64 0, i64 0), i64 %lld)\n",
            region->array_root_value,
            region->loop_bound_const,
            region->needle_len + 1,
            region->needle_reg,
            region->needle_len,
            region->suffix_len + 1,
            region->suffix_reg,
            region->suffix_len);
      }
      auto void emit_array_text_combined_region_call(
          const struct ArrayTextCombinedRegionRouteMetadata* region) {
        const char* symbol = NULL;
        if (!region) return;
        symbol = region->byte_boundary_safe
            ? "nyash.array.string_lenhalf_insert_mid_periodic_indexof_suffix_region_ascii_hiisiiisisi"
            : "nyash.array.string_lenhalf_insert_mid_periodic_indexof_suffix_region_hiisiiisisi";
        fprintf(
            f,
            "  %%r%lld = call i64 @\"%s\"(i64 %%r%lld, i64 %lld, i64 %lld, ptr getelementptr inbounds ([%lld x i8], ptr @.str_reg_%lld, i64 0, i64 0), i64 %lld, i64 %lld, i64 %lld, ptr getelementptr inbounds ([%lld x i8], ptr @.str_reg_%lld, i64 0, i64 0), i64 %lld, ptr getelementptr inbounds ([%lld x i8], ptr @.str_reg_%lld, i64 0, i64 0), i64 %lld)\n",
            region->accumulator_phi_value,
            symbol,
            region->array_value,
            region->loop_bound_const,
            region->row_modulus_const,
            region->middle_len + 1,
            region->middle_reg,
            region->middle_len,
            region->observer_period_const,
            region->observer_bound_const,
            region->needle_len + 1,
            region->needle_reg,
            region->needle_len,
            region->suffix_len + 1,
            region->suffix_reg,
            region->suffix_len);
        set_type(region->accumulator_phi_value, T_I64);
      }
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
      auto int should_emit_string_compare(yyjson_val* ins, long long lhs_reg, long long rhs_reg){
        const char* cmp_kind = read_str(ins, "cmp_kind");
        long long lhs_base = resolve_copy_source_reg(lhs_reg);
        long long rhs_base = resolve_copy_source_reg(rhs_reg);
        int lhs_org = get_origin(lhs_base);
        int rhs_org = get_origin(rhs_base);
        int lhs_scan = get_scan_origin(lhs_base);
        int rhs_scan = get_scan_origin(rhs_base);
        if (cmp_kind && strcmp(cmp_kind, "string") == 0) return 1;
        if (lhs_org == ORG_STRING && rhs_org == ORG_STRING) return 1;
        if (lhs_scan == ORG_STRING && rhs_scan == ORG_STRING) return 1;
        return 0;
      }
      auto void emit_string_compare(long long dst, const char* pred, long long lhs_reg, long long rhs_reg){
        long long lhs_base = resolve_copy_source_reg(lhs_reg);
        long long rhs_base = resolve_copy_source_reg(rhs_reg);
        if (strcmp(pred, "eq") == 0) {
          EMIT("  %%str_eq_%lld = call i64 @\"nyash.string.eq_hh\"(i64 %%r%lld, i64 %%r%lld)\n", dst, lhs_base, rhs_base);
          EMIT("  %%r%lld = icmp ne i64 %%str_eq_%lld, 0\n", dst, dst);
        } else if (strcmp(pred, "ne") == 0) {
          EMIT("  %%str_eq_%lld = call i64 @\"nyash.string.eq_hh\"(i64 %%r%lld, i64 %%r%lld)\n", dst, lhs_base, rhs_base);
          EMIT("  %%r%lld = icmp eq i64 %%str_eq_%lld, 0\n", dst, dst);
        } else if (strcmp(pred, "slt") == 0) {
          EMIT("  %%str_lt_%lld = call i64 @\"nyash.string.lt_hh\"(i64 %%r%lld, i64 %%r%lld)\n", dst, lhs_base, rhs_base);
          EMIT("  %%r%lld = icmp ne i64 %%str_lt_%lld, 0\n", dst, dst);
        } else if (strcmp(pred, "sgt") == 0) {
          EMIT("  %%str_gt_%lld = call i64 @\"nyash.string.lt_hh\"(i64 %%r%lld, i64 %%r%lld)\n", dst, rhs_base, lhs_base);
          EMIT("  %%r%lld = icmp ne i64 %%str_gt_%lld, 0\n", dst, dst);
        } else if (strcmp(pred, "sle") == 0) {
          EMIT("  %%str_gt_%lld = call i64 @\"nyash.string.lt_hh\"(i64 %%r%lld, i64 %%r%lld)\n", dst, rhs_base, lhs_base);
          EMIT("  %%r%lld = icmp eq i64 %%str_gt_%lld, 0\n", dst, dst);
        } else if (strcmp(pred, "sge") == 0) {
          EMIT("  %%str_lt_%lld = call i64 @\"nyash.string.lt_hh\"(i64 %%r%lld, i64 %%r%lld)\n", dst, lhs_base, rhs_base);
          EMIT("  %%r%lld = icmp eq i64 %%str_lt_%lld, 0\n", dst, dst);
        } else {
          EMIT("  %%r%lld = icmp eq i64 0, 1\n", dst);
        }
        set_type(dst, T_I1);
      }
      #include "hako_llvmc_ffi_module_generic_string_function_emit.inc"
      if (emit_module_function_generic_string_definitions(f) < 0) {
        fclose(f);
        yyjson_doc_free(d);
        goto GEN_ABORT;
      }
      // Unboxer (declare opportunistically; low cost)
      hako_llvmc_emit_entry_header(f, &selection);
      auto int should_skip_string_const_box_for_known_len_use(
          size_t ii,
          size_t ilen,
          yyjson_val* insts,
          long long reg) {
        yyjson_val* ins = NULL;
        yyjson_val* mc = NULL;
        yyjson_val* cal = NULL;
        const char* op = NULL;
        const char* ctype = NULL;
        const char* mname = NULL;
        long long recv = 0;
        long long known_len = 0;
        if (!reg || ii >= ilen) return 0;
        if (!get_known_string_len(reg, &known_len)) return 0;
        ins = yyjson_arr_get(insts, ii);
        op = ins ? read_str(ins, "op") : NULL;
        if (!(op && strcmp(op, "mir_call") == 0)) return 0;
        mc = yyjson_obj_get(ins, "mir_call");
        cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
        ctype = cal ? read_str(cal, "type") : NULL;
        mname = cal ? (read_str(cal, "method") ? read_str(cal, "method")
                                               : read_str(cal, "name"))
                    : NULL;
        recv = cal ? resolve_copy_source_reg(read_int(cal, "receiver")) : 0;
        if (!(ctype && strcmp(ctype, "Method") == 0 && recv == resolve_copy_source_reg(reg))) {
          return 0;
        }
        return mname && (strcmp(mname, "length") == 0 || strcmp(mname, "size") == 0);
      }

      for (size_t bi=0; bi<blen; bi++) {
        struct GenericPureBlockView block;
        if (!read_generic_pure_block_view(bi, &block)) continue;
        yyjson_val* b = block.block;
        long long bid = block.id;
        if (active_combined_region_emitted &&
            (bid == active_combined_region.header_block ||
             bid == active_combined_region.edit_block ||
             bid == active_combined_region.observer_begin_block ||
             bid == active_combined_region.observer_header_block ||
             bid == active_combined_region.observer_block ||
             bid == active_combined_region.observer_store_block ||
             bid == active_combined_region.observer_latch_block ||
             bid == active_combined_region.observer_exit_block ||
             bid == active_combined_region.latch_block)) {
          emit_block_label(bid);
          EMIT("  unreachable\n");
          continue;
        }
        if (active_region_executor_matched &&
            (bid == active_region_executor.header_block ||
             bid == active_region_executor.body_block)) {
          emit_block_label(bid);
          EMIT("  unreachable\n");
          continue;
        }
        if (active_observer_store_region_available &&
            (bid == active_observer_store_region.header_block ||
             bid == active_observer_store_region.observer_block ||
             bid == active_observer_store_region.then_store_block ||
             bid == active_observer_store_region.latch_block)) {
          emit_block_label(bid);
          EMIT("  unreachable\n");
          continue;
        }
        if (!active_region_executor_matched &&
            match_array_text_residence_session_region_begin_metadata(
                bid,
                &active_region_executor)) {
          emit_block_label(bid);
          emit_array_text_region_executor_call(&active_region_executor);
          EMIT("  br label %%bb%lld\n", active_region_executor.exit_block);
          active_region_executor_matched = 1;
          continue;
        }
        if (active_combined_region_available &&
            !active_combined_region_emitted &&
            bid == active_combined_region.begin_block) {
          emit_block_label(bid);
          emit_array_text_combined_region_call(&active_combined_region);
          EMIT("  br label %%bb%lld\n", active_combined_region.exit_block);
          active_combined_region_emitted = 1;
          continue;
        }
        if (active_observer_store_region_available &&
            !active_observer_store_region_emitted &&
            bid == active_observer_store_region.begin_block) {
          emit_block_label(bid);
          emit_array_text_observer_store_region_call(&active_observer_store_region);
          EMIT("  br label %%bb%lld\n", active_observer_store_region.exit_block);
          active_observer_store_region_emitted = 1;
          continue;
        }
        emit_block_label(bid);
        // phi first
        for (int pi=0; pi<phi_n; pi++) if (phis[pi].bid==bid){ for (int r=0;r<phis[pi].rec_n;r++){ emit_phi(phis[pi].recs[r].dst, phis[pi].recs[r].in, phis[pi].recs[r].in_n); } }
        if (bi == 0) {
          emit_hoisted_string_const_defs(f);
        }
        yyjson_val* insts = block.instructions;
        size_t ilen = block.instruction_count;
        unsigned char skip_inst[ilen ? ilen : 1];
        memset(skip_inst, 0, sizeof(skip_inst));
        for (size_t ii=0; ii<ilen; ii++) {
          if (skip_inst[ii]) continue;
          yyjson_val* ins = yyjson_arr_get(insts, ii);
          const char* op = read_str(ins, "op");
          if (!op) {
            note_pure_unsupported_shape(bid, ii, "unknown", "missing_op", NULL);
            yyjson_doc_free(d);
            goto GEN_ABORT;
          }
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
            if (should_skip_hoisted_string_const_box(dst)) {
              set_type(dst, T_I64);
              set_origin(dst, ORG_STRING);
              set_scan_origin(dst, ORG_STRING);
              continue;
            }
            if (emit_string_const_box_result(f, dst)) continue;
            continue;
          }
          if (strcmp(op, "keepalive")==0) {
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
                if (should_skip_hoisted_string_const_box(arg0) &&
                    !string_const_box_was_emitted(arg0) &&
                    !emit_materialized_string_const_box_result(f, arg0)) {
                  note_pure_unsupported_shape(
                      bid,
                      ii,
                      op,
                      "string_const_materialization_failed",
                      NULL);
                  yyjson_doc_free(d);
                  goto GEN_ABORT;
                }
                set_copy_alias(dst, resolve_copy_source_reg(arg0));
                set_type(dst, T_I64);
                set_origin(dst, ORG_STRING);
                set_scan_origin(dst, ORG_STRING);
                continue;
              }
            }
            note_pure_unsupported_shape(bid, ii, op, "unsupported_newbox_type", NULL);
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
            if (!p2) {
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "unsupported_compare_predicate",
                  NULL);
              yyjson_doc_free(d); goto GEN_ABORT;
            }
            long long dst = read_int(ins, "dst"); long long lhs = read_int(ins, "lhs"); if (!lhs) lhs=read_int(ins, "left"); long long rhs = read_int(ins, "rhs"); if (!rhs) rhs=read_int(ins, "right"); if (!lhs && !rhs){
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "malformed_compare_operands",
                  NULL);
              yyjson_doc_free(d); goto GEN_ABORT;
            }
            if (should_emit_string_compare(ins, lhs, rhs)) {
              emit_string_compare(dst, p2, lhs, rhs);
              continue;
            }
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
            if (!irop) {
              note_pure_unsupported_shape(bid, ii, op, "unsupported_binop_kind", NULL);
              yyjson_doc_free(d); goto GEN_ABORT;
            }
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
          if (strcmp(op, "call")==0) {
            yyjson_val* cal = yyjson_obj_get(ins, "callee");
            const char* ctype = cal ? read_str(cal, "type") : NULL;
            const char* bname = cal
                ? (read_str(cal, "box_name")
                    ? read_str(cal, "box_name")
                    : read_str(cal, "box_type"))
                : NULL;
            const char* mname = NULL;
            long long dst = read_int(ins, "dst");
            long long recv = cal ? read_int(cal, "receiver") : 0;
            yyjson_val* args = yyjson_obj_get(ins, "args");
            long long a0 = 0, a1 = 0;
            if (!cal) {
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "legacy_call_missing_structured_callee",
                  NULL);
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            if (ctype && (!strcmp(ctype, "Global") || !strcmp(ctype, "Extern"))) {
              mname = read_str(cal, "name");
            } else if (ctype && !strcmp(ctype, "Method")) {
              mname = read_str(cal, "method")
                  ? read_str(cal, "method")
                  : read_str(cal, "name");
            } else if (ctype && !strcmp(ctype, "Constructor")) {
              mname = read_str(cal, "name");
            }
            if (args && yyjson_is_arr(args)) {
              if (yyjson_arr_size(args) >= 1) {
                a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
              }
              if (yyjson_arr_size(args) >= 2) {
                a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
              }
            }
            if (emit_mir_call_dispatch(
                    f, blocks, bid, ii, ilen, insts, ctype, bname, mname,
                    dst, recv, args, a0, a1, skip_inst) < 0) {
              if (ctype && !strcmp(ctype, "Global")) {
                if (!note_global_call_lowering_plan_unsupported_shape(bid, ii, op)) {
                  note_pure_unsupported_shape_detail(
                      bid,
                      ii,
                      op,
                      "missing_multi_function_emitter",
                      "?",
                      "structured_call_global_user_target",
                      mname,
                      "same_module_global_call_requires_uniform_emitter");
                }
              } else {
                note_pure_unsupported_shape_detail(
                    bid,
                    ii,
                    op,
                    "structured_call_no_route",
                    NULL,
                    ctype,
                    mname,
                    NULL);
              }
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
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
              if (!note_global_call_lowering_plan_unsupported_shape(bid, ii, op)) {
                note_pure_unsupported_shape(bid, ii, op, "mir_call_no_route", NULL);
              }
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            continue;
          }
          // any other op unsupported
          note_pure_unsupported_shape(bid, ii, op, "unknown_op", NULL);
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
      free_owned_str_consts();
      yyjson_doc_free(d);
      if (rc == 0) return 0;
    GEN_ABORT:;
      // fall through to pattern lowers
    GEN_END: ;
    } while(0);
