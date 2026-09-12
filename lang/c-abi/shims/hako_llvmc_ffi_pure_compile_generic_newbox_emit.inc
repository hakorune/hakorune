/* Allocation emission, lexically owned by the existing active instruction walk. */
          if (strcmp(op, "newbox")==0) {
            long long dst = read_int(ins, "dst");
            int intrinsic = hako_llvmc_published_intrinsic_array_take_v1(
                &invocation->published_call_rows,
                fn0 ? read_str(fn0, "name") : NULL, bid, ii, ins);
            if (intrinsic < 0) {
              note_pure_unsupported_shape(bid, ii, op, "intrinsic_array_contract", NULL);
              goto GEN_ABORT;
            }
            if (intrinsic > 0) {
              if (static_v2_original_demanded(ins) && hako_llvmc_direct_array_i64_exact_enabled()) {
                note_pure_unsupported_shape(bid, ii, op, "static_v2_non_host_handle", NULL);
                goto GEN_ABORT;
              }
              const char* symbol = hako_llvmc_direct_array_i64_exact_enabled()
                  ? "nyash.array.direct_i64.birth_h" : "nyash.array.birth_h";
              EMIT("  %%r%lld = call i64 @\"%s\"()\n", dst, symbol);
              set_type(dst, T_I64);
              set_origin(dst, ORG_ARRAY_BIRTH);
              set_scan_origin(dst, ORG_ARRAY_BIRTH);
              if (!static_v2_consume_original(ins)) goto GEN_ABORT;
              continue;
            }

            const char* tname = read_str(ins, "type");
            struct GenericPureTypedObjectNewPlanView typed_new_plan;
            yyjson_val* nb_args = yyjson_obj_get(ins, "args");
            yyjson_val* nb0 = nb_args && yyjson_is_arr(nb_args)
                ? yyjson_arr_get_first(nb_args) : NULL;
            long long arg0 = nb0 ? (long long)yyjson_get_sint(nb0) : 0;
            const struct NamedAllocationOutcome* outcome = named_allocation_outcomes_find(
                named_outcomes, fn0, ins, NAMED_ALLOCATION_GENERIC);
            if (!outcome) {
              note_pure_unsupported_shape(bid, ii, op,
                  named_outcomes->storage_failed ? "named_outcome_storage_failed"
                                                  : "named_outcome_missing", tname);
              goto GEN_ABORT;
            }
            enum NamedAllocationConsumer consumer = outcome->consumer;
            if (static_v2_original_demanded(ins) &&
                (consumer == NAMED_ALLOCATION_DIRECT_ARRAY || consumer == NAMED_ALLOCATION_TYPED_OBJECT)) {
              note_pure_unsupported_shape(bid, ii, op, "static_v2_non_host_handle", NULL);
              goto GEN_ABORT;
            }
            if (consumer == NAMED_ALLOCATION_TYPED_OBJECT) typed_new_plan = outcome->plan;
            if (consumer == NAMED_ALLOCATION_DIRECT_ARRAY) {
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.array.direct_i64.birth_h\"()\n", dst);
                set_type(dst, T_I64);
                set_origin(dst, ORG_ARRAY_BIRTH);
                set_scan_origin(dst, ORG_ARRAY_BIRTH);
              } else {
                EMIT("  call i64 @\"nyash.array.direct_i64.birth_h\"()\n");
              }
              if (!static_v2_consume_original(ins)) goto GEN_ABORT;
              continue;
            }
            if (consumer == NAMED_ALLOCATION_ARRAY) {
              const char* birth_symbol = "nyash.array.birth_h";
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"%s\"()\n", dst, birth_symbol);
                set_type(dst, T_I64);
                set_origin(dst, ORG_ARRAY_BIRTH);
                set_scan_origin(dst, ORG_ARRAY_BIRTH);
              } else {
                EMIT("  call i64 @\"%s\"()\n", birth_symbol);
              }
              if (!static_v2_consume_original(ins)) goto GEN_ABORT;
              continue;
            }
            if (consumer == NAMED_ALLOCATION_MAP) {
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.map.birth_h\"()\n", dst);
                set_type(dst, T_I64);
                set_origin(dst, ORG_MAP_BIRTH);
                set_scan_origin(dst, ORG_MAP_BIRTH);
              } else {
                EMIT("  call i64 @\"nyash.map.birth_h\"()\n");
              }
              if (!static_v2_consume_original(ins)) goto GEN_ABORT;
              continue;
            }
            if (consumer == NAMED_ALLOCATION_FILE) {
              if (dst) {
                EMIT("  %%r%lld = call i64 @\"nyash.env.box.new\"(ptr getelementptr inbounds ([8 x i8], ptr @.hako_type_FileBox, i64 0, i64 0))\n", dst);
                set_type(dst, T_I64);
                set_origin(dst, ORG_FILEBOX);
                set_scan_origin(dst, ORG_FILEBOX);
              } else {
                EMIT("  call i64 @\"nyash.env.box.new\"(ptr getelementptr inbounds ([8 x i8], ptr @.hako_type_FileBox, i64 0, i64 0))\n");
              }
              if (!static_v2_consume_original(ins)) goto GEN_ABORT;
              continue;
            }
            if (consumer == NAMED_ALLOCATION_ALIAS_OPERAND_ZERO) {
              int vi = hako_static_v2_value_find(&invocation->static_v2, fn0, (uint32_t)dst);
              if (vi >= 0 && invocation->static_v2.frame->values[vi].action == HAKO_LLVMC_MAP_ACTION_ORIGINAL_VALUE) {
                note_pure_unsupported_shape(bid, ii, op, "static_v2_alias_is_not_allocation", NULL);
                goto GEN_ABORT;
              }
              if (should_skip_hoisted_string_const_box(arg0) &&
                  !string_const_box_was_emitted(arg0) &&
                  !emit_materialized_string_const_box_result(f, arg0)) {
                note_pure_unsupported_shape(
                    bid,
                    ii,
                    op,
                    "string_const_materialization_failed",
                    NULL);
                goto GEN_ABORT;
              }
              set_copy_alias(dst, resolve_copy_source_reg(arg0));
              set_type(dst, T_I64);
              set_origin(dst, ORG_STRING);
              set_scan_origin(dst, ORG_STRING);
              if (!static_v2_consume_original(ins)) goto GEN_ABORT;
              continue;
            }
            if (consumer == NAMED_ALLOCATION_TYPED_OBJECT) {
              EMIT("  %%r%lld = call i64 @\"nyash.object.new_typed_hi\"(i64 %lld, i64 %lld)\n",
                   dst,
                   typed_new_plan.type_id,
                   typed_new_plan.field_count);
              generic_pure_publish_typed_object_plan_index(
                  dst,
                  typed_new_plan.typed_plan_index);
              if (!static_v2_consume_original(ins)) goto GEN_ABORT;
              continue;
            }
            if (consumer == NAMED_ALLOCATION_INVALID_PLAN) {
              note_pure_unsupported_shape(
                  bid,
                  ii,
                  op,
                  "typed_object_plan_invalid_new",
                  NULL);
              goto GEN_ABORT;
            }
            note_pure_unsupported_shape(bid, ii, op, "unsupported_newbox_type", NULL);
            goto GEN_ABORT;
          }
