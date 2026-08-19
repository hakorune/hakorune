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
        #include "hako_llvmc_ffi_pinned_text_provenance_block_dispatch.inc"
        // phi first
        for (int pi=0; pi<phi_n; pi++) if (phis[pi].bid==bid){ for (int r=0;r<phis[pi].rec_n;r++){ emit_phi(bid, phis[pi].recs[r].dst, phis[pi].recs[r].in, phis[pi].recs[r].in_n); } }
        if (bi == 0) {
          hako_llvmc_emit_direct_exact_runtime_mode_check(f);
          if (!emit_typed_object_layout_registrations(f)) {
            note_pure_unsupported_shape(
                -1,
                0,
                "typed_object_layout",
                "typed_object_layout_registration_failed",
                NULL);
            HAKO_LLVMC_CLOSE_GENERIC_OUTPUT();
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
            HAKO_LLVMC_CLOSE_GENERIC_OUTPUT();
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
