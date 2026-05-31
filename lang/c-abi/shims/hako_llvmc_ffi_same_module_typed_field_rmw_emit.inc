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

      auto int same_module_function_match_result_capsule_reset_batch_plan(
          yyjson_val* insts,
          size_t ilen,
          unsigned char* skip_inst,
          struct SameModuleResultCapsuleResetBatchPlan* plan) {
        static const char* const expected_fields[4] = {
            "last_page_id",
            "last_block_id",
            "last_reason",
            "last_ok",
        };
        static const long long expected_slots[4] = {0, 1, 2, 3};
        static const long long expected_values[4] = {-1, -1, 0, 0};
        size_t search_from = 0;
        long long box_reg = 0;
        if (!(insts && skip_inst && plan)) return 0;
        memset(plan, 0, sizeof(*plan));
        if (!same_module_function_is_selected_result_capsule_reset_batch_target()) return 0;
        for (size_t k = 0; k < 4; k++) {
          int found = 0;
          for (size_t ii = search_from; ii < ilen; ii++) {
            yyjson_val* ins = yyjson_arr_get(insts, ii);
            const char* field = NULL;
            long long set_box = 0;
            long long slot = -1;
            long long value_reg = 0;
            long long value_base = 0;
            long long value_const = 0;
            if (!same_module_function_instruction_is_field_access(
                    ins,
                    "field_set",
                    &set_box,
                    &field)) {
              continue;
            }
            if (!(field && !strcmp(field, expected_fields[k]))) continue;
            if (k == 0) {
              box_reg = set_box;
            } else if (resolve_copy_source_reg(set_box) != resolve_copy_source_reg(box_reg)) {
              return 0;
            }
            if (!same_module_function_typed_field_slot_i64(set_box, field, &slot)) return 0;
            if (slot != expected_slots[k]) return 0;
            value_reg = read_int(ins, "value");
            value_base = resolve_copy_source_reg(value_reg);
            if (!has_const(value_base, &value_const)) return 0;
            if (value_const != expected_values[k]) return 0;
            plan->set_indices[k] = ii;
            search_from = ii + 1;
            found = 1;
            break;
          }
          if (!found) {
            memset(plan, 0, sizeof(*plan));
            return 0;
          }
        }
        plan->active = 1;
        plan->first_set_index = plan->set_indices[0];
        plan->box_reg = box_reg;
        for (size_t k = 1; k < 4; k++) {
          skip_inst[plan->set_indices[k]] = 1;
        }
        return 1;
      }

      auto int same_module_function_register_direct_use_count(
          yyjson_val* insts,
          size_t ilen,
          long long reg) {
        int count = 0;
        if (!(insts && reg)) return 0;
        for (size_t ii = 0; ii < ilen; ii++) {
          yyjson_val* ins = yyjson_arr_get(insts, ii);
          const char* op = read_str(ins, "op");
          if (!op) continue;
          if (!strcmp(op, "binop") || !strcmp(op, "compare")) {
            if (read_int(ins, "lhs") == reg) count++;
            if (read_int(ins, "rhs") == reg) count++;
          } else if (!strcmp(op, "copy")) {
            if (read_int(ins, "src") == reg) count++;
          } else if (!strcmp(op, "field_set")) {
            if (read_int(ins, "value") == reg) count++;
          } else if (!strcmp(op, "branch")) {
            if (read_int(ins, "cond") == reg) count++;
          } else if (!strcmp(op, "ret")) {
            if (read_int(ins, "value") == reg) count++;
          }
        }
        return count;
      }

      auto int same_module_function_match_typed_field_rmw_fusion_plan_at(
          yyjson_val* insts,
          size_t ilen,
          size_t gi,
          struct SameModuleTypedFieldRmwFusionPlan* plan) {
        yyjson_val* get_ins = NULL;
        const char* get_field = NULL;
        long long get_box = 0;
        long long get_dst = 0;
        long long slot = -1;
        if (!(insts && plan && gi < ilen)) return 0;
        memset(plan, 0, sizeof(*plan));
        if (!same_module_function_is_selected_typed_field_rmw_target()) return 0;
        get_ins = yyjson_arr_get(insts, gi);
        get_dst = read_int(get_ins, "dst");
        if (!get_dst) return 0;
        if (!same_module_function_instruction_is_field_access(
                get_ins,
                "field_get",
                &get_box,
                &get_field)) {
          return 0;
        }
        if (!same_module_function_typed_field_slot_u64(get_box, get_field, &slot)) return 0;
        if (same_module_function_register_direct_use_count(insts, ilen, get_dst) != 1) {
          return 0;
        }
        for (size_t bi = gi + 1; bi < ilen; bi++) {
          yyjson_val* binop = yyjson_arr_get(insts, bi);
          const char* op = read_str(binop, "op");
          const char* kind = read_str(binop, "op_kind");
          long long binop_dst = 0;
          long long lhs = 0;
          long long rhs = 0;
          long long lhs_base = 0;
          long long rhs_base = 0;
          long long delta_reg = 0;
          if (!(op && !strcmp(op, "binop"))) continue;
          if (!kind) kind = read_str(binop, "operation");
          if (!(kind && (!strcmp(kind, "+") || !strcmp(kind, "Add")))) continue;
          binop_dst = read_int(binop, "dst");
          lhs = read_int(binop, "lhs");
          rhs = read_int(binop, "rhs");
          lhs_base = resolve_copy_source_reg(lhs);
          rhs_base = resolve_copy_source_reg(rhs);
          if (lhs_base == get_dst) delta_reg = rhs;
          else if (rhs_base == get_dst) delta_reg = lhs;
          else continue;
          for (size_t si = bi + 1; si < ilen; si++) {
            yyjson_val* set_ins = yyjson_arr_get(insts, si);
            const char* set_field = NULL;
            long long set_box = 0;
            long long set_value = 0;
            if (!same_module_function_instruction_is_field_access(
                    set_ins,
                    "field_set",
                    &set_box,
                    &set_field)) {
              continue;
            }
            set_value = resolve_copy_source_reg(read_int(set_ins, "value"));
            if (set_value != binop_dst) continue;
            if (resolve_copy_source_reg(set_box) != resolve_copy_source_reg(get_box)) continue;
            if (!(set_field && get_field && !strcmp(set_field, get_field))) continue;
            plan->active = 1;
            plan->get_index = gi;
            plan->binop_index = bi;
            plan->set_index = si;
            plan->get_dst = get_dst;
            plan->binop_dst = binop_dst;
            plan->box_reg = get_box;
            plan->slot = slot;
            plan->delta_reg = delta_reg;
            return 1;
          }
        }
        memset(plan, 0, sizeof(*plan));
        return 0;
      }

      auto size_t same_module_function_match_typed_field_rmw_fusion_plans(
          yyjson_val* insts,
          size_t ilen,
          unsigned char* skip_inst,
          struct SameModuleTypedFieldRmwFusionPlan* plans,
          size_t plans_cap) {
        size_t count = 0;
        if (!(insts && skip_inst && plans && plans_cap)) return 0;
        if (!same_module_function_is_selected_typed_field_rmw_target()) return 0;
        for (size_t gi = 0; gi < ilen && count < plans_cap; gi++) {
          struct SameModuleTypedFieldRmwFusionPlan plan;
          if (skip_inst[gi]) continue;
          memset(&plan, 0, sizeof(plan));
          if (!same_module_function_match_typed_field_rmw_fusion_plan_at(
                  insts,
                  ilen,
                  gi,
                  &plan)) {
            continue;
          }
          skip_inst[plan.get_index] = 1;
          skip_inst[plan.binop_index] = 1;
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
          return 1;
        }
        if (!strcmp(op, "variant_tag")) {
          long long dst = read_int(ins, "dst");
          long long value = read_int(ins, "value");
          struct GenericPureVariantBinding binding;
          memset(&binding, 0, sizeof(binding));
          if (!get_variant_binding(value, &binding)) return 0;
          if (binding.tag_reg) {
            EMIT("  %%r%lld = add i64 %%r%lld_tag, 0\n", dst, binding.tag_reg);
          } else {
            put_const(dst, binding.tag_const);
          }
          set_type(dst, T_I64);
          return 1;
        }
        if (!strcmp(op, "variant_project")) {
          long long dst = read_int(ins, "dst");
          long long value = read_int(ins, "value");
          struct GenericPureVariantBinding binding;
          memset(&binding, 0, sizeof(binding));
          if (!get_variant_binding(value, &binding) || !binding.has_payload) return 0;
          set_copy_alias(dst, resolve_copy_source_reg(binding.payload_reg));
          set_type(dst, T_I64);
          return 1;
        }
        if (!strcmp(op, "newbox")) {
          return same_module_function_emit_newbox(ins);
        }
        if (!strcmp(op, "field_set")) {
          return same_module_function_emit_typed_object_field_set(ins, bid, ii);
        }
        if (!strcmp(op, "field_get")) {
          return same_module_function_emit_typed_object_field_get(ins, bid, ii);
        }
        if (!strcmp(op, "compare")) {
          return same_module_function_emit_compare(ins);
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
