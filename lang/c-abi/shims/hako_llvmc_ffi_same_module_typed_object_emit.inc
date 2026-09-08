      /*
       * Same-module typed-object body emit.
       *
       * This file emits typed-object newbox and field access bodies from
       * MIR-owned TypedObjectPlan bindings only. It must not infer layout or
       * slots from raw declaration names.
       */
      enum {
        SAME_MODULE_DIRECT_SLOT_OBJECT_HEADER_BYTES = 24,
        SAME_MODULE_DIRECT_SLOT_CELL_BYTES = 16,
        SAME_MODULE_DIRECT_SLOT_CELL_PAYLOAD_OFFSET_BYTES = 8
      };

      auto int same_module_function_direct_slot_nativedirect_backend_enabled(void) {
        return (allocation_config.runtime_flags &
                HAKO_LLVMC_RUNTIME_MODE_REQUIRES_DIRECT_SLOT_EXACT) != 0;
      }

      auto int same_module_function_direct_slot_nativedirect_storage_supported(
          const char* storage) {
        return typed_object_storage_is_exact_slot_i64(storage) ||
               typed_object_storage_is_exact_slot_u64(storage) ||
               typed_object_storage_is_exact_slot_handle(storage);
      }

      auto void same_module_function_emit_direct_slot_payload_ptr(
          long long box_ref_reg,
          long long slot,
          const char* ptr_name,
          long long bid,
          size_t ii) {
        char receiver_ref[96];
        long long payload_offset =
            SAME_MODULE_DIRECT_SLOT_OBJECT_HEADER_BYTES +
            slot * SAME_MODULE_DIRECT_SLOT_CELL_BYTES +
            SAME_MODULE_DIRECT_SLOT_CELL_PAYLOAD_OFFSET_BYTES;
        same_module_function_format_i64_ref(
            receiver_ref,
            sizeof(receiver_ref),
            box_ref_reg);
        EMIT("  %%direct_slot_base_%lld_%zu = and i64 %s, -2\n",
             bid,
             ii,
             receiver_ref);
        EMIT("  %%direct_slot_payload_addr_%lld_%zu = add i64 %%direct_slot_base_%lld_%zu, %lld\n",
             bid,
             ii,
             bid,
             ii,
             payload_offset);
        EMIT("  %%%s = inttoptr i64 %%direct_slot_payload_addr_%lld_%zu to ptr\n",
             ptr_name,
             bid,
             ii);
      }

      auto int same_module_function_emit_newbox(long long bid, size_t ii, yyjson_val* ins) {
        long long dst = read_int(ins, "dst");
        int intrinsic = hako_llvmc_published_intrinsic_array_take_v1(
            fn0 ? read_str(fn0, "name") : NULL, bid, ii, ins);
        if (intrinsic < 0) return -1;
        if (intrinsic > 0) {
          const char* symbol = hako_llvmc_direct_array_i64_exact_enabled()
              ? "nyash.array.direct_i64.birth_h" : "nyash.array.birth_h";
          EMIT("  %%r%lld = call i64 @\"%s\"()\n", dst, symbol);
          same_module_function_publish_i64_origin(dst, ORG_ARRAY_BIRTH);
          return 1;
        }

        const char* tname = read_str(ins, "type");
        struct GenericPureTypedObjectNewPlanView plan;
        enum NamedAllocationConsumer consumer = select_named_allocation_consumer(
                root,
            NAMED_ALLOCATION_SAME_MODULE, tname, dst, 0,
            hako_llvmc_direct_array_i64_exact_enabled(), &plan);
        if (consumer == NAMED_ALLOCATION_DIRECT_ARRAY || consumer == NAMED_ALLOCATION_ARRAY) {
          same_module_function_emit_i64_birth_call(
              dst,
              consumer == NAMED_ALLOCATION_DIRECT_ARRAY
                  ? "nyash.array.direct_i64.birth_h" : "nyash.array.birth_h",
              ORG_ARRAY_BIRTH);
          return 1;
        }
        if (consumer == NAMED_ALLOCATION_MAP) {
          same_module_function_emit_i64_birth_call(dst, "nyash.map.birth_h", ORG_MAP_BIRTH);
          return 1;
        }
        if (consumer == NAMED_ALLOCATION_TYPED_OBJECT) {
          EMIT("  %%r%lld = call i64 @\"nyash.object.new_typed_hi\"(i64 %lld, i64 %lld)\n",
               dst, plan.type_id, plan.field_count);
          same_module_function_publish_i64_origin(dst, ORG_TYPED_OBJECT);
          set_typed_object_binding(dst, plan.typed_plan_index);
          return 1;
        }
        return 0;
      }

      auto void same_module_function_emit_exact_status_trap(
          const char* status_name,
          long long bid,
          size_t ii) {
        EMIT("  %%exact_status_ok_%lld_%zu = icmp ne i64 %%%s, 0\n", bid, ii, status_name);
        EMIT("  br i1 %%exact_status_ok_%lld_%zu, label %%exact_status_continue_%lld_%zu, label %%exact_status_trap_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("exact_status_trap_%lld_%zu:\n", bid, ii);
        EMIT("  call void @llvm.trap()\n");
        EMIT("  unreachable\n");
        EMIT("exact_status_continue_%lld_%zu:\n", bid, ii);
        note_exact_status_continue_label(bid, ii);
      }

      auto void same_module_function_emit_unsigned_nonnegative_trap(
          long long value_reg,
          long long bid,
          size_t ii) {
        EMIT("  %%exact_unsigned_nonnegative_%lld_%zu = icmp sge i64 %%r%lld, 0\n",
             bid,
             ii,
             value_reg);
        EMIT("  br i1 %%exact_unsigned_nonnegative_%lld_%zu, label %%exact_unsigned_continue_%lld_%zu, label %%exact_unsigned_trap_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("exact_unsigned_trap_%lld_%zu:\n", bid, ii);
        EMIT("  call void @llvm.trap()\n");
        EMIT("  unreachable\n");
        EMIT("exact_unsigned_continue_%lld_%zu:\n", bid, ii);
      }

      auto int same_module_function_emit_typed_object_field_set(
          yyjson_val* ins,
          long long bid,
          size_t ii) {
        long long box_reg = read_int(ins, "box");
        long long box_ref_reg = box_reg;
        long long value_reg = read_int(ins, "value");
        const char* field = read_str(ins, "field");
        int typed_plan_index = -1;
        long long slot = -1;
        const char* storage = NULL;
        long long value_base = resolve_copy_source_reg(value_reg);
        long long value_const = 0;
        int value_is_const = has_const(value_base, &value_const);
        char ab[256];
        get_receiver_forward_alias_src(box_reg, &box_ref_reg);
        if (!(box_reg && value_reg)) return 0;
        if (!get_typed_object_binding(box_reg, &typed_plan_index)) return 0;
        if (!typed_object_plan_field_runtime_slot_with_storage(
                typed_plan_index,
                field,
                &slot,
                &storage)) return 0;
        if (same_module_function_direct_slot_nativedirect_backend_enabled() &&
            same_module_function_direct_slot_nativedirect_storage_supported(storage)) {
          char ptr_name[96];
          char value_ref[96];
          if (typed_object_storage_is_exact_slot_u64(storage)) {
            if (value_is_const && value_const < 0) return 0;
            if (!value_is_const) {
              same_module_function_emit_unsigned_nonnegative_trap(
                  value_base,
                  bid,
                  ii);
            }
          }
          snprintf(ptr_name, sizeof(ptr_name), "direct_slot_payload_ptr_%lld_%zu", bid, ii);
          same_module_function_emit_direct_slot_payload_ptr(
              box_ref_reg,
              slot,
              ptr_name,
              bid,
              ii);
          same_module_function_format_i64_ref(
              value_ref,
              sizeof(value_ref),
              value_reg);
          EMIT("  store i64 %s, ptr %%%s\n", value_ref, ptr_name);
          EMIT("  br label %%exact_status_continue_%lld_%zu\n", bid, ii);
          EMIT("exact_status_continue_%lld_%zu:\n", bid, ii);
          note_exact_status_continue_label(bid, ii);
          return 1;
        }
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), box_ref_reg, 1);
        {
          size_t used = strlen(ab);
          if (used >= sizeof(ab)) return 0;
          snprintf(ab + used, sizeof(ab) - used, ", i64 %lld", slot);
        }
        append_i64_arg_ref(ab, sizeof(ab), value_reg, 0);
        if (typed_object_exact_slot_helper_enabled() &&
            typed_object_storage_is_exact_slot_u64(storage)) {
          char status_name[96];
          if (value_is_const && value_const < 0) return 0;
          if (!value_is_const) {
            same_module_function_emit_unsigned_nonnegative_trap(
                value_base,
                bid,
                ii);
          }
          snprintf(status_name, sizeof(status_name), "exact_slot_set_u64_%lld_%zu", bid, ii);
          EMIT("  %%%s = call i64 @\"nyash.object.exact_slot_set_u64_hiu\"(%s)\n",
               status_name,
               ab);
          same_module_function_emit_exact_status_trap(status_name, bid, ii);
          return 1;
        }
        if (typed_object_exact_slot_helper_enabled() &&
            typed_object_storage_is_exact_slot_i64(storage)) {
          char status_name[96];
          snprintf(status_name, sizeof(status_name), "exact_slot_set_i64_%lld_%zu", bid, ii);
          EMIT("  %%%s = call i64 @\"nyash.object.exact_slot_set_i64_hii\"(%s)\n",
               status_name,
               ab);
          same_module_function_emit_exact_status_trap(status_name, bid, ii);
          return 1;
        }
        if (typed_object_exact_slot_helper_enabled() &&
            typed_object_storage_is_exact_slot_handle(storage)) {
          char status_name[96];
          snprintf(status_name, sizeof(status_name), "exact_slot_set_handle_%lld_%zu", bid, ii);
          EMIT("  %%%s = call i64 @\"nyash.object.exact_slot_set_handle_hii\"(%s)\n",
               status_name,
               ab);
          same_module_function_emit_exact_status_trap(status_name, bid, ii);
          return 1;
        }
        if (typed_object_storage_is_unsigned_exact(storage)) {
          char status_name[96];
          if (value_is_const && value_const < 0) return 0;
          if (!value_is_const) {
            same_module_function_emit_unsigned_nonnegative_trap(
                value_base,
                bid,
                ii);
          }
          snprintf(status_name, sizeof(status_name), "exact_field_set_u64_%lld_%zu", bid, ii);
          EMIT("  %%%s = call i64 @\"nyash.object.field_set_u64_hiu\"(%s)\n",
               status_name,
               ab);
          same_module_function_emit_exact_status_trap(status_name, bid, ii);
          return 1;
        }
        if (typed_object_storage_is_signed_exact(storage)) {
          char status_name[96];
          snprintf(status_name, sizeof(status_name), "exact_field_set_i64_%lld_%zu", bid, ii);
          EMIT("  %%%s = call i64 @\"nyash.object.field_set_i64_hii\"(%s)\n",
               status_name,
               ab);
          same_module_function_emit_exact_status_trap(status_name, bid, ii);
          return 1;
        }
        EMIT("  call void @\"nyash.object.field_set_hii\"(%s)\n", ab);
        return 1;
      }

      auto int same_module_function_emit_typed_object_field_get(
          yyjson_val* ins,
          long long bid,
          size_t ii) {
        long long dst = read_int(ins, "dst");
        long long box_reg = read_int(ins, "box");
        long long box_ref_reg = box_reg;
        const char* field = read_str(ins, "field");
        int typed_plan_index = -1;
        long long slot = -1;
        const char* storage = NULL;
        char ab[256];
        get_receiver_forward_alias_src(box_reg, &box_ref_reg);
        if (!(dst && box_reg)) return 0;
        if (!get_typed_object_binding(box_reg, &typed_plan_index)) return 0;
        if (!typed_object_plan_field_runtime_slot_with_storage(
                typed_plan_index,
                field,
                &slot,
                &storage)) return 0;
        if (same_module_function_direct_slot_nativedirect_backend_enabled() &&
            same_module_function_direct_slot_nativedirect_storage_supported(storage)) {
          char ptr_name[96];
          snprintf(ptr_name, sizeof(ptr_name), "direct_slot_payload_ptr_%lld_%zu", bid, ii);
          same_module_function_emit_direct_slot_payload_ptr(
              box_ref_reg,
              slot,
              ptr_name,
              bid,
              ii);
          EMIT("  %%r%lld = load i64, ptr %%%s\n", dst, ptr_name);
          if (!same_module_function_publish_reg_metadata(dst)) {
            set_type(dst, T_I64);
          }
          return 1;
        }
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), box_ref_reg, 1);
        {
          size_t used = strlen(ab);
          if (used >= sizeof(ab)) return 0;
          snprintf(ab + used, sizeof(ab) - used, ", i64 %lld", slot);
        }
        if (typed_object_exact_slot_helper_enabled() &&
            typed_object_storage_is_exact_slot_u64(storage)) {
          EMIT("  %%r%lld = call i64 @\"nyash.object.exact_slot_get_u64_hii\"(%s)\n", dst, ab);
        } else if (typed_object_exact_slot_helper_enabled() &&
                   typed_object_storage_is_exact_slot_i64(storage)) {
          EMIT("  %%r%lld = call i64 @\"nyash.object.exact_slot_get_i64_hii\"(%s)\n", dst, ab);
        } else if (typed_object_exact_slot_helper_enabled() &&
                   typed_object_storage_is_exact_slot_handle(storage)) {
          EMIT("  %%r%lld = call i64 @\"nyash.object.exact_slot_get_handle_hii\"(%s)\n", dst, ab);
        } else if (typed_object_storage_is_unsigned_exact(storage)) {
          EMIT("  %%r%lld = call i64 @\"nyash.object.field_get_u64_hii\"(%s)\n", dst, ab);
        } else if (typed_object_storage_is_signed_exact(storage)) {
          EMIT("  %%r%lld = call i64 @\"nyash.object.field_get_i64_hii\"(%s)\n", dst, ab);
        } else {
          EMIT("  %%r%lld = call i64 @\"nyash.object.field_get_hii\"(%s)\n", dst, ab);
        }
        if (!same_module_function_publish_reg_metadata(dst)) {
          set_type(dst, T_I64);
        }
        return 1;
      }
