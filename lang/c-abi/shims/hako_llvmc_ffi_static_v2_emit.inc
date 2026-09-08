      /* Shared actual consumer. Status and bits are physical ABI inputs, never
       * inferred from old register contents. Both walkers call this after skip. */
      auto int emit_static_v2_instruction(size_t fn_index, long long bid, size_t ii, yyjson_val* ins) {
        struct HakoStaticV2Index* index = &invocation->static_v2;
        if (!index->frame) return 0;
        uint32_t dst;
        if (hako_static_v2_u32(yyjson_obj_get(ins, "dst"), &dst)) {
          const struct GenericPureStringConst* text = get_str_const_record(dst);
          const char* op = read_str(ins, "op");
          if (text && text->reg == dst && op && (!strcmp(op, "const") || !strcmp(op, "binop"))) {
            char global[96];
            if (fn0 == program->entry.fn) snprintf(global, sizeof(global), "@.str_reg_%u", dst);
            else snprintf(global, sizeof(global), "@.fnstr_%zu_%u", fn_index, dst);
            emit_string_const_handle(f, dst, global, text->byte_len);
            EMIT("  %%literal_ok_%lld_%zu = icmp ne i64 %%r%u, 0\n", bid, ii, dst);
            EMIT("  br i1 %%literal_ok_%lld_%zu, label %%exact_status_continue_%lld_%zu, label %%literal_contract_%lld_%zu\n",
                 bid, ii, bid, ii, bid, ii);
            EMIT("literal_contract_%lld_%zu:\n  call void @llvm.trap()\n  unreachable\n", bid, ii);
            EMIT("exact_status_continue_%lld_%zu:\n", bid, ii);
            note_exact_status_continue_label(bid, ii);
            mark_string_const_box_emitted(dst);
            generic_pure_publish_i64_origin(dst, ORG_STRING);
            return 1;
          }
          int vi = hako_static_v2_value_find(index, fn0, dst);
          if (vi >= 0) {
            if (index->values[vi].instruction != ins || index->values[vi].used) return -1;
            index->values[vi].used = 1;
            /* Exact constants need no side register. Retain the old lane only
             * when the original-consumer closure demands it. */
            if (!index->frame->values[vi].flags) return 1;
          }
        }
        int mi = hako_static_v2_map_find(index, fn0, ins);
        if (mi < 0) return 0;
        if (index->maps[mi].used) return -1;
        index->maps[mi].used = 1;
        const hako_llvmc_map_operation_v2* row = &index->frame->map_operations[mi];
        if (row->kind == HAKO_LLVMC_MAP_OP_ALLOCATE) {
          uint32_t result;
          if (!hako_static_v2_u32(yyjson_obj_get(ins, "dst"), &result)) return -1;
          EMIT("  %%r%u = call i64 @\"nyash.map.birth_h\"()\n", result);
          generic_pure_publish_i64_origin(result, ORG_MAP_BIRTH);
          return 1;
        }
        uint32_t receiver, key, value;
        if (!hako_static_v2_u32(yyjson_obj_get(ins, "receiver"), &receiver) ||
            !hako_static_v2_u32(yyjson_obj_get(ins, "key"), &key) ||
            !hako_static_v2_u32(yyjson_obj_get(ins, "value"), &value)) return -1;
        int vi = hako_static_v2_value_find(index, fn0, value);
        if (vi < 0) return -1;
        const hako_llvmc_value_projection_v2* projection = &index->frame->values[vi];
        char refs[2][64];
        uint32_t regs[2] = {receiver, key};
        for (size_t i = 0; i < 2; i++) {
          long long bits = 0, base = resolve_copy_source_reg(regs[i]);
          if (has_const(base, &bits)) snprintf(refs[i], sizeof(refs[i]), "%lld", bits);
          else snprintf(refs[i], sizeof(refs[i]), "%%r%lld", base);
        }
        EMIT("  %%map_status_%lld_%zu = call i32 @\"nyash.map.literal_store_v1\"(i64 %s, i64 %s, i32 %u, i64 %llu)\n",
             bid, ii, refs[0], refs[1], projection->value_kind,
             (unsigned long long)projection->payload);
        EMIT("  %%map_ok_%lld_%zu = icmp eq i32 %%map_status_%lld_%zu, 0\n", bid, ii, bid, ii);
        EMIT("  br i1 %%map_ok_%lld_%zu, label %%exact_status_continue_%lld_%zu, label %%map_contract_%lld_%zu\n",
             bid, ii, bid, ii, bid, ii);
        EMIT("map_contract_%lld_%zu:\n  call void @llvm.trap()\n  unreachable\n", bid, ii);
        EMIT("exact_status_continue_%lld_%zu:\n", bid, ii);
        note_exact_status_continue_label(bid, ii);
        return 1;
      }
