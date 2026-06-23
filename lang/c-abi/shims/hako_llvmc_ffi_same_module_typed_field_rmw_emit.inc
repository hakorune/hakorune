      auto int same_module_function_instruction_is_field_access(
          yyjson_val* ins,
          const char* expected_op,
          long long* box_out,
          const char** field_out) {
        const char* op = read_str(ins, "op");
        const char* field = read_str(ins, "field");
        long long box_reg = read_int(ins, "box");
        if (!(op && expected_op && !strcmp(op, expected_op) && field && box_reg)) {
          return 0;
        }
        if (box_out) *box_out = box_reg;
        if (field_out) *field_out = field;
        return 1;
      }

      auto int same_module_function_typed_field_slot_u64(
          long long box_reg,
          const char* field,
          long long* slot_out) {
        int typed_plan_index = -1;
        long long slot = -1;
        const char* storage = NULL;
        if (!typed_object_exact_slot_helper_enabled()) return 0;
        if (!get_typed_object_binding(box_reg, &typed_plan_index)) return 0;
        if (!typed_object_plan_field_runtime_slot_with_storage(
                typed_plan_index,
                field,
                &slot,
                &storage)) return 0;
        if (!typed_object_storage_is_exact_slot_u64(storage)) return 0;
        if (slot_out) *slot_out = slot;
        return 1;
      }

      auto int same_module_function_typed_field_slot_i64(
          long long box_reg,
          const char* field,
          long long* slot_out) {
        int typed_plan_index = -1;
        long long slot = -1;
        const char* storage = NULL;
        if (!typed_object_exact_slot_helper_enabled()) return 0;
        if (!get_typed_object_binding(box_reg, &typed_plan_index)) return 0;
        if (!typed_object_plan_field_runtime_slot_with_storage(
                typed_plan_index,
                field,
                &slot,
                &storage)) return 0;
        if (!typed_object_storage_is_exact_slot_i64(storage)) return 0;
        if (slot_out) *slot_out = slot;
        return 1;
      }

      auto int same_module_function_read_result_capsule_reset_batch_plan_row(
          yyjson_val* row,
          long long bid,
          size_t ilen,
          unsigned char* skip_inst,
          struct SameModuleResultCapsuleResetBatchPlan* plan) {
        const char* kind = row ? read_str(row, "kind") : NULL;
        const char* helper = row ? read_str(row, "helper_symbol") : NULL;
        const char* storage = row ? read_str(row, "storage") : NULL;
        yyjson_val* set_indices = row ? yyjson_obj_get(row, "set_instruction_indices") : NULL;
        yyjson_val* skip_vals = row ? yyjson_obj_get(row, "skip_instruction_indices") : NULL;
        yyjson_val* slots = row ? yyjson_obj_get(row, "slots") : NULL;
        yyjson_val* values = row ? yyjson_obj_get(row, "values") : NULL;
        long long block = 0;
        long long first_set_index = -1;
        long long box_reg = 0;
        static const long long expected_slots[4] = {0, 1, 2, 3};
        static const long long expected_values[4] = {-1, -1, 0, 0};
        if (!(row && skip_inst && plan)) return 0;
        memset(plan, 0, sizeof(*plan));
        if (!(kind && !strcmp(kind, "same_module_result_capsule_reset_batch_i64") &&
              helper && !strcmp(helper, "nyash.object.exact_slot_set4_i64_hiiiii") &&
              storage && !strcmp(storage, "exact_slot_i64") &&
              set_indices && yyjson_is_arr(set_indices) && yyjson_arr_size(set_indices) == 4 &&
              skip_vals && yyjson_is_arr(skip_vals) &&
              slots && yyjson_is_arr(slots) && yyjson_arr_size(slots) == 4 &&
              values && yyjson_is_arr(values) && yyjson_arr_size(values) == 4)) {
          return 0;
        }
        if (!hako_llvmc_json_read_i64_field(row, "block", &block) ||
            !hako_llvmc_json_read_i64_field(row, "first_set_instruction_index", &first_set_index) ||
            !hako_llvmc_json_read_i64_field(row, "box_reg", &box_reg)) {
          return 0;
        }
        if (block != bid || first_set_index < 0 || (size_t)first_set_index >= ilen) return 0;
        for (size_t k = 0; k < 4; k++) {
          yyjson_val* set_index_val = yyjson_arr_get(set_indices, k);
          yyjson_val* slot_val = yyjson_arr_get(slots, k);
          yyjson_val* value_val = yyjson_arr_get(values, k);
          long long set_index = -1;
          if (!(set_index_val && yyjson_is_num(set_index_val) &&
                slot_val && yyjson_is_num(slot_val) &&
                value_val && yyjson_is_num(value_val))) return 0;
          set_index = (long long)yyjson_get_sint(set_index_val);
          if (set_index < 0 || (size_t)set_index >= ilen) return 0;
          if ((long long)yyjson_get_sint(slot_val) != expected_slots[k]) return 0;
          if ((long long)yyjson_get_sint(value_val) != expected_values[k]) return 0;
          plan->set_indices[k] = (size_t)set_index;
        }
        for (size_t si = 0; si < yyjson_arr_size(skip_vals); si++) {
          yyjson_val* skip_val = yyjson_arr_get(skip_vals, si);
          long long skip_index = -1;
          if (!(skip_val && yyjson_is_num(skip_val))) return 0;
          skip_index = (long long)yyjson_get_sint(skip_val);
          if (skip_index < 0 || (size_t)skip_index >= ilen) return 0;
          skip_inst[skip_index] = 1;
        }
        plan->active = 1;
        plan->first_set_index = (size_t)first_set_index;
        plan->box_reg = box_reg;
        return 1;
      }

      auto int same_module_function_read_result_capsule_reset_batch_plan(
          long long bid,
          size_t ilen,
          unsigned char* skip_inst,
          struct SameModuleResultCapsuleResetBatchPlan* plan) {
        yyjson_val* rows = NULL;
        if (!(fn_metadata && skip_inst && plan)) return 0;
        rows = yyjson_obj_get(fn_metadata, "same_module_fusion_plans");
        if (!(rows && yyjson_is_arr(rows))) return 0;
        for (size_t ri = 0; ri < yyjson_arr_size(rows); ri++) {
          if (same_module_function_read_result_capsule_reset_batch_plan_row(
                  yyjson_arr_get(rows, ri),
                  bid,
                  ilen,
                  skip_inst,
                  plan)) {
            return 1;
          }
        }
        return 0;
      }

      auto int same_module_function_read_typed_field_rmw_plan_row(
          yyjson_val* row,
          long long bid,
          size_t ilen,
          unsigned char* skip_inst,
          struct SameModuleTypedFieldRmwFusionPlan* plan) {
        const char* kind = row ? read_str(row, "kind") : NULL;
        const char* helper = row ? read_str(row, "helper_symbol") : NULL;
        const char* storage = row ? read_str(row, "storage") : NULL;
        const char* field = row ? read_str(row, "field") : NULL;
        yyjson_val* skip_vals = row ? yyjson_obj_get(row, "skip_instruction_indices") : NULL;
        long long block = 0;
        long long get_index = -1;
        long long binop_index = -1;
        long long set_index = -1;
        long long direct_use_count = 0;
        long long slot = -1;
        long long get_dst = 0;
        long long binop_dst = 0;
        long long box_reg = 0;
        long long delta_reg = 0;
        if (!(row && skip_inst && plan)) return 0;
        memset(plan, 0, sizeof(*plan));
        if (!(kind && !strcmp(kind, "same_module_typed_field_rmw_add_u64") &&
              helper && !strcmp(helper, "nyash.object.exact_slot_rmw_add_u64_hiii") &&
              storage && !strcmp(storage, "exact_slot_u64") &&
              field && skip_vals && yyjson_is_arr(skip_vals))) {
          return 0;
        }
        if (!hako_llvmc_json_read_i64_field(row, "block", &block) ||
            !hako_llvmc_json_read_i64_field(row, "get_instruction_index", &get_index) ||
            !hako_llvmc_json_read_i64_field(row, "binop_instruction_index", &binop_index) ||
            !hako_llvmc_json_read_i64_field(row, "set_instruction_index", &set_index) ||
            !hako_llvmc_json_read_i64_field(row, "direct_use_count", &direct_use_count) ||
            !hako_llvmc_json_read_i64_field(row, "slot", &slot) ||
            !hako_llvmc_json_read_i64_field(row, "get_dst", &get_dst) ||
            !hako_llvmc_json_read_i64_field(row, "binop_dst", &binop_dst) ||
            !hako_llvmc_json_read_i64_field(row, "box_reg", &box_reg) ||
            !hako_llvmc_json_read_i64_field(row, "delta_reg", &delta_reg)) {
          return 0;
        }
        if (block != bid) return 0;
        if (direct_use_count != 1) return 0;
        if (get_index < 0 || binop_index < 0 || set_index < 0 ||
            (size_t)get_index >= ilen ||
            (size_t)binop_index >= ilen ||
            (size_t)set_index >= ilen) {
          return 0;
        }
        for (size_t si = 0; si < yyjson_arr_size(skip_vals); si++) {
          yyjson_val* skip_val = yyjson_arr_get(skip_vals, si);
          long long skip_index = -1;
          if (!(skip_val && yyjson_is_num(skip_val))) return 0;
          skip_index = (long long)yyjson_get_sint(skip_val);
          if (skip_index < 0 || (size_t)skip_index >= ilen) return 0;
          skip_inst[skip_index] = 1;
        }
        plan->active = 1;
        plan->get_index = (size_t)get_index;
        plan->binop_index = (size_t)binop_index;
        plan->set_index = (size_t)set_index;
        plan->get_dst = get_dst;
        plan->binop_dst = binop_dst;
        plan->box_reg = box_reg;
        plan->slot = slot;
        plan->delta_reg = delta_reg;
        return 1;
      }

      auto size_t same_module_function_read_typed_field_rmw_fusion_plans(
          long long bid,
          size_t ilen,
          unsigned char* skip_inst,
          struct SameModuleTypedFieldRmwFusionPlan* plans,
          size_t plans_cap) {
        yyjson_val* rows = NULL;
        size_t count = 0;
        if (!(fn_metadata && skip_inst && plans && plans_cap)) return 0;
        rows = yyjson_obj_get(fn_metadata, "same_module_fusion_plans");
        if (!(rows && yyjson_is_arr(rows))) return 0;
        for (size_t ri = 0; ri < yyjson_arr_size(rows) && count < plans_cap; ri++) {
          struct SameModuleTypedFieldRmwFusionPlan plan;
          memset(&plan, 0, sizeof(plan));
          if (!same_module_function_read_typed_field_rmw_plan_row(
                  yyjson_arr_get(rows, ri),
                  bid,
                  ilen,
                  skip_inst,
                  &plan)) {
            continue;
          }
          plans[count++] = plan;
        }
        return count;
      }

      auto const struct SameModuleTypedFieldRmwFusionPlan*
      same_module_function_find_typed_field_rmw_fusion_plan(
          const struct SameModuleTypedFieldRmwFusionPlan* plans,
          size_t plans_n,
          size_t set_index) {
        if (!plans) return NULL;
        for (size_t i = 0; i < plans_n; i++) {
          if (plans[i].active && plans[i].set_index == set_index) return &plans[i];
        }
        return NULL;
      }

      auto int same_module_function_emit_typed_field_rmw_fusion(
          const struct SameModuleTypedFieldRmwFusionPlan* plan,
          long long bid,
          size_t ii) {
        char ab[256];
        long long box_ref_reg = 0;
        if (!(plan && plan->active)) return 0;
        box_ref_reg = plan->box_reg;
        get_receiver_forward_alias_src(plan->box_reg, &box_ref_reg);
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), box_ref_reg, 1);
        {
          size_t used = strlen(ab);
          if (used >= sizeof(ab)) return 0;
          snprintf(ab + used, sizeof(ab) - used, ", i64 %lld", plan->slot);
        }
        append_i64_arg_ref(ab, sizeof(ab), plan->delta_reg, 0);
        EMIT("  %%r%lld = call i64 @\"nyash.object.exact_slot_rmw_add_u64_hiii\"(%s)\n",
             plan->binop_dst,
             ab);
        EMIT("  %%typed_field_rmw_ok_%lld_%zu = icmp sge i64 %%r%lld, 0\n",
             bid,
             ii,
             plan->binop_dst);
        EMIT("  br i1 %%typed_field_rmw_ok_%lld_%zu, label %%exact_status_continue_%lld_%zu, label %%exact_status_trap_%lld_%zu\n",
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
        set_type(plan->binop_dst, T_I64);
        return 1;
      }

      auto int same_module_function_emit_result_capsule_reset_batch(
          const struct SameModuleResultCapsuleResetBatchPlan* plan,
          long long bid,
          size_t ii) {
        char ab[256];
        char status_name[96];
        long long box_ref_reg = 0;
        if (!(plan && plan->active)) return 0;
        box_ref_reg = plan->box_reg;
        get_receiver_forward_alias_src(plan->box_reg, &box_ref_reg);
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), box_ref_reg, 1);
        {
          size_t used = strlen(ab);
          if (used >= sizeof(ab)) return 0;
          snprintf(ab + used, sizeof(ab) - used, ", i64 0, i64 -1, i64 -1, i64 0, i64 0");
        }
        snprintf(status_name, sizeof(status_name), "result_capsule_reset_batch_%lld_%zu", bid, ii);
        EMIT("  %%%s = call i64 @\"nyash.object.exact_slot_set4_i64_hiiiii\"(%s)\n",
             status_name,
             ab);
        same_module_function_emit_exact_status_trap(status_name, bid, ii);
        return 1;
      }

      auto int same_module_function_emit_record_success_helper_body(void) {
        int helper_kind = same_module_function_selected_record_success_helper_kind();
        struct GenericPureBlockView entry;
        char ab[256];
        char status_name[96];
        if (!helper_kind) return 0;
        if (!read_generic_pure_block_view(0, &entry)) return -1;
        emit_block_label(entry.id);
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), 0, 1);
        append_i64_arg_ref(ab, sizeof(ab), 1, 0);
        if (helper_kind == 1) {
          snprintf(status_name, sizeof(status_name), "record_alloc_success_%lld", entry.id);
          EMIT("  %%%s = call i64 @\"nyash.object.exact_slot_record_alloc_success_hii\"(%s)\n",
               status_name,
               ab);
        } else {
          append_i64_arg_ref(ab, sizeof(ab), 2, 0);
          snprintf(status_name, sizeof(status_name), "record_release_success_%lld", entry.id);
          EMIT("  %%%s = call i64 @\"nyash.object.exact_slot_record_release_success_hiii\"(%s)\n",
               status_name,
               ab);
        }
        same_module_function_emit_exact_status_trap(status_name, entry.id, 0);
        EMIT("  ret i64 1\n");
        return 1;
      }

      auto int same_module_function_emit_instruction(
          FILE* f,
          size_t fn_index,
          long long bid,
          size_t ii,
          yyjson_val* ins,
          yyjson_val* insts,
          size_t ilen) {
        const char* op = read_str(ins, "op");
        if (!op) return 0;
        if (!strcmp(op, "phi")) return 1;
        if (!strcmp(op, "const")) {
          return same_module_function_emit_const_instruction(
              f,
              fn_index,
              bid,
              ii,
              ins);
        }
        if (!strcmp(op, "copy")) {
          if (same_module_function_should_forward_selected_receiver_copy(
                  ins,
                  ii,
                  insts,
                  ilen)) {
            return same_module_function_emit_selected_receiver_forwarding_copy(
                read_int(ins, "dst"),
                read_int(ins, "src"));
          }
          return same_module_function_emit_copy(
              read_int(ins, "dst"),
              read_int(ins, "src"));
        }
        if (!strcmp(op, "variant_make")) {
          long long dst = read_int(ins, "dst");
          long long payload = read_int(ins, "payload");
          long long tag = read_int(ins, "tag");
          const char* enum_name = read_str(ins, "enum");
          const char* payload_storage = read_str(ins, "boxed_sum_payload_storage");
          int boxed_plan_index = -1;
          boxed_sum_abi_plan_index_from_site(ins, &boxed_plan_index);
          int boxed_emit = emit_boxed_sum_variant_make_lowering(
              bid, ii, op, dst, tag, payload, enum_name, payload_storage, boxed_plan_index);
          if (boxed_emit != 0) return boxed_emit > 0;
          return 1;
        }
        if (!strcmp(op, "variant_tag")) {
          long long dst = read_int(ins, "dst");
          long long value = read_int(ins, "value");
          const char* enum_name = read_str(ins, "enum");
          int boxed_plan_index = -1;
          boxed_sum_abi_plan_index_from_site(ins, &boxed_plan_index);
          int boxed_emit = emit_boxed_sum_variant_tag_lowering(
              bid,
              ii,
              op,
              dst,
              value,
              enum_name,
              boxed_plan_index,
              "boxed_sum_abi_variant_tag_emit_unavailable",
              0);
          if (boxed_emit != 0) return boxed_emit > 0 ? 1 : -1;
          return 0;
        }
        if (!strcmp(op, "variant_project")) {
          long long dst = read_int(ins, "dst");
          long long value = read_int(ins, "value");
          long long tag = read_int(ins, "tag");
          const char* enum_name = read_str(ins, "enum");
          const char* payload_storage = read_str(ins, "boxed_sum_payload_storage");
          int boxed_plan_index = -1;
          boxed_sum_abi_plan_index_from_site(ins, &boxed_plan_index);
          int boxed_emit = emit_boxed_sum_variant_project_lowering(
              bid,
              ii,
              op,
              dst,
              value,
              tag,
              enum_name,
              payload_storage,
              boxed_plan_index,
              NULL,
              0);
          if (boxed_emit != 0) return boxed_emit > 0 ? 1 : -1;
          return 0;
        }
        if (!strcmp(op, "newbox")) {
          return same_module_function_emit_newbox(ins);
        }
        if (!strcmp(op, "field_set")) {
          return same_module_function_emit_typed_object_field_set(ins, bid, ii);
        }
        if (!strcmp(op, "field_get")) {
          if (same_module_function_emit_typed_object_field_get(ins, bid, ii)) {
            return 1;
          }
          return same_module_function_emit_map_field_get(f, fn_index, ins, bid, ii);
        }
        if (!strcmp(op, "compare")) {
          return same_module_function_emit_compare(ins, bid, ii);
        }
        if (!strcmp(op, "unop")) {
          return same_module_function_emit_unop(ins);
        }
        if (!strcmp(op, "select")) {
          return same_module_function_emit_select(ins);
        }
        if (!strcmp(op, "binop")) {
          return same_module_function_emit_binop(ins);
        }
        if (!strcmp(op, "branch")) {
          return same_module_function_emit_branch_instruction(ins);
        }
        if (!strcmp(op, "jump")) {
          return same_module_function_emit_jump_instruction(ins);
        }
        if (!strcmp(op, "ret")) {
          return same_module_function_emit_ret_instruction(bid, ins);
        }
        if (!strcmp(op, "mir_call") || !strcmp(op, "call") || !strcmp(op, "boxcall")) {
          return same_module_function_emit_call_instruction(f, bid, ii, ins);
        }
        if (!strcmp(op, "externcall")) {
          return same_module_function_emit_externcall_instruction(
              f,
              bid,
              ii,
              ins);
        }
        if (!strcmp(op, "keepalive") || !strcmp(op, "release_strong")) {
          return 1;
        }
        return 0;
      }
