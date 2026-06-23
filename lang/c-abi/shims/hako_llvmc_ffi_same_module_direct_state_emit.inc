      /*
       * Same-module direct-state emit helpers.
       *
       * This seam owns exact-slot state mutation for selected same-module
       * recordSuccess calls. It consumes typed-object slot metadata that has
       * already been selected upstream; it must not discover new call routes.
       */

      auto int same_module_function_direct_state_slot_matches(
          long long recv,
          const char* field,
          long long expected_slot,
          int expect_u64) {
        int typed_plan_index = -1;
        long long slot = -1;
        const char* storage = NULL;
        long long recv_base = resolve_copy_source_reg(recv);
        if (!(recv && field)) return 0;
        if (!get_typed_object_binding(recv, &typed_plan_index) &&
            !get_typed_object_binding(recv_base, &typed_plan_index)) {
          return 0;
        }
        if (!typed_object_plan_field_runtime_slot_with_storage(
                typed_plan_index,
                field,
                &slot,
                &storage)) {
          return 0;
        }
        if (slot != expected_slot) return 0;
        return expect_u64
                   ? typed_object_storage_is_exact_slot_u64(storage)
                   : typed_object_storage_is_exact_slot_i64(storage);
      }

      auto int same_module_function_alloc_result_direct_state_slots_ok(long long recv) {
        return same_module_function_direct_state_slot_matches(recv, "last_reason", 2, 0) &&
               same_module_function_direct_state_slot_matches(recv, "last_ok", 3, 0) &&
               same_module_function_direct_state_slot_matches(recv, "success_count", 5, 1) &&
               same_module_function_direct_state_slot_matches(
                   recv,
                   "reusable_success_count",
                   7,
                   1) &&
               same_module_function_direct_state_slot_matches(
                   recv,
                   "active_success_count",
                   8,
                   1);
      }

      auto void same_module_function_emit_direct_state_payload_ptr(
          long long recv,
          long long slot,
          const char* tag,
          long long bid,
          size_t ii) {
        char recv_ref_text[96];
        long long recv_ref = recv;
        long long payload_offset =
            SAME_MODULE_DIRECT_SLOT_OBJECT_HEADER_BYTES +
            slot * SAME_MODULE_DIRECT_SLOT_CELL_BYTES +
            SAME_MODULE_DIRECT_SLOT_CELL_PAYLOAD_OFFSET_BYTES;
        get_receiver_forward_alias_src(recv, &recv_ref);
        same_module_function_format_i64_ref(
            recv_ref_text,
            sizeof(recv_ref_text),
            recv_ref);
        EMIT("  %%direct_state_%s_base_%lld_%zu = and i64 %s, -2\n",
             tag,
             bid,
             ii,
             recv_ref_text);
        EMIT("  %%direct_state_%s_payload_addr_%lld_%zu = add i64 %%direct_state_%s_base_%lld_%zu, %lld\n",
             tag,
             bid,
             ii,
             tag,
             bid,
             ii,
             payload_offset);
        EMIT("  %%direct_state_%s_ptr_%lld_%zu = inttoptr i64 %%direct_state_%s_payload_addr_%lld_%zu to ptr\n",
             tag,
             bid,
             ii,
             tag,
             bid,
             ii);
      }

      auto void same_module_function_emit_direct_state_store_const(
          long long recv,
          long long slot,
          long long value,
          const char* tag,
          long long bid,
          size_t ii) {
        same_module_function_emit_direct_state_payload_ptr(recv, slot, tag, bid, ii);
        EMIT("  store i64 %lld, ptr %%direct_state_%s_ptr_%lld_%zu\n",
             value,
             tag,
             bid,
             ii);
      }

      auto void same_module_function_emit_direct_state_rmw_add1(
          long long recv,
          long long slot,
          const char* tag,
          long long bid,
          size_t ii) {
        same_module_function_emit_direct_state_payload_ptr(recv, slot, tag, bid, ii);
        EMIT("  %%direct_state_%s_value_%lld_%zu = load i64, ptr %%direct_state_%s_ptr_%lld_%zu\n",
             tag,
             bid,
             ii,
             tag,
             bid,
             ii);
        EMIT("  %%direct_state_%s_next_%lld_%zu = add i64 %%direct_state_%s_value_%lld_%zu, 1\n",
             tag,
             bid,
             ii,
             tag,
             bid,
             ii);
        EMIT("  store i64 %%direct_state_%s_next_%lld_%zu, ptr %%direct_state_%s_ptr_%lld_%zu\n",
             tag,
             bid,
             ii,
             tag,
             bid,
             ii);
      }

      auto int same_module_function_emit_direct_state_record_alloc_success_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          long long dst,
          long long recv,
          yyjson_val* args,
          const char* bname,
          const char* mname) {
        long long selected_kind = 0;
        long long selected_kind_const = 0;
        int selected_kind_is_const = 0;
        char kind_ref[64];
        (void)f;
        if (!(bname && !strcmp(bname, "HakoAllocObjectLifecycleAllocResult") &&
              mname && !strcmp(mname, "recordSuccess"))) {
          return 0;
        }
        if (!same_module_function_direct_slot_nativedirect_backend_enabled()) {
          return 0;
        }
        if (!(args && yyjson_is_arr(args) && yyjson_arr_size(args) == 1)) return -1;
        if (!same_module_function_read_arg_reg(args, 0, &selected_kind)) return -1;
        if (!(recv && selected_kind && is_plain_i64_value(selected_kind))) return -1;
        if (!same_module_function_alloc_result_direct_state_slots_ok(recv)) return -1;

        same_module_function_format_i64_ref(kind_ref, sizeof(kind_ref), selected_kind);
        selected_kind_is_const =
            has_const(resolve_copy_source_reg(selected_kind), &selected_kind_const);
        same_module_function_emit_direct_state_store_const(
            recv,
            2,
            0,
            "alloc_last_reason",
            bid,
            ii);
        same_module_function_emit_direct_state_store_const(
            recv,
            3,
            1,
            "alloc_last_ok",
            bid,
            ii);
        same_module_function_emit_direct_state_rmw_add1(
            recv,
            5,
            "alloc_success_count",
            bid,
            ii);

        if (selected_kind_is_const) {
          if (selected_kind_const == 1) {
            same_module_function_emit_direct_state_rmw_add1(
                recv,
                7,
                "alloc_reusable_success_count",
                bid,
                ii);
          } else if (selected_kind_const == 2) {
            same_module_function_emit_direct_state_rmw_add1(
                recv,
                8,
                "alloc_active_success_count",
                bid,
                ii);
          }
          if (dst) {
            EMIT("  %%r%lld = add i64 1, 0\n", dst);
            set_type(dst, T_I64);
          }
          return 1;
        }

        EMIT("  %%direct_state_alloc_reusable_kind_%lld_%zu = icmp eq i64 %s, 1\n",
             bid,
             ii,
             kind_ref);
        EMIT("  br i1 %%direct_state_alloc_reusable_kind_%lld_%zu, label %%direct_state_alloc_reusable_%lld_%zu, label %%direct_state_alloc_after_reusable_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("direct_state_alloc_reusable_%lld_%zu:\n", bid, ii);
        same_module_function_emit_direct_state_rmw_add1(
            recv,
            7,
            "alloc_reusable_success_count",
            bid,
            ii);
        EMIT("  br label %%direct_state_alloc_after_reusable_%lld_%zu\n", bid, ii);
        EMIT("direct_state_alloc_after_reusable_%lld_%zu:\n", bid, ii);

        EMIT("  %%direct_state_alloc_active_kind_%lld_%zu = icmp eq i64 %s, 2\n",
             bid,
             ii,
             kind_ref);
        EMIT("  br i1 %%direct_state_alloc_active_kind_%lld_%zu, label %%direct_state_alloc_active_%lld_%zu, label %%direct_state_alloc_done_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("direct_state_alloc_active_%lld_%zu:\n", bid, ii);
        same_module_function_emit_direct_state_rmw_add1(
            recv,
            8,
            "alloc_active_success_count",
            bid,
            ii);
        EMIT("  br label %%direct_state_alloc_done_%lld_%zu\n", bid, ii);
        EMIT("direct_state_alloc_done_%lld_%zu:\n", bid, ii);

        if (dst) {
          EMIT("  %%r%lld = add i64 1, 0\n", dst);
          set_type(dst, T_I64);
        }
        return 1;
      }
