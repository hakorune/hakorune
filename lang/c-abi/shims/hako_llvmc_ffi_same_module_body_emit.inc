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
        for (size_t bi = 0; bi < generic_pure_block_count; bi++) {
          struct GenericPureBlockView block;
          if (!read_generic_pure_block_view(bi, &block)) continue;
          for (size_t ii = 0; ii < block.instruction_count; ii++) {
            yyjson_val* ins = yyjson_arr_get(block.instructions, ii);
            const char* op = read_str(ins, "op");
            const char* field = read_str(ins, "field");
            if (!(op && !strcmp(op, "field_get") && field)) continue;
            size_t flen = strlen(field);
            fprintf(f, "@.fnfield_%zu_%lld_%zu = private unnamed_addr constant [%zu x i8] c\"",
                    fn_index,
                    block.id,
                    ii,
                    flen + 1);
            hako_llvmc_emit_llvm_byte_string(f, field, (long long)flen);
            fprintf(f, "\\00\", align 1\n");
          }
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

      auto int same_module_function_emit_map_field_get(
          FILE* f,
          size_t fn_index,
          yyjson_val* ins,
          long long bid,
          size_t ii) {
        long long dst = read_int(ins, "dst");
        long long box_reg = read_int(ins, "box");
        long long box_ref_reg = box_reg;
        const char* field = read_str(ins, "field");
        char map_ref[64];
        if (!(dst && box_reg && field)) return 0;
        box_ref_reg = resolve_copy_source_reg(box_reg);
        if (get_origin(box_ref_reg) != ORG_MAP_BIRTH) return 0;
        same_module_function_format_i64_ref(map_ref, sizeof(map_ref), box_ref_reg);
        EMIT("  %%map_field_key_%lld_%zu = call i64 @\"nyash.box.from_i8_string_const\"(ptr getelementptr inbounds ([%zu x i8], ptr @.fnfield_%zu_%lld_%zu, i64 0, i64 0))\n",
             bid,
             ii,
             strlen(field) + 1,
             fn_index,
             bid,
             ii);
        EMIT("  %%r%lld = call i64 @\"nyash.runtime_data.get_hh\"(i64 %s, i64 %%map_field_key_%lld_%zu)\n",
             dst,
             map_ref,
             bid,
             ii);
        if (!same_module_function_publish_reg_metadata(dst)) {
          set_type(dst, T_I64);
        }
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
          {
            int boxed_sum_plan_index = -1;
            if (get_boxed_sum_binding(src, &boxed_sum_plan_index)) {
              set_boxed_sum_binding(dst, boxed_sum_plan_index);
            }
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
        {
          int boxed_sum_plan_index = -1;
          if (get_boxed_sum_binding(src, &boxed_sum_plan_index)) {
            set_boxed_sum_binding(dst, boxed_sum_plan_index);
          }
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

      auto int same_module_function_emit_compare(yyjson_val* ins, long long bid, size_t ii) {
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
          int boxed_sum_compare_emit = emit_boxed_sum_unit_compare(
              bid,
              ii,
              "compare",
              dst,
              p2,
              lhs,
              rhs);
          if (boxed_sum_compare_emit < 0) return 0;
          if (boxed_sum_compare_emit > 0) return 1;
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

      auto int same_module_function_emit_typeop(yyjson_val* ins) {
        const char* kind = read_str(ins, "operation");
        const char* target_type = read_str(ins, "target_type");
        long long dst = read_int(ins, "dst");
        long long src = read_int(ins, "src");
        long long src_base = 0;
        const char* type_global = NULL;
        size_t type_global_len = 0;
        if (!kind) kind = read_str(ins, "op_kind");
        if (!src) src = read_int(ins, "value");
        if (!(kind && (!strcmp(kind, "check") || !strcmp(kind, "Check")) &&
              target_type && dst)) {
          return 0;
        }
        if (!strcmp(target_type, "MapBox")) {
          type_global = "@.hako_type_MapBox";
          type_global_len = 7;
        } else if (!strcmp(target_type, "ArrayBox")) {
          type_global = "@.hako_type_ArrayBox";
          type_global_len = 9;
        } else if (!strcmp(target_type, "StringBox") || !strcmp(target_type, "String")) {
          type_global = "@.hako_type_StringBox";
          type_global_len = 10;
        }
        if (!type_global) return 0;
        src_base = resolve_copy_source_reg(src);
        EMIT("  %%r%lld = call i64 @\"nyash.any.is_type_h\"(i64 %%r%lld, ptr getelementptr inbounds ([%zu x i8], ptr %s, i64 0, i64 0))\n",
             dst,
             src_base,
             type_global_len,
             type_global);
        set_type(dst, T_I64);
        return 1;
      }

      #include "hako_llvmc_ffi_same_module_typed_object_emit.inc"
      #include "hako_llvmc_ffi_same_module_generic_method_emit.inc"
      #include "hako_llvmc_ffi_same_module_direct_state_emit.inc"

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
          int direct_state_rc =
              same_module_function_emit_direct_state_record_alloc_success_mir_call(
                  f,
                  bid,
                  ii,
                  dst,
                  recv,
                  args,
                  bname,
                  mname);
          if (direct_state_rc != 0) {
            return direct_state_rc;
          }
        }
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
          long long a0 = 0;
          if (classify_mir_call_string_extern_surface(read_str(cal, "name")) ==
                  HAKO_LLVMC_MIR_CALL_STRING_EXTERN_SURFACE_CONSOLE_LOG &&
              same_module_function_read_arg_reg(args, 0, &a0)) {
            rc = emit_global_mir_call(
                f,
                HAKO_LLVMC_MIR_CALL_GLOBAL_SURFACE_PRINT,
                bid,
                ii,
                a0);
          } else {
            rc = 0;
          }
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

      auto int same_module_function_selected_record_success_helper_kind(void) {
        const char* name = fn0 ? read_str(fn0, "name") : NULL;
        if (!typed_object_exact_slot_helper_enabled()) return 0;
        if (name && !strcmp(name, "HakoAllocObjectLifecycleAllocResult.recordSuccess/1")) {
          return 1;
        }
        if (name && !strcmp(name, "HakoAllocObjectLifecycleReleaseResult.recordSuccess/2")) {
          return 2;
        }
        return 0;
      }
      #include "hako_llvmc_ffi_same_module_array_direct_op_emit.inc"

      #include "hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc"
      auto int same_module_function_emit_active_body(FILE* f, size_t fn_index) {
        same_module_function_body_failed_bid = -1;
        same_module_function_body_failed_ii = 0;
        same_module_function_body_failed_op = NULL;
        {
          int record_success_rc = same_module_function_emit_record_success_helper_body();
          if (record_success_rc != 0) return record_success_rc == 1;
        }
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
          same_module_function_read_result_capsule_reset_batch_plan(
              block.id,
              block.instruction_count,
              skip_inst,
              &result_capsule_reset_batch_plan);
          typed_field_rmw_plans_n =
              same_module_function_read_typed_field_rmw_fusion_plans(
                  block.id,
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
