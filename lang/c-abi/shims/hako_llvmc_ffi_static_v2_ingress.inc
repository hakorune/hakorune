      /* Check the same first-match plan view used by the actual consumer.
       * Source callee text is deliberately not a target authority. */
      auto int static_v2_check_expanded_ingress(long long bid, size_t ii, yyjson_val* ins) {
        const struct HakoStaticV2Index* index = &invocation->static_v2;
        if (!index->frame || !index->frame->expanded_function_count) return 1;
        if (hako_static_v2_expanded_find(index, program->entry.fn) >= 0) {
          note_pure_unsupported_shape(bid, ii, "formal", "static_v2_expanded_entry", NULL);
          return 0;
        }
        const char* entry_symbol = program->selected_dynamic ? read_str(program->entry.fn, "name")
            : hako_llvmc_effective_entry_symbol(&program->entry.selection);
        for (size_t ei = 0; ei < index->frame->expanded_function_count; ei++) {
          const char* target = index->frame->expanded_functions[ei].internal_target;
          if ((entry_symbol && !strcmp(target, entry_symbol)) ||
              (!program->selected_dynamic && !strcmp(target, hako_llvmc_entry_symbol_default()))) {
            note_pure_unsupported_shape(bid, ii, "formal", "static_v2_entry_symbol_collision", NULL);
            return 0;
          }
        }
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanGlobalCallView global;
        struct LoweringPlanUserBoxMethodView user;
        struct LoweringPlanExternCallView external;
        struct LoweringPlanGenericMethodView method;
        const char* symbol = NULL;
        if (read_lowering_plan_global_call_view(entry, &global)) symbol = global.target_symbol;
        else if (read_lowering_plan_user_box_method_view(entry, &user)) symbol = user.target_symbol;
        else if (read_lowering_plan_extern_call_view(entry, &external)) symbol = external.symbol;
        else if (read_lowering_plan_generic_method_view(entry, &method)) symbol = method.symbol;
        if (!symbol) return 1;
        for (size_t ei = 0; ei < index->frame->expanded_function_count; ei++) {
          if (!strcmp(symbol, index->frame->expanded_functions[ei].internal_target)) {
            note_pure_unsupported_shape(bid, ii, "mir_call", "static_v2_internal_symbol_ingress", NULL);
            return 0;
          }
        }
        if (hako_static_v2_expanded_target(index, symbol) < 0) return 1;
        yyjson_val* call = yyjson_obj_get(ins, "mir_call");
        const hako_llvmc_published_static_method_call_v1* selected = NULL;
        int status = hako_llvmc_published_static_method_peek_i64_global_row_v1(
            read_str(fn0, "name"), bid, ii, read_str(yyjson_obj_get(call, "callee"), "type"),
            read_int(ins, "dst"), yyjson_obj_get(call, "args"), &selected);
        if (hako_static_v2_text(yyjson_obj_get(ins, "op"), "mir_call") &&
            status == HAKO_LLVMC_PUBLISHED_ROW_READY && selected &&
            (selected->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_STATIC_METHOD ||
             selected->kind == HAKO_LLVMC_PUBLISHED_CALL_KIND_FREE_FUNCTION)) return 1;
        note_pure_unsupported_shape(bid, ii, "mir_call", "static_v2_old_abi_ingress", NULL);
        return 0;
      }
