      /* Same-module body emit: lower the already accepted prepass shape to LLVM text. */

      auto void same_module_function_emit_const_globals(FILE* f, size_t fn_index) {
        for (size_t sci = 0; sci < str_consts_n; sci++) {
          size_t slen = strlen(str_consts[sci].val);
          fprintf(f, "@.fnstr_%zu_%lld = private unnamed_addr constant [%zu x i8] c\"",
                  fn_index,
                  str_consts[sci].reg,
                  slen + 1);
          hako_llvmc_emit_llvm_byte_string(f, str_consts[sci].val, (long long)slen);
          fprintf(f, "\\00\", align 1\n");
        }
      }

      auto int same_module_function_emit_const_box(
          FILE* f,
          size_t fn_index,
          long long dst) {
        const char* sval = get_str_const(dst);
        size_t slen = sval ? strlen(sval) : 0;
        if (!sval) return 0;
        EMIT("  %%r%lld = call i64 @\"nyash.box.from_i8_string_const\"(ptr getelementptr inbounds ([%zu x i8], ptr @.fnstr_%zu_%lld, i64 0, i64 0))\n",
             dst,
             slen + 1,
             fn_index,
             dst);
        mark_string_const_box_emitted(dst);
        same_module_function_publish_i64_origin(dst, ORG_STRING);
        return 1;
      }

      auto int same_module_function_emit_copy(long long dst, long long src) {
        int src_ty = get_type(src);
        int src_org = get_origin(src);
        int src_scan = get_scan_origin(src);
        if (src == 0) {
          EMIT("  %%r%lld = add i64 %%r0, 0\n", dst);
        } else {
          set_copy_alias(dst, resolve_copy_source_reg(src));
        }
        if (src_ty != T_NONE) set_type(dst, src_ty);
        if (src_org != ORG_NONE) set_origin(dst, src_org);
        if (src_scan != ORG_NONE) set_scan_origin(dst, src_scan);
        {
          int typed_plan_index = -1;
          if (get_typed_object_binding(src, &typed_plan_index)) {
            set_typed_object_binding(dst, typed_plan_index);
          }
        }
        return 1;
      }

      auto int same_module_function_is_selected_receiver_block_entry_copy_forwarding_target(void) {
        const char* name = fn0 ? read_str(fn0, "name") : NULL;
        return name && !strcmp(name, "HakoAllocPageModel.acquire_usize/1");
      }

      auto int same_module_function_copy_dst_has_same_block_receiver_sink(
          long long dst,
          size_t ii,
          yyjson_val* insts,
          size_t ilen) {
        if (!(dst && insts)) return 0;
        for (size_t si = ii + 1; si < ilen; si++) {
          yyjson_val* use = yyjson_arr_get(insts, si);
          const char* op = read_str(use, "op");
          if (!op) continue;
          if ((!strcmp(op, "field_get") || !strcmp(op, "field_set")) &&
              read_int(use, "box") == dst) {
            return 1;
          }
          if (!strcmp(op, "mir_call")) {
            yyjson_val* cal = yyjson_obj_get(use, "mir_call");
            yyjson_val* callee = cal ? yyjson_obj_get(cal, "callee") : NULL;
            yyjson_val* args = cal ? yyjson_obj_get(cal, "args") : NULL;
            if (callee && read_int(callee, "receiver") == dst) return 0;
            if (args && yyjson_is_arr(args)) {
              for (size_t ai = 0; ai < yyjson_arr_size(args); ai++) {
                yyjson_val* arg = yyjson_arr_get(args, ai);
                if (arg && yyjson_is_int(arg) && (long long)yyjson_get_sint(arg) == dst) {
                  return 0;
                }
              }
            }
          }
        }
        return 0;
      }

      auto int same_module_function_should_forward_selected_receiver_copy(
          yyjson_val* ins,
          size_t ii,
          yyjson_val* insts,
          size_t ilen) {
        long long dst = read_int(ins, "dst");
        long long src = read_int(ins, "src");
        if (!same_module_function_is_selected_receiver_block_entry_copy_forwarding_target()) {
          return 0;
        }
        if (!(dst && src == 0)) return 0;
        if (ii > 2) return 0;
        return same_module_function_copy_dst_has_same_block_receiver_sink(
            dst,
            ii,
            insts,
            ilen);
      }

      auto int same_module_function_emit_selected_receiver_forwarding_copy(
          long long dst,
          long long src) {
        int src_ty = get_type(src);
        int src_org = get_origin(src);
        int src_scan = get_scan_origin(src);
        int typed_plan_index = -1;
        set_receiver_forward_alias(dst, src);
        if (src_ty != T_NONE) set_type(dst, src_ty);
        if (src_org != ORG_NONE) set_origin(dst, src_org);
        if (src_scan != ORG_NONE) set_scan_origin(dst, src_scan);
        if (get_typed_object_binding(src, &typed_plan_index)) {
          set_typed_object_binding(dst, typed_plan_index);
        }
        return 1;
      }

      auto int same_module_function_emit_unop(yyjson_val* ins) {
        const char* kind = read_str(ins, "op_kind");
        long long dst = read_int(ins, "dst");
        long long src = read_int(ins, "src");
        long long base = 0;
        long long cv = 0;
        int has_cv = 0;
        int src_ty = T_NONE;
        if (!kind) kind = read_str(ins, "operation");
        if (!src) src = read_int(ins, "operand");
        if (!(kind && (!strcmp(kind, "not") || !strcmp(kind, "Not")))) return 0;
        if (!(dst && src)) return 0;
        base = resolve_copy_source_reg(src);
        has_cv = has_const(base, &cv);
        src_ty = get_type(base);
        if (src_ty == T_I1) {
          if (has_cv) {
            EMIT("  %%r%lld = xor i1 %s, true\n", dst, cv ? "true" : "false");
          } else {
            EMIT("  %%r%lld = xor i1 %%r%lld, true\n", dst, base);
          }
        } else if (has_cv) {
          EMIT("  %%r%lld = icmp eq i64 %lld, 0\n", dst, cv);
        } else {
          EMIT("  %%r%lld = icmp eq i64 %%r%lld, 0\n", dst, base);
        }
        set_type(dst, T_I1);
        return 1;
      }

      auto int same_module_function_emit_binop(yyjson_val* ins) {
        const char* kind = read_str(ins, "op_kind");
        const char* irop = NULL;
        long long dst = read_int(ins, "dst");
        long long lhs = read_int(ins, "lhs");
        long long rhs = read_int(ins, "rhs");
        long long lhs_base = resolve_copy_source_reg(lhs);
        long long rhs_base = resolve_copy_source_reg(rhs);
        char lhs_ref[64];
        char rhs_ref[64];
        if (!kind) kind = read_str(ins, "operation");
        if (kind && (!strcmp(kind, "+") || !strcmp(kind, "Add")) &&
            (same_module_function_value_is_string(lhs) ||
             same_module_function_value_is_string(rhs))) {
          if (!try_emit_string_concat3_chain_call(f, dst, lhs_base, rhs_base)) {
            emit_string_concat_hh_call(f, dst, lhs_base, rhs_base);
          }
          return 1;
        }
        if (kind) {
          if (!strcmp(kind, "+") || !strcmp(kind, "Add")) irop = "add";
          else if (!strcmp(kind, "-") || !strcmp(kind, "Sub")) irop = "sub";
          else if (!strcmp(kind, "*") || !strcmp(kind, "Mul")) irop = "mul";
          else if (!strcmp(kind, "/") || !strcmp(kind, "Div")) irop = "sdiv";
          else if (!strcmp(kind, "%") || !strcmp(kind, "Mod")) irop = "srem";
        }
        if (!irop) return 0;
        same_module_function_format_i64_ref(lhs_ref, sizeof(lhs_ref), lhs);
        same_module_function_format_i64_ref(rhs_ref, sizeof(rhs_ref), rhs);
        EMIT("  %%r%lld = %s i64 %s, %s\n", dst, irop, lhs_ref, rhs_ref);
        set_type(dst, T_I64);
        return 1;
      }

      auto int same_module_function_emit_compare(yyjson_val* ins) {
        const char* pred = read_str(ins, "cmp");
        const char* p2 = NULL;
        long long dst = read_int(ins, "dst");
        long long lhs = read_int(ins, "lhs");
        long long rhs = read_int(ins, "rhs");
        if (!lhs) lhs = read_int(ins, "left");
        if (!rhs) rhs = read_int(ins, "right");
        if (!pred) pred = read_str(ins, "operation");
        if (pred) {
          if (!strcmp(pred, "Lt") || !strcmp(pred, "lt") || !strcmp(pred, "<")) p2 = "slt";
          else if (!strcmp(pred, "Le") || !strcmp(pred, "le") || !strcmp(pred, "<=")) p2 = "sle";
          else if (!strcmp(pred, "Eq") || !strcmp(pred, "eq") || !strcmp(pred, "==")) p2 = "eq";
          else if (!strcmp(pred, "Ne") || !strcmp(pred, "ne") || !strcmp(pred, "!=")) p2 = "ne";
          else if (!strcmp(pred, "Ge") || !strcmp(pred, "ge") || !strcmp(pred, ">=")) p2 = "sge";
          else if (!strcmp(pred, "Gt") || !strcmp(pred, "gt") || !strcmp(pred, ">")) p2 = "sgt";
        }
        if (!p2) return 0;
        if (should_emit_string_compare(ins, lhs, rhs)) {
          emit_string_compare(dst, p2, lhs, rhs);
        } else {
          emit_icmp(dst, p2, lhs, rhs);
        }
        return 1;
      }

      auto int same_module_function_emit_select(yyjson_val* ins) {
        long long dst = read_int(ins, "dst");
        long long cond = read_int(ins, "cond");
        long long then_val = read_int(ins, "then_val");
        long long else_val = read_int(ins, "else_val");
        long long cond_cv = 0;
        long long then_cv = 0;
        long long else_cv = 0;
        int cond_ty = get_type(cond);
        int then_ty = get_type(then_val);
        int else_ty = get_type(else_val);
        int val_ty = T_I64;
        long long cond_base = resolve_copy_source_reg(cond);
        long long then_base = resolve_copy_source_reg(then_val);
        long long else_base = resolve_copy_source_reg(else_val);
        int cond_const = has_const(cond_base, &cond_cv);
        int then_const = has_const(then_base, &then_cv);
        int else_const = has_const(else_base, &else_cv);
        int then_org = get_origin(then_val);
        int else_org = get_origin(else_val);
        int then_scan = get_scan_origin(then_val);
        int else_scan = get_scan_origin(else_val);
        char cond_ref[64];
        char then_ref[64];
        char else_ref[64];

        if (!(dst && cond && then_val && else_val)) return 0;
        if (then_ty == T_I1 && else_ty == T_I1) val_ty = T_I1;
        else if (then_ty != T_NONE && then_ty == else_ty) val_ty = then_ty;
        else if (then_ty != T_NONE && else_ty == T_NONE) val_ty = then_ty;
        else if (else_ty != T_NONE && then_ty == T_NONE) val_ty = else_ty;

        if (cond_ty == T_I1) {
          if (cond_const) snprintf(cond_ref, sizeof(cond_ref), "%s", cond_cv ? "true" : "false");
          else snprintf(cond_ref, sizeof(cond_ref), "%%r%lld", cond_base);
        } else if (cond_const) {
          EMIT("  %%t%lld = icmp ne i64 %lld, 0\n", dst, cond_cv);
          snprintf(cond_ref, sizeof(cond_ref), "%%t%lld", dst);
        } else {
          EMIT("  %%t%lld = icmp ne i64 %%r%lld, 0\n", dst, cond_base);
          snprintf(cond_ref, sizeof(cond_ref), "%%t%lld", dst);
        }

        if (val_ty == T_I1) {
          if (then_const) snprintf(then_ref, sizeof(then_ref), "%s", then_cv ? "true" : "false");
          else snprintf(then_ref, sizeof(then_ref), "%%r%lld", then_base);
          if (else_const) snprintf(else_ref, sizeof(else_ref), "%s", else_cv ? "true" : "false");
          else snprintf(else_ref, sizeof(else_ref), "%%r%lld", else_base);
          EMIT("  %%r%lld = select i1 %s, i1 %s, i1 %s\n",
               dst,
               cond_ref,
               then_ref,
               else_ref);
          set_type(dst, T_I1);
        } else {
          if (then_const) snprintf(then_ref, sizeof(then_ref), "%lld", then_cv);
          else if (then_ty == T_I1) {
            EMIT("  %%t%lld_then_i64 = zext i1 %%r%lld to i64\n", dst, then_base);
            snprintf(then_ref, sizeof(then_ref), "%%t%lld_then_i64", dst);
          } else snprintf(then_ref, sizeof(then_ref), "%%r%lld", then_base);
          if (else_const) snprintf(else_ref, sizeof(else_ref), "%lld", else_cv);
          else if (else_ty == T_I1) {
            EMIT("  %%t%lld_else_i64 = zext i1 %%r%lld to i64\n", dst, else_base);
            snprintf(else_ref, sizeof(else_ref), "%%t%lld_else_i64", dst);
          } else snprintf(else_ref, sizeof(else_ref), "%%r%lld", else_base);
          EMIT("  %%r%lld = select i1 %s, i64 %s, i64 %s\n",
               dst,
               cond_ref,
               then_ref,
               else_ref);
          set_type(dst, T_I64);
        }
        if (then_org != ORG_NONE && then_org == else_org) set_origin(dst, then_org);
        if (then_scan != ORG_NONE && then_scan == else_scan) set_scan_origin(dst, then_scan);
        return 1;
      }

      #include "hako_llvmc_ffi_same_module_typed_object_emit.inc"
      #include "hako_llvmc_ffi_same_module_generic_method_emit.inc"

      auto int same_module_function_emit_global_call_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          yyjson_val* cal,
          yyjson_val* args,
          long long dst) {
        int rc = emit_global_call_lowering_plan_mir_call(f, bid, ii, dst, args);
        if (rc == 1) {
          yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
          struct LoweringPlanGlobalCallView global_view;
          if (read_lowering_plan_global_call_view(entry, &global_view)) {
            same_module_function_publish_result_box_or_reg_metadata(
                dst,
                global_view.target_result_box_name);
          }
        }
        if (rc != 1) {
          long long a0 = 0;
          if (same_module_function_read_arg_reg(args, 0, &a0)) {
            rc = emit_global_mir_call(
                f,
                classify_mir_call_global_surface(read_str(cal, "name")),
                bid,
                ii,
                a0);
          }
        }
        return rc;
      }

      auto int same_module_function_emit_method_call_mir_call(
          FILE* f,
          long long bid,
          size_t ii,
          const char* op,
          yyjson_val* cal,
          yyjson_val* args,
          long long dst) {
        long long recv = same_module_function_call_receiver(op, cal);
        const char* ctype = same_module_function_call_type(op, cal);
        const char* mname = same_module_function_call_name(op, cal);
        const char* bname = same_module_function_call_box_name(op, cal);
        yyjson_val* entry = find_lowering_plan_entry_for_site(bid, ii);
        struct LoweringPlanGenericMethodView method_view;
        const struct LoweringPlanGenericMethodView* method_view_ptr = NULL;
        {
          int user_box_method_rc =
              emit_user_box_method_lowering_plan_mir_call(f, bid, ii, dst, recv, args);
          if (user_box_method_rc != 0) {
            return user_box_method_rc;
          }
        }
        if (same_module_function_call_is_array_or_map_birth(
                ctype, mname, bname)) {
          if (dst) {
            put_const(dst, 0);
            set_type(dst, T_I64);
          }
          return 1;
        }
        if (read_lowering_plan_generic_method_view(entry, &method_view)) {
          method_view_ptr = &method_view;
        }
        return same_module_function_emit_generic_method_mir_call(
            f,
            bid,
            ii,
            method_view_ptr,
            dst,
            recv,
            args);
      }

      auto int same_module_function_emit_call_instruction(
          FILE* f,
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        const char* op = read_str(ins, "op");
        const char* ctype = NULL;
        long long dst = read_int(ins, "dst");
        int rc = 0;
        if (!same_module_function_read_call_payload(ins, &cal, &args)) return 0;
        ctype = same_module_function_call_type(op, cal);
        rc = emit_extern_call_lowering_plan_mir_call(f, bid, ii, dst, args);
        if (rc != 0) {
          return rc;
        }
        if (ctype && !strcmp(ctype, "Extern")) {
          rc = 0;
        } else if (ctype && !strcmp(ctype, "Global")) {
          rc = same_module_function_emit_global_call_mir_call(
              f,
              bid,
              ii,
              cal,
              args,
              dst);
        } else if (ctype && !strcmp(ctype, "Method")) {
          rc = same_module_function_emit_method_call_mir_call(
              f,
              bid,
              ii,
              op,
              cal,
              args,
              dst);
        } else {
          return 0;
        }
        return rc == 1;
      }

      auto int same_module_function_emit_externcall_instruction(
          FILE* f,
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        yyjson_val* args = yyjson_obj_get(ins, "args");
        int rc = emit_extern_call_lowering_plan_mir_call(
            f,
            bid,
            ii,
            read_int(ins, "dst"),
            args);
        return rc == 1;
      }

      auto int same_module_function_emit_ret_instruction(
          long long bid,
          yyjson_val* ins) {
        long long v = read_int(ins, "value");
        long long cv = 0;
        long long base = resolve_copy_source_reg(v);
        char value_ref[64];
        if (has_const(base, &cv)) {
          EMIT("  ret i64 %lld\n", cv);
        } else if (get_type(base) == T_I1) {
          EMIT("  %%ret_zext_%lld_%lld = zext i1 %%r%lld to i64\n", bid, base, base);
          EMIT("  ret i64 %%ret_zext_%lld_%lld\n", bid, base);
        } else {
          same_module_function_format_i64_ref(value_ref, sizeof(value_ref), v);
          EMIT("  ret i64 %s\n", value_ref);
        }
        return 1;
      }

      auto int same_module_function_emit_const_instruction(
          FILE* f,
          size_t fn_index,
          long long bid,
          size_t ii,
          yyjson_val* ins) {
        const char* sval = NULL;
        long long ival = 0;
        long long dst = read_int(ins, "dst");
        if (same_module_function_read_string_const(ins, &sval)) {
          if (!same_module_function_emit_const_box(f, fn_index, dst)) {
            note_pure_unsupported_shape(
                bid,
                ii,
                "const",
                "module_generic_string_const_missing",
                NULL);
            return 0;
          }
        } else if (same_module_function_read_i64_const(ins, &ival)) {
          put_const(dst, ival);
          set_type(dst, T_I64);
        } else if (same_module_function_read_void_sentinel_const(ins)) {
          same_module_function_publish_void_sentinel_const(dst);
        } else {
          return 0;
        }
        return 1;
      }

      auto int same_module_function_emit_branch_instruction(yyjson_val* ins) {
        emit_branch(
            read_int(ins, "cond"),
            read_int(ins, "then"),
            read_int(ins, "else"));
        return 1;
      }

      auto int same_module_function_emit_jump_instruction(yyjson_val* ins) {
        EMIT("  br label %%bb%lld\n", read_int(ins, "target"));
        return 1;
      }

      struct SameModuleArraySlotDirectOpPlan {
        int active;
        size_t get_index;
        size_t set_index;
        long long get_dst;
        long long src_handle;
        long long src_idx;
        long long dst_handle;
        long long store_value;
        long long set_dst;
        long long carrier_dsts[16];
        size_t carrier_count;
      };

      struct SameModuleTypedFieldRmwFusionPlan {
        int active;
        size_t get_index;
        size_t binop_index;
        size_t set_index;
        long long get_dst;
        long long binop_dst;
        long long box_reg;
        long long slot;
        long long delta_reg;
      };

      struct SameModuleResultCapsuleResetBatchPlan {
        int active;
        size_t first_set_index;
        size_t set_indices[4];
        long long box_reg;
      };

      auto int same_module_function_is_selected_array_direct_op_target(long long bid) {
        const char* name = fn0 ? read_str(fn0, "name") : NULL;
        return name &&
               !strcmp(name, "HakoAllocPageModel.acquire_usize/1") &&
               bid == 45;
      }

      auto int same_module_function_name_is_in_list(
          const char* name,
          const char* const* names,
          size_t names_n) {
        if (!(name && names)) return 0;
        for (size_t i = 0; i < names_n; i++) {
          if (!strcmp(name, names[i])) return 1;
        }
        return 0;
      }

      auto int same_module_function_name_is_selected_facade_get_set_fusion_target(
          const char* name) {
        static const char* const selected_facade_get_set_fusion_targets[] = {
            "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2",
            "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseKnownPageIndex/1",
            "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
            "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAllocAligned/2",
        };
        return same_module_function_name_is_in_list(
            name,
            selected_facade_get_set_fusion_targets,
            ARR_LEN(selected_facade_get_set_fusion_targets));
      }

      auto int same_module_function_is_selected_typed_field_rmw_target(void) {
        const char* name = fn0 ? read_str(fn0, "name") : NULL;
        return name &&
               (!strcmp(name, "HakoAllocPageModel.acquire_usize/1") ||
                same_module_function_name_is_selected_facade_get_set_fusion_target(name));
      }

      auto int same_module_function_is_selected_result_capsule_reset_batch_target(void) {
        const char* name = fn0 ? read_str(fn0, "name") : NULL;
        static const char* const selected_result_capsule_reset_targets[] = {
            "HakoAllocObjectLifecycleAllocResult.reset/0",
            "HakoAllocObjectLifecycleReleaseResult.reset/0",
        };
        return same_module_function_name_is_in_list(
            name,
            selected_result_capsule_reset_targets,
            ARR_LEN(selected_result_capsule_reset_targets));
      }

      auto int same_module_function_instruction_is_array_method(
          yyjson_val* ins,
          const char* expected_method,
          yyjson_val** args_out,
          long long* receiver_out) {
        yyjson_val* cal = NULL;
        yyjson_val* args = NULL;
        const char* op = read_str(ins, "op");
        const char* ctype = NULL;
        const char* bname = NULL;
        const char* mname = NULL;
        long long recv = 0;
        if (!same_module_function_read_call_payload(ins, &cal, &args)) return 0;
        ctype = same_module_function_call_type(op, cal);
        bname = same_module_function_call_box_name(op, cal);
        mname = same_module_function_call_name(op, cal);
        recv = same_module_function_call_receiver(op, cal);
        if (!(ctype && !strcmp(ctype, "Method") &&
              bname && !strcmp(bname, "ArrayBox") &&
              mname && expected_method && !strcmp(mname, expected_method))) {
          return 0;
        }
        if (args_out) *args_out = args;
        if (receiver_out) *receiver_out = recv;
        return 1;
      }

      auto int same_module_function_array_arg_at(
          yyjson_val* args,
          size_t index,
          long long* out) {
        yyjson_val* val = args && yyjson_is_arr(args) && yyjson_arr_size(args) > index
                            ? yyjson_arr_get(args, index)
                            : NULL;
        if (!(val && yyjson_is_num(val))) return 0;
        if (out) *out = (long long)yyjson_get_sint(val);
        return 1;
      }

      auto int same_module_function_plan_contains_carrier(
          const struct SameModuleArraySlotDirectOpPlan* plan,
          long long reg) {
        if (!plan || !reg) return 0;
        for (size_t i = 0; i < plan->carrier_count; i++) {
          if (plan->carrier_dsts[i] == reg) return 1;
        }
        return 0;
      }

      auto int same_module_function_plan_add_carrier(
          struct SameModuleArraySlotDirectOpPlan* plan,
          long long reg) {
        if (!plan || !reg) return 0;
        if (same_module_function_plan_contains_carrier(plan, reg)) return 1;
        if (plan->carrier_count >= ARR_LEN(plan->carrier_dsts)) return 0;
        plan->carrier_dsts[plan->carrier_count++] = reg;
        return 1;
      }

      auto int same_module_function_match_array_slot_direct_op_plan(
          long long bid,
          yyjson_val* insts,
          size_t ilen,
          unsigned char* skip_inst,
          struct SameModuleArraySlotDirectOpPlan* plan) {
        if (!plan) return 0;
        memset(plan, 0, sizeof(*plan));
        if (!(insts && skip_inst)) return 0;
        if (!same_module_function_is_selected_array_direct_op_target(bid)) return 0;
        for (size_t gi = 0; gi < ilen; gi++) {
          yyjson_val* get_ins = yyjson_arr_get(insts, gi);
          yyjson_val* get_args = NULL;
          long long get_recv = 0;
          long long get_arg0 = 0;
          long long get_idx = 0;
          long long get_dst = read_int(get_ins, "dst");
          if (!get_dst) continue;
          if (!same_module_function_instruction_is_array_method(
                  get_ins,
                  "get",
                  &get_args,
                  &get_recv)) {
            continue;
          }
          if (!same_module_function_array_arg_at(get_args, 0, &get_arg0)) continue;
          if (yyjson_arr_size(get_args) >= 2) {
            if (!same_module_function_array_arg_at(get_args, 1, &get_idx)) continue;
          } else {
            get_idx = get_arg0;
          }
          memset(skip_inst, 0, ilen);
          plan->get_index = gi;
          plan->get_dst = get_dst;
          plan->src_handle = get_recv ? get_recv : get_arg0;
          plan->src_idx = get_idx;
          plan->carrier_count = 0;
          if (!same_module_function_plan_add_carrier(plan, get_dst)) return 0;

          for (size_t si = gi + 1; si < ilen; si++) {
            yyjson_val* ins = yyjson_arr_get(insts, si);
            const char* op = read_str(ins, "op");
            if (op && !strcmp(op, "copy")) {
              long long src = read_int(ins, "src");
              long long dst = read_int(ins, "dst");
              if (same_module_function_plan_contains_carrier(plan, src)) {
                if (!same_module_function_plan_add_carrier(plan, dst)) return 0;
                skip_inst[si] = 1;
              }
              continue;
            }
            if (op && !strcmp(op, "mir_call")) {
              yyjson_val* set_args = NULL;
              long long set_recv = 0;
              long long set_arg0 = 0;
              long long set_key = 0;
              long long set_value = 0;
              size_t argc = 0;
              if (!same_module_function_instruction_is_array_method(
                      ins,
                      "set",
                      &set_args,
                      &set_recv)) {
                continue;
              }
              argc = set_args && yyjson_is_arr(set_args) ? yyjson_arr_size(set_args) : 0;
              if (argc == 3) {
                if (!same_module_function_array_arg_at(set_args, 0, &set_arg0)) return 0;
                if (!same_module_function_array_arg_at(set_args, 1, &set_key)) return 0;
                if (!same_module_function_array_arg_at(set_args, 2, &set_value)) return 0;
              } else if (argc == 2) {
                if (!same_module_function_array_arg_at(set_args, 0, &set_key)) return 0;
                if (!same_module_function_array_arg_at(set_args, 1, &set_value)) return 0;
              } else {
                return 0;
              }
              if (!same_module_function_plan_contains_carrier(plan, set_key)) {
                continue;
              }
              plan->set_index = si;
              plan->dst_handle = set_recv ? set_recv : set_arg0;
              plan->store_value = set_value;
              plan->set_dst = read_int(ins, "dst");
              plan->active = 1;
              skip_inst[gi] = 1;
              return 1;
            }
          }
        }
        memset(plan, 0, sizeof(*plan));
        memset(skip_inst, 0, ilen);
        return 0;
      }

      auto int same_module_function_emit_array_slot_direct_op(
          const struct SameModuleArraySlotDirectOpPlan* plan) {
        char ab[512];
        if (!(plan && plan->active)) return 0;
        ab[0] = '\0';
        append_i64_arg_ref(ab, sizeof(ab), plan->src_handle, 1);
        append_i64_arg_ref(ab, sizeof(ab), plan->src_idx, 0);
        append_i64_arg_ref(ab, sizeof(ab), plan->dst_handle, 0);
        append_i64_arg_ref(ab, sizeof(ab), plan->store_value, 0);
        same_module_function_emit_i64_call(
            plan->get_dst,
            "nyash.array.slot_load_store_i64_hihi",
            ab);
        for (size_t i = 0; i < plan->carrier_count; i++) {
          long long dst = plan->carrier_dsts[i];
          if (dst == plan->get_dst) continue;
          set_copy_alias(dst, resolve_copy_source_reg(plan->get_dst));
          set_type(dst, T_I64);
        }
        if (plan->set_dst) {
          put_const(plan->set_dst, 1);
          set_type(plan->set_dst, T_I64);
        }
        return 1;
      }

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
          return same_module_function_emit_typed_object_field_get(ins);
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

      auto int same_module_function_emit_active_body(FILE* f, size_t fn_index) {
        same_module_function_body_failed_bid = -1;
        same_module_function_body_failed_ii = 0;
        same_module_function_body_failed_op = NULL;
        for (size_t bi = 0; bi < generic_pure_block_count; bi++) {
          struct GenericPureBlockView block;
          if (!read_generic_pure_block_view(bi, &block)) return 0;
          emit_block_label(block.id);
          for (int pi = 0; pi < phi_n; pi++) {
            if (phis[pi].bid != block.id) continue;
            for (int ri = 0; ri < phis[pi].rec_n; ri++) {
              emit_phi(phis[pi].recs[ri].dst, phis[pi].recs[ri].in, phis[pi].recs[ri].in_n);
            }
          }
          unsigned char skip_inst[block.instruction_count ? block.instruction_count : 1];
          struct SameModuleArraySlotDirectOpPlan array_direct_plan;
          struct SameModuleResultCapsuleResetBatchPlan result_capsule_reset_batch_plan;
          struct SameModuleTypedFieldRmwFusionPlan typed_field_rmw_plans[16];
          size_t typed_field_rmw_plans_n = 0;
          memset(skip_inst, 0, sizeof(skip_inst));
          memset(&result_capsule_reset_batch_plan, 0, sizeof(result_capsule_reset_batch_plan));
          memset(typed_field_rmw_plans, 0, sizeof(typed_field_rmw_plans));
          same_module_function_match_array_slot_direct_op_plan(
              block.id,
              block.instructions,
              block.instruction_count,
              skip_inst,
              &array_direct_plan);
          same_module_function_match_result_capsule_reset_batch_plan(
              block.instructions,
              block.instruction_count,
              skip_inst,
              &result_capsule_reset_batch_plan);
          typed_field_rmw_plans_n =
              same_module_function_match_typed_field_rmw_fusion_plans(
                  block.instructions,
                  block.instruction_count,
                  skip_inst,
                  typed_field_rmw_plans,
                  ARR_LEN(typed_field_rmw_plans));
          for (size_t ii = 0; ii < block.instruction_count; ii++) {
            yyjson_val* ins = yyjson_arr_get(block.instructions, ii);
            const struct SameModuleTypedFieldRmwFusionPlan* typed_field_rmw_plan =
                same_module_function_find_typed_field_rmw_fusion_plan(
                    typed_field_rmw_plans,
                    typed_field_rmw_plans_n,
                    ii);
            if (skip_inst[ii]) continue;
            if (array_direct_plan.active && ii == array_direct_plan.set_index) {
              if (!same_module_function_emit_array_slot_direct_op(&array_direct_plan)) {
                same_module_function_record_body_failure(block.id, ii, ins);
                return 0;
              }
              continue;
            }
            if (result_capsule_reset_batch_plan.active &&
                ii == result_capsule_reset_batch_plan.first_set_index) {
              if (!same_module_function_emit_result_capsule_reset_batch(
                      &result_capsule_reset_batch_plan,
                      block.id,
                      ii)) {
                same_module_function_record_body_failure(block.id, ii, ins);
                return 0;
              }
              continue;
            }
            if (typed_field_rmw_plan) {
              if (!same_module_function_emit_typed_field_rmw_fusion(
                      typed_field_rmw_plan,
                      block.id,
                      ii)) {
                same_module_function_record_body_failure(block.id, ii, ins);
                return 0;
              }
              continue;
            }
            if (!same_module_function_emit_instruction(
                    f,
                    fn_index,
                    block.id,
                    ii,
                    ins,
                    block.instructions,
                    block.instruction_count)) {
              same_module_function_record_body_failure(block.id, ii, ins);
              return 0;
            }
          }
        }
        return 1;
      }
