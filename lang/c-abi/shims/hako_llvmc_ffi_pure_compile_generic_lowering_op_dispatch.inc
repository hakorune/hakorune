          if (strcmp(op, "const")==0) {
            long long dst = read_int(ins, "dst");
            const char* sval = get_str_const(dst);
            if (!sval) continue; // numeric const stays inlined
            if (should_skip_string_const_box_for_concat_suffix(ii, ilen, insts, dst)) continue;
            if (should_skip_hoisted_string_const_box(dst)) {
              set_type(dst, T_I64);
              set_origin(dst, ORG_STRING);
              set_scan_origin(dst, ORG_STRING);
              continue;
            }
            if (emit_string_const_box_result(f, dst)) continue;
            continue;
          }
          if (strcmp(op, "keepalive")==0) {
            continue;
          }
          if (strcmp(op, "release_strong")==0) {
            continue;
          }
          if (strcmp(op, "newbox")==0) {
            long long dst = read_int(ins, "dst");
            const char* tname = read_str(ins, "type");
            struct GenericPureTypedObjectNewPlanView typed_new_plan;
            enum GenericPureTypedObjectNewPlanStatus typed_new_status =
                GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_MISSING;
            if (tname && strcmp(tname, "DirectArrayI64") == 0) {
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.array.direct_i64.birth_h\"()\n", dst);
                set_type(dst, T_I64);
                set_origin(dst, ORG_ARRAY_BIRTH);
                set_scan_origin(dst, ORG_ARRAY_BIRTH);
              } else {
                EMIT("  call i64 @\"nyash.array.direct_i64.birth_h\"()\n");
              }
              continue;
            }
            if (tname && strcmp(tname, "ArrayBox") == 0) {
              const char* birth_symbol = hako_llvmc_direct_array_i64_exact_enabled()
                                             ? "nyash.array.direct_i64.birth_h"
                                             : "nyash.array.birth_h";
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"%s\"()\n", dst, birth_symbol);
                set_type(dst, T_I64);
                set_origin(dst, ORG_ARRAY_BIRTH);
                set_scan_origin(dst, ORG_ARRAY_BIRTH);
              } else {
                EMIT("  call i64 @\"%s\"()\n", birth_symbol);
              }
              continue;
            }
            if (tname && strcmp(tname, "MapBox") == 0) {
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.map.birth_h\"()\n", dst);
                set_type(dst, T_I64);
                set_origin(dst, ORG_MAP_BIRTH);
                set_scan_origin(dst, ORG_MAP_BIRTH);
              } else {
                EMIT("  call i64 @\"nyash.map.birth_h\"()\n");
              }
              continue;
            }
            if (tname && strcmp(tname, "FileBox") == 0) {
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.env.box.new\"(ptr getelementptr inbounds ([8 x i8], ptr @.hako_type_FileBox, i64 0, i64 0))\n", dst);
                set_type(dst, T_I64);
                set_origin(dst, ORG_FILEBOX);
                set_scan_origin(dst, ORG_FILEBOX);
              } else {
                EMIT("  call i64 @\"nyash.env.box.new\"(ptr getelementptr inbounds ([8 x i8], ptr @.hako_type_FileBox, i64 0, i64 0))\n");
              }
              continue;
            }
            if (tname && strcmp(tname, "StringBox") == 0) {
              yyjson_val* nb_args = yyjson_obj_get(ins, "args");
              yyjson_val* nb0 = nb_args && yyjson_is_arr(nb_args) ? yyjson_arr_get_first(nb_args) : NULL;
              long long arg0 = nb0 ? (long long)yyjson_get_sint(nb0) : 0;
              if (dst && arg0) {
                if (should_skip_hoisted_string_const_box(arg0) &&
                    !string_const_box_was_emitted(arg0) &&
                    !emit_materialized_string_const_box_result(f, arg0)) {
                  note_pure_unsupported_shape(
                      bid,
                      ii,
                      op,
                      "string_const_materialization_failed",
                      NULL);
                  yyjson_doc_free(d);
                  goto GEN_ABORT;
                }
                set_copy_alias(dst, resolve_copy_source_reg(arg0));
                set_type(dst, T_I64);
                set_origin(dst, ORG_STRING);
                set_scan_origin(dst, ORG_STRING);
                continue;
              }
            }
            typed_new_status = generic_pure_validate_typed_object_new_plan(
                dst,
                tname,
                &typed_new_plan);
            if (typed_new_status == GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_OK) {
              EMIT("  %%r%lld = call i64 @\"nyash.object.new_typed_hi\"(i64 %lld, i64 %lld)\n",
                   dst,
                   typed_new_plan.type_id,
                   typed_new_plan.field_count);
              generic_pure_publish_typed_object_plan_index(
                  dst,
                  typed_new_plan.typed_plan_index);
              continue;
            }
            if (typed_new_status == GENERIC_PURE_TYPED_OBJECT_NEW_PLAN_INVALID) {
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "typed_object_plan_invalid_new",
                  NULL);
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            note_pure_unsupported_shape(bid, ii, op, "unsupported_newbox_type", NULL);
            yyjson_doc_free(d); goto GEN_ABORT;
          }
          if (strcmp(op, "field_set")==0) {
            long long box_reg = read_int(ins, "box");
            long long value_reg = read_int(ins, "value");
            const char* field = read_str(ins, "field");
            struct GenericPureTypedObjectFieldSlotView field_slot;
            long long value_base = resolve_copy_source_reg(value_reg);
            long long value_const = 0;
            if (field &&
                !strcmp(field, "alignment_result") &&
                object_storage_plan_alignment_result_owner_receiver_ready(box_reg)) {
              if (!emit_flattened_nested_alignment_result_reset(f, bid, ii, box_reg)) {
                note_pure_unsupported_shape(
                    bid,
                    ii,
                    op,
                    "flattened_nested_field_set_lowering_failed",
                    NULL);
                yyjson_doc_free(d);
                goto GEN_ABORT;
              }
              continue;
            }
            if (value_reg &&
                generic_pure_read_typed_object_field_slot(
                    box_reg,
                    field,
                    &field_slot)) {
              long long box_base = resolve_copy_source_reg(box_reg);
              int exact_unsigned = typed_object_storage_is_unsigned_exact(field_slot.storage);
              int exact_signed = typed_object_storage_is_signed_exact(field_slot.storage);
              int exact_slot_u64 =
                  typed_object_exact_slot_helper_enabled() &&
                  typed_object_storage_is_exact_slot_u64(field_slot.storage);
              int exact_slot_i64 =
                  typed_object_exact_slot_helper_enabled() &&
                  typed_object_storage_is_exact_slot_i64(field_slot.storage);
              int exact_slot_handle =
                  typed_object_exact_slot_helper_enabled() &&
                  typed_object_storage_is_exact_slot_handle(field_slot.storage);
              int status_returning_set =
                  exact_unsigned || exact_signed || exact_slot_u64 || exact_slot_i64 ||
                  exact_slot_handle;
              const char* set_symbol = exact_slot_u64
                  ? "nyash.object.exact_slot_set_u64_hiu"
                  : (exact_slot_i64
                         ? "nyash.object.exact_slot_set_i64_hii"
                         : (exact_slot_handle
                                ? "nyash.object.exact_slot_set_handle_hii"
                                : (exact_unsigned
                                       ? "nyash.object.field_set_u64_hiu"
                                       : (exact_signed
                                              ? "nyash.object.field_set_i64_hii"
                                              : "nyash.object.field_set_hii"))));
              char status_name[96];
              if (exact_unsigned || exact_slot_u64) {
                if (has_const(value_base, &value_const)) {
                  if (value_const < 0) {
                    note_pure_unsupported_shape(
                        bid,
                        ii,
                        op,
                        "typed_object_unsigned_negative_const",
                        NULL);
                    yyjson_doc_free(d);
                    goto GEN_ABORT;
                  }
                } else {
                  EMIT("  %%exact_unsigned_nonnegative_%lld_%zu = icmp sge i64 %%r%lld, 0\n",
                       bid,
                       ii,
                       value_base);
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
              }
              if (has_const(value_base, &value_const)) {
                if (status_returning_set) {
                  snprintf(status_name, sizeof(status_name), "exact_field_set_%lld_%zu", bid, ii);
                  EMIT("  %%%s = call i64 @\"%s\"(i64 %%r%lld, i64 %lld, i64 %lld)\n",
                       status_name,
                       set_symbol,
                       box_base,
                       field_slot.slot,
                       value_const);
                } else {
                  EMIT("  call void @\"%s\"(i64 %%r%lld, i64 %lld, i64 %lld)\n",
                       set_symbol,
                       box_base,
                       field_slot.slot,
                       value_const);
                }
              } else {
                if (status_returning_set) {
                  snprintf(status_name, sizeof(status_name), "exact_field_set_%lld_%zu", bid, ii);
                  EMIT("  %%%s = call i64 @\"%s\"(i64 %%r%lld, i64 %lld, i64 %%r%lld)\n",
                       status_name,
                       set_symbol,
                       box_base,
                       field_slot.slot,
                       value_base);
                } else {
                  EMIT("  call void @\"%s\"(i64 %%r%lld, i64 %lld, i64 %%r%lld)\n",
                       set_symbol,
                       box_base,
                       field_slot.slot,
                       value_base);
                }
              }
              if (status_returning_set) {
                EMIT("  %%exact_status_ok_%lld_%zu = icmp ne i64 %%%s, 0\n",
                     bid,
                     ii,
                     status_name);
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
              continue;
            }
            note_pure_unsupported_shape(
                bid,
                ii,
                op,
                "typed_object_field_set_plan_missing",
                NULL);
            yyjson_doc_free(d);
            goto GEN_ABORT;
          }
          if (strcmp(op, "field_get")==0) {
            long long dst = read_int(ins, "dst");
            long long box_reg = read_int(ins, "box");
            const char* field = read_str(ins, "field");
            struct GenericPureTypedObjectFieldSlotView field_slot;
            if (dst &&
                field &&
                !strcmp(field, "alignment_result") &&
                object_storage_plan_alignment_result_owner_receiver_ready(box_reg)) {
              EMIT("  %%r%lld = add i64 %%r%lld, 0\n",
                   dst,
                   resolve_copy_source_reg(box_reg));
              set_flattened_nested_binding(dst, box_reg);
              set_type(dst, T_I64);
              continue;
            }
            if (dst &&
                generic_pure_read_typed_object_field_slot(
                    box_reg,
                    field,
                    &field_slot)) {
              long long box_base = resolve_copy_source_reg(box_reg);
              const char* get_symbol =
                  (typed_object_exact_slot_helper_enabled() &&
                   typed_object_storage_is_exact_slot_u64(field_slot.storage))
                      ? "nyash.object.exact_slot_get_u64_hii"
                      : ((typed_object_exact_slot_helper_enabled() &&
                          typed_object_storage_is_exact_slot_i64(field_slot.storage))
                             ? "nyash.object.exact_slot_get_i64_hii"
                             : ((typed_object_exact_slot_helper_enabled() &&
                                 typed_object_storage_is_exact_slot_handle(field_slot.storage))
                                    ? "nyash.object.exact_slot_get_handle_hii"
                                    : (typed_object_storage_is_unsigned_exact(field_slot.storage)
                                           ? "nyash.object.field_get_u64_hii"
                                           : (typed_object_storage_is_signed_exact(field_slot.storage)
                                                  ? "nyash.object.field_get_i64_hii"
                                                  : "nyash.object.field_get_hii"))));
              EMIT("  %%r%lld = call i64 @\"%s\"(i64 %%r%lld, i64 %lld)\n",
                   dst,
                   get_symbol,
                   box_base,
                   field_slot.slot);
              if (!generic_pure_publish_reg_metadata(dst)) {
                set_type(dst, T_I64);
              }
              continue;
            }
            note_pure_unsupported_shape(
                bid,
                ii,
                op,
                "typed_object_field_get_plan_missing",
                NULL);
            yyjson_doc_free(d);
            goto GEN_ABORT;
          }
          if (strcmp(op, "static_data_load")==0) {
            char reason[64];
            if (!hako_llvmc_emit_static_data_load_instruction(
                    ins,
                    reason,
                    sizeof(reason))) {
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  reason[0] ? reason : "static_data_load_lowering_failed",
                  NULL);
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            continue;
          }
          if (strcmp(op, "copy")==0) {
            long long dst = read_int(ins, "dst");
            long long src = read_int(ins, "src");
            long long cv = 0;
            int src_ty = get_type(src);
            int src_origin = get_origin(src);
            int src_scan_origin = get_scan_origin(src);
            set_copy_alias(dst, resolve_copy_source_reg(src));
            if (has_const(src, &cv)) {
              put_const(dst, cv);
              set_type(dst, T_I64);
            } else if (src_ty == T_I1) {
              set_type(dst, T_I1);
            } else {
              set_type(dst, src_ty == T_NONE ? T_I64 : src_ty);
            }
            if (src_origin != ORG_NONE) set_origin(dst, src_origin);
            if (src_scan_origin != ORG_NONE) set_scan_origin(dst, src_scan_origin);
            {
              int typed_plan_index = -1;
              if (get_typed_object_binding(src, &typed_plan_index)) {
                set_typed_object_binding(dst, typed_plan_index);
              }
              {
                long long flattened_owner_reg = 0;
                if (get_flattened_nested_binding(src, &flattened_owner_reg)) {
                  set_flattened_nested_binding(dst, flattened_owner_reg);
                }
              }
              generic_pure_publish_reg_metadata(dst);
            }
            continue;
          }
          if (strcmp(op, "unop")==0) {
            const char* kind = read_str(ins, "op_kind");
            long long dst = read_int(ins, "dst");
            long long src = read_int(ins, "src");
            long long base = 0;
            long long cv = 0;
            int src_ty = T_NONE;
            int src_const = 0;
            if (!kind) kind = read_str(ins, "operation");
            if (!src) src = read_int(ins, "operand");
            if (!(kind && (!strcmp(kind, "not") || !strcmp(kind, "Not")))) {
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "unsupported_unop_kind",
                  NULL);
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            if (!(dst && src)) {
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "malformed_unop_operands",
                  NULL);
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            base = resolve_copy_source_reg(src);
            src_ty = get_type(base);
            src_const = has_const(base, &cv);
            if (src_ty == T_I1) {
              if (src_const) {
                EMIT("  %%r%lld = xor i1 %s, true\n", dst, cv ? "true" : "false");
              } else {
                EMIT("  %%r%lld = xor i1 %%r%lld, true\n", dst, base);
              }
            } else if (src_const) {
              EMIT("  %%r%lld = icmp eq i64 %lld, 0\n", dst, cv);
            } else {
              EMIT("  %%r%lld = icmp eq i64 %%r%lld, 0\n", dst, base);
            }
            set_type(dst, T_I1);
            continue;
          }
          #include "hako_llvmc_ffi_pure_compile_variant_dispatch.inc"
          if (strcmp(op, "compare")==0) {
            const char* pred = read_str(ins, "cmp"); if (!pred) pred = read_str(ins, "operation");
            const char* p2 = NULL; if (pred){ if (!strcmp(pred,"Lt")||!strcmp(pred,"lt")||!strcmp(pred,"<")) p2="slt"; else if (!strcmp(pred,"Le")||!strcmp(pred,"le")||!strcmp(pred,"<=")) p2="sle"; else if (!strcmp(pred,"Eq")||!strcmp(pred,"eq")||!strcmp(pred,"==")) p2="eq"; else if (!strcmp(pred,"Ne")||!strcmp(pred,"ne")||!strcmp(pred,"!=")) p2="ne"; else if (!strcmp(pred,"Ge")||!strcmp(pred,"ge")||!strcmp(pred,">=")) p2="sge"; else if (!strcmp(pred,"Gt")||!strcmp(pred,"gt")||!strcmp(pred,">")) p2="sgt"; }
            if (!p2) {
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "unsupported_compare_predicate",
                  NULL);
              yyjson_doc_free(d); goto GEN_ABORT;
            }
            long long dst = read_int(ins, "dst"); long long lhs = read_int(ins, "lhs"); if (!lhs) lhs=read_int(ins, "left"); long long rhs = read_int(ins, "rhs"); if (!rhs) rhs=read_int(ins, "right"); if (!lhs && !rhs){
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "malformed_compare_operands",
                  NULL);
              yyjson_doc_free(d); goto GEN_ABORT;
            }
            if (should_emit_string_compare(ins, lhs, rhs)) {
              emit_string_compare(dst, p2, lhs, rhs);
              continue;
            }
            int boxed_sum_compare_emit = emit_boxed_sum_unit_compare(
                bid,
                ii,
                op,
                dst,
                p2,
                lhs,
                rhs);
            if (boxed_sum_compare_emit < 0) {
              yyjson_doc_free(d); goto GEN_ABORT;
            }
            if (boxed_sum_compare_emit > 0) {
              continue;
            }
            // emit icmp
            emit_icmp(dst, p2, lhs, rhs);
            continue;
          }
          if (strcmp(op, "select")==0) {
            long long dst = read_int(ins, "dst");
            long long cond = read_int(ins, "cond");
            long long then_val = read_int(ins, "then_val");
            long long else_val = read_int(ins, "else_val");
            long long cond_cv = 0, then_cv = 0, else_cv = 0;
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
            continue;
          }
          if (strcmp(op, "branch")==0) { long long cond=read_int(ins, "cond"); long long th=read_int(ins,"then"); long long el=read_int(ins,"else"); if(!el) el=read_int(ins,"else_id"); emit_branch(cond, th, el); continue; }
          if (strcmp(op, "jump")==0) { long long tgt=read_int(ins, "target"); EMIT("  br label %%bb%lld\n", tgt); continue; }
          if (strcmp(op, "ret")==0) {
            long long v=read_int(ins, "value"); long long cv;
            if (has_const(v,&cv)) { EMIT("  ret i64 %lld\n", cv); }
            else {
              int org = get_origin(v);
              long long base = resolve_copy_source_reg(v);
              if (org == ORG_MAP_GET) {
                // Auto-unbox map.get result as Integer (minimal MVP)
                EMIT("  %%map_get_unbox_%lld = call i64 @\"nyash.integer.get_h\"(i64 %%r%lld)\n", v, base);
                EMIT("  ret i64 %%map_get_unbox_%lld\n", v);
              } else if (get_type(v) == T_I1 || get_type(base) == T_I1) {
                EMIT("  %%ret_zext_%lld = zext i1 %%r%lld to i64\n", v, base);
                EMIT("  ret i64 %%ret_zext_%lld\n", v);
              } else {
                EMIT("  ret i64 %%r%lld\n", base);
              }
            }
            continue;
          }
          if (strcmp(op, "binop")==0) {
            const char* k = read_str(ins, "op_kind"); if (!k) k = read_str(ins, "operation");
            const char* irop = NULL;
            if (k) {
              if (!strcmp(k, "Add") || !strcmp(k, "+")) irop = "add";
              else if (!strcmp(k, "Sub") || !strcmp(k, "-")) irop = "sub";
              else if (!strcmp(k, "Mul") || !strcmp(k, "*")) irop = "mul";
              else if (!strcmp(k, "Div") || !strcmp(k, "/")) irop = "sdiv";
              else if (!strcmp(k, "Mod") || !strcmp(k, "%")) irop = "srem";
            }
            if (!irop) {
              note_pure_unsupported_shape(bid, ii, op, "unsupported_binop_kind", NULL);
              yyjson_doc_free(d); goto GEN_ABORT;
            }
            long long dst = read_int(ins, "dst"); long long lhs = read_int(ins, "lhs"); long long rhs = read_int(ins, "rhs");
            if (!strcmp(irop, "add") &&
                (generic_pure_value_has_string_origin(lhs) ||
                 generic_pure_value_has_string_origin(rhs))) {
              emit_string_concat_pair_by_policy(f, ii, ilen, insts, dst, lhs, rhs);
              continue;
            }
            long long lhs_base = resolve_copy_source_reg(lhs);
            long long rhs_base = resolve_copy_source_reg(rhs);
            long long vL, vR; int lc = has_const(lhs_base, &vL), rc2 = has_const(rhs_base, &vR);
            if (lc && rc2) {
              EMIT("  %%r%lld = %s i64 %lld, %lld\n", dst, irop, vL, vR);
            } else {
              EMIT("  %%r%lld = %s i64 %s%lld%s, %s%lld%s\n", dst, irop,
                   lc?"":"%r", lc?vL:lhs_base, lc?"":"",
                   rc2?"":"%r", rc2?vR:rhs_base, rc2?"":"");
            }
            set_type(dst, T_I64);
            continue;
          }
          if (strcmp(op, "call")==0) {
            yyjson_val* cal = yyjson_obj_get(ins, "callee");
            const char* ctype = cal ? read_str(cal, "type") : NULL;
            const char* bname = cal
                ? (read_str(cal, "box_name")
                    ? read_str(cal, "box_name")
                    : read_str(cal, "box_type"))
                : NULL;
            const char* mname = NULL;
            long long dst = read_int(ins, "dst");
            long long recv = cal ? read_int(cal, "receiver") : 0;
            yyjson_val* args = yyjson_obj_get(ins, "args");
            long long a0 = 0, a1 = 0;
            if (!cal) {
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "legacy_call_missing_structured_callee",
                  NULL);
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            if (ctype && (!strcmp(ctype, "Global") || !strcmp(ctype, "Extern"))) {
              mname = read_str(cal, "name");
            } else if (ctype && !strcmp(ctype, "Method")) {
              mname = read_str(cal, "method")
                  ? read_str(cal, "method")
                  : read_str(cal, "name");
            } else if (ctype && !strcmp(ctype, "Constructor")) {
              mname = read_str(cal, "name");
            }
            if (args && yyjson_is_arr(args)) {
              if (yyjson_arr_size(args) >= 1) {
                a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
              }
              if (yyjson_arr_size(args) >= 2) {
                a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
              }
            }
            if (emit_mir_call_dispatch(
                    f, blocks, bid, ii, ilen, insts, ctype, bname, mname,
                    dst, recv, args, a0, a1, skip_inst) < 0) {
              if (ctype && !strcmp(ctype, "Global")) {
                if (!note_global_call_lowering_plan_unsupported_shape(bid, ii, op)) {
                  note_pure_unsupported_shape_detail(
                      bid,
                      ii,
                      op,
                      "missing_multi_function_emitter",
                      "?",
                      "structured_call_global_user_target",
                      mname,
                      "same_module_global_call_requires_uniform_emitter");
                }
              } else {
                note_pure_unsupported_shape_route_detail(
                    bid,
                    ii,
                    op,
                    "structured_call_no_route",
                    NULL,
                    ctype,
                    mname,
                    NULL,
                    mname,
                    bname,
                    NULL,
                    NULL,
                    "check_callee_route_or_receiver_origin");
              }
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            continue;
          }
          if (strcmp(op, "mir_call")==0) {
            yyjson_val* mc = yyjson_obj_get(ins, "mir_call");
            yyjson_val* cal = mc ? yyjson_obj_get(mc, "callee") : NULL;
            const char* ctype = cal ? read_str(cal, "type") : NULL;
            const char* bname = cal ? (read_str(cal, "box_name") ? read_str(cal, "box_name") : read_str(cal, "box_type")) : NULL;
            const char* mname = cal ? (read_str(cal, "method") ? read_str(cal, "method") : read_str(cal, "name")) : NULL;
            long long dst = read_int(ins, "dst");
            long long recv = read_int(cal, "receiver");
            yyjson_val* args = mc ? yyjson_obj_get(mc, "args") : NULL;
            long long a0 = 0, a1 = 0;
            if (args && yyjson_is_arr(args)) {
              if (yyjson_arr_size(args) >= 1) a0 = (long long)yyjson_get_sint(yyjson_arr_get(args, 0));
              if (yyjson_arr_size(args) >= 2) a1 = (long long)yyjson_get_sint(yyjson_arr_get(args, 1));
            }
            if (emit_mir_call_dispatch(
                    f, blocks, bid, ii, ilen, insts, ctype, bname, mname,
                    dst, recv, args, a0, a1, skip_inst) < 0) {
              if (!note_global_call_lowering_plan_unsupported_shape(bid, ii, op)) {
                note_pure_unsupported_shape_route_detail(
                    bid,
                    ii,
                    op,
                    "mir_call_no_route",
                    NULL,
                    ctype,
                    NULL,
                    NULL,
                    mname,
                    bname,
                    NULL,
                    NULL,
                    "check_callee_route_or_receiver_origin");
              }
              yyjson_doc_free(d);
              goto GEN_ABORT;
            }
            continue;
          }
          // any other op unsupported
          note_pure_unsupported_shape(bid, ii, op, "unknown_op", NULL);
          yyjson_doc_free(d); goto GEN_ABORT;
