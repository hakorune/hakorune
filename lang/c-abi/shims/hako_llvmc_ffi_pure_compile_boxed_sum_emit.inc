      auto int emit_boxed_sum_unit_variant_make(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          long long tag,
          const char* enum_name,
          int boxed_plan_index_hint) {
        int boxed_plan_index = boxed_plan_index_hint;
        if (boxed_plan_index >= 0 &&
            !boxed_sum_abi_variant_payload_storage_is_none(boxed_plan_index, tag)) {
          boxed_plan_index = -1;
        }
        if (!(boxed_plan_index >= 0)) {
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
        set_boxed_sum_binding(dst, boxed_plan_index);
        return 1;
      }

      auto int emit_boxed_sum_handle_variant_make(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          long long tag,
          long long payload,
          const char* enum_name,
          int boxed_plan_index_hint) {
        int boxed_plan_index = boxed_plan_index_hint;
        if (boxed_plan_index >= 0 &&
            !boxed_sum_abi_variant_payload_storage_is_handle(boxed_plan_index, tag)) {
          boxed_plan_index = -1;
        }
        if (!(boxed_plan_index >= 0)) {
          return 0;
        }
        long long type_id = boxed_sum_abi_runtime_type_id(boxed_plan_index);
        long long payload_base = resolve_copy_source_reg(payload);
        if (!(dst && type_id > 0)) {
          note_pure_unsupported_shape(
              bid,
              ii,
              op,
              "boxed_sum_abi_handle_variant_make_malformed_operands",
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
        EMIT("  call void @\"nyash.object.field_set_hii\"(i64 %%r%lld, i64 1, i64 %%r%lld)\n",
             dst,
             payload_base);
        set_variant_binding_with_enum(dst, tag, 0, payload_base, 1, enum_name);
        set_type(dst, T_I64);
        set_boxed_sum_binding(dst, boxed_plan_index);
        return 1;
      }

      auto int emit_boxed_sum_i64_variant_make(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          long long tag,
          long long payload,
          const char* enum_name,
          int boxed_plan_index_hint) {
        int boxed_plan_index = boxed_plan_index_hint;
        if (boxed_plan_index >= 0 &&
            !boxed_sum_abi_variant_payload_storage_is_i64(boxed_plan_index, tag)) {
          boxed_plan_index = -1;
        }
        if (!(boxed_plan_index >= 0)) {
          return 0;
        }
        long long type_id = boxed_sum_abi_runtime_type_id(boxed_plan_index);
        long long payload_base = resolve_copy_source_reg(payload);
        long long payload_const = 0;
        if (!(dst && type_id > 0)) {
          note_pure_unsupported_shape(
              bid,
              ii,
              op,
              "boxed_sum_abi_i64_variant_make_malformed_operands",
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
        if (has_const(payload_base, &payload_const)) {
          EMIT("  %%boxed_sum_i64_payload_%lld_%zu = add i64 %lld, 0\n",
               bid,
               ii,
               payload_const);
        } else {
          EMIT("  %%boxed_sum_i64_payload_%lld_%zu = add i64 %%r%lld, 0\n",
               bid,
               ii,
               payload_base);
        }
        EMIT("  %%boxed_sum_i64_set_%lld_%zu = call i64 @\"nyash.object.field_set_i64_hii\"(i64 %%r%lld, i64 1, i64 %%boxed_sum_i64_payload_%lld_%zu)\n",
             bid,
             ii,
             dst,
             bid,
             ii);
        set_variant_binding_with_enum(dst, tag, 0, payload_base, 1, enum_name);
        set_type(dst, T_I64);
        set_boxed_sum_binding(dst, boxed_plan_index);
        return 1;
      }

      auto int emit_boxed_sum_tag_read(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          long long value,
          const char* enum_name,
          int boxed_plan_index_hint) {
        yyjson_val* plans = boxed_sum_abi_plans_array();
        size_t plan_count = boxed_sum_abi_plan_count();
        if (!(enum_name && plans && plan_count > 0)) {
          return 0;
        }
        long long value_base = resolve_copy_source_reg(value);
        if (!dst) {
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
        int matched_plan_count = 0;
        if (!(boxed_plan_index_hint >= 0)) {
          return 0;
        }
        size_t start_plan = (size_t)boxed_plan_index_hint;
        size_t end_plan = start_plan + 1;
        for (size_t pi = start_plan; pi < end_plan; pi++) {
          yyjson_val* plan = yyjson_arr_get(plans, pi);
          const char* plan_enum = plan ? read_str(plan, "enum_name") : NULL;
          const char* version = plan ? read_str(plan, "version") : NULL;
          long long type_id = boxed_sum_abi_runtime_type_id((int)pi);
          if (!(plan_enum &&
                version &&
                !strcmp(version, "boxed_runtime_v2") &&
                !strcmp(plan_enum, enum_name) &&
                type_id > 0)) {
            continue;
          }
          EMIT("  %%boxed_sum_type_ok_%lld_%zu_%d = icmp eq i64 %%boxed_sum_type_%lld_%zu, %lld\n",
               bid,
               ii,
               matched_plan_count,
               bid,
               ii,
               type_id);
          if (matched_plan_count == 0) {
            EMIT("  %%boxed_sum_type_ok_acc_%lld_%zu_%d = add i1 %%boxed_sum_type_ok_%lld_%zu_%d, false\n",
                 bid,
                 ii,
                 matched_plan_count,
                 bid,
                 ii,
                 matched_plan_count);
          } else {
            EMIT("  %%boxed_sum_type_ok_acc_%lld_%zu_%d = or i1 %%boxed_sum_type_ok_acc_%lld_%zu_%d, %%boxed_sum_type_ok_%lld_%zu_%d\n",
                 bid,
                 ii,
                 matched_plan_count,
                 bid,
                 ii,
                 matched_plan_count - 1,
                 bid,
                 ii,
                 matched_plan_count);
          }
          matched_plan_count++;
        }
        if (matched_plan_count == 0) {
          note_pure_unsupported_shape(
              bid,
              ii,
              op,
              "boxed_sum_abi_variant_tag_plan_missing",
              enum_name);
          return 0;
        }
        EMIT("  br i1 %%boxed_sum_type_ok_acc_%lld_%zu_%d, label %%boxed_sum_type_continue_%lld_%zu, label %%boxed_sum_type_trap_%lld_%zu\n",
             bid,
             ii,
             matched_plan_count - 1,
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

      auto int emit_boxed_sum_project_handle(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          long long value,
          long long expected_tag,
          const char* enum_name,
          int boxed_plan_index_hint) {
        int boxed_plan_index = boxed_plan_index_hint;
        if (boxed_plan_index >= 0 &&
            !boxed_sum_abi_variant_payload_storage_is_handle(boxed_plan_index, expected_tag)) {
          boxed_plan_index = -1;
        }
        if (!(boxed_plan_index >= 0)) {
          return 0;
        }
        long long type_id = boxed_sum_abi_runtime_type_id(boxed_plan_index);
        long long value_base = resolve_copy_source_reg(value);
        if (!(dst && type_id > 0)) {
          note_pure_unsupported_shape(
              bid,
              ii,
              op,
              "boxed_sum_abi_project_malformed_operands",
              NULL);
          return -1;
        }
        EMIT("  %%boxed_sum_project_type_%lld_%zu = call i64 @\"nyash.object.type_id_h\"(i64 %%r%lld)\n",
             bid,
             ii,
             value_base);
        EMIT("  %%boxed_sum_project_type_ok_%lld_%zu = icmp eq i64 %%boxed_sum_project_type_%lld_%zu, %lld\n",
             bid,
             ii,
             bid,
             ii,
             type_id);
        EMIT("  br i1 %%boxed_sum_project_type_ok_%lld_%zu, label %%boxed_sum_project_type_continue_%lld_%zu, label %%boxed_sum_project_type_trap_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("boxed_sum_project_type_trap_%lld_%zu:\n", bid, ii);
        EMIT("  call void @llvm.trap()\n");
        EMIT("  unreachable\n");
        EMIT("boxed_sum_project_type_continue_%lld_%zu:\n", bid, ii);
        note_exact_status_continue_label(bid, ii);
        EMIT("  %%boxed_sum_project_tag_%lld_%zu = call i64 @\"nyash.object.field_get_i64_hii\"(i64 %%r%lld, i64 0)\n",
             bid,
             ii,
             value_base);
        EMIT("  %%boxed_sum_project_tag_ok_%lld_%zu = icmp eq i64 %%boxed_sum_project_tag_%lld_%zu, %lld\n",
             bid,
             ii,
             bid,
             ii,
             expected_tag);
        EMIT("  br i1 %%boxed_sum_project_tag_ok_%lld_%zu, label %%boxed_sum_project_tag_continue_%lld_%zu, label %%boxed_sum_project_tag_trap_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("boxed_sum_project_tag_trap_%lld_%zu:\n", bid, ii);
        EMIT("  call void @llvm.trap()\n");
        EMIT("  unreachable\n");
        EMIT("boxed_sum_project_tag_continue_%lld_%zu:\n", bid, ii);
        note_exact_status_continue_label(bid, ii);
        EMIT("  %%r%lld = call i64 @\"nyash.object.field_get_hii\"(i64 %%r%lld, i64 1)\n",
             dst,
             value_base);
        set_type(dst, T_I64);
        return 1;
      }

      auto int emit_boxed_sum_project_i64(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          long long value,
          long long expected_tag,
          const char* enum_name,
          int boxed_plan_index_hint) {
        int boxed_plan_index = boxed_plan_index_hint;
        if (boxed_plan_index >= 0 &&
            !boxed_sum_abi_variant_payload_storage_is_i64(boxed_plan_index, expected_tag)) {
          boxed_plan_index = -1;
        }
        if (!(boxed_plan_index >= 0)) {
          return 0;
        }
        long long type_id = boxed_sum_abi_runtime_type_id(boxed_plan_index);
        long long value_base = resolve_copy_source_reg(value);
        if (!(dst && type_id > 0)) {
          note_pure_unsupported_shape(
              bid,
              ii,
              op,
              "boxed_sum_abi_project_i64_malformed_operands",
              NULL);
          return -1;
        }
        EMIT("  %%boxed_sum_project_type_%lld_%zu = call i64 @\"nyash.object.type_id_h\"(i64 %%r%lld)\n",
             bid,
             ii,
             value_base);
        EMIT("  %%boxed_sum_project_type_ok_%lld_%zu = icmp eq i64 %%boxed_sum_project_type_%lld_%zu, %lld\n",
             bid,
             ii,
             bid,
             ii,
             type_id);
        EMIT("  br i1 %%boxed_sum_project_type_ok_%lld_%zu, label %%boxed_sum_project_type_continue_%lld_%zu, label %%boxed_sum_project_type_trap_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("boxed_sum_project_type_trap_%lld_%zu:\n", bid, ii);
        EMIT("  call void @llvm.trap()\n");
        EMIT("  unreachable\n");
        EMIT("boxed_sum_project_type_continue_%lld_%zu:\n", bid, ii);
        note_exact_status_continue_label(bid, ii);
        EMIT("  %%boxed_sum_project_tag_%lld_%zu = call i64 @\"nyash.object.field_get_i64_hii\"(i64 %%r%lld, i64 0)\n",
             bid,
             ii,
             value_base);
        EMIT("  %%boxed_sum_project_tag_ok_%lld_%zu = icmp eq i64 %%boxed_sum_project_tag_%lld_%zu, %lld\n",
             bid,
             ii,
             bid,
             ii,
             expected_tag);
        EMIT("  br i1 %%boxed_sum_project_tag_ok_%lld_%zu, label %%boxed_sum_project_tag_continue_%lld_%zu, label %%boxed_sum_project_tag_trap_%lld_%zu\n",
             bid,
             ii,
             bid,
             ii,
             bid,
             ii);
        EMIT("boxed_sum_project_tag_trap_%lld_%zu:\n", bid, ii);
        EMIT("  call void @llvm.trap()\n");
        EMIT("  unreachable\n");
        EMIT("boxed_sum_project_tag_continue_%lld_%zu:\n", bid, ii);
        note_exact_status_continue_label(bid, ii);
        EMIT("  %%r%lld = call i64 @\"nyash.object.field_get_i64_hii\"(i64 %%r%lld, i64 1)\n",
             dst,
             value_base);
        set_type(dst, T_I64);
        return 1;
      }

      auto int emit_boxed_sum_variant_make_lowering(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          long long tag,
          long long payload,
          const char* enum_name,
          const char* payload_storage,
          int boxed_plan_index) {
        if (!payload_storage) {
          note_pure_unsupported_shape(
              bid, ii, op, "boxed_sum_abi_missing_site_payload_storage", enum_name);
          return -1;
        }
        if (!payload) {
          return emit_boxed_sum_unit_variant_make(
              bid, ii, op, dst, tag, enum_name, boxed_plan_index);
        }
        if (!strcmp(payload_storage, "i64")) {
          return emit_boxed_sum_i64_variant_make(
              bid, ii, op, dst, tag, payload, enum_name, boxed_plan_index);
        }
        return emit_boxed_sum_handle_variant_make(
            bid, ii, op, dst, tag, payload, enum_name, boxed_plan_index);
      }

      auto int emit_boxed_sum_variant_tag_lowering(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          long long value,
          const char* enum_name,
          int boxed_plan_index,
          const char* unavailable_reason,
          int unavailable_is_error) {
        struct GenericPureVariantBinding binding;
        memset(&binding, 0, sizeof(binding));
        if (!get_variant_binding(value, &binding)) {
          int boxed_emit = emit_boxed_sum_tag_read(
              bid, ii, op, dst, value, enum_name, boxed_plan_index);
          if (boxed_emit != 0) return boxed_emit;
          if (unavailable_reason) {
            note_pure_unsupported_shape(bid, ii, op, unavailable_reason, enum_name);
          }
          return unavailable_is_error ? -1 : 0;
        }
        if (binding.tag_reg) {
          EMIT("  %%r%lld = add i64 %%r%lld_tag, 0\n", dst, binding.tag_reg);
        } else {
          put_const(dst, binding.tag_const);
        }
        set_type(dst, T_I64);
        return 1;
      }

      auto int emit_boxed_sum_variant_project_lowering(
          long long bid,
          size_t ii,
          const char* op,
          long long dst,
          long long value,
          long long tag,
          const char* enum_name,
          const char* payload_storage,
          int boxed_plan_index,
          const char* unavailable_reason,
          int unavailable_is_error) {
        struct GenericPureVariantBinding binding;
        memset(&binding, 0, sizeof(binding));
        if (!get_variant_binding(value, &binding) || !binding.has_payload) {
          int boxed_emit = 0;
          if (!payload_storage) {
            note_pure_unsupported_shape(
                bid, ii, op, "boxed_sum_abi_missing_site_payload_storage", enum_name);
            return -1;
          }
          boxed_emit = !strcmp(payload_storage, "i64")
              ? emit_boxed_sum_project_i64(
                    bid, ii, op, dst, value, tag, enum_name, boxed_plan_index)
              : emit_boxed_sum_project_handle(
                    bid, ii, op, dst, value, tag, enum_name, boxed_plan_index);
          if (boxed_emit != 0) return boxed_emit;
          if (unavailable_reason) {
            note_pure_unsupported_shape(bid, ii, op, unavailable_reason, enum_name);
          }
          return unavailable_is_error ? -1 : 0;
        }
        set_copy_alias(dst, resolve_copy_source_reg(binding.payload_reg));
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
        if (!(type_id > 0)) {
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
