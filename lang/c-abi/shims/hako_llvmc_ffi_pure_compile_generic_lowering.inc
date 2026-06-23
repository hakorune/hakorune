      #include "hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc"
      if (!generic_capacity_ok()) {
        fclose(f);
        yyjson_doc_free(d);
        goto GEN_ABORT;
      }
      #include "hako_llvmc_ffi_emit_seam.inc"
      #include "hako_llvmc_ffi_static_data_emit.inc"
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
          note_pure_unsupported_shape_route_detail(
              bid,
              ii,
              op,
              (global_view.reason && global_view.reason[0])
                  ? global_view.reason
                  : "lowering_plan_unsupported_global_call",
              global_view.target_return_type,
              global_view.target_shape_reason,
              global_view.target_shape_blocker_symbol,
              global_view.target_shape_blocker_reason,
              (global_view.target_symbol && global_view.target_symbol[0])
                  ? global_view.target_symbol
                  : global_view.callee_name,
              NULL,
              global_view.route_id,
              global_view.target_result_box_name,
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
      struct ArrayTextObserverStoreLenSumRegionRouteMetadata
          active_observer_store_len_sum_region;
      int active_observer_store_len_sum_region_available = 0;
      int active_observer_store_len_sum_region_ready = 0;
      int active_observer_store_len_sum_region_emitted = 0;
      struct ArrayTextLoopSessionPlanMetadata active_loop_session_region;
      int active_loop_session_region_emitted = 0;
      struct ArrayTextIndexOfConstRegionPlanMetadata active_indexof_const_region;
      int active_indexof_const_region_emitted = 0;
      struct StringDeadTextRegionPlanMetadata active_string_dead_text_region;
      int active_string_dead_text_region_emitted = 0;
      memset(&active_region_executor, 0, sizeof(active_region_executor));
      memset(&active_combined_region, 0, sizeof(active_combined_region));
      memset(&active_observer_store_region, 0, sizeof(active_observer_store_region));
      memset(
          &active_observer_store_len_sum_region,
          0,
          sizeof(active_observer_store_len_sum_region));
      memset(&active_loop_session_region, 0, sizeof(active_loop_session_region));
      memset(&active_indexof_const_region, 0, sizeof(active_indexof_const_region));
      memset(&active_string_dead_text_region, 0, sizeof(active_string_dead_text_region));
      active_combined_region_available =
          match_array_text_combined_region_metadata_any(&active_combined_region);
      active_observer_store_region_available =
          match_array_text_observer_store_region_metadata_any(
              &active_observer_store_region);
      active_observer_store_len_sum_region_available =
          match_array_text_observer_store_len_sum_region_metadata_any(
              &active_observer_store_len_sum_region);
      active_observer_store_len_sum_region_ready =
          array_text_observer_store_len_sum_region_ready(
              &active_observer_store_len_sum_region);
      (void)active_observer_store_len_sum_region_available;
      (void)active_observer_store_len_sum_region_ready;
      (void)active_observer_store_len_sum_region_emitted;
      (void)active_observer_store_len_sum_region;
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
      auto void emit_array_text_observer_store_len_sum_region_call(
          const struct ArrayTextObserverStoreLenSumRegionRouteMetadata* region) {
        if (!region) return;
        fprintf(
            f,
            "  %%r%lld = call i64 @\"nyash.array.string_indexof_suffix_store_len_sum_region_hiisisi\"(i64 %%r%lld, i64 %lld, i64 %lld, ptr getelementptr inbounds ([%lld x i8], ptr @.str_reg_%lld, i64 0, i64 0), i64 %lld, ptr getelementptr inbounds ([%lld x i8], ptr @.str_reg_%lld, i64 0, i64 0), i64 %lld)\n",
            region->accumulator_phi_value,
            region->array_root_value,
            region->loop_bound_const,
            region->row_modulus_const,
            region->needle_len + 1,
            region->needle_reg,
            region->needle_len,
            region->suffix_len + 1,
            region->suffix_reg,
            region->suffix_len);
        set_type(region->accumulator_phi_value, T_I64);
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
      auto int array_text_loop_session_region_ready(
          const struct ArrayTextLoopSessionPlanMetadata* region) {
        return region &&
               region->matched &&
               region->region_payload_present &&
               region->backend_session_lowering_allowed &&
               region->region_loop_index_initial_const == 0 &&
               region->region_accumulator_initial_const == 0 &&
               region->region_loop_bound_const >= 0 &&
               region->region_row_modulus_const > 0 &&
               region->region_exit_accumulator_value == region->region_accumulator_phi_value;
      }
      auto void emit_array_text_loop_session_region_call(
          const struct ArrayTextLoopSessionPlanMetadata* region) {
        if (!region) return;
        fprintf(
            f,
            "  %%r%lld = call i64 @\"nyash.array.string_len_sum_region_hiii\"(i64 %%r%lld, i64 %lld, i64 %lld, i64 %lld)\n",
            region->region_exit_accumulator_value,
            region->region_array_root_value,
            region->region_loop_bound_const,
            region->region_row_modulus_const,
            region->region_accumulator_initial_const);
        set_type(region->region_exit_accumulator_value, T_I64);
      }
      auto int array_text_indexof_const_region_ready(
          const struct ArrayTextIndexOfConstRegionPlanMetadata* region) {
        return region &&
               region->matched &&
               region->region_payload_present &&
               region->consumer_shape &&
               !strcmp(region->consumer_shape, "found_predicate") &&
               region->needle_const_text &&
               region->needle_byte_len > 0 &&
               region->region_loop_index_initial_const == 0 &&
               region->region_accumulator_initial_const == 0 &&
               region->region_loop_bound_const >= 0 &&
               region->region_row_modulus_const > 0 &&
               region->observer_arg0_value > 0;
      }
      auto void emit_array_text_indexof_const_region_call(
          const struct ArrayTextIndexOfConstRegionPlanMetadata* region) {
        if (!region) return;
        fprintf(
            f,
            "  %%r%lld = call i64 @\"hako.array_text.indexof_const_found_count_region\"(i64 %%r%lld, i64 %lld, i64 %lld, ptr getelementptr inbounds ([%lld x i8], ptr @.str_reg_%lld, i64 0, i64 0), i64 %lld)\n",
            region->region_accumulator_phi_value,
            region->region_array_root_value,
            region->region_loop_bound_const,
            region->region_row_modulus_const,
            region->needle_byte_len + 1,
            region->observer_arg0_value,
            region->needle_byte_len);
        set_type(region->region_accumulator_phi_value, T_I64);
      }
      auto int string_dead_text_region_ready(
          const struct StringDeadTextRegionPlanMetadata* region) {
        return region &&
               region->matched &&
               region->publication_boundary &&
               !strcmp(region->publication_boundary, "none") &&
               !region->final_text_content_observed &&
               region->loop_index_initial_const == 0 &&
               region->accumulator_initial_const == 0 &&
               region->loop_bound_const >= 0 &&
               region->base_len_const > 0 &&
               region->inserted_len_const >= 0 &&
               region->accumulator_delta_const ==
                   region->base_len_const + region->inserted_len_const &&
               region->final_return_value ==
                   region->accumulator_initial_const +
                       region->loop_bound_const * region->accumulator_delta_const +
                       region->base_len_const;
      }
      auto void emit_string_dead_text_region_closed_form_return(
          const struct StringDeadTextRegionPlanMetadata* region) {
        if (!region) return;
        fprintf(f, "  ret i64 %lld\n", region->final_return_value);
      }
      auto void emit_block_label(long long bid){ EMIT("bb%lld:\n", bid); }
      auto void set_block_tail_label(long long bid, const char* label) {
        if (bid < 0 || !label || !label[0]) return;
        for (size_t i = 0; i < block_tail_labels_n; i++) {
          if (block_tail_labels[i].bid == bid) {
            snprintf(block_tail_labels[i].label, sizeof(block_tail_labels[i].label), "%s", label);
            return;
          }
        }
        if (block_tail_labels_n < ARR_LEN(block_tail_labels)) {
          block_tail_labels[block_tail_labels_n].bid = bid;
          snprintf(
              block_tail_labels[block_tail_labels_n].label,
              sizeof(block_tail_labels[block_tail_labels_n].label),
              "%s",
              label);
          block_tail_labels_n++;
        } else {
          note_generic_capacity_overflow("generic_block_tail_label_inventory_full");
        }
      }
      auto void note_exact_status_continue_label(long long bid, size_t ii) {
        char label[96];
        snprintf(label, sizeof(label), "exact_status_continue_%lld_%zu", bid, ii);
        set_block_tail_label(bid, label);
      }
      auto int find_exact_status_tail_label_for_block(
          long long bid,
          char* label,
          size_t label_len) {
        size_t last_exact_status_ii = (size_t)-1;
        if (!label || label_len == 0) return 0;
        for (size_t bi = 0; bi < generic_pure_block_count; bi++) {
          struct GenericPureBlockView block;
          if (!read_generic_pure_block_view(bi, &block)) continue;
          if (block.id != bid) continue;
          for (size_t ii = 0; ii < block.instruction_count; ii++) {
            yyjson_val* ins = yyjson_arr_get(block.instructions, ii);
            const char* op = read_str(ins, "op");
            long long box_reg = 0;
            const char* field = NULL;
            struct GenericPureTypedObjectFieldSlotView field_slot;
            if (!op || strcmp(op, "field_set") != 0) continue;
            box_reg = read_int(ins, "box");
            field = read_str(ins, "field");
            if (generic_pure_read_typed_object_field_slot(
                    box_reg,
                    field,
                    &field_slot) &&
                (typed_object_storage_is_unsigned_exact(field_slot.storage) ||
                 typed_object_storage_is_signed_exact(field_slot.storage))) {
              last_exact_status_ii = ii;
            }
          }
          break;
        }
        if (last_exact_status_ii == (size_t)-1) return 0;
        snprintf(
            label,
            label_len,
            "exact_status_continue_%lld_%zu",
            bid,
            last_exact_status_ii);
        return 1;
      }
      auto const char* phi_predecessor_label(long long pred, char* buf, size_t buf_len) {
        if (!buf || buf_len == 0) return "%bb0";
        for (size_t i = 0; i < block_tail_labels_n; i++) {
          if (block_tail_labels[i].bid == pred && block_tail_labels[i].label[0]) {
            snprintf(buf, buf_len, "%%%s", block_tail_labels[i].label);
            return buf;
          }
        }
        {
          char tail_label[96];
          if (find_exact_status_tail_label_for_block(
                  pred,
                  tail_label,
                  sizeof(tail_label))) {
            set_block_tail_label(pred, tail_label);
            snprintf(buf, buf_len, "%%%s", tail_label);
            return buf;
          }
        }
        snprintf(buf, buf_len, "%%bb%lld", pred);
        return buf;
      }
      auto void emit_phi(long long dst, struct Incoming* in, int in_n){
        int phi_ty = get_type(dst);
        struct GenericPureVariantBinding variant_binding;
        int variant_phi = get_variant_binding(dst, &variant_binding) &&
            variant_binding.tag_reg == dst &&
            variant_binding.has_payload;
        EMIT("  %%r%lld = phi %s ", dst, phi_ty == T_I1 ? "i1" : "i64");
        for (int i=0;i<in_n;i++){
          char pred_label[128];
          long long cv;
          long long val_reg = resolve_copy_source_reg(in[i].val_reg);
          int hc=has_const(val_reg,&cv);
          if (phi_ty == T_I1) {
            if (hc) EMIT("[ %s, %s ]%s", cv ? "true" : "false", phi_predecessor_label(in[i].pred, pred_label, sizeof(pred_label)), (i+1<in_n)?", ":"");
            else EMIT("[ %%r%lld, %s ]%s", val_reg, phi_predecessor_label(in[i].pred, pred_label, sizeof(pred_label)), (i+1<in_n)?", ":"");
          } else {
            EMIT("[ %s%lld%s, %s ]%s", hc?"":"%r", hc?cv:val_reg, hc?"":"", phi_predecessor_label(in[i].pred, pred_label, sizeof(pred_label)), (i+1<in_n)?", ":"");
          }
        }
        EMIT("\n");
        if (variant_phi) {
          EMIT("  %%r%lld_tag = phi i64 ", dst);
          for (int i = 0; i < in_n; i++) {
            char pred_label[128];
            struct GenericPureVariantBinding incoming_variant;
            memset(&incoming_variant, 0, sizeof(incoming_variant));
            if (get_variant_binding(in[i].val_reg, &incoming_variant) &&
                incoming_variant.tag_reg) {
              EMIT("[ %%r%lld_tag, %s ]%s",
                   incoming_variant.tag_reg,
                   phi_predecessor_label(in[i].pred, pred_label, sizeof(pred_label)),
                   (i + 1 < in_n) ? ", " : "");
            } else {
              long long tag_value = incoming_variant.tag_const;
              EMIT("[ %lld, %s ]%s",
                   tag_value,
                   phi_predecessor_label(in[i].pred, pred_label, sizeof(pred_label)),
                   (i + 1 < in_n) ? ", " : "");
            }
          }
          EMIT("\n");
        }
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
      #include "hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc"
      #include "hako_llvmc_ffi_same_module_function_emit.inc"
      if (hako_llvmc_emit_static_data_globals_for_root(f, root) < 0) {
        note_pure_unsupported_shape(
            -1,
            0,
            "static_data_plan",
            "unsupported_static_data_plan",
            NULL);
        fclose(f);
        yyjson_doc_free(d);
        goto GEN_ABORT;
      }
      if (emit_same_module_function_definitions(f) < 0) {
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
        if (active_observer_store_len_sum_region_ready &&
            (bid == active_observer_store_len_sum_region.observer_block ||
             bid == active_observer_store_len_sum_region.then_store_block ||
             bid == active_observer_store_len_sum_region.latch_block)) {
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
        if (active_loop_session_region_emitted &&
            bid == active_loop_session_region.loop_body) {
          emit_block_label(bid);
          EMIT("  unreachable\n");
          continue;
        }
        if (active_indexof_const_region_emitted &&
            bid == active_indexof_const_region.loop_body) {
          emit_block_label(bid);
          EMIT("  unreachable\n");
          continue;
        }
        if (active_string_dead_text_region_emitted &&
            (bid == active_string_dead_text_region.loop_body ||
             bid == active_string_dead_text_region.loop_exit)) {
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
        if (active_observer_store_len_sum_region_ready &&
            !active_observer_store_len_sum_region_emitted &&
            bid == active_observer_store_len_sum_region.header_block) {
          emit_block_label(bid);
          emit_array_text_observer_store_len_sum_region_call(
              &active_observer_store_len_sum_region);
          EMIT("  br label %%bb%lld\n", active_observer_store_len_sum_region.exit_block);
          active_observer_store_len_sum_region_emitted = 1;
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
        if (!active_loop_session_region_emitted &&
            match_array_text_loop_session_plan_by_header_metadata(
                bid,
                &active_loop_session_region) &&
            array_text_loop_session_region_ready(&active_loop_session_region)) {
          emit_block_label(bid);
          emit_array_text_loop_session_region_call(&active_loop_session_region);
          EMIT("  br label %%bb%lld\n", active_loop_session_region.loop_exit);
          active_loop_session_region_emitted = 1;
          continue;
        }
        if (!active_indexof_const_region_emitted &&
            match_array_text_indexof_const_region_plan_by_header_metadata(
                bid,
                &active_indexof_const_region) &&
            array_text_indexof_const_region_ready(&active_indexof_const_region)) {
          emit_block_label(bid);
          emit_array_text_indexof_const_region_call(&active_indexof_const_region);
          EMIT("  br label %%bb%lld\n", active_indexof_const_region.loop_exit);
          active_indexof_const_region_emitted = 1;
          continue;
        }
        if (!active_string_dead_text_region_emitted &&
            match_string_dead_text_region_plan_by_header_metadata(
                bid,
                &active_string_dead_text_region) &&
            string_dead_text_region_ready(&active_string_dead_text_region)) {
          emit_block_label(bid);
          emit_string_dead_text_region_closed_form_return(&active_string_dead_text_region);
          active_string_dead_text_region_emitted = 1;
          continue;
        }
        emit_block_label(bid);
        // phi first
        for (int pi=0; pi<phi_n; pi++) if (phis[pi].bid==bid){ for (int r=0;r<phis[pi].rec_n;r++){ emit_phi(phis[pi].recs[r].dst, phis[pi].recs[r].in, phis[pi].recs[r].in_n); } }
        if (bi == 0) {
          hako_llvmc_emit_direct_exact_runtime_mode_check(f);
          if (!emit_typed_object_layout_registrations(f)) {
            note_pure_unsupported_shape(
                -1,
                0,
                "typed_object_layout",
                "typed_object_layout_registration_failed",
                NULL);
            fclose(f);
            yyjson_doc_free(d);
            goto GEN_ABORT;
          }
          if (!emit_boxed_sum_abi_layout_registrations(f)) {
            note_pure_unsupported_shape(
                -1,
                0,
                "boxed_sum_abi_layout",
                "boxed_sum_abi_layout_registration_failed",
                NULL);
            fclose(f);
            yyjson_doc_free(d);
            goto GEN_ABORT;
          }
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
      #include "hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch.inc"
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
