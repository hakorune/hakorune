      auto int emit_boxed_sum_unit_variant_make(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          long long tag,
          const char* enum_name) {
        int boxed_plan_index = -1;
        if (!(enum_name &&
              find_boxed_sum_abi_plan_index(enum_name, &boxed_plan_index) &&
              boxed_sum_abi_variant_payload_storage_is_none(boxed_plan_index, tag))) {
          return 0;
        }
        long long type_id = boxed_sum_abi_runtime_type_id(boxed_plan_index);
        if (!(dst && type_id > 0)) {
          note_pure_unsupported_shape(
              bid,
              ii,
              op,
              "boxed_sum_abi_variant_make_invalid_plan",
              NULL);
          return -1;
        }
        EMIT("  %%r%lld = call i64 @\"nyash.object.new_typed_hi\"(i64 %lld, i64 2)\n",
             dst,
             type_id);
        EMIT("  %%boxed_sum_tag_set_%lld_%zu = call i64 @\"nyash.object.field_set_i64_hii\"(i64 %%r%lld, i64 0, i64 %lld)\n",
             bid,
             ii,
             dst,
             tag);
        set_type(dst, T_I64);
        return 1;
      }

      auto int emit_boxed_sum_tag_read(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          long long value,
          const char* enum_name) {
        int boxed_plan_index = -1;
        if (!(enum_name && find_boxed_sum_abi_plan_index(enum_name, &boxed_plan_index))) {
          return 0;
        }
        long long type_id = boxed_sum_abi_runtime_type_id(boxed_plan_index);
        long long value_base = resolve_copy_source_reg(value);
        if (!(dst && value_base && type_id > 0)) {
          note_pure_unsupported_shape(
              bid,
              ii,
              op,
              "boxed_sum_abi_variant_tag_malformed_operands",
              NULL);
          return -1;
        }
        EMIT("  %%boxed_sum_type_%lld_%zu = call i64 @\"nyash.object.type_id_h\"(i64 %%r%lld)\n",
             bid,
             ii,
             value_base);
        EMIT("  %%boxed_sum_type_ok_%lld_%zu = icmp eq i64 %%boxed_sum_type_%lld_%zu, %lld\n",
             bid,
             ii,
             bid,
             ii,
             type_id);
        EMIT("  br i1 %%boxed_sum_type_ok_%lld_%zu, label %%boxed_sum_type_continue_%lld_%zu, label %%boxed_sum_type_trap_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("boxed_sum_type_trap_%lld_%zu:\n", bid, ii);
        EMIT("  call void @llvm.trap()\n");
        EMIT("  unreachable\n");
        EMIT("boxed_sum_type_continue_%lld_%zu:\n", bid, ii);
        note_exact_status_continue_label(bid, ii);
        EMIT("  %%r%lld = call i64 @\"nyash.object.field_get_i64_hii\"(i64 %%r%lld, i64 0)\n",
             dst,
             value_base);
        set_type(dst, T_I64);
        return 1;
      }

      auto int boxed_sum_unit_binding_plan_index(
          struct GenericPureVariantBinding* binding,
          int* out_plan_index) {
        if (!(binding && binding->enum_name && !binding->has_payload)) {
          return 0;
        }
        int plan_index = -1;
        if (!find_boxed_sum_abi_plan_index(binding->enum_name, &plan_index)) {
          return 0;
        }
        if (!boxed_sum_abi_variant_payload_storage_is_none(plan_index, binding->tag_const)) {
          return 0;
        }
        if (out_plan_index) {
          *out_plan_index = plan_index;
        }
        return 1;
      }

      auto int emit_boxed_sum_tag_operand_ref(
          long long bid,
          size_t ii,
          const char* op,
          const char* side,
          long long operand,
          int plan_index,
          struct GenericPureVariantBinding* binding,
          char* out_ref,
          size_t out_ref_len) {
        if (boxed_sum_unit_binding_plan_index(binding, NULL)) {
          snprintf(out_ref, out_ref_len, "%lld", binding->tag_const);
          return 1;
        }
        long long value_base = resolve_copy_source_reg(operand);
        long long type_id = boxed_sum_abi_runtime_type_id(plan_index);
        if (!(value_base && type_id > 0)) {
          note_pure_unsupported_shape(
              bid,
              ii,
              op,
              "boxed_sum_abi_compare_malformed_operand",
              side);
          return 0;
        }
        EMIT("  %%boxed_sum_cmp_type_%lld_%zu_%s = call i64 @\"nyash.object.type_id_h\"(i64 %%r%lld)\n",
             bid,
             ii,
             side,
             value_base);
        EMIT("  %%boxed_sum_cmp_type_ok_%lld_%zu_%s = icmp eq i64 %%boxed_sum_cmp_type_%lld_%zu_%s, %lld\n",
             bid,
             ii,
             side,
             bid,
             ii,
             side,
             type_id);
        EMIT("  br i1 %%boxed_sum_cmp_type_ok_%lld_%zu_%s, label %%boxed_sum_cmp_continue_%lld_%zu_%s, label %%boxed_sum_cmp_trap_%lld_%zu_%s\n",
             bid,
             ii,
             side,
             bid,
             ii,
             side,
             bid,
             ii,
             side);
        EMIT("boxed_sum_cmp_trap_%lld_%zu_%s:\n", bid, ii, side);
        EMIT("  call void @llvm.trap()\n");
        EMIT("  unreachable\n");
        EMIT("boxed_sum_cmp_continue_%lld_%zu_%s:\n", bid, ii, side);
        note_exact_status_continue_label(bid, ii);
        EMIT("  %%boxed_sum_cmp_tag_%lld_%zu_%s = call i64 @\"nyash.object.field_get_i64_hii\"(i64 %%r%lld, i64 0)\n",
             bid,
             ii,
             side,
             value_base);
        snprintf(out_ref, out_ref_len, "%%boxed_sum_cmp_tag_%lld_%zu_%s", bid, ii, side);
        return 1;
      }

      auto int emit_boxed_sum_unit_compare(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          const char* pred,
          long long lhs,
          long long rhs) {
        if (!(pred && (!strcmp(pred, "eq") || !strcmp(pred, "ne")))) {
          return 0;
        }
        struct GenericPureVariantBinding lhs_binding;
        struct GenericPureVariantBinding rhs_binding;
        memset(&lhs_binding, 0, sizeof(lhs_binding));
        memset(&rhs_binding, 0, sizeof(rhs_binding));
        int has_lhs_binding = get_variant_binding(lhs, &lhs_binding);
        int has_rhs_binding = get_variant_binding(rhs, &rhs_binding);
        int lhs_plan_index = -1;
        int rhs_plan_index = -1;
        int has_lhs_plan = has_lhs_binding &&
            boxed_sum_unit_binding_plan_index(&lhs_binding, &lhs_plan_index);
        int has_rhs_plan = has_rhs_binding &&
            boxed_sum_unit_binding_plan_index(&rhs_binding, &rhs_plan_index);
        if (!(has_lhs_plan || has_rhs_plan)) {
          return 0;
        }
        int plan_index = has_lhs_plan ? lhs_plan_index : rhs_plan_index;
        if ((has_lhs_plan && lhs_plan_index != plan_index) ||
            (has_rhs_plan && rhs_plan_index != plan_index)) {
          note_pure_unsupported_shape(
              bid,
              ii,
              op,
              "boxed_sum_abi_compare_plan_mismatch",
              NULL);
          return -1;
        }
        char lhs_ref[96];
        char rhs_ref[96];
        if (!emit_boxed_sum_tag_operand_ref(
                bid, ii, op, "lhs", lhs, plan_index,
                has_lhs_binding ? &lhs_binding : NULL, lhs_ref, sizeof(lhs_ref)) ||
            !emit_boxed_sum_tag_operand_ref(
                bid, ii, op, "rhs", rhs, plan_index,
                has_rhs_binding ? &rhs_binding : NULL, rhs_ref, sizeof(rhs_ref))) {
          return -1;
        }
        EMIT("  %%r%lld = icmp %s i64 %s, %s\n", dst, pred, lhs_ref, rhs_ref);
        set_type(dst, T_I1);
        return 1;
      }
